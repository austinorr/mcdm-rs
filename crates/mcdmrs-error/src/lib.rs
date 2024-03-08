pub use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MCDMRSError {
    #[error("ERROR: {0}")]
    Error(String),
    // #[error(transparent)]
    // Other(#[from] anyhow::Error),
}
