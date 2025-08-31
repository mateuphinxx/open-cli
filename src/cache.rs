use crate::result::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use std::collections::HashMap;

pub struct CacheManager {
    cache_file: PathBuf,
}

impl CacheManager {
    pub fn new(base_dir: &Path) -> Self {
        Self {
            cache_file: base_dir.join("cache.txt"),
        }
    }
    
    pub async fn store_hash(&self, filename: &str, argon2_hash: &str) -> Result<()> {
        let entry = format!("{}\nargon2:{}\n", filename, argon2_hash);
        
        if let Some(parent) = self.cache_file.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let existing_content = if self.cache_file.exists() {
            fs::read_to_string(&self.cache_file).await.unwrap_or_default()
        } else {
            String::new()
        };
        
        let updated_content = format!("{}{}", existing_content, entry);
        fs::write(&self.cache_file, updated_content).await?;
        
        Ok(())
    }
    
    pub async fn get_hash(&self, filename: &str) -> Result<Option<String>> {
        if !self.cache_file.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&self.cache_file).await?;
        let mut current_file = None;
        
        for line in content.lines() {
            if line.starts_with("argon2:") {
                if let Some(file) = &current_file {
                    if file == filename {
                        return Ok(Some(line.strip_prefix("argon2:").unwrap().to_string()));
                    }
                }
            } else if !line.is_empty() {
                current_file = Some(line.to_string());
            }
        }
        
        Ok(None)
    }
    
    pub async fn load_all_hashes(&self) -> Result<HashMap<String, String>> {
        let mut hashes = HashMap::new();
        
        if !self.cache_file.exists() {
            return Ok(hashes);
        }
        
        let content = fs::read_to_string(&self.cache_file).await?;
        let mut current_file = None;
        
        for line in content.lines() {
            if line.starts_with("argon2:") {
                if let Some(file) = current_file.take() {
                    let hash = line.strip_prefix("argon2:").unwrap().to_string();
                    hashes.insert(file, hash);
                }
            } else if !line.is_empty() {
                current_file = Some(line.to_string());
            }
        }
        
        Ok(hashes)
    }
    
    pub async fn remove_hash(&self, filename: &str) -> Result<()> {
        if !self.cache_file.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.cache_file).await?;
        let mut new_content = String::new();
        let mut current_file = None;
        let mut skip_next = false;
        
        for line in content.lines() {
            if skip_next {
                skip_next = false;
                continue;
            }
            
            if line.starts_with("argon2:") {
                if let Some(file) = &current_file {
                    if file != filename {
                        new_content.push_str(&format!("{}\n{}\n", file, line));
                    }
                }
                current_file = None;
            } else if !line.is_empty() {
                if line == filename {
                    skip_next = true;
                    current_file = None;
                } else {
                    current_file = Some(line.to_string());
                }
            }
        }
        
        fs::write(&self.cache_file, new_content).await?;
        Ok(())
    }
}
