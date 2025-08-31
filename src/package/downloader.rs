use crate::result::{Result, OpenCliError};
use crate::package::version::{Version, VersionConstraint};
use octocrab::Octocrab;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::fs::{File, create_dir_all};
use tokio::io::AsyncWriteExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use once_cell::sync::Lazy;
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive;
use std::io::Read;

static GITHUB_REPO_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([^/]+)/([^/]+)$").unwrap()
});

static INCLUDE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\.inc$").unwrap()
});

static BINARY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\.(dll|so|dylib)$").unwrap()
});

static AMX_LIB_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[Aa][Mm][Xx]|[Ll][Ii][Bb]").unwrap()
});

pub struct PackageDownloader {
    github: std::sync::Arc<Octocrab>,
    client: Client,
}

#[derive(Debug, Clone)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Clone)]
pub struct GitHubAsset {
    pub name: String,
    pub download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct PackageFiles {
    pub includes: Vec<PathBuf>,
    pub binaries: Vec<PathBuf>,
    pub root_binaries: Vec<PathBuf>,
    pub component_binaries: Vec<PathBuf>,
    pub plugin_binaries: Vec<PathBuf>,
}

impl PackageDownloader {
    pub fn new() -> Self {
        let client = Client::new();
        
        let github = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            if !token.is_empty() {
                let crab = Octocrab::builder().personal_token(token).build().unwrap_or_else(|_| Octocrab::default());
                octocrab::initialise(crab);
                octocrab::instance()
            } else {
                octocrab::instance()
            }
        } else {
            octocrab::instance()
        };
        
        Self { github, client }
    }
    
    pub async fn get_releases(&self, repo: &str) -> Result<Vec<GitHubRelease>> {
        let (owner, repo_name) = self.parse_repo(repo)?;
        
        let releases = self.github
            .repos(owner, repo_name)
            .releases()
            .list()
            .send()
            .await
            .map_err(|e| OpenCliError::Process(format!("Failed to fetch releases: {}", e).into()))?;
        
        let mut github_releases = Vec::new();
        
        for release in releases.items {
            let assets = release.assets.into_iter()
                .map(|asset| GitHubAsset {
                    name: asset.name,
                    download_url: asset.browser_download_url.to_string(),
                    size: asset.size as u64,
                })
                .collect();
            
            github_releases.push(GitHubRelease {
                tag_name: release.tag_name,
                assets,
            });
        }
        
        Ok(github_releases)
    }
    
    pub async fn find_matching_version(&self, repo: &str, constraint: &VersionConstraint) -> Result<GitHubRelease> {
        let releases = self.get_releases(repo).await?;
        
        let versions: Vec<(Version, &GitHubRelease)> = releases.iter()
            .filter_map(|release| {
                Version::parse(&release.tag_name).ok().map(|v| (v, release))
            })
            .collect();
        
        let version_refs: Vec<&Version> = versions.iter().map(|(v, _)| v).collect();
        if let Some(matched_version) = constraint.latest_matching(&version_refs.iter().map(|&v| v.clone()).collect::<Vec<_>>()) {
            if let Some((_, release)) = versions.iter().find(|(ver, _)| ver == matched_version) {
                Ok((*release).clone())
            } else {
                Err(OpenCliError::NotFound(format!("No matching version found for constraint").into()))
            }
        } else {
            Err(OpenCliError::NotFound(format!("No matching version found for constraint").into()))
        }
    }
    
