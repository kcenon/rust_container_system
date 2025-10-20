//! Value container implementation with header information.

use super::error::Result;
use super::value::Value;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Default maximum number of values per container (prevents memory exhaustion)
pub const DEFAULT_MAX_VALUES: usize = 10_000;

/// Absolute maximum number of values (cannot be exceeded even with custom config)
pub const ABSOLUTE_MAX_VALUES: usize = 100_000;

/// Thread-safe container for values with message header information
///
/// Uses Arc<RwLock<>> pattern for safe sharing across threads.
///
/// # Example
/// ```
/// use rust_container_system::prelude::*;
/// use std::sync::Arc;
///
/// let mut container = ValueContainer::new();
/// container.set_source("sender", "session_1");
/// container.set_target("receiver", "main");
/// container.set_message_type("user_event");
///
/// container.add_value(Arc::new(IntValue::new("user_id", 123)));
/// container.add_value(Arc::new(StringValue::new("action", "login")));
/// ```
#[derive(Debug, Clone)]
pub struct ValueContainer {
    inner: Arc<RwLock<ContainerInner>>,
}

/// Internal container data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContainerInner {
    source_id: String,
    source_sub_id: String,
    target_id: String,
    target_sub_id: String,
    message_type: String,
    version: String,

    #[serde(skip)]
    values: Vec<Arc<dyn Value>>,

    #[serde(skip)]
    // Improved: Direct Arc references instead of indices for O(1) removal
    value_map: HashMap<String, Vec<Arc<dyn Value>>>,

    #[serde(skip)]
    /// Maximum number of values allowed in this container
    max_values: usize,
}

