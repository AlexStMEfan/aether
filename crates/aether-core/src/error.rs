use thiserror::Error;

pub type Result<T, E = AetherError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum AetherError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Discovery error: {0}")]
    Discovery(String),
    #[error("Security error: {0}")]
    Security(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