    pub async fn download_package(&self, repo: &str, release: &GitHubRelease, temp_dir: &Path) -> Result<PackageFiles> {
        create_dir_all(temp_dir).await?;
        
        let mut package_files = PackageFiles {
            includes: Vec::new(),
            binaries: Vec::new(),
            root_binaries: Vec::new(),
            component_binaries: Vec::new(),
            plugin_binaries: Vec::new(),
        };
        
        for asset in &release.assets {
            let asset_path = temp_dir.join(&asset.name);
            self.download_asset(asset, &asset_path).await?;
            
            if self.is_archive(&asset.name) {
                let extracted = self.extract_archive(&asset_path, temp_dir).await?;
                package_files.includes.extend(extracted.includes);
                package_files.binaries.extend(extracted.binaries);
                package_files.root_binaries.extend(extracted.root_binaries);
                package_files.component_binaries.extend(extracted.component_binaries);
                package_files.plugin_binaries.extend(extracted.plugin_binaries);
            } else if INCLUDE_REGEX.is_match(&asset.name) {
                package_files.includes.push(asset_path);
            } else if BINARY_REGEX.is_match(&asset.name) {
                self.categorize_binary(&asset_path, &mut package_files);
            }
        }
        
        if package_files.includes.is_empty() && 
           package_files.binaries.is_empty() && 
           package_files.root_binaries.is_empty() && 
           package_files.component_binaries.is_empty() && 
           package_files.plugin_binaries.is_empty() {
            self.download_repo_content(repo, &release.tag_name, temp_dir, &mut package_files).await?;
        }
        
        Ok(package_files)
    }
    
    async fn download_asset(&self, asset: &GitHubAsset, output_path: &Path) -> Result<()> {
        let response = self.client
            .get(&asset.download_url)
            .header("User-Agent", "opencli/0.1.0")
            .send()
            .await
            .map_err(|e| OpenCliError::Process(format!("Download failed: {}", e).into()))?;
        
        if !response.status().is_success() {
            return Err(OpenCliError::Process(format!("Download failed: HTTP {}", response.status()).into()));
        }
        
        let pb = ProgressBar::new(asset.size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", asset.name));
        
        let bytes = response.bytes().await
            .map_err(|e| OpenCliError::Process(format!("Download failed: {}", e).into()))?;
        
        pb.set_position(bytes.len() as u64);
        
        let mut file = File::create(output_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;
        
        pb.finish_with_message(format!("Downloaded {}", asset.name));
        Ok(())
    }
    
    async fn extract_archive(&self, archive_path: &Path, extract_dir: &Path) -> Result<PackageFiles> {
        let file_name = archive_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if file_name.ends_with(".zip") {
            self.extract_zip(archive_path, extract_dir).await
        } else if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
            self.extract_tar_gz(archive_path, extract_dir).await
        } else {
            Ok(PackageFiles {
                includes: Vec::new(),
                binaries: Vec::new(),
                root_binaries: Vec::new(),
                component_binaries: Vec::new(),
                plugin_binaries: Vec::new(),
            })
        }
    }
    
    async fn extract_zip(&self, zip_path: &Path, extract_dir: &Path) -> Result<PackageFiles> {
        let file = std::fs::File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| OpenCliError::Process(format!("Invalid ZIP archive: {}", e).into()))?;
        
        let mut package_files = PackageFiles {
            includes: Vec::new(),
            binaries: Vec::new(),
            root_binaries: Vec::new(),
            component_binaries: Vec::new(),
            plugin_binaries: Vec::new(),
        };
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| OpenCliError::Process(format!("ZIP extraction error: {}", e).into()))?;
            
            let file_path = extract_dir.join(file.name());
            
            if file.is_dir() {
                create_dir_all(&file_path).await?;
            } else {
                if let Some(parent) = file_path.parent() {
                    create_dir_all(parent).await?;
                }
                
                let mut output = File::create(&file_path).await?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| OpenCliError::Process(format!("ZIP read error: {}", e).into()))?;
                output.write_all(&buffer).await?;
                
