pub mod run;
pub mod build;
pub mod setup;
pub mod install;

use crate::result::Result;
use smol_str::SmolStr;

#[derive(Debug)]
pub enum CommandType {
    Run { 
        server_path: Option<SmolStr> 
    },
    Build { 
        config: Option<SmolStr>, 
        verbose: bool, 
        force_download: bool, 
        update_config: bool 
    },
    Setup { 
        force: bool 
    },
    InstallCompiler { 
        version: Option<SmolStr>, 
        force: bool 
    },
}

impl CommandType {
    pub async fn execute(self) -> Result<()> {
        match self {
            CommandType::Run { server_path } => {
                run::execute(server_path.as_deref()).await
            }
            CommandType::Build { config, verbose, force_download, update_config } => {
                build::execute(
                    config.as_deref(), 
                    verbose, 
                    force_download, 
                    update_config
                ).await
            }
            CommandType::Setup { force } => {
                setup::execute(force).await
            }
            CommandType::InstallCompiler { version, force } => {
                install::execute_compiler(
                    version.as_deref(), 
                    force
                ).await
            }
        }
    }
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn run_server(&mut self, server_path: Option<String>) -> Result<()> {
        CommandType::Run { 
            server_path: server_path.map(|s| s.into()) 
        }.execute().await
    }
    
    pub async fn build_project(&mut self, config: Option<String>, verbose: bool, force_download: bool, update_config: bool) -> Result<()> {
        CommandType::Build { 
            config: config.map(|s| s.into()), 
            verbose, 
            force_download, 
            update_config 
        }.execute().await
    }
    
    pub async fn setup_project(&mut self, force: bool) -> Result<()> {
        CommandType::Setup { force }.execute().await
    }
    
    pub async fn install_compiler(&mut self, version: Option<String>, force: bool) -> Result<()> {
        CommandType::InstallCompiler { 
            version: version.map(|s| s.into()), 
            force 
        }.execute().await
    }
}
