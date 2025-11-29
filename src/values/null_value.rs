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

//! Null value implementation.
//!
//! This module provides the [`NullValue`] type which represents an absent or
//! undefined value. This is useful for optional fields or placeholder values.
//!
//! # Type Code
//!
//! - Wire protocol type: `0` (matches C++ `null_value`)
//! - JSON representation: `null`
//! - Size: 0 bytes (no payload)
//!
//! # Example
//!
//! ```rust
//! use rust_container_system::values::NullValue;
//! use rust_container_system::core::Value;
//!
//! let null_val = NullValue::new("optional_field");
//! assert!(null_val.is_null());
//! assert_eq!(null_val.size(), 0);
//! ```

use crate::core::{Result, Value, ValueType};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;

/// Null or undefined value.
///
/// Represents the absence of a value. This type has no payload and is used
/// to represent optional or missing data in a type-safe manner.
///
/// # Wire Protocol
///
/// - Type code: `0`
/// - Payload size: 0 bytes
///
/// # C++ Compatibility
///
/// This type corresponds to `null_value` in the C++ container system,
/// which uses `std::monostate` as the underlying type.
///
/// # Example
///
/// ```rust
/// use rust_container_system::values::NullValue;
/// use rust_container_system::core::Value;
///
/// let null_val = NullValue::new("deleted_field");
///
/// assert!(null_val.is_null());
/// assert_eq!(null_val.to_string(), "null");
/// assert_eq!(null_val.size(), 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NullValue {
    name: String,
}

impl NullValue {
    /// Create a new null value with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The identifier/key for this value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::values::NullValue;
    ///
    /// let null_val = NullValue::new("optional_data");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Check if this is a null value.
    ///
    /// Always returns `true` for `NullValue`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::values::NullValue;
    ///
    /// let null_val = NullValue::new("field");
    /// assert!(null_val.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        true
    }
}

impl Value for NullValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Null
    }

    fn size(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "null".to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Complete binary format with header
        // Format: [type:1][name_len:4][name][value_size:4]
        // Note: No value payload for null
        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len() as u32;
        let value_size = 0u32; // null has no payload

        let mut result = Vec::with_capacity(1 + 4 + name_bytes.len() + 4);

        // Type (1 byte) - NullValue = 0
        result.push(ValueType::Null as u8);

        // Name length (4 bytes, little-endian)
        result.extend_from_slice(&name_len.to_le_bytes());

        // Name (UTF-8 bytes)
        result.extend_from_slice(name_bytes);

        // Value size (4 bytes, little-endian) - always 0 for null
        result.extend_from_slice(&value_size.to_le_bytes());

        // No value payload

        result
    }

    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information
        let tagged = serde_json::json!({
            "type": "null",
            "value": null
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok("<null/>".to_string())
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_value_creation() {
        let null_val = NullValue::new("test_null");
        assert_eq!(null_val.name(), "test_null");
        assert!(null_val.is_null());
    }

    #[test]
    fn test_null_value_type() {
        let null_val = NullValue::new("test");
        assert_eq!(null_val.value_type(), ValueType::Null);
    }

    #[test]
    fn test_null_value_size() {
        let null_val = NullValue::new("test");
        assert_eq!(null_val.size(), 0);
    }

    #[test]
    fn test_null_value_to_string() {
        let null_val = NullValue::new("test");
        assert_eq!(null_val.to_string(), "null");
    }

    #[test]
    fn test_null_value_to_bytes() {
        let null_val = NullValue::new("test");
        let bytes = null_val.to_bytes();

        // Verify format: [type:1][name_len:4][name:4][value_size:4]
        assert_eq!(bytes[0], 0); // Type = Null = 0
        let name_len = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        assert_eq!(name_len, 4); // "test" = 4 bytes
        assert_eq!(&bytes[5..9], b"test");
        let value_size = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
        assert_eq!(value_size, 0); // Null has no payload
    }

    #[test]
    fn test_null_value_to_json() {
        let null_val = NullValue::new("test");
        let json = null_val.to_json().unwrap();
        assert!(json.contains("\"type\":\"null\""));
        assert!(json.contains("\"value\":null"));
    }

    #[test]
    fn test_null_value_to_xml() {
        let null_val = NullValue::new("test");
        let xml = null_val.to_xml().unwrap();
        assert_eq!(xml, "<null/>");
    }

    #[test]
    fn test_null_value_clone() {
        let null_val = NullValue::new("test");
        let cloned = null_val.clone_value();
        assert_eq!(cloned.name(), "test");
        assert_eq!(cloned.value_type(), ValueType::Null);
    }
}
