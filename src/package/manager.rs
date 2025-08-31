use crate::result::{Result, OpenCliError};
use crate::build::{BuildConfig, PackageSpec, PackageTarget};
use crate::package::{PackageDownloader, VersionConstraint, WorkspaceDetector};
use crate::security::SecurityManager;
use std::path::{Path, PathBuf};
use tokio::fs;
use indicatif::{ProgressBar, ProgressStyle};

pub struct PackageManager {
    downloader: PackageDownloader,
    workspace: WorkspaceDetector,
    #[allow(dead_code)]
    security: SecurityManager,
    config_path: PathBuf,
}

impl PackageManager {
    pub fn new<P: AsRef<Path>>(workspace_root: P, config_path: P) -> Self {
        Self {
            downloader: PackageDownloader::new(),
            workspace: WorkspaceDetector::new(&workspace_root),
            security: SecurityManager::new(),
            config_path: config_path.as_ref().to_path_buf(),
        }
    }
    
    pub async fn install_package(&mut self, repo: &str, version_spec: Option<&str>, target: Option<PackageTarget>) -> Result<()> {
        let spinner = self.create_spinner("Installing package...");
        
        let constraint = if let Some(spec) = version_spec {
            VersionConstraint::parse(spec)?
        } else {
            VersionConstraint::parse("*")?
        };
        
        spinner.set_message(format!("Finding version for {}", repo));
        let release = self.downloader.find_matching_version(repo, &constraint).await?;
        
        spinner.set_message("Downloading package files...");
        let temp_dir = self.get_temp_dir(repo)?;
        let package_files = self.downloader.download_package(repo, &release, &temp_dir).await?;
        
        spinner.set_message("Installing package files...");
        self.install_package_files(repo, &package_files, target.as_ref()).await?;
        
        spinner.set_message("Updating configuration...");
        self.update_config(repo, &release.tag_name, target).await?;
        
        self.cleanup_temp_dir(&temp_dir).await?;
        
        spinner.finish_with_message(format!("Successfully installed {} {}", repo, release.tag_name));
        log::info!("Package installed: {} {}", repo, release.tag_name);
        
        Ok(())
    }
    
    pub async fn install_all_packages(&mut self) -> Result<()> {
        let config = BuildConfig::from_file(self.config_path.to_string_lossy().as_ref()).await?;
        
        if let Some(packages) = config.get_packages() {
            for (repo, spec) in packages {
                let version = spec.version();
                let target = spec.target().cloned();
                
                println!("Installing package: {} = {}", repo, version);
                if let Err(e) = self.install_package(repo, Some(version), target).await {
                    eprintln!("Failed to install {}: {}", repo, e);
                    log::error!("Package installation failed: {} - {}", repo, e);
                }
            }
        } else {
            println!("No packages defined in configuration");
        }
        
        Ok(())
    }
    
    pub async fn remove_package(&mut self, repo: &str) -> Result<()> {
        let spinner = self.create_spinner(format!("Removing package {}...", repo));
        
        self.remove_package_files(repo).await?;
        self.remove_from_config(repo).await?;
        
        spinner.finish_with_message(format!("Successfully removed {}", repo));
        log::info!("Package removed: {}", repo);
        
        Ok(())
    }
    
    pub async fn list_packages(&self) -> Result<()> {
        let config = BuildConfig::from_file(self.config_path.to_string_lossy().as_ref()).await?;
        
        if let Some(packages) = config.get_packages() {
            println!("Installed packages:");
            for (repo, spec) in packages {
                let target_info = spec.target()
                    .map(|t| format!(" ({})", match t {
                        PackageTarget::Components => "components",
                        PackageTarget::Plugins => "plugins",
                    }))
                    .unwrap_or_default();
                
                println!("  {} = {}{}", repo, spec.version(), target_info);
            }
        } else {
            println!("No packages installed");
        }
        
        Ok(())
    }
    
    pub async fn update_package(&mut self, repo: &str) -> Result<()> {
        let config = BuildConfig::from_file(self.config_path.to_string_lossy().as_ref()).await?;
        
        if let Some(packages) = config.get_packages() {
            if let Some(spec) = packages.get(repo) {
                let _constraint = VersionConstraint::parse(spec.version())?;
                let target = spec.target().cloned();
                
                self.remove_package_files(repo).await?;
                self.install_package(repo, Some(spec.version()), target).await?;
            } else {
                return Err(OpenCliError::NotFound(format!("Package {} not found", repo).into()));
            }
        } else {
            return Err(OpenCliError::NotFound("No packages installed".into()));
        }
        
        Ok(())
    }
    
