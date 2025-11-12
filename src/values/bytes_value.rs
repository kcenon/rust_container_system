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

//! Binary data value implementation with Base64 encoding support.

use crate::core::{Result, Value, ValueType};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;

/// Binary data value (raw bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesValue {
    name: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

impl BytesValue {
    /// Create a new bytes value
    pub fn new(name: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            data,
        }
    }

    /// Create from byte slice
    pub fn from_slice(name: impl Into<String>, data: &[u8]) -> Self {
        Self {
            name: name.into(),
            data: data.to_vec(),
        }
    }

    /// Get byte data as slice
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Value for BytesValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Bytes
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn to_string(&self) -> String {
        format!("<{} bytes>", self.data.len())
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Complete binary format with header
        // Format: [type:1][name_len:4][name][value_size:4][bytes]
        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len() as u32;
        let value_size = self.data.len() as u32;

        let mut result = Vec::with_capacity(1 + 4 + name_bytes.len() + 4 + self.data.len());

        // Type (1 byte) - BytesValue = 12
        result.push(ValueType::Bytes as u8);

        // Name length (4 bytes, little-endian)
        result.extend_from_slice(&name_len.to_le_bytes());

        // Name (UTF-8 bytes)
        result.extend_from_slice(name_bytes);

        // Value size (4 bytes, little-endian)
        result.extend_from_slice(&value_size.to_le_bytes());

        // Raw bytes
        result.extend_from_slice(&self.data);

        result
    }

    /// Serialize to JSON using Base64 encoding with type tag
    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information
        let tagged = serde_json::json!({
            "type": "bytes",
            "value": base64_encode(&self.data)?
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    /// Serialize to XML using Base64 encoding
    fn to_xml(&self) -> Result<String> {
        Ok(format!("<bytes>{}</bytes>", crate::core::xml_escape(&base64_encode(&self.data)?)))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Encode bytes to Base64 string
fn base64_encode(data: &[u8]) -> Result<String> {
    use base64::Engine;
    // Use simpler, non-panicking API
    Ok(base64::engine::general_purpose::STANDARD.encode(data))
}

/// Custom serde module for efficient byte serialization
mod serde_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bytes.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<u8>::deserialize(deserializer)
    }
}

// From implementations for ergonomic value creation

impl From<(String, Vec<u8>)> for BytesValue {
    fn from((name, data): (String, Vec<u8>)) -> Self {
        Self::new(name, data)
    }
}

impl From<(&str, Vec<u8>)> for BytesValue {
    fn from((name, data): (&str, Vec<u8>)) -> Self {
        Self::new(name, data)
    }
}

impl From<(String, &[u8])> for BytesValue {
    fn from((name, data): (String, &[u8])) -> Self {
        Self::from_slice(name, data)
    }
}

impl From<(&str, &[u8])> for BytesValue {
    fn from((name, data): (&str, &[u8])) -> Self {
        Self::from_slice(name, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_from_tuple() {
        let data = vec![1u8, 2, 3, 4, 5];
        let value1: BytesValue = ("data", data.clone()).into();
        assert_eq!(value1.name(), "data");
        assert_eq!(value1.data(), &[1u8, 2, 3, 4, 5]);

        let value2: BytesValue = (String::from("bytes"), &[10u8, 20, 30][..]).into();
        assert_eq!(value2.name(), "bytes");
        assert_eq!(value2.data(), &[10u8, 20, 30]);

        let slice = &[99u8, 88, 77][..];
        let value3: BytesValue = ("slice", slice).into();
        assert_eq!(value3.name(), "slice");
        assert_eq!(value3.data(), &[99u8, 88, 77]);
    }
}