                if INCLUDE_REGEX.is_match(file.name()) {
                    package_files.includes.push(file_path);
                } else if BINARY_REGEX.is_match(file.name()) {
                    self.categorize_binary_by_path(&file_path, file.name(), &mut package_files);
                }
            }
        }
        
        Ok(package_files)
    }
    
    async fn extract_tar_gz(&self, tar_path: &Path, extract_dir: &Path) -> Result<PackageFiles> {
        let file = std::fs::File::open(tar_path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        
        let mut package_files = PackageFiles {
            includes: Vec::new(),
            binaries: Vec::new(),
            root_binaries: Vec::new(),
            component_binaries: Vec::new(),
            plugin_binaries: Vec::new(),
        };
        
        for entry in archive.entries()
            .map_err(|e| OpenCliError::Process(format!("TAR extraction error: {}", e).into()))? {
            let mut entry = entry
                .map_err(|e| OpenCliError::Process(format!("TAR entry error: {}", e).into()))?;
            
            let file_path = extract_dir.join(entry.path()
                .map_err(|e| OpenCliError::Process(format!("TAR path error: {}", e).into()))?);
            
            if entry.header().entry_type().is_file() {
                if let Some(parent) = file_path.parent() {
                    create_dir_all(parent).await?;
                }
                
                let mut output = File::create(&file_path).await?;
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer)
                    .map_err(|e| OpenCliError::Process(format!("TAR read error: {}", e).into()))?;
                output.write_all(&buffer).await?;
                
                if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                    if INCLUDE_REGEX.is_match(file_name) {
                        package_files.includes.push(file_path);
                    } else if BINARY_REGEX.is_match(file_name) {
                        let entry_path = entry.path()
                            .map_err(|e| OpenCliError::Process(format!("TAR path error: {}", e).into()))?;
                        self.categorize_binary_by_path(&file_path, entry_path.to_str().unwrap_or(""), &mut package_files);
                    }
                }
            }
        }
        
        Ok(package_files)
    }
    
    async fn download_repo_content(&self, repo: &str, tag: &str, temp_dir: &Path, package_files: &mut PackageFiles) -> Result<()> {
        let (owner, repo_name) = self.parse_repo(repo)?;
        
        let repo_obj = self.github.repos(owner, repo_name);
        let contents = repo_obj.get_content()
            .r#ref(tag)
            .send()
            .await
            .map_err(|e| OpenCliError::Process(format!("Failed to get repo content: {}", e).into()))?;
        
        for item in contents.items {
            if item.r#type == "file" {
                let name = item.name;
                if INCLUDE_REGEX.is_match(&name) {
                    let file_path = temp_dir.join(&name);
                    
                    if let Some(download_url) = &item.download_url {
                        let response = self.client.get(download_url).send().await
                            .map_err(|e| OpenCliError::Process(format!("Failed to download file: {}", e).into()))?;
                        
                        let content = response.bytes().await
                            .map_err(|e| OpenCliError::Process(format!("Failed to read file: {}", e).into()))?;
                        
                        let mut file = File::create(&file_path).await?;
                        file.write_all(&content).await?;
                        
                        package_files.includes.push(file_path);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_repo<'a>(&self, repo: &'a str) -> Result<(&'a str, &'a str)> {
        if let Some(caps) = GITHUB_REPO_REGEX.captures(repo) {
            Ok((caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
        } else {
            Err(OpenCliError::Config(format!("Invalid GitHub repository format: {}", repo).into()))
        }
    }
    
    fn categorize_binary(&self, file_path: &Path, package_files: &mut PackageFiles) {
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            if AMX_LIB_REGEX.is_match(file_name) {
                package_files.root_binaries.push(file_path.to_path_buf());
            } else {
                package_files.binaries.push(file_path.to_path_buf());
            }
        }
    }
    
    fn categorize_binary_by_path(&self, file_path: &Path, archive_path: &str, package_files: &mut PackageFiles) {
        let archive_path_lower = archive_path.to_lowercase();
        
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            if AMX_LIB_REGEX.is_match(file_name) {
                package_files.root_binaries.push(file_path.to_path_buf());
            } else if archive_path_lower.contains("/components/") || archive_path_lower.contains("\\components\\") {
                package_files.component_binaries.push(file_path.to_path_buf());
            } else if archive_path_lower.contains("/plugins/") || archive_path_lower.contains("\\plugins\\") || 
                      archive_path_lower.contains("/plugin/") || archive_path_lower.contains("\\plugin\\") {
                package_files.plugin_binaries.push(file_path.to_path_buf());
            } else {
                package_files.binaries.push(file_path.to_path_buf());
            }
        }
    }
    
    fn is_archive(&self, filename: &str) -> bool {
        filename.ends_with(".zip") || filename.ends_with(".tar.gz") || filename.ends_with(".tgz") || filename.ends_with(".rar")
    }
}
