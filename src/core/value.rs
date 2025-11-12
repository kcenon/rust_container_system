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

//! Value trait definition and base implementation.

use super::error::{ContainerError, Result};
use super::value_types::ValueType;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// Common interface for all value types
///
/// All value types (Bool, Int, String, etc.) implement this trait,
/// providing a unified interface for container operations.
///
/// # Example
/// ```
/// use rust_container_system::prelude::*;
/// use std::sync::Arc;
///
/// let value: Arc<dyn Value> = Arc::new(IntValue::new("age", 25));
/// println!("Name: {}", value.name());
/// println!("Type: {}", value.value_type());
/// println!("Is numeric: {}", value.is_numeric());
/// ```
pub trait Value: Debug + Send + Sync {
    // Required methods

    /// Get the name/key of this value
    fn name(&self) -> &str;

    /// Get the type of this value
    fn value_type(&self) -> ValueType;

    /// Get the size in bytes
    fn size(&self) -> usize;

    // Type checking methods (with default implementations)

    /// Check if this is a null value
    fn is_null(&self) -> bool {
        self.value_type() == ValueType::Null
    }

    /// Check if this is binary data
    fn is_bytes(&self) -> bool {
        self.value_type() == ValueType::Bytes
    }

    /// Check if this is a boolean
    fn is_boolean(&self) -> bool {
        self.value_type() == ValueType::Bool
    }

    /// Check if this is a numeric type
    fn is_numeric(&self) -> bool {
        self.value_type().is_numeric()
    }

    /// Check if this is a string
    fn is_string(&self) -> bool {
        self.value_type() == ValueType::String
    }

    /// Check if this is a nested container
    fn is_container(&self) -> bool {
        self.value_type() == ValueType::Container
    }

    // Type conversion methods (default to error, override as needed)

    /// Convert to boolean
    fn to_bool(&self) -> Result<bool> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "bool".to_string(),
        })
    }

    /// Convert to 16-bit signed integer
    fn to_short(&self) -> Result<i16> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "i16".to_string(),
        })
    }

    /// Convert to 16-bit unsigned integer
    fn to_ushort(&self) -> Result<u16> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "u16".to_string(),
        })
    }

    /// Convert to 32-bit signed integer
    fn to_int(&self) -> Result<i32> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "i32".to_string(),
        })
    }

    /// Convert to 32-bit unsigned integer
    fn to_uint(&self) -> Result<u32> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "u32".to_string(),
        })
    }

    /// Convert to 64-bit signed integer
    fn to_long(&self) -> Result<i64> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "i64".to_string(),
        })
    }

    /// Convert to 64-bit unsigned integer
    fn to_ulong(&self) -> Result<u64> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "u64".to_string(),
        })
    }

    /// Convert to 32-bit float
    fn to_float(&self) -> Result<f32> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "f32".to_string(),
        })
    }

    /// Convert to 64-bit float
    fn to_double(&self) -> Result<f64> {
        Err(ContainerError::InvalidTypeConversion {
            from: format!("{}", self.value_type()),
            to: "f64".to_string(),
        })
    }

    // Serialization methods

    /// Convert to string representation
    fn to_string(&self) -> String;

    /// Convert to byte array
    fn to_bytes(&self) -> Vec<u8>;

    /// Serialize to JSON string
    fn to_json(&self) -> Result<String>;

    /// Serialize to XML string
    fn to_xml(&self) -> Result<String>;

    // Meta methods

    /// Clone this value as an Arc<dyn Value>
    fn clone_value(&self) -> Arc<dyn Value>;

    /// Cast to Any for runtime type checking
    fn as_any(&self) -> &dyn Any;
}

/// Base value implementation for generic value storage
///
/// Stores name, type, and raw data as bytes.
///
/// # Example
/// ```
/// use rust_container_system::core::{BaseValue, ValueType};
///
/// let null_val = BaseValue::null("empty");
/// let data = vec![1, 2, 3, 4];
/// let value = BaseValue::new("custom", ValueType::Bytes, data);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseValue {
    name: String,
    value_type: ValueType,
    data: Vec<u8>,
}

impl BaseValue {
    /// Create a null value
    pub fn null(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value_type: ValueType::Null,
            data: Vec::new(),
        }
    }

    /// Create a value with specified type and data
    pub fn new(name: impl Into<String>, value_type: ValueType, data: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            value_type,
            data,
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the type
    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    /// Get the raw data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get the size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
