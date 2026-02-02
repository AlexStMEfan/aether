pub mod error;
pub mod types;

pub use error::{AetherError, Result};
pub use types::{BackendId, BackendKind};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
