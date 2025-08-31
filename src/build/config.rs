use serde::{Deserialize, Serialize};
use crate::result::{Result, OpenCliError};
use tokio::fs;
use std::path::PathBuf;
// use smol_str::SmolStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub build: Build,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub entry_file: PathBuf,
    pub output_file: PathBuf,
    pub compiler_version: String,
    pub includes: Option<BuildIncludes>,
    pub args: Option<BuildArgs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildIncludes {
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArgs {
    pub args: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            build: Build {
                entry_file: "gamemode.pwn".into(),
                output_file: "gamemode.amx".into(),
                compiler_version: "v3.10.11".to_string(),
                includes: Some(BuildIncludes {
                    paths: vec![
                        "include".into(),
                        "qawno/include".into(),
                    ],
                }),
                args: Some(BuildArgs {
                    args: vec![
                        "-d3".to_string(),
                        "-;+".to_string(),
                        "-(+".to_string(),
                        "-\\+".to_string(),
                        "-Z+".to_string(),
                        "-O2".to_string(),
                    ],
                }),
            },
        }
    }
}

impl BuildConfig {
    pub async fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        let config: BuildConfig = toml::from_str(&content)
            .map_err(|e| OpenCliError::Config(format!("Invalid build config format: {}", e).into()))?;
        
        Ok(config)
    }
    
    pub async fn save_to_file(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| OpenCliError::Config(format!("Failed to serialize build config: {}", e).into()))?;
        
        fs::write(path, content).await?;
        Ok(())
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.build.entry_file.as_os_str().is_empty() {
            return Err(OpenCliError::Config("Entry file cannot be empty".into()));
        }
        
        if self.build.output_file.as_os_str().is_empty() {
            return Err(OpenCliError::Config("Output file cannot be empty".into()));
        }
        
        if self.build.compiler_version.is_empty() {
            return Err(OpenCliError::Config("Compiler version cannot be empty".into()));
        }
        
        Ok(())
    }
}
