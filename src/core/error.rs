// BSD 3-Clause License
//
// Copyright (c) 2021-2025, 🍀☀🌕🌥 🌊
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
