//! Aether Core — базовые примитивы и типы для всего проекта

// Макросы должны быть объявлены первыми
#[macro_use]
extern crate tracing;

// Модули ядра
pub mod error;
pub mod net;
pub mod sync;
pub mod time;
pub mod types;

// Публичный экспорт для удобства
pub use error::{AetherError, Result};
pub use types::{BackendId, BackendKind, SocketAddrExt};

/// Версия проекта (автогенерируется при сборке)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Git commit хеш (для отладки)
pub const GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");