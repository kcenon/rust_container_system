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

//! JSON v2.0 Adapter for cross-language compatibility
//!
//! This module implements the unified JSON v2.0 format for data interchange
//! between C++, Python, .NET, Go, and Rust container system implementations.
//!
//! # Unified JSON v2.0 Format
//!
//! ```json
//! {
//!   "container": {
//!     "version": "2.0",
//!     "metadata": {
//!       "message_type": "user_profile",
//!       "protocol_version": "1.0.0.0",
//!       "source": { "id": "client", "sub_id": "session" },
//!       "target": { "id": "server", "sub_id": "handler" }
//!     },
//!     "values": [
//!       { "name": "username", "type": 12, "type_name": "string", "data": "john_doe" }
//!     ]
//!   }
//! }
//! ```
//!
//! # Example
//!
//! ```rust
//! use rust_container_system::prelude::*;
//! use rust_container_system::core::json_v2_adapter::JsonV2Adapter;
//! use std::sync::Arc;
//!
//! let mut container = ValueContainer::new();
//! container.set_source("client", "session");
//! container.set_target("server", "handler");
//! container.set_message_type("user_data");
//! container.add_value(Arc::new(IntValue::new("count", 42))).unwrap();
//! container.add_value(Arc::new(StringValue::new("name", "Alice"))).unwrap();
//!
//! // Serialize to JSON v2.0
//! let json = JsonV2Adapter::to_v2_json(&container, true).unwrap();
//!
//! // Deserialize from JSON v2.0
//! let restored = JsonV2Adapter::from_v2_json(&json).unwrap();
//! assert_eq!(restored.message_type(), "user_data");
//! ```

use crate::core::{ContainerError, Result, Value, ValueContainer, ValueType};
use crate::values::*;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::{json, Map, Value as JsonValue};
use std::sync::Arc;

/// JSON format version constants
pub const V2_FORMAT_VERSION: &str = "2.0";

/// Detected serialization format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    /// Unified JSON v2.0 format
    JsonV2,
    /// C++ nested JSON format (header + values object)
    CppJson,
    /// Python/.NET flat JSON format (flat + values array)
    PythonJson,
    /// C++ Wire Protocol (text-based)
    WireProtocol,
    /// Unknown or invalid format
    Unknown,
}

impl std::fmt::Display for SerializationFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationFormat::JsonV2 => write!(f, "v2.0"),
            SerializationFormat::CppJson => write!(f, "cpp"),
            SerializationFormat::PythonJson => write!(f, "python"),
            SerializationFormat::WireProtocol => write!(f, "wire"),
            SerializationFormat::Unknown => write!(f, "unknown"),
        }
    }
}

/// Type name mapping for human-readable type names (matches C++/Python/.NET)
fn type_name_from_value_type(vt: ValueType) -> &'static str {
    match vt {
        ValueType::Null => "null",
        ValueType::Bool => "bool",
        ValueType::Short => "short",
        ValueType::UShort => "ushort",
        ValueType::Int => "int",
        ValueType::UInt => "uint",
        ValueType::Long => "long",
        ValueType::ULong => "ulong",
        ValueType::LLong => "llong",
        ValueType::ULLong => "ullong",
        ValueType::Float => "float",
        ValueType::Double => "double",
        ValueType::String => "string",
        ValueType::Bytes => "bytes",
        ValueType::Container => "container",
        ValueType::Array => "array",
    }
}

/// Reverse mapping from type name to ValueType
fn value_type_from_name(name: &str) -> Option<ValueType> {
    match name {
        "null" => Some(ValueType::Null),
        "bool" => Some(ValueType::Bool),
        "short" => Some(ValueType::Short),
        "ushort" => Some(ValueType::UShort),
        "int" => Some(ValueType::Int),
        "uint" => Some(ValueType::UInt),
        "long" => Some(ValueType::Long),
        "ulong" => Some(ValueType::ULong),
        "llong" => Some(ValueType::LLong),
        "ullong" => Some(ValueType::ULLong),
        "float" => Some(ValueType::Float),
        "double" => Some(ValueType::Double),
        "string" => Some(ValueType::String),
        "bytes" => Some(ValueType::Bytes),
        "container" => Some(ValueType::Container),
        "array" => Some(ValueType::Array),
        _ => None,
    }
}

