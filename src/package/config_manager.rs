use crate::result::Result;
use crate::package::PackageLock;
use crate::build::PackageTarget;
use serde_json::{Value, Map};
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct ConfigManager {
    workspace_root: PathBuf,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(workspace_root: &Path) -> Self {
        let config_path = workspace_root.join("config.json");
        Self {
            workspace_root: workspace_root.to_path_buf(),
            config_path,
        }
    }

    pub async fn update_legacy_plugins(&self, lock_path: &Path) -> Result<()> {
        let lock = PackageLock::load_from_file(lock_path).await?;
        let legacy_plugins = self.extract_legacy_plugin_names(&lock);
        
        if !legacy_plugins.is_empty() {
            self.update_config_json(&legacy_plugins).await?;
        }
        
        Ok(())
    }

    fn extract_legacy_plugin_names(&self, lock: &PackageLock) -> Vec<String> {
        let mut plugin_names = Vec::new();
        
        for (_repo, package) in &lock.installed {
            if let Some(PackageTarget::Plugins) = &package.target {
                for file_path in &package.files {
                    if let Some(binary_name) = self.extract_binary_name_from_path(file_path) {
                        plugin_names.push(binary_name);
                    }
                }
            }
        }
        
        plugin_names.sort();
        plugin_names.dedup();
        plugin_names
    }

    fn extract_binary_name_from_path(&self, file_path: &str) -> Option<String> {
        let path = std::path::Path::new(file_path);
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.ends_with(".dll") || file_name.ends_with(".so") {
                if path.to_string_lossy().contains("plugins") {
                    return Some(file_name.trim_end_matches(".dll").trim_end_matches(".so").to_string());
                }
            }
        }
        None
    }

    async fn update_config_json(&self, legacy_plugins: &[String]) -> Result<()> {
        let config_content = if self.config_path.exists() {
            fs::read_to_string(&self.config_path).await?
        } else {
            "{}".to_string()
        };

        let mut config: Value = serde_json::from_str(&config_content)
            .unwrap_or_else(|_| serde_json::Value::Object(Map::new()));

        if let Value::Object(ref mut map) = config {
            if let Some(existing) = map.get("legacy_plugins") {
                if let Some(existing_array) = existing.as_array() {
                    let updated = self.merge_plugin_arrays(existing_array, legacy_plugins);
                    map.insert("legacy_plugins".to_string(), Value::Array(updated));
                    println!("Updated config.json");
                } else if let Some(existing_str) = existing.as_str() {
                    let existing_plugins = self.string_to_plugin_array(existing_str);
                    let updated = self.merge_plugin_arrays(&existing_plugins, legacy_plugins);
                    map.insert("legacy_plugins".to_string(), Value::Array(updated));
                    println!("Updated config.json");
                } else {
                    let plugins_array = self.strings_to_json_array(legacy_plugins);
                    map.insert("legacy_plugins".to_string(), Value::Array(plugins_array));
                    println!("Updated config.json");
                }
            } else {
                let plugins_array = self.strings_to_json_array(legacy_plugins);
                map.insert("legacy_plugins".to_string(), Value::Array(plugins_array));
                println!("Updated config.json");
            }
        }

        let formatted_json = serde_json::to_string_pretty(&config)?;
        fs::write(&self.config_path, formatted_json).await?;
        
        log::info!("Updated config.json with legacy plugins: {}", legacy_plugins.join(", "));
        
        Ok(())
    }

    fn strings_to_json_array(&self, plugins: &[String]) -> Vec<Value> {
        plugins.iter()
            .map(|p| Value::String(p.clone()))
            .collect()
    }

    fn string_to_plugin_array(&self, existing: &str) -> Vec<Value> {
        let plugins: Vec<String> = if existing.contains(',') {
            existing.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
        } else {
            existing.split_whitespace().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
        };
        
        self.strings_to_json_array(&plugins)
    }

    fn merge_plugin_arrays(&self, existing_array: &[Value], new_plugins: &[String]) -> Vec<Value> {
        let mut existing_plugins: Vec<String> = existing_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        for plugin in new_plugins {
            if !existing_plugins.contains(plugin) {
                existing_plugins.push(plugin.clone());
            }
        }

        existing_plugins.sort();
        existing_plugins.dedup();
        
        self.strings_to_json_array(&existing_plugins)
    }

    pub async fn remove_legacy_plugin(&self, repo: &str, lock_path: &Path) -> Result<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let lock = PackageLock::load_from_file(lock_path).await?;
        let mut plugins_to_remove = Vec::new();
        
        if let Some(package) = lock.installed.get(repo) {
            if let Some(PackageTarget::Plugins) = &package.target {
                for file_path in &package.files {
                    if let Some(binary_name) = self.extract_binary_name_from_path(file_path) {
                        plugins_to_remove.push(binary_name);
                    }
                }
            }
        }

        if plugins_to_remove.is_empty() {
            return Ok(());
        }

        let config_content = fs::read_to_string(&self.config_path).await?;
        let mut config: Value = serde_json::from_str(&config_content)
            .unwrap_or_else(|_| serde_json::Value::Object(Map::new()));

        if let Value::Object(ref mut map) = config {
            if let Some(existing) = map.get("legacy_plugins") {
                if let Some(existing_array) = existing.as_array() {
                    let updated = self.remove_plugins_from_array(existing_array, &plugins_to_remove);
                    if updated.is_empty() {
                        map.remove("legacy_plugins");
                    } else {
                        map.insert("legacy_plugins".to_string(), Value::Array(updated));
                    }
                    println!("Updated config.json");
                } else if let Some(existing_str) = existing.as_str() {
                    let existing_plugins = self.string_to_plugin_array(existing_str);
                    let updated = self.remove_plugins_from_array(&existing_plugins, &plugins_to_remove);
                    if updated.is_empty() {
                        map.remove("legacy_plugins");
                    } else {
                        map.insert("legacy_plugins".to_string(), Value::Array(updated));
                    }
                    println!("Updated config.json");
                }
            }
        }

        let formatted_json = serde_json::to_string_pretty(&config)?;
        fs::write(&self.config_path, formatted_json).await?;
        
        log::info!("Removed legacy plugins from config.json: {}", plugins_to_remove.join(", "));
        
        Ok(())
    }

    fn remove_plugins_from_array(&self, existing_array: &[Value], plugins_to_remove: &[String]) -> Vec<Value> {
        let filtered_plugins: Vec<String> = existing_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .filter(|plugin| !plugins_to_remove.contains(plugin))
            .collect();

        self.strings_to_json_array(&filtered_plugins)
    }
}
