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

//! UTF-8 string value implementation.
//!
//! This module provides [`StringValue`], a type-safe container for UTF-8 encoded
//! string data. It implements the [`crate::core::Value`] trait for seamless integration with
//! the container system.
//!
//! # Features
//!
//! - Full UTF-8 support including multi-byte characters (CJK, emoji, etc.)
//! - Efficient serialization to JSON, XML, and binary wire protocol
//! - Zero-copy access to underlying string data
//! - Compatible with C++ `string_value` (type code 12)
//!
//! # Example
//!
//! ```rust
//! use rust_container_system::values::StringValue;
//! use rust_container_system::core::Value;
//!
//! // Create a string value
//! let greeting = StringValue::new("message", "Hello, 世界! 🌍");
//!
//! // Access properties
//! assert_eq!(greeting.name(), "message");
//! assert_eq!(greeting.value(), "Hello, 世界! 🌍");
//! assert_eq!(greeting.size(), 19); // Byte length, not character count
//! ```
//!
//! # Cross-Language Compatibility
//!
//! StringValue uses type code 12 to match C++ `string_value`. Binary serialization
//! uses UTF-8 encoding with little-endian length prefixes for cross-platform compatibility.

use crate::core::{Result, Value, ValueType};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;

/// UTF-8 encoded string value.
///
/// `StringValue` stores a named string value with full UTF-8 support.
/// It is the standard type for text data in the container system.
///
/// # Type Code
///
/// - Wire protocol type: `12` (matches C++ `string_value`)
/// - JSON type tag: `"string"`
/// - XML element: `<string>...</string>`
///
/// # Example
///
/// ```rust
/// use rust_container_system::values::StringValue;
/// use rust_container_system::core::Value;
/// use std::sync::Arc;
///
/// // Basic usage
/// let name = StringValue::new("username", "alice");
/// assert_eq!(name.to_string(), "alice");
///
/// // UTF-8 support
/// let korean = StringValue::new("greeting", "안녕하세요");
/// assert_eq!(korean.value(), "안녕하세요");
///
/// // Use as trait object
/// let value: Arc<dyn Value> = Arc::new(StringValue::new("key", "value"));
/// println!("Type: {:?}", value.value_type());
/// ```
///
/// # Binary Format
///
/// ```text
/// [type:1][name_len:4 LE][name:N][value_size:4 LE][utf8_bytes:M]
/// ```
///
/// - `type`: 12 (ValueType::String)
/// - `name_len`: Length of name in bytes (little-endian u32)
/// - `name`: UTF-8 encoded name
/// - `value_size`: Length of string value in bytes (little-endian u32)
/// - `utf8_bytes`: UTF-8 encoded string content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringValue {
    name: String,
    value: String,
}

impl StringValue {
    /// Create a new string value.
    ///
    /// # Arguments
    ///
    /// * `name` - The identifier/key for this value (any type implementing `Into<String>`)
    /// * `value` - The string content (any type implementing `Into<String>`)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::values::StringValue;
    ///
    /// // From &str
    /// let s1 = StringValue::new("key", "value");
    ///
    /// // From String
    /// let s2 = StringValue::new(String::from("name"), String::from("Alice"));
    ///
    /// // Mixed types
    /// let s3 = StringValue::new("config", format!("value_{}", 42));
    /// ```
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Get the string value as a reference.
    ///
    /// Returns a reference to the underlying string data without copying.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::values::StringValue;
    ///
    /// let s = StringValue::new("greeting", "Hello, World!");
    /// assert_eq!(s.value(), "Hello, World!");
    ///
    /// // Use with string operations
    /// assert!(s.value().contains("World"));
    /// assert_eq!(s.value().len(), 13);
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Value for StringValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::String
    }

    /// Returns byte length (not character count)
    fn size(&self) -> usize {
        self.value.len()
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Complete binary format with header
        // Format: [type:1][name_len:4][name][value_size:4][string_bytes]
        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len() as u32;

        let value_bytes = self.value.as_bytes();
        let value_size = value_bytes.len() as u32;

        let mut result = Vec::with_capacity(1 + 4 + name_bytes.len() + 4 + value_bytes.len());

        // Type (1 byte) - StringValue = 1
        result.push(ValueType::String as u8);

        // Name length (4 bytes, little-endian)
        result.extend_from_slice(&name_len.to_le_bytes());

        // Name (UTF-8 bytes)
        result.extend_from_slice(name_bytes);

        // Value size (4 bytes, little-endian)
        result.extend_from_slice(&value_size.to_le_bytes());

        // String bytes (UTF-8)
        result.extend_from_slice(value_bytes);

        result
    }

    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information
        let tagged = serde_json::json!({
            "type": "string",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!(
            "<string>{}</string>",
            crate::core::xml_escape(&self.value)
        ))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// From implementations for ergonomic value creation

impl From<(String, String)> for StringValue {
    fn from((name, value): (String, String)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, &str)> for StringValue {
    fn from((name, value): (&str, &str)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, String)> for StringValue {
    fn from((name, value): (&str, String)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, &str)> for StringValue {
    fn from((name, value): (String, &str)) -> Self {
        Self::new(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_from_tuple() {
        let value1: StringValue = ("name", "value").into();
        assert_eq!(value1.name(), "name");
        assert_eq!(value1.value(), "value");

        let value2: StringValue = (String::from("key"), String::from("data")).into();
        assert_eq!(value2.name(), "key");
        assert_eq!(value2.value(), "data");

        let value3: StringValue = ("test", String::from("mixed")).into();
        assert_eq!(value3.name(), "test");
        assert_eq!(value3.value(), "mixed");
    }
}
