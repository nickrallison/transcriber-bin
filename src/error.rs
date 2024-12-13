use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Output Directory: {0}, does not exist")]
    OutputDirectoryDoesNotExist(PathBuf),

    #[error("{0}")]
    Transcribe(#[from] transcriber::error::Error),

    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("{0}")]
    Io(#[from] std::io::Error),
}