/// JSON v2.0 Adapter for cross-language compatibility
///
/// This adapter provides methods to:
/// - Convert ValueContainer to unified JSON v2.0 format
/// - Parse JSON v2.0 format into ValueContainer
/// - Convert between different JSON formats (C++ nested, Python/.NET flat, v2.0 unified)
/// - Detect format automatically
pub struct JsonV2Adapter;

impl JsonV2Adapter {
    /// Convert ValueContainer to unified JSON v2.0 format
    ///
    /// # Arguments
    ///
    /// * `container` - ValueContainer to convert
    /// * `pretty` - If true, format with indentation for readability
    ///
    /// # Returns
    ///
    /// JSON string in v2.0 unified format
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::prelude::*;
    /// use rust_container_system::core::json_v2_adapter::JsonV2Adapter;
    /// use std::sync::Arc;
    ///
    /// let mut container = ValueContainer::new();
    /// container.add_value(Arc::new(IntValue::new("count", 42))).unwrap();
    /// let json = JsonV2Adapter::to_v2_json(&container, true).unwrap();
    /// assert!(json.contains("\"version\": \"2.0\""));
    /// ```
    pub fn to_v2_json(container: &ValueContainer, pretty: bool) -> Result<String> {
        let mut values_array = Vec::new();

        container.with_values(|values| {
            for value in values {
                values_array.push(Self::value_to_v2_dict(value));
            }
        });

        let v2_data = json!({
            "container": {
                "version": V2_FORMAT_VERSION,
                "metadata": {
                    "message_type": container.message_type(),
                    "protocol_version": container.version(),
                    "source": {
                        "id": container.source_id(),
                        "sub_id": container.source_sub_id()
                    },
                    "target": {
                        "id": container.target_id(),
                        "sub_id": container.target_sub_id()
                    }
                },
                "values": values_array
            }
        });

        if pretty {
            serde_json::to_string_pretty(&v2_data)
                .map_err(|e| ContainerError::SerializationError(e.to_string()))
        } else {
            serde_json::to_string(&v2_data)
                .map_err(|e| ContainerError::SerializationError(e.to_string()))
        }
    }

    /// Parse JSON v2.0 format into ValueContainer
    ///
    /// # Arguments
    ///
    /// * `json_str` - JSON string in v2.0 unified format
    ///
    /// # Returns
    ///
    /// ValueContainer object populated with parsed data
    ///
    /// # Errors
    ///
    /// Returns error if JSON format is invalid or incompatible
    pub fn from_v2_json(json_str: &str) -> Result<ValueContainer> {
        let data: JsonValue = serde_json::from_str(json_str)
            .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid JSON: {}", e)))?;

        let container_data = data.get("container").ok_or_else(|| {
            ContainerError::InvalidDataFormat(
                "Missing 'container' root element in JSON v2.0".to_string(),
            )
        })?;

        // Check version
        let version = container_data
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0");

        if version != V2_FORMAT_VERSION {
            return Err(ContainerError::InvalidDataFormat(format!(
                "Unsupported JSON version: {} (expected {})",
                version, V2_FORMAT_VERSION
            )));
        }

        // Parse metadata
        let metadata = container_data.get("metadata").unwrap_or(&JsonValue::Null);
        let source = metadata.get("source").unwrap_or(&JsonValue::Null);
        let target = metadata.get("target").unwrap_or(&JsonValue::Null);

        let source_id = source.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let source_sub_id = source.get("sub_id").and_then(|v| v.as_str()).unwrap_or("");
        let target_id = target.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let target_sub_id = target.get("sub_id").and_then(|v| v.as_str()).unwrap_or("");
        let message_type = metadata
            .get("message_type")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Create container
        let mut container = ValueContainer::new();
        container.set_source(source_id, source_sub_id);
        container.set_target(target_id, target_sub_id);
        container.set_message_type(message_type);

        // Parse values
        if let Some(values_array) = container_data.get("values").and_then(|v| v.as_array()) {
            for value_data in values_array {
                if let Some(value) = Self::v2_dict_to_value(value_data) {
                    container.add_value(value)?;
                }
            }
        }

        Ok(container)
    }

