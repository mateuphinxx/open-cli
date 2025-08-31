use thiserror::Error;
use std::borrow::Cow;

pub type Result<T> = std::result::Result<T, OpenCliError>;

#[derive(Error, Debug)]
pub enum OpenCliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Process error: {0}")]
    Process(Cow<'static, str>),
    
    #[error("Config error: {0}")]
    Config(Cow<'static, str>),
    
    #[error("Server error: {0}")]
    Server(Cow<'static, str>),
    
    #[error("Not found: {0}")]
    NotFound(Cow<'static, str>),
}

// Static error constants
impl OpenCliError {
    pub const COMPILER_NOT_FOUND: &'static str = "Compiler binary not found";
    pub const INVALID_CONFIG: &'static str = "Invalid configuration format";
    pub const SERVER_START_FAILED: &'static str = "Failed to start server";
    pub const DOWNLOAD_FAILED: &'static str = "Download failed";
    pub const EXTRACTION_FAILED: &'static str = "Extraction failed";
    
    pub fn process(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::Process(msg.into())
    }
    
    pub fn config(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::Config(msg.into())
    }
    
    pub fn server(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::Server(msg.into())
    }
    
    pub fn not_found(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::NotFound(msg.into())
    }
}
