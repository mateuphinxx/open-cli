use opencli::cli::Cli;
use opencli::result::Result;
use clap::Parser;
use std::fs::OpenOptions;
use log::LevelFilter;
use env_logger::Builder;
use dirs::config_dir;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    init_logging().await;
    
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            e.print().expect("Failed to print clap error");
            std::process::exit(e.exit_code());
        }
    };
    cli.execute().await
}

async fn init_logging() {
    let log_file = get_log_file_path();
    
    if let Some(parent) = log_file.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    
    Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file)
                .unwrap_or_else(|_| std::fs::File::create(&log_file).expect("Failed to create log file"))
        )))
        .filter_level(LevelFilter::Info)
        .init();
    
    log::info!("OpenCLI started");
}

fn get_log_file_path() -> std::path::PathBuf {
    if let Some(config_dir) = config_dir() {
        config_dir.join("opencli").join("opencli.log")
    } else {
        std::env::current_dir()
            .map(|p| p.join("opencli.log"))
            .unwrap_or_else(|_| "opencli.log".into())
    }
}