impl ValueContainer {
    /// Create a new empty container
    ///
    /// Default message_type is "data_container", version is "1.0.0.0".
    /// Maximum values is set to DEFAULT_MAX_VALUES (10,000).
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ContainerInner {
                source_id: String::new(),
                source_sub_id: String::new(),
                target_id: String::new(),
                target_sub_id: String::new(),
                message_type: "data_container".to_string(),
                version: "1.0.0.0".to_string(),
                values: Vec::new(),
                value_map: HashMap::new(),
                max_values: DEFAULT_MAX_VALUES,
            })),
        }
    }

    /// Create a container with a custom maximum value count
    ///
    /// # Arguments
    ///
    /// * `max_values` - Maximum number of values (capped at ABSOLUTE_MAX_VALUES)
    ///
    /// # Example
    /// ```
    /// use rust_container_system::prelude::*;
    ///
    /// let container = ValueContainer::with_max_values(1000);
    /// ```
    pub fn with_max_values(max_values: usize) -> Self {
        let container = Self::new();
        let actual_max = max_values.min(ABSOLUTE_MAX_VALUES);
        container.inner.write().max_values = actual_max;
        container
    }

    /// Create container with specified message type
    pub fn with_message_type(message_type: impl Into<String>) -> Self {
        let container = Self::new();
        let mut inner = container.inner.write();
        inner.message_type = message_type.into();
        drop(inner);
        container
    }

    /// Set source (sender) information
    pub fn set_source(&mut self, id: impl Into<String>, sub_id: impl Into<String>) {
        let mut inner = self.inner.write();
        inner.source_id = id.into();
        inner.source_sub_id = sub_id.into();
    }

    /// Set target (receiver) information
    pub fn set_target(&mut self, id: impl Into<String>, sub_id: impl Into<String>) {
        let mut inner = self.inner.write();
        inner.target_id = id.into();
        inner.target_sub_id = sub_id.into();
    }

    /// Set message type
    pub fn set_message_type(&mut self, message_type: impl Into<String>) {
        let mut inner = self.inner.write();
        inner.message_type = message_type.into();
    }

    /// Swap source and target (useful for creating response messages)
    pub fn swap_header(&mut self) {
        let mut inner = self.inner.write();
        // Temporary variables approach - safe and clear
        let temp_id = std::mem::take(&mut inner.source_id);
        inner.source_id = std::mem::take(&mut inner.target_id);
        inner.target_id = temp_id;

        let temp_sub_id = std::mem::take(&mut inner.source_sub_id);
        inner.source_sub_id = std::mem::take(&mut inner.target_sub_id);
        inner.target_sub_id = temp_sub_id;
    }

    /// Get source ID (clone-free version that executes callback with reference)
    pub fn with_source_id<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let inner = self.inner.read();
        f(&inner.source_id)
    }

    /// Get source ID (clones the string)
    pub fn source_id(&self) -> String {
        self.inner.read().source_id.clone()
    }

    /// Get source sub ID (clone-free version that executes callback with reference)
    pub fn with_source_sub_id<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let inner = self.inner.read();
        f(&inner.source_sub_id)
    }

    /// Get source sub ID (clones the string)
    pub fn source_sub_id(&self) -> String {
        self.inner.read().source_sub_id.clone()
    }

    /// Get target ID (clone-free version that executes callback with reference)
    pub fn with_target_id<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let inner = self.inner.read();
        f(&inner.target_id)
    }

    /// Get target ID (clones the string)
    pub fn target_id(&self) -> String {
        self.inner.read().target_id.clone()
    }

    /// Get target sub ID (clone-free version that executes callback with reference)
    pub fn with_target_sub_id<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let inner = self.inner.read();
        f(&inner.target_sub_id)
    }

    /// Get target sub ID (clones the string)
    pub fn target_sub_id(&self) -> String {
        self.inner.read().target_sub_id.clone()
    }

    /// Get message type (clone-free version that executes callback with reference)
    pub fn with_message_type_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let inner = self.inner.read();
        f(&inner.message_type)
    }

    /// Get message type (clones the string)
    pub fn message_type(&self) -> String {
        self.inner.read().message_type.clone()
    }

    /// Get version (clone-free version that executes callback with reference)
    pub fn with_version_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let inner = self.inner.read();
        f(&inner.version)
    }

    /// Get version (clones the string)
    pub fn version(&self) -> String {
        self.inner.read().version.clone()
    }

    /// Add a value to the container
    ///
    /// Values are stored with direct Arc references for efficient access and removal.
    ///
    /// # Errors
    ///
    /// Returns an error if the container has reached its maximum value count.
    pub fn add_value(&mut self, value: Arc<dyn Value>) -> Result<()> {
        let mut inner = self.inner.write();

        // Check value limit to prevent memory exhaustion
        if inner.values.len() >= inner.max_values {
            return Err(crate::core::ContainerError::InvalidDataFormat(
                format!(
                    "Container value limit reached ({}/{})",
                    inner.values.len(),
                    inner.max_values
                )
            ));
        }

        let name = value.name().to_string();

        // Store in both Vec and HashMap for dual access patterns
        inner.values.push(Arc::clone(&value));
        inner
            .value_map
            .entry(name)
            .or_default()
            .push(value);

        Ok(())
    }

    /// Try to add a value, returns false if limit reached (non-failing version)
    pub fn try_add_value(&mut self, value: Arc<dyn Value>) -> bool {
        self.add_value(value).is_ok()
    }

    /// Get the first value with the specified name
    #[inline]
    pub fn get_value(&self, name: &str) -> Option<Arc<dyn Value>> {
        let inner = self.inner.read();
        inner
            .value_map
            .get(name)
            .and_then(|values| values.first())
            .cloned()
    }

    /// Get all values with the specified name
    pub fn get_value_array(&self, name: &str) -> Vec<Arc<dyn Value>> {
        let inner = self.inner.read();
        inner
            .value_map
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    /// Zero-copy access to values with the specified name via callback
    ///
    /// This avoids cloning the Vec by executing a callback with a reference.
    ///
    /// # Example
    /// ```
    /// # use rust_container_system::prelude::*;
    /// # use std::sync::Arc;
    /// # let mut container = ValueContainer::new();
    /// # container.add_value(Arc::new(IntValue::new("test", 42))).unwrap();
    /// let count = container.with_value_array("test", |values| values.len());
    /// ```
    pub fn with_value_array<F, R>(&self, name: &str, f: F) -> Option<R>
    where
        F: FnOnce(&[Arc<dyn Value>]) -> R,
    {
        let inner = self.inner.read();
        inner.value_map.get(name).map(|values| f(values))
    }

    /// Get all values
    pub fn values(&self) -> Vec<Arc<dyn Value>> {
        self.inner.read().values.clone()
    }

    /// Zero-copy access to all values via callback
    ///
    /// This avoids cloning the entire Vec by executing a callback with a reference.
    ///
    /// # Example
    /// ```
    /// # use rust_container_system::prelude::*;
    /// # use std::sync::Arc;
    /// # let mut container = ValueContainer::new();
    /// # container.add_value(Arc::new(IntValue::new("test", 42))).unwrap();
    /// let count = container.with_values(|values| values.len());
    /// let names: Vec<String> = container.with_values(|values| {
    ///     values.iter().map(|v| v.name().to_string()).collect()
    /// });
    /// ```
    pub fn with_values<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[Arc<dyn Value>]) -> R,
    {
        let inner = self.inner.read();
        f(&inner.values)
    }

    /// Remove all values with the specified name
    ///
    /// Returns true if any values were removed.
    ///
    /// Time complexity: O(n) where n is total number of values
    /// (Optimized using HashSet for O(n) instead of O(n × m))
    pub fn remove_value(&mut self, name: &str) -> bool {
        let mut inner = self.inner.write();

        // Remove from HashMap - O(1)
        if let Some(removed_values) = inner.value_map.remove(name) {
            // Build HashSet of pointers to remove - O(m) where m is number of values with this name
            use std::collections::HashSet;
            let removed_ptrs: HashSet<*const dyn Value> = removed_values
                .iter()
                .map(Arc::as_ptr)
                .collect();

            // Remove from Vec by filtering with HashSet lookup - O(n) single pass
            inner.values.retain(|value| {
                !removed_ptrs.contains(&Arc::as_ptr(value))
            });
            true
        } else {
            false
        }
    }

    /// Clear all values (header information is preserved)
    pub fn clear_values(&mut self) {
        let mut inner = self.inner.write();
        inner.values.clear();
        inner.value_map.clear();
    }

    /// Get the number of values
    #[inline]
    pub fn value_count(&self) -> usize {
        self.inner.read().values.len()
    }

    /// Check if container is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.read().values.is_empty()
    }

    /// Copy this container
    ///
    /// # Parameters
    /// - `including_values`: If true, copy values; if false, copy header only
    pub fn copy(&self, including_values: bool) -> Self {
        let inner = self.inner.read();
        let new_inner = if including_values {
            // Clone all values
            let cloned_values: Vec<Arc<dyn Value>> = inner.values.iter().map(|v| v.clone_value()).collect();

            // Rebuild value_map from freshly cloned values to avoid sharing Arc refs
            let mut new_value_map: HashMap<String, Vec<Arc<dyn Value>>> = HashMap::new();
            for value in &cloned_values {
                let name = value.name().to_string();
                new_value_map
                    .entry(name)
                    .or_default()
                    .push(Arc::clone(value));
            }

            ContainerInner {
                source_id: inner.source_id.clone(),
                source_sub_id: inner.source_sub_id.clone(),
                target_id: inner.target_id.clone(),
                target_sub_id: inner.target_sub_id.clone(),
                message_type: inner.message_type.clone(),
                version: inner.version.clone(),
                values: cloned_values,
                value_map: new_value_map,
                max_values: inner.max_values,
            }
        } else {
            ContainerInner {
                source_id: inner.source_id.clone(),
                source_sub_id: inner.source_sub_id.clone(),
                target_id: inner.target_id.clone(),
                target_sub_id: inner.target_sub_id.clone(),
                message_type: inner.message_type.clone(),
                version: inner.version.clone(),
                values: Vec::new(),
                value_map: HashMap::new(),
                max_values: inner.max_values,
            }
        };

        Self {
            inner: Arc::new(RwLock::new(new_inner)),
        }
    }

    /// Serialize to JSON with type-preserving value serialization
    ///
    /// Each value is serialized using its own to_json() method, which preserves
    /// type-specific formatting (e.g., BytesValue uses base64 encoding).
    pub fn to_json(&self) -> Result<String> {
        let inner = self.inner.read();
        let mut json_obj = serde_json::json!({
            "source_id": inner.source_id,
            "source_sub_id": inner.source_sub_id,
            "target_id": inner.target_id,
            "target_sub_id": inner.target_sub_id,
            "message_type": inner.message_type,
            "version": inner.version,
            "values": []
        });

        if let Some(values_array) = json_obj["values"].as_array_mut() {
            for value in &inner.values {
                // Call each value's to_json() to preserve type-specific formatting
                // (e.g., BytesValue uses base64, not "<n bytes>" string representation)
                let value_json_str = value.to_json()?;
                let value_json_parsed: serde_json::Value = serde_json::from_str(&value_json_str)?;

                let value_obj = serde_json::json!({
                    "name": value.name(),
                    "type": value.value_type().to_str(),
                    "value": value_json_parsed
                });
                values_array.push(value_obj);
            }
        } else {
            return Err(crate::core::ContainerError::InvalidDataFormat(
                "Failed to access values array in JSON object".to_string()
            ));
        }

        serde_json::to_string_pretty(&json_obj).map_err(Into::into)
    }

    /// Serialize to XML with optimized string building
    ///
    /// Uses std::fmt::Write to avoid intermediate string allocations
    /// from format!() calls, improving performance especially for containers
    /// with many values.
    pub fn to_xml(&self) -> Result<String> {
        use std::fmt::Write;

        let inner = self.inner.read();

        // Pre-allocate with reasonable capacity to reduce reallocations
        // Estimate: ~200 bytes header + ~100 bytes per value
        let estimated_size = 200 + (inner.values.len() * 100);
        let mut xml = String::with_capacity(estimated_size);

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<container>\n");
        xml.push_str("  <header>\n");

        // Use writeln!() instead of format!() + push_str() to avoid intermediate allocations
        writeln!(&mut xml, "    <source_id>{}</source_id>", Self::xml_escape(&inner.source_id))
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;
        writeln!(&mut xml, "    <source_sub_id>{}</source_sub_id>", Self::xml_escape(&inner.source_sub_id))
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;
        writeln!(&mut xml, "    <target_id>{}</target_id>", Self::xml_escape(&inner.target_id))
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;
        writeln!(&mut xml, "    <target_sub_id>{}</target_sub_id>", Self::xml_escape(&inner.target_sub_id))
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;
        writeln!(&mut xml, "    <message_type>{}</message_type>", Self::xml_escape(&inner.message_type))
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;
        writeln!(&mut xml, "    <version>{}</version>", Self::xml_escape(&inner.version))
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;

        xml.push_str("  </header>\n");
        xml.push_str("  <values>\n");

        for value in &inner.values {
            // Call value.to_xml() to get type-specific formatting (e.g., base64 for BytesValue)
            // then extract the content by stripping the outer type tags
            let xml_str = value.to_xml()?;
            let type_tag = value.value_type().to_str();
            let start_tag = format!("<{}>", type_tag);
            let end_tag = format!("</{}>", type_tag);

            // Extract content from typed XML (already escaped)
            // Example: "<bytes>base64</bytes>" -> "base64"
            let content = if let Some(stripped) = xml_str.strip_prefix(&start_tag) {
                stripped.strip_suffix(&end_tag).unwrap_or(stripped)
            } else {
                &xml_str
            };

            writeln!(
                &mut xml,
                "    <value name=\"{}\" type=\"{}\">{}</value>",
                Self::xml_escape(value.name()),
                type_tag,
                content  // Already escaped by value.to_xml()
            ).map_err(|e| crate::core::ContainerError::InvalidDataFormat(format!("XML write error: {}", e)))?;
        }

        xml.push_str("  </values>\n");
        xml.push_str("</container>\n");
        Ok(xml)
    }

    /// Escape XML special characters to prevent injection attacks
    fn xml_escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    /// Serialize to bytes (currently uses JSON)
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let json = self.to_json()?;
        Ok(json.into_bytes())
    }

    /// Deserialize from JSON string
    ///
    /// Parses a JSON string and reconstructs a ValueContainer with all its values.
    /// Each value is deserialized based on its type field.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_container_system::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let mut original = ValueContainer::new();
    /// original.set_source("sender", "session_1");
    /// original.add_value(Arc::new(IntValue::new("count", 42))).unwrap();
    ///
    /// let json = original.to_json().unwrap();
    /// let restored = ValueContainer::from_json(&json).unwrap();
    ///
    /// assert_eq!(restored.source_id(), "sender");
    /// assert_eq!(restored.value_count(), 1);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - JSON is malformed
    /// - Value type is unknown
    /// - Value parsing fails
    pub fn from_json(json_str: &str) -> Result<Self> {
        use crate::values::*;

        // Parse JSON
        let json_value: serde_json::Value = serde_json::from_str(json_str)?;

        // Extract header fields
        let source_id = json_value["source_id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let source_sub_id = json_value["source_sub_id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let target_id = json_value["target_id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let target_sub_id = json_value["target_sub_id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let message_type = json_value["message_type"]
            .as_str()
            .unwrap_or("data_container")
            .to_string();
        let version = json_value["version"]
            .as_str()
            .unwrap_or("1.0.0.0")
            .to_string();

        // Create container with header
        let mut container = ValueContainer::new();
        container.set_source(source_id, source_sub_id);
        container.set_target(target_id, target_sub_id);
        container.set_message_type(message_type);
        {
            let mut inner = container.inner.write();
            inner.version = version;
        }

        // Parse values array
        if let Some(values_array) = json_value["values"].as_array() {
            for value_obj in values_array {
                let name = value_obj["name"]
                    .as_str()
                    .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                        "Missing 'name' field in value".to_string()
                    ))?;
                let value_type_str = value_obj["type"]
                    .as_str()
                    .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                        "Missing 'type' field in value".to_string()
                    ))?;
                let value_data_obj = &value_obj["value"];

                // Parse ValueType from type code (numeric strings like "1", "4", "13")
                // or from string names (for backward compatibility)
                let value_type = super::value_types::ValueType::from_type_code(value_type_str)
                    .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                        format!("Unknown value type code: '{}'", value_type_str)
                    ))?;

                // Extract the actual value from nested structure
                // Each value has its own to_json() that creates {"type": "...", "value": actual_value}
                let value_data = &value_data_obj["value"];

                // Parse value based on type
                let parsed_value: Arc<dyn Value> = match value_type {
                    super::value_types::ValueType::Bool => {
                        let val = value_data.as_bool()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid bool value for '{}'", name)
                            ))?;
                        Arc::new(BoolValue::new(name, val))
                    }
                    super::value_types::ValueType::Short => {
                        let val = value_data.as_i64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid short value for '{}'", name)
                            ))? as i16;
                        Arc::new(ShortValue::new(name, val))
                    }
                    super::value_types::ValueType::UShort => {
                        let val = value_data.as_u64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid ushort value for '{}'", name)
                            ))? as u16;
                        Arc::new(UShortValue::new(name, val))
                    }
                    super::value_types::ValueType::Int => {
                        let val = value_data.as_i64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid int value for '{}'", name)
                            ))? as i32;
                        Arc::new(IntValue::new(name, val))
                    }
                    super::value_types::ValueType::UInt => {
                        let val = value_data.as_u64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid uint value for '{}'", name)
                            ))? as u32;
                        Arc::new(UIntValue::new(name, val))
                    }
                    super::value_types::ValueType::Long | super::value_types::ValueType::LLong => {
                        let val = value_data.as_i64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid long value for '{}'", name)
                            ))?;
                        Arc::new(LongValue::new(name, val))
                    }
                    super::value_types::ValueType::ULong | super::value_types::ValueType::ULLong => {
                        let val = value_data.as_u64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid ulong value for '{}'", name)
                            ))?;
                        Arc::new(ULongValue::new(name, val))
                    }
                    super::value_types::ValueType::Float => {
                        let val = value_data.as_f64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid float value for '{}'", name)
                            ))? as f32;
                        Arc::new(FloatValue::new(name, val))
                    }
                    super::value_types::ValueType::Double => {
                        let val = value_data.as_f64()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid double value for '{}'", name)
                            ))?;
                        Arc::new(DoubleValue::new(name, val))
                    }
                    super::value_types::ValueType::String => {
                        let val = value_data.as_str()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid string value for '{}'", name)
                            ))?;
                        Arc::new(StringValue::new(name, val))
                    }
                    super::value_types::ValueType::Bytes => {
                        // Bytes are stored as base64 in JSON
                        let base64_str = value_data.as_str()
                            .ok_or_else(|| crate::core::ContainerError::InvalidDataFormat(
                                format!("Invalid bytes value for '{}'", name)
                            ))?;

                        // Decode base64
                        use base64::{Engine as _, engine::general_purpose};
                        let bytes = general_purpose::STANDARD.decode(base64_str)
                            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(
                                format!("Failed to decode base64 for '{}': {}", name, e)
                            ))?;

                        Arc::new(BytesValue::new(name, bytes))
                    }
                    super::value_types::ValueType::Null | super::value_types::ValueType::Container => {
                        return Err(crate::core::ContainerError::InvalidDataFormat(
                            format!("Unsupported value type for deserialization: {:?}", value_type)
                        ));
                    }
                };

                container.add_value(parsed_value)?;
            }
        }

        Ok(container)
    }

    /// Deserialize from bytes (currently uses JSON)
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        let json_str = std::str::from_utf8(data)
            .map_err(|e| crate::core::ContainerError::InvalidDataFormat(
                format!("Invalid UTF-8 in serialized data: {}", e)
            ))?;
        Self::from_json(json_str)
    }
}

impl Default for ValueContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing ValueContainer with a fluent API
///
/// # Example
/// ```
/// use rust_container_system::prelude::*;
///
/// let container = ValueContainer::builder()
///     .source("sender", "session_1")
///     .target("receiver", "main")
///     .message_type("user_event")
///     .max_values(1000)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ValueContainerBuilder {
    source_id: String,
    source_sub_id: String,
    target_id: String,
    target_sub_id: String,
    message_type: String,
    max_values: usize,
}

impl ValueContainerBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            source_id: String::new(),
            source_sub_id: String::new(),
            target_id: String::new(),
            target_sub_id: String::new(),
            message_type: "data_container".to_string(),
            max_values: DEFAULT_MAX_VALUES,
        }
    }

    /// Set source (sender) information
    #[must_use = "builder methods return a new value"]
    pub fn source(mut self, id: impl Into<String>, sub_id: impl Into<String>) -> Self {
        self.source_id = id.into();
        self.source_sub_id = sub_id.into();
        self
    }

    /// Set target (receiver) information
    #[must_use = "builder methods return a new value"]
    pub fn target(mut self, id: impl Into<String>, sub_id: impl Into<String>) -> Self {
        self.target_id = id.into();
        self.target_sub_id = sub_id.into();
        self
    }

    /// Set message type
    #[must_use = "builder methods return a new value"]
    pub fn message_type(mut self, msg_type: impl Into<String>) -> Self {
        self.message_type = msg_type.into();
        self
    }

    /// Set maximum number of values
    #[must_use = "builder methods return a new value"]
    pub fn max_values(mut self, max: usize) -> Self {
        self.max_values = max;
        self
    }

    /// Build the ValueContainer
    pub fn build(self) -> ValueContainer {
        let mut container = ValueContainer::with_max_values(self.max_values);
        container.set_source(&self.source_id, &self.source_sub_id);
        container.set_target(&self.target_id, &self.target_sub_id);
        container.set_message_type(&self.message_type);
        container
    }
}