    /// Convert C++ nested JSON format to ValueContainer
    ///
    /// C++ JSON format has "header" object and "values" object (not array)
    pub fn from_cpp_json(json_str: &str) -> Result<ValueContainer> {
        let data: JsonValue = serde_json::from_str(json_str)
            .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid JSON: {}", e)))?;

        let header = data.get("header").ok_or_else(|| {
            ContainerError::InvalidDataFormat("Missing 'header' field in C++ JSON".to_string())
        })?;

        let message_type = header
            .get("message_type")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let source_id = header
            .get("source_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let source_sub_id = header
            .get("source_sub_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let target_id = header
            .get("target_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let target_sub_id = header
            .get("target_sub_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut container = ValueContainer::new();
        container.set_source(source_id, source_sub_id);
        container.set_target(target_id, target_sub_id);
        container.set_message_type(message_type);

        // Parse values (C++ format: values is an object with keys)
        if let Some(values_obj) = data.get("values").and_then(|v| v.as_object()) {
            for (name, value_data) in values_obj {
                if let Some(value) = Self::cpp_value_to_value(name, value_data) {
                    container.add_value(value)?;
                }
            }
        }

        Ok(container)
    }

    /// Convert ValueContainer to C++ nested JSON format
    pub fn to_cpp_json(container: &ValueContainer, pretty: bool) -> Result<String> {
        let mut values_obj = Map::new();

        container.with_values(|values| {
            for value in values {
                let value_data = json!({
                    "type": value.value_type() as u8,
                    "data": Self::value_to_string_data(value)
                });
                values_obj.insert(value.name().to_string(), value_data);
            }
        });

        let cpp_data = json!({
            "header": {
                "message_type": container.message_type(),
                "version": container.version(),
                "source_id": container.source_id(),
                "source_sub_id": container.source_sub_id(),
                "target_id": container.target_id(),
                "target_sub_id": container.target_sub_id()
            },
            "values": values_obj
        });

        if pretty {
            serde_json::to_string_pretty(&cpp_data)
                .map_err(|e| ContainerError::SerializationError(e.to_string()))
        } else {
            serde_json::to_string(&cpp_data)
                .map_err(|e| ContainerError::SerializationError(e.to_string()))
        }
    }

    /// Detect JSON format type automatically
    ///
    /// # Arguments
    ///
    /// * `data` - String data to analyze
    ///
    /// # Returns
    ///
    /// SerializationFormat indicating the detected format
    pub fn detect_format(data: &str) -> SerializationFormat {
        // Check for wire protocol first
        let trimmed = data.trim();
        if trimmed.starts_with("@header={{") || trimmed.starts_with("@header={") {
            return SerializationFormat::WireProtocol;
        }

        // Try to parse as JSON
        let json_data: JsonValue = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => return SerializationFormat::Unknown,
        };

        // Check for v2.0 format
        if let Some(container) = json_data.get("container") {
            if let Some(version) = container.get("version").and_then(|v| v.as_str()) {
                if version == V2_FORMAT_VERSION {
                    return SerializationFormat::JsonV2;
                }
            }
        }

        // Check for C++ format (has "header" object)
        if json_data.get("header").is_some()
            && json_data
                .get("values")
                .and_then(|v| v.as_object())
                .is_some()
        {
            return SerializationFormat::CppJson;
        }

        // Check for Python/.NET format (flat with values array)
        if json_data.get("message_type").is_some()
            && json_data.get("values").and_then(|v| v.as_array()).is_some()
        {
            return SerializationFormat::PythonJson;
        }

        SerializationFormat::Unknown
    }

    /// Convert between different serialization formats
    ///
    /// # Arguments
    ///
    /// * `data` - Input data string
    /// * `target_format` - Target format to convert to
    /// * `pretty` - Format output with indentation
    ///
    /// # Returns
    ///
    /// String in target format
    pub fn convert_format(
        data: &str,
        target_format: SerializationFormat,
        pretty: bool,
    ) -> Result<String> {
        let source_format = Self::detect_format(data);

        // Parse to container based on source format
        let container = match source_format {
            SerializationFormat::JsonV2 => Self::from_v2_json(data)?,
            SerializationFormat::CppJson => Self::from_cpp_json(data)?,
            SerializationFormat::WireProtocol => {
                crate::core::wire_protocol::deserialize_cpp_wire(data)?
            }
            SerializationFormat::PythonJson => {
                // Python flat format - parse manually
                Self::from_python_json(data)?
            }
            SerializationFormat::Unknown => {
                return Err(ContainerError::InvalidDataFormat(format!(
                    "Unsupported source format: {}",
                    source_format
                )));
            }
        };

        // Convert to target format
        match target_format {
            SerializationFormat::JsonV2 => Self::to_v2_json(&container, pretty),
            SerializationFormat::CppJson => Self::to_cpp_json(&container, pretty),
            SerializationFormat::WireProtocol => {
                crate::core::wire_protocol::serialize_cpp_wire(&container)
            }
            SerializationFormat::PythonJson => Self::to_python_json(&container, pretty),
            SerializationFormat::Unknown => Err(ContainerError::InvalidDataFormat(
                "Cannot convert to unknown format".to_string(),
            )),
        }
    }

    /// Convert ValueContainer to Python/.NET flat JSON format
    pub fn to_python_json(container: &ValueContainer, pretty: bool) -> Result<String> {
        let mut values_array = Vec::new();

        container.with_values(|values| {
            for value in values {
                values_array.push(Self::value_to_v2_dict(value));
            }
        });

        let python_data = json!({
            "message_type": container.message_type(),
            "version": container.version(),
            "source_id": container.source_id(),
            "source_sub_id": container.source_sub_id(),
            "target_id": container.target_id(),
            "target_sub_id": container.target_sub_id(),
            "values": values_array
        });

        if pretty {
            serde_json::to_string_pretty(&python_data)
                .map_err(|e| ContainerError::SerializationError(e.to_string()))
        } else {
            serde_json::to_string(&python_data)
                .map_err(|e| ContainerError::SerializationError(e.to_string()))
        }
    }

    /// Parse Python/.NET flat JSON format
    fn from_python_json(json_str: &str) -> Result<ValueContainer> {
        let data: JsonValue = serde_json::from_str(json_str)
            .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid JSON: {}", e)))?;

        let message_type = data
            .get("message_type")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let source_id = data.get("source_id").and_then(|v| v.as_str()).unwrap_or("");
        let source_sub_id = data
            .get("source_sub_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let target_id = data.get("target_id").and_then(|v| v.as_str()).unwrap_or("");
        let target_sub_id = data
            .get("target_sub_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut container = ValueContainer::new();
        container.set_source(source_id, source_sub_id);
        container.set_target(target_id, target_sub_id);
        container.set_message_type(message_type);

        // Parse values array
        if let Some(values_array) = data.get("values").and_then(|v| v.as_array()) {
            for value_data in values_array {
                if let Some(value) = Self::v2_dict_to_value(value_data) {
                    container.add_value(value)?;
                }
            }
        }

        Ok(container)
    }

    // Private helper methods

    fn value_to_v2_dict(value: &Arc<dyn Value>) -> JsonValue {
        let value_type = value.value_type();
        let type_id = value_type as u8;
        let type_name = type_name_from_value_type(value_type);

        let mut obj = Map::new();
        obj.insert("name".to_string(), json!(value.name()));
        obj.insert("type".to_string(), json!(type_id));
        obj.insert("type_name".to_string(), json!(type_name));

        // Handle different value types
        match value_type {
            ValueType::Bool => {
                obj.insert("data".to_string(), json!(value.to_bool().unwrap_or(false)));
            }
            ValueType::Short => {
                obj.insert("data".to_string(), json!(value.to_short().unwrap_or(0)));
            }
            ValueType::UShort => {
                obj.insert("data".to_string(), json!(value.to_ushort().unwrap_or(0)));
            }
            ValueType::Int => {
                obj.insert("data".to_string(), json!(value.to_int().unwrap_or(0)));
            }
            ValueType::UInt => {
                obj.insert("data".to_string(), json!(value.to_uint().unwrap_or(0)));
            }
            ValueType::Long | ValueType::LLong => {
                obj.insert("data".to_string(), json!(value.to_long().unwrap_or(0)));
            }
            ValueType::ULong | ValueType::ULLong => {
                obj.insert("data".to_string(), json!(value.to_ulong().unwrap_or(0)));
            }
            ValueType::Float => {
                obj.insert("data".to_string(), json!(value.to_float().unwrap_or(0.0)));
            }
            ValueType::Double => {
                obj.insert("data".to_string(), json!(value.to_double().unwrap_or(0.0)));
            }
            ValueType::String => {
                obj.insert("data".to_string(), json!(value.to_string()));
            }
            ValueType::Bytes => {
                // Base64 encode binary data (matches Python/.NET)
                // Must downcast to BytesValue to get raw data (to_bytes() returns serialized format)
                if let Some(bytes_val) = value.as_any().downcast_ref::<BytesValue>() {
                    let b64 = BASE64.encode(bytes_val.data());
                    obj.insert("data".to_string(), json!(b64));
                    obj.insert("encoding".to_string(), json!("base64"));
                } else {
                    // Fallback: try to get bytes from serialized format
                    obj.insert("data".to_string(), json!(""));
                    obj.insert("encoding".to_string(), json!("base64"));
                }
            }
            ValueType::Container => {
                // Nested container
                if let Some(container_val) = value.as_any().downcast_ref::<ContainerValue>() {
                    let children: Vec<JsonValue> = container_val
                        .children()
                        .iter()
                        .map(Self::value_to_v2_dict)
                        .collect();
                    obj.insert("data".to_string(), json!(children));
                    obj.insert(
                        "child_count".to_string(),
                        json!(container_val.child_count()),
                    );
                } else {
                    obj.insert("data".to_string(), json!([]));
                    obj.insert("child_count".to_string(), json!(0));
                }
            }
            ValueType::Array => {
                // Array of values
                if let Some(array_val) = value.as_any().downcast_ref::<ArrayValue>() {
                    let elements: Vec<JsonValue> = array_val
                        .elements()
                        .iter()
                        .map(Self::value_to_v2_dict)
                        .collect();
                    obj.insert("data".to_string(), json!(elements));
                    obj.insert("element_count".to_string(), json!(array_val.count()));
                } else {
                    obj.insert("data".to_string(), json!([]));
                    obj.insert("element_count".to_string(), json!(0));
                }
            }
            ValueType::Null => {
                obj.insert("data".to_string(), JsonValue::Null);
            }
        }

        JsonValue::Object(obj)
    }

    fn v2_dict_to_value(value_data: &JsonValue) -> Option<Arc<dyn Value>> {
        let name = value_data.get("name")?.as_str()?;
        let type_id = value_data.get("type")?.as_u64()? as u8;

        // Convert type ID to ValueType
        let value_type = match type_id {
            0 => ValueType::Null,
            1 => ValueType::Bool,
            2 => ValueType::Short,
            3 => ValueType::UShort,
            4 => ValueType::Int,
            5 => ValueType::UInt,
            6 => ValueType::Long,
            7 => ValueType::ULong,
            8 => ValueType::LLong,
            9 => ValueType::ULLong,
            10 => ValueType::Float,
            11 => ValueType::Double,
            12 => ValueType::String,
            13 => ValueType::Bytes,
            14 => ValueType::Container,
            15 => ValueType::Array,
            _ => {
                // Try type_name if type ID is invalid
                let type_name = value_data.get("type_name").and_then(|v| v.as_str())?;
                value_type_from_name(type_name)?
            }
        };

        let data = value_data.get("data");

        match value_type {
            ValueType::Null => Some(Arc::new(ContainerValue::new(name, vec![]))),
            ValueType::Bool => {
                let val = data?.as_bool().unwrap_or(false);
                Some(Arc::new(BoolValue::new(name, val)))
            }
            ValueType::Short => {
                let val = data?.as_i64().unwrap_or(0) as i16;
                Some(Arc::new(ShortValue::new(name, val)))
            }
            ValueType::UShort => {
                let val = data?.as_u64().unwrap_or(0) as u16;
                Some(Arc::new(UShortValue::new(name, val)))
            }
            ValueType::Int => {
                let val = data?.as_i64().unwrap_or(0) as i32;
                Some(Arc::new(IntValue::new(name, val)))
            }
            ValueType::UInt => {
                let val = data?.as_u64().unwrap_or(0) as u32;
                Some(Arc::new(UIntValue::new(name, val)))
            }
            ValueType::Long => {
                let val = data?.as_i64().unwrap_or(0);
                LongValue::new(name, val)
                    .ok()
                    .map(|v| Arc::new(v) as Arc<dyn Value>)
            }
            ValueType::ULong => {
                let val = data?.as_u64().unwrap_or(0);
                ULongValue::new(name, val)
                    .ok()
                    .map(|v| Arc::new(v) as Arc<dyn Value>)
            }
            ValueType::LLong => {
                let val = data?.as_i64().unwrap_or(0);
                Some(Arc::new(LLongValue::new(name, val)))
            }
            ValueType::ULLong => {
                let val = data?.as_u64().unwrap_or(0);
                Some(Arc::new(ULLongValue::new(name, val)))
            }
            ValueType::Float => {
                let val = data?.as_f64().unwrap_or(0.0) as f32;
                Some(Arc::new(FloatValue::new(name, val)))
            }
            ValueType::Double => {
                let val = data?.as_f64().unwrap_or(0.0);
                Some(Arc::new(DoubleValue::new(name, val)))
            }
            ValueType::String => {
                let val = data?.as_str().unwrap_or("");
                Some(Arc::new(StringValue::new(name, val)))
            }
            ValueType::Bytes => {
                // Decode base64
                let encoding = value_data
                    .get("encoding")
                    .and_then(|v| v.as_str())
                    .unwrap_or("base64");
                let data_str = data?.as_str().unwrap_or("");

                if encoding == "base64" {
                    match BASE64.decode(data_str) {
                        Ok(bytes) => Some(Arc::new(BytesValue::new(name, bytes))),
                        Err(_) => Some(Arc::new(BytesValue::new(name, vec![]))),
                    }
                } else {
                    Some(Arc::new(BytesValue::new(
                        name,
                        data_str.as_bytes().to_vec(),
                    )))
                }
            }
            ValueType::Container => {
                let mut children = Vec::new();
                if let Some(children_data) = data.and_then(|d| d.as_array()) {
                    for child_data in children_data {
                        if let Some(child) = Self::v2_dict_to_value(child_data) {
                            children.push(child);
                        }
                    }
                }
                Some(Arc::new(ContainerValue::new(name, children)))
            }
            ValueType::Array => {
                let mut elements = Vec::new();
                if let Some(elements_data) = data.and_then(|d| d.as_array()) {
                    for elem_data in elements_data {
                        if let Some(elem) = Self::v2_dict_to_value(elem_data) {
                            elements.push(elem);
                        }
                    }
                }
                Some(Arc::new(ArrayValue::new(name, elements)))
            }
        }
    }

    fn cpp_value_to_value(name: &str, value_data: &JsonValue) -> Option<Arc<dyn Value>> {
        let type_id = value_data.get("type")?.as_u64()? as u8;
        let data_str = value_data.get("data")?.as_str().unwrap_or("");

        let value_type = match type_id {
            0 => ValueType::Null,
            1 => ValueType::Bool,
            2 => ValueType::Short,
            3 => ValueType::UShort,
            4 => ValueType::Int,
            5 => ValueType::UInt,
            6 => ValueType::Long,
            7 => ValueType::ULong,
            8 => ValueType::LLong,
            9 => ValueType::ULLong,
            10 => ValueType::Float,
            11 => ValueType::Double,
            12 => ValueType::String,
            13 => ValueType::Bytes,
            14 => ValueType::Container,
            15 => ValueType::Array,
            _ => return None,
        };

        match value_type {
            ValueType::Bool => {
                let val = data_str.to_lowercase() == "true" || data_str == "1";
                Some(Arc::new(BoolValue::new(name, val)))
            }
            ValueType::Short => data_str
                .parse::<i16>()
                .ok()
                .map(|v| Arc::new(ShortValue::new(name, v)) as Arc<dyn Value>),
            ValueType::UShort => data_str
                .parse::<u16>()
                .ok()
                .map(|v| Arc::new(UShortValue::new(name, v)) as Arc<dyn Value>),
            ValueType::Int => data_str
                .parse::<i32>()
                .ok()
                .map(|v| Arc::new(IntValue::new(name, v)) as Arc<dyn Value>),
            ValueType::UInt => data_str
                .parse::<u32>()
                .ok()
                .map(|v| Arc::new(UIntValue::new(name, v)) as Arc<dyn Value>),
            ValueType::Long => data_str
                .parse::<i64>()
                .ok()
                .and_then(|v| LongValue::new(name, v).ok())
                .map(|v| Arc::new(v) as Arc<dyn Value>),
            ValueType::ULong => data_str
                .parse::<u64>()
                .ok()
                .and_then(|v| ULongValue::new(name, v).ok())
                .map(|v| Arc::new(v) as Arc<dyn Value>),
            ValueType::LLong => data_str
                .parse::<i64>()
                .ok()
                .map(|v| Arc::new(LLongValue::new(name, v)) as Arc<dyn Value>),
            ValueType::ULLong => data_str
                .parse::<u64>()
                .ok()
                .map(|v| Arc::new(ULLongValue::new(name, v)) as Arc<dyn Value>),
            ValueType::Float => data_str
                .parse::<f32>()
                .ok()
                .map(|v| Arc::new(FloatValue::new(name, v)) as Arc<dyn Value>),
            ValueType::Double => data_str
                .parse::<f64>()
                .ok()
                .map(|v| Arc::new(DoubleValue::new(name, v)) as Arc<dyn Value>),
            ValueType::String => Some(Arc::new(StringValue::new(name, data_str))),
            ValueType::Bytes => BASE64
                .decode(data_str)
                .ok()
                .map(|bytes| Arc::new(BytesValue::new(name, bytes)) as Arc<dyn Value>),
            _ => None,
        }
    }

    fn value_to_string_data(value: &Arc<dyn Value>) -> String {
        match value.value_type() {
            ValueType::Bool => {
                if value.to_bool().unwrap_or(false) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            ValueType::Bytes => BASE64.encode(value.to_bytes()),
            _ => value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_v2_json() {
        let mut container = ValueContainer::new();
        container.set_source("client", "session");
        container.set_target("server", "handler");
        container.set_message_type("test_msg");
        container
            .add_value(Arc::new(IntValue::new("count", 42)))
            .unwrap();
        container
            .add_value(Arc::new(StringValue::new("name", "Alice")))
            .unwrap();

        let json = JsonV2Adapter::to_v2_json(&container, true).unwrap();

        assert!(json.contains("\"version\": \"2.0\""));
        assert!(json.contains("\"message_type\": \"test_msg\""));
        assert!(json.contains("\"count\""));
        assert!(json.contains("\"Alice\""));
    }

    #[test]
    fn test_from_v2_json() {
        let json = r#"{
            "container": {
                "version": "2.0",
                "metadata": {
                    "message_type": "test_msg",
                    "protocol_version": "1.0.0.0",
                    "source": { "id": "client", "sub_id": "session" },
                    "target": { "id": "server", "sub_id": "handler" }
                },
                "values": [
                    { "name": "count", "type": 4, "type_name": "int", "data": 42 },
                    { "name": "name", "type": 12, "type_name": "string", "data": "Alice" }
                ]
            }
        }"#;

        let container = JsonV2Adapter::from_v2_json(json).unwrap();

        assert_eq!(container.source_id(), "client");
        assert_eq!(container.source_sub_id(), "session");
        assert_eq!(container.target_id(), "server");
        assert_eq!(container.target_sub_id(), "handler");
        assert_eq!(container.message_type(), "test_msg");
        assert_eq!(container.value_count(), 2);

        let count = container.get_value("count").unwrap();
        assert_eq!(count.to_int().unwrap(), 42);

        let name = container.get_value("name").unwrap();
        assert_eq!(name.to_string(), "Alice");
    }

    #[test]
    fn test_roundtrip_v2_json() {
        let mut original = ValueContainer::new();
        original.set_source("sender", "s1");
        original.set_message_type("data");
        original
            .add_value(Arc::new(IntValue::new("x", 100)))
            .unwrap();
        original
            .add_value(Arc::new(BoolValue::new("flag", true)))
            .unwrap();
        original
            .add_value(Arc::new(DoubleValue::new("pi", std::f64::consts::PI)))
            .unwrap();

        let json = JsonV2Adapter::to_v2_json(&original, false).unwrap();
        let restored = JsonV2Adapter::from_v2_json(&json).unwrap();

        assert_eq!(restored.source_id(), "sender");
        assert_eq!(restored.source_sub_id(), "s1");
        assert_eq!(restored.value_count(), 3);

        let x = restored.get_value("x").unwrap();
        assert_eq!(x.to_int().unwrap(), 100);

        let flag = restored.get_value("flag").unwrap();
        assert!(flag.to_bool().unwrap());

        let pi = restored.get_value("pi").unwrap();
        assert!((pi.to_double().unwrap() - std::f64::consts::PI).abs() < 0.00001);
    }

    #[test]
    fn test_bytes_base64_encoding() {
        let mut container = ValueContainer::new();
        let test_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        container
            .add_value(Arc::new(BytesValue::new("data", test_bytes.clone())))
            .unwrap();

        let json = JsonV2Adapter::to_v2_json(&container, false).unwrap();
        assert!(json.contains("SGVsbG8=")); // Base64 for "Hello"
                                            // Check encoding field (compact JSON has no spaces)
        assert!(
            json.contains("\"encoding\":\"base64\"") || json.contains("\"encoding\": \"base64\"")
        );

        let restored = JsonV2Adapter::from_v2_json(&json).unwrap();
        let data = restored.get_value("data").unwrap();

        let bytes_val = data.as_any().downcast_ref::<BytesValue>().unwrap();
        assert_eq!(bytes_val.data(), &test_bytes[..]);
    }

    #[test]
    fn test_detect_format() {
        // JSON v2.0
        let v2_json = r#"{"container": {"version": "2.0", "metadata": {}, "values": []}}"#;
        assert_eq!(
            JsonV2Adapter::detect_format(v2_json),
            SerializationFormat::JsonV2
        );

        // C++ JSON
        let cpp_json =
            r#"{"header": {"message_type": "test"}, "values": {"key": {"type": 4, "data": "42"}}}"#;
        assert_eq!(
            JsonV2Adapter::detect_format(cpp_json),
            SerializationFormat::CppJson
        );

        // Python JSON
        let python_json =
            r#"{"message_type": "test", "values": [{"name": "key", "type": 4, "data": 42}]}"#;
        assert_eq!(
            JsonV2Adapter::detect_format(python_json),
            SerializationFormat::PythonJson
        );

        // Wire Protocol
        let wire = "@header={{[5,test];[6,1.0.0.0];}};@data={{[x,int_value,42];}};";
        assert_eq!(
            JsonV2Adapter::detect_format(wire),
            SerializationFormat::WireProtocol
        );
    }

    #[test]
    fn test_convert_format() {
        // Create a container and convert to different formats
        let mut container = ValueContainer::new();
        container.set_message_type("test");
        container
            .add_value(Arc::new(IntValue::new("count", 42)))
            .unwrap();

        let v2_json = JsonV2Adapter::to_v2_json(&container, false).unwrap();

        // Convert v2.0 to C++ format
        let cpp_json =
            JsonV2Adapter::convert_format(&v2_json, SerializationFormat::CppJson, false).unwrap();

        assert!(cpp_json.contains("\"header\""));
        assert!(cpp_json.contains("\"message_type\""));

        // Convert back to v2.0
        let restored_v2 =
            JsonV2Adapter::convert_format(&cpp_json, SerializationFormat::JsonV2, false).unwrap();

        let restored = JsonV2Adapter::from_v2_json(&restored_v2).unwrap();
        assert_eq!(restored.message_type(), "test");
    }

    #[test]
    fn test_nested_container() {
        let mut container = ValueContainer::new();

        let child1 = Arc::new(IntValue::new("inner_int", 100)) as Arc<dyn Value>;
        let child2 = Arc::new(StringValue::new("inner_str", "hello")) as Arc<dyn Value>;
        let nested = ContainerValue::new("nested", vec![child1, child2]);

        container.add_value(Arc::new(nested)).unwrap();

        let json = JsonV2Adapter::to_v2_json(&container, true).unwrap();
        let restored = JsonV2Adapter::from_v2_json(&json).unwrap();

        assert_eq!(restored.value_count(), 1);

        let nested_val = restored.get_value("nested").unwrap();
        let container_val = nested_val
            .as_any()
            .downcast_ref::<ContainerValue>()
            .unwrap();
        assert_eq!(container_val.child_count(), 2);
    }
}
