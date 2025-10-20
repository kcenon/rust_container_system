//! Error types for the container system.

use thiserror::Error;

/// Result type alias for container operations
pub type Result<T> = std::result::Result<T, ContainerError>;

/// Container operation errors
#[derive(Error, Debug)]
pub enum ContainerError {
    /// Type conversion failed
    #[error("Invalid type conversion: cannot convert {from} to {to}")]
    InvalidTypeConversion { from: String, to: String },

    /// Value not found by key
    #[error("Value not found: {0}")]
    ValueNotFound(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Invalid data format
    #[error("Invalid data format: {0}")]
    InvalidDataFormat(String),

    /// IO error (auto-converted from std::io::Error)
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON error (auto-converted from serde_json::Error)
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// XML error
    #[error("XML error: {0}")]
    XmlError(String),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Thread safety error
    #[error("Thread safety error: {0}")]
    ThreadSafetyError(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Convert quick_xml::Error to ContainerError
impl From<quick_xml::Error> for ContainerError {
    fn from(err: quick_xml::Error) -> Self {
        ContainerError::XmlError(err.to_string())
    }
}

/// Convert quick_xml::DeError to ContainerError
impl From<quick_xml::DeError> for ContainerError {
    fn from(err: quick_xml::DeError) -> Self {
        ContainerError::XmlError(err.to_string())
    }
}
