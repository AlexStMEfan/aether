//! Единая система ошибок для всего проекта

use thiserror::Error;

/// Алиас для удобства
pub type Result<T, E = AetherError> = std::result::Result<T, E>;

/// Единый тип ошибки для всего проекта
#[derive(Error, Debug)]
pub enum AetherError {
    /// Ошибки конфигурации
    #[error("Configuration error: {0}")]
    Config(String),

    /// Ошибки валидации
    #[error("Validation error: {0}")]
    Validation(String),

    /// Сетевые ошибки
    #[error("Network error: {0}")]
    Network(String),

    /// Ошибки обнаружения сервисов
    #[error("Service discovery error: {0}")]
    Discovery(String),

    /// Ошибки безопасности
    #[error("Security error: {0}")]
    Security(String),

    /// Внутренние ошибки
    #[error("Internal error: {0}")]
    Internal(String),

    /// Обертка для стандартных ошибок
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Обертка для ошибок времени выполнения
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

/// Макрос для быстрого создания ошибок конфигурации
#[macro_export]
macro_rules! config_error {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::error::AetherError::Config(format!($msg $(, $arg)*))
    };
}

/// Макрос для быстрого создания ошибок валидации
#[macro_export]
macro_rules! validation_error {
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::error::AetherError::Validation(format!($msg $(, $arg)*))
    };
}