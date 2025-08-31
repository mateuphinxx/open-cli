pub mod parser;

use crate::commands::CommandExecutor;
use crate::result::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "opencli")]
#[command(about = "CLI tool for open.mp server management")]
#[command(version = "0.1.0")]
#[command(author = "mateuphinxx")]
#[command(arg_required_else_help = true)]
#[command(help_template = "{before-help}{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    #[command(about = "Run open.mp server")]
    Run {
        #[arg(long, help = "Custom path to omp-server executable")]
        server_path: Option<String>,
    },
    
    #[command(about = "Build open.mp project")]
    Build {
        #[arg(short, long, help = "Build configuration file")]
        config: Option<String>,
        
        #[arg(short, long, help = "Enable verbose output")]
        verbose: bool,
        
        #[arg(long, help = "Force compiler redownload")]
        force_download: bool,
        
        #[arg(long, help = "Update compiler configuration from remote")]
        update_config: bool,
    },
    
    #[command(about = "Setup project with default opencli.toml")]
    Setup {
        #[arg(long, help = "Force overwrite existing opencli.toml")]
        force: bool,
    },
    
    #[command(about = "Install components")]
    Install {
        #[command(subcommand)]
        component: InstallComponent,
    },
}

#[derive(Parser)]
pub enum InstallComponent {
    #[command(about = "Install Pawn compiler")]
    Compiler {
        #[arg(long, help = "Compiler version to install (default: v3.10.11)")]
        version: Option<String>,
        
        #[arg(long, help = "Force reinstall even if already exists")]
        force: bool,
    },
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
    
    pub async fn execute(self) -> Result<()> {
        let mut executor = CommandExecutor::new();
        
        match self.command {
            Commands::Run { server_path } => {
                executor.run_server(server_path).await
            },
            Commands::Build { config, verbose, force_download, update_config } => {
                executor.build_project(config, verbose, force_download, update_config).await
            },
            Commands::Setup { force } => {
                executor.setup_project(force).await
            },
            Commands::Install { component } => {
                match component {
                    InstallComponent::Compiler { version, force } => {
                        executor.install_compiler(version, force).await
                    }
                }
            },
        }
    }
}