impl Default for ValueContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueContainer {
    /// Create a builder for ValueContainer
    ///
    /// # Example
    /// ```
    /// use rust_container_system::prelude::*;
    ///
    /// let container = ValueContainer::builder()
    ///     .source("sender", "session_1")
    ///     .target("receiver", "main")
    ///     .message_type("user_event")
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> ValueContainerBuilder {
        ValueContainerBuilder::new()
    }
}

/// Iterator over values in a ValueContainer
///
/// # Example
/// ```
/// use rust_container_system::prelude::*;
/// use std::sync::Arc;
///
/// let mut container = ValueContainer::new();
/// container.add_value(Arc::new(IntValue::new("a", 1))).unwrap();
/// container.add_value(Arc::new(IntValue::new("b", 2))).unwrap();
///
/// for value in &container {
///     println!("{}: {}", value.name(), value.to_string());
/// }
/// ```
pub struct ValueIter {
    values: Vec<Arc<dyn Value>>,
    index: usize,
}

impl Iterator for ValueIter {
    type Item = Arc<dyn Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let value = Arc::clone(&self.values[self.index]);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.values.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ValueIter {
    fn len(&self) -> usize {
        self.values.len() - self.index
    }
}

impl IntoIterator for &ValueContainer {
    type Item = Arc<dyn Value>;
    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        let values = self.values();
        ValueIter { values, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::*;

    #[test]
    fn test_container_creation() {
        let container = ValueContainer::new();
        assert_eq!(container.message_type(), "data_container");
        assert_eq!(container.version(), "1.0.0.0");
        assert!(container.is_empty());
    }

    #[test]
    fn test_add_and_get_value() {
        let mut container = ValueContainer::new();
        let value = Arc::new(StringValue::new("test_key", "test_value"));
        container.add_value(value).unwrap();

        let retrieved = container.get_value("test_key").unwrap();
        assert_eq!(retrieved.name(), "test_key");
        assert_eq!(retrieved.to_string(), "test_value");
    }

    #[test]
    fn test_swap_header() {
        let mut container = ValueContainer::new();
        container.set_source("source1", "sub1");
        container.set_target("target1", "sub2");

        container.swap_header();

        assert_eq!(container.source_id(), "target1");
        assert_eq!(container.source_sub_id(), "sub2");
        assert_eq!(container.target_id(), "source1");
        assert_eq!(container.target_sub_id(), "sub1");
    }

    #[test]
    fn test_xml_escape() {
        // Test XML injection prevention
        let mut container = ValueContainer::new();
        container.set_source("<script>alert('xss')</script>", "test&data");
        container.set_message_type("test<>type");

        let xml = container.to_xml().unwrap();

        // Should contain escaped characters
        assert!(xml.contains("&lt;script&gt;"));
        assert!(xml.contains("&amp;data"));
        assert!(!xml.contains("<script>"));
        assert!(!xml.contains("alert('xss')"));
    }

    #[test]
    fn test_xml_special_chars() {
        let mut container = ValueContainer::new();
        container.add_value(Arc::new(StringValue::new(
            "test'name\"",
            "value&with<special>chars"
        ))).unwrap();

        let xml = container.to_xml().unwrap();

        // All special chars should be escaped
        assert!(xml.contains("&apos;"));
        assert!(xml.contains("&quot;"));
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
    }

    #[test]
    fn test_value_limit() {
        // Test with small limit
        let mut container = ValueContainer::with_max_values(3);

        // Add values up to the limit
        assert!(container.add_value(Arc::new(StringValue::new("key1", "value1"))).is_ok());
        assert!(container.add_value(Arc::new(StringValue::new("key2", "value2"))).is_ok());
        assert!(container.add_value(Arc::new(StringValue::new("key3", "value3"))).is_ok());

        // Attempt to exceed limit
        let result = container.add_value(Arc::new(StringValue::new("key4", "value4")));
        assert!(result.is_err());
        assert_eq!(container.value_count(), 3);

        // Test try_add_value
        assert!(!container.try_add_value(Arc::new(StringValue::new("key5", "value5"))));

        // Remove a value and try again
        container.remove_value("key1");
        assert_eq!(container.value_count(), 2);
        assert!(container.add_value(Arc::new(StringValue::new("key4", "value4"))).is_ok());
        assert_eq!(container.value_count(), 3);
    }

    #[test]
    fn test_absolute_max_values() {
        // Even if requesting more than ABSOLUTE_MAX_VALUES, it should be capped
        let container = ValueContainer::with_max_values(ABSOLUTE_MAX_VALUES + 1000);
        assert_eq!(container.inner.read().max_values, ABSOLUTE_MAX_VALUES);
    }

    #[test]
    fn test_builder_pattern() {
        let container = ValueContainer::builder()
            .source("sender", "session_1")
            .target("receiver", "main")
            .message_type("user_event")
            .max_values(500)
            .build();

        assert_eq!(container.source_id(), "sender");
        assert_eq!(container.source_sub_id(), "session_1");
        assert_eq!(container.target_id(), "receiver");
        assert_eq!(container.target_sub_id(), "main");
        assert_eq!(container.message_type(), "user_event");
        assert_eq!(container.inner.read().max_values, 500);
    }

    #[test]
    fn test_builder_with_values() {
        let mut container = ValueContainer::builder()
            .source("test", "1")
            .message_type("data")
            .build();

        // Should be able to add values after building
        assert!(container.add_value(Arc::new(StringValue::new("key", "value"))).is_ok());
        assert_eq!(container.value_count(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut container = ValueContainer::new();
        container.add_value(Arc::new(StringValue::new("a", "first"))).unwrap();
        container.add_value(Arc::new(StringValue::new("b", "second"))).unwrap();
        container.add_value(Arc::new(StringValue::new("c", "third"))).unwrap();

        let names: Vec<String> = (&container).into_iter()
            .map(|v| v.name().to_string())
            .collect();

        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_iterator_size_hint() {
        let mut container = ValueContainer::new();
        for i in 0..5 {
            container.add_value(Arc::new(StringValue::new(format!("val_{}", i), "test"))).unwrap();
        }

        let iter = (&container).into_iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.size_hint(), (5, Some(5)));
    }

    #[test]
    fn test_iterator_for_loop() {
        let mut container = ValueContainer::new();
        container.add_value(Arc::new(StringValue::new("x", "1"))).unwrap();
        container.add_value(Arc::new(StringValue::new("y", "2"))).unwrap();

        let mut count = 0;
        for _value in &container {
            count += 1;
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn test_deserialization_basic() {
        // Create original container
        let mut original = ValueContainer::new();
        original.set_source("sender", "session_1");
        original.set_target("receiver", "main");
        original.set_message_type("test_message");
        original.add_value(Arc::new(IntValue::new("count", 42))).unwrap();
        original.add_value(Arc::new(StringValue::new("name", "test"))).unwrap();

        // Serialize to JSON
        let json = original.to_json().unwrap();

        // Deserialize from JSON
        let restored = ValueContainer::from_json(&json).unwrap();

        // Verify header
        assert_eq!(restored.source_id(), "sender");
        assert_eq!(restored.source_sub_id(), "session_1");
        assert_eq!(restored.target_id(), "receiver");
        assert_eq!(restored.target_sub_id(), "main");
        assert_eq!(restored.message_type(), "test_message");

        // Verify values
        assert_eq!(restored.value_count(), 2);

        let count_val = restored.get_value("count").unwrap();
        assert_eq!(count_val.to_int().unwrap(), 42);

        let name_val = restored.get_value("name").unwrap();
        assert_eq!(name_val.to_string(), "test");
    }

    #[test]
    fn test_deserialization_all_types() {
        // Create container with all value types
        let mut original = ValueContainer::new();
        original.add_value(Arc::new(BoolValue::new("flag", true))).unwrap();
        original.add_value(Arc::new(ShortValue::new("s", 100i16))).unwrap();
        original.add_value(Arc::new(UShortValue::new("us", 200u16))).unwrap();
        original.add_value(Arc::new(IntValue::new("i", 1000))).unwrap();
        original.add_value(Arc::new(UIntValue::new("ui", 2000u32))).unwrap();
        original.add_value(Arc::new(LongValue::new("l", 100000i64))).unwrap();
        original.add_value(Arc::new(ULongValue::new("ul", 200000u64))).unwrap();
        original.add_value(Arc::new(FloatValue::new("f", std::f32::consts::PI))).unwrap();
        original.add_value(Arc::new(DoubleValue::new("d", std::f64::consts::E))).unwrap();
        original.add_value(Arc::new(StringValue::new("str", "hello"))).unwrap();
        original.add_value(Arc::new(BytesValue::new("bytes", vec![0xFF, 0xFE, 0xFD]))).unwrap();

        // Round-trip through JSON
        let json = original.to_json().unwrap();
        let restored = ValueContainer::from_json(&json).unwrap();

        // Verify all values
        assert_eq!(restored.value_count(), 11);

        assert!(restored.get_value("flag").unwrap().to_bool().unwrap());
        assert_eq!(restored.get_value("s").unwrap().to_int().unwrap(), 100);
        assert_eq!(restored.get_value("us").unwrap().to_int().unwrap(), 200);
        assert_eq!(restored.get_value("i").unwrap().to_int().unwrap(), 1000);
        assert_eq!(restored.get_value("ui").unwrap().to_long().unwrap(), 2000);
        assert_eq!(restored.get_value("l").unwrap().to_long().unwrap(), 100000);
        assert_eq!(restored.get_value("ul").unwrap().to_long().unwrap(), 200000);
        assert!((restored.get_value("f").unwrap().to_double().unwrap() - std::f64::consts::PI).abs() < 0.01);
        assert!((restored.get_value("d").unwrap().to_double().unwrap() - std::f64::consts::E).abs() < 0.00001);
        assert_eq!(restored.get_value("str").unwrap().to_string(), "hello");

        let bytes_val = restored.get_value("bytes").unwrap();
        assert_eq!(bytes_val.size(), 3);
    }

    #[test]
    fn test_deserialization_bytes_base64() {
        // Test that bytes are properly encoded/decoded as base64
        let mut original = ValueContainer::new();
        let test_data = vec![0x00, 0x01, 0x02, 0xFE, 0xFF];
        original.add_value(Arc::new(BytesValue::new("data", test_data.clone()))).unwrap();

        let json = original.to_json().unwrap();

        // Verify JSON contains base64
        assert!(json.contains("AAEC/v8=") || json.contains("base64"));

        // Deserialize and verify
        let restored = ValueContainer::from_json(&json).unwrap();
        let bytes_val = restored.get_value("data").unwrap();
        assert_eq!(bytes_val.size(), 5);
    }

    #[test]
    fn test_deserialization_empty_container() {
        let original = ValueContainer::new();
        let json = original.to_json().unwrap();
        let restored = ValueContainer::from_json(&json).unwrap();

        assert_eq!(restored.value_count(), 0);
        assert!(restored.is_empty());
        assert_eq!(restored.message_type(), "data_container");
    }

    #[test]
    fn test_deserialization_invalid_json() {
        let result = ValueContainer::from_json("not valid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialization_unknown_type() {
        let json = r#"{
            "source_id": "",
            "source_sub_id": "",
            "target_id": "",
            "target_sub_id": "",
            "message_type": "data_container",
            "version": "1.0.0.0",
            "values": [
                {
                    "name": "test",
                    "type": "999",
                    "value": 123
                }
            ]
        }"#;

        let result = ValueContainer::from_json(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown value type"));
    }

    #[test]
    fn test_deserialization_missing_field() {
        let json = r#"{
            "source_id": "",
            "source_sub_id": "",
            "target_id": "",
            "target_sub_id": "",
            "message_type": "data_container",
            "version": "1.0.0.0",
            "values": [
                {
                    "type": "int",
                    "value": 123
                }
            ]
        }"#;

        let result = ValueContainer::from_json(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing 'name' field"));
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        // Create complex container
        let mut original = ValueContainer::builder()
            .source("app1", "instance1")
            .target("app2", "instance2")
            .message_type("metrics_data")
            .build();

        original.add_value(Arc::new(LongValue::new("timestamp", 1234567890))).unwrap();
        original.add_value(Arc::new(DoubleValue::new("cpu_usage", 45.7))).unwrap();
        original.add_value(Arc::new(StringValue::new("hostname", "server1"))).unwrap();

        // Serialize
        let bytes = original.serialize().unwrap();

        // Deserialize
        let restored = ValueContainer::deserialize(&bytes).unwrap();

        // Verify everything matches
        assert_eq!(restored.source_id(), "app1");
        assert_eq!(restored.source_sub_id(), "instance1");
        assert_eq!(restored.target_id(), "app2");
        assert_eq!(restored.target_sub_id(), "instance2");
        assert_eq!(restored.message_type(), "metrics_data");
        assert_eq!(restored.value_count(), 3);

        assert_eq!(restored.get_value("timestamp").unwrap().to_long().unwrap(), 1234567890);
        assert!((restored.get_value("cpu_usage").unwrap().to_double().unwrap() - 45.7).abs() < 0.0001);
        assert_eq!(restored.get_value("hostname").unwrap().to_string(), "server1");
    }

    #[test]
    fn test_deserialization_multiple_values_same_name() {
        // Create container with multiple values with same name
        let mut original = ValueContainer::new();
        original.add_value(Arc::new(IntValue::new("tag", 1))).unwrap();
        original.add_value(Arc::new(IntValue::new("tag", 2))).unwrap();
        original.add_value(Arc::new(IntValue::new("tag", 3))).unwrap();

        let json = original.to_json().unwrap();
        let restored = ValueContainer::from_json(&json).unwrap();

        // All three values should be restored
        assert_eq!(restored.value_count(), 3);
        let tags = restored.get_value_array("tag");
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].to_int().unwrap(), 1);
        assert_eq!(tags[1].to_int().unwrap(), 2);
        assert_eq!(tags[2].to_int().unwrap(), 3);
    }
}