    async fn install_package_files(&self, _repo: &str, package_files: &crate::package::downloader::PackageFiles, target: Option<&PackageTarget>) -> Result<()> {
        self.workspace.ensure_workspace_structure().await?;
        
        let include_paths = self.get_include_paths().await?;
        let workspace_info = self.workspace.get_workspace_info();
        
        for include_file in &package_files.includes {
            for include_path in &include_paths {
                let dest_path = include_path.join(include_file.file_name().unwrap());
                fs::copy(include_file, &dest_path).await?;
                log::info!("Copied include: {} -> {}", include_file.display(), dest_path.display());
            }
        }
        
        for binary_file in &package_files.root_binaries {
            let dest_path = workspace_info.root.join(binary_file.file_name().unwrap());
            fs::copy(binary_file, &dest_path).await?;
            log::info!("Copied root binary: {} -> {}", binary_file.display(), dest_path.display());
        }
        
        let component_files = if !package_files.component_binaries.is_empty() {
            &package_files.component_binaries
        } else {
            &package_files.binaries
        };
        
        let plugin_files = if !package_files.plugin_binaries.is_empty() {
            &package_files.plugin_binaries
        } else {
            &package_files.binaries
        };
        
        match target {
            Some(PackageTarget::Components) => {
                for binary_file in component_files {
                    let dest_path = workspace_info.components.join(binary_file.file_name().unwrap());
                    fs::copy(binary_file, &dest_path).await?;
                    log::info!("Copied component binary: {} -> {}", binary_file.display(), dest_path.display());
                }
            }
            Some(PackageTarget::Plugins) => {
                for binary_file in plugin_files {
                    let dest_path = workspace_info.plugins.join(binary_file.file_name().unwrap());
                    fs::copy(binary_file, &dest_path).await?;
                    log::info!("Copied plugin binary: {} -> {}", binary_file.display(), dest_path.display());
                }
            }
            None => {
                for binary_file in component_files {
                    let target_folder = self.detect_binary_target(binary_file).await?;
                    let dest_path = target_folder.join(binary_file.file_name().unwrap());
                    fs::copy(binary_file, &dest_path).await?;
                    log::info!("Copied auto-detected binary: {} -> {}", binary_file.display(), dest_path.display());
                }
            }
        }
        
        Ok(())
    }
    
    async fn remove_package_files(&self, repo: &str) -> Result<()> {
        let package_name = repo.split('/').last().unwrap_or(repo);
        
        let include_paths = self.get_include_paths().await?;
        for include_path in include_paths {
            let _pattern = format!("{}*.inc", package_name.to_lowercase());
            if let Ok(entries) = fs::read_dir(&include_path).await {
                let mut entries = entries;
                while let Some(entry) = entries.next_entry().await? {
                    let file_name = entry.file_name();
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.to_lowercase().contains(&package_name.to_lowercase()) && name_str.ends_with(".inc") {
                            fs::remove_file(entry.path()).await?;
                            log::info!("Removed include: {}", entry.path().display());
                        }
                    }
                }
            }
        }
        
        let workspace_info = self.workspace.get_workspace_info();
        for folder in [&workspace_info.components, &workspace_info.plugins] {
            if let Ok(entries) = fs::read_dir(folder).await {
                let mut entries = entries;
                while let Some(entry) = entries.next_entry().await? {
                    let file_name = entry.file_name();
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.to_lowercase().contains(&package_name.to_lowercase()) {
                            fs::remove_file(entry.path()).await?;
                            log::info!("Removed binary: {}", entry.path().display());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn update_config(&self, repo: &str, version: &str, target: Option<PackageTarget>) -> Result<()> {
        let mut config = BuildConfig::from_file(self.config_path.to_string_lossy().as_ref()).await?;
        
        let spec = if let Some(target) = target {
            PackageSpec::new_detailed(version, Some(target))
        } else {
            PackageSpec::new_simple(version)
        };
        
        config.add_package(repo.into(), spec);
        config.save_to_file(self.config_path.to_string_lossy().as_ref()).await?;
        
        Ok(())
    }
    
    async fn remove_from_config(&self, repo: &str) -> Result<()> {
        let mut config = BuildConfig::from_file(self.config_path.to_string_lossy().as_ref()).await?;
        
        if config.remove_package(repo) {
            config.save_to_file(self.config_path.to_string_lossy().as_ref()).await?;
        }
        
        Ok(())
    }
    
    async fn get_include_paths(&self) -> Result<Vec<PathBuf>> {
        let config = BuildConfig::from_file(self.config_path.to_string_lossy().as_ref()).await?;
        Ok(config.get_include_paths())
    }
    
    async fn detect_binary_target(&self, binary_path: &Path) -> Result<PathBuf> {
        if let Some(file_name) = binary_path.file_name().and_then(|n| n.to_str()) {
            let file_name_lower = file_name.to_lowercase();
            
            if file_name_lower.contains("omp") || file_name_lower.contains("component") {
                self.workspace.detect_components_folder().await
            } else {
                self.workspace.detect_plugins_folder().await
            }
        } else {
            self.workspace.detect_plugins_folder().await
        }
    }
    
    fn get_temp_dir(&self, repo: &str) -> Result<PathBuf> {
        let temp_name = repo.replace('/', "_");
        let temp_dir = std::env::temp_dir().join("opencli").join("packages").join(temp_name);
        Ok(temp_dir)
    }
    
    async fn cleanup_temp_dir(&self, temp_dir: &Path) -> Result<()> {
        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir).await?;
        }
        Ok(())
    }
    
    fn create_spinner(&self, message: impl Into<String>) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        spinner.set_message(message.into());
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        spinner
    }
}
