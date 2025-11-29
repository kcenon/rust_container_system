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

//! Domain-agnostic value storage for the container system.
//!
//! Pure value storage layer without messaging-specific fields.
//! Can be used as a general-purpose serialization container.
//!
//! # Features
//!
//! - Type-safe value storage
//! - JSON/Binary serialization support
//! - Thread-safe operations (optional via RwLock)
//! - Key-value storage interface
//! - Statistics tracking
//!
//! # Example
//!
//! ```rust
//! use rust_container_system::core::ValueStore;
//! use rust_container_system::values::IntValue;
//! use std::sync::Arc;
//!
//! let store = ValueStore::new();
//! store.add("count".to_string(), Arc::new(IntValue::new("count", 42)));
//!
//! if let Some(value) = store.get("count") {
//!     println!("Count: {:?}", value.to_int());
//! }
//! ```

use crate::core::error::ContainerError;
use crate::core::value::Value;
use crate::core::value_types::ValueType;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Helper function to convert u8 to ValueType
fn value_type_from_u8(byte: u8) -> ValueType {
    match byte {
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
        _ => ValueType::Null,
    }
}

/// Binary format version for compatibility
pub const BINARY_VERSION: u8 = 1;

/// Statistics for ValueStore operations
#[derive(Debug, Clone, Default)]
pub struct ValueStoreStats {
    /// Number of read operations
    pub read_count: u64,
    /// Number of write operations
    pub write_count: u64,
    /// Number of serialization operations
    pub serialization_count: u64,
}

/// Domain-agnostic value storage.
///
/// Pure value storage layer without messaging-specific fields.
/// Can be used as a general-purpose serialization container.
///
/// # Thread Safety
///
/// By default, operations are not thread-safe for maximum performance.
/// Call `enable_thread_safety()` to enable thread-safe mode with RwLock protection.
pub struct ValueStore {
    /// Internal key-value storage
    values: RwLock<HashMap<String, Arc<dyn Value>>>,

    /// Whether thread safety is enabled
    thread_safe_enabled: AtomicBool,

    /// Statistics
    read_count: AtomicU64,
    write_count: AtomicU64,
    serialization_count: AtomicU64,
}

impl Default for ValueStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueStore {
    /// Creates a new empty ValueStore.
    pub fn new() -> Self {
        Self {
            values: RwLock::new(HashMap::new()),
            thread_safe_enabled: AtomicBool::new(false),
            read_count: AtomicU64::new(0),
            write_count: AtomicU64::new(0),
            serialization_count: AtomicU64::new(0),
        }
    }

    /// Creates a new ValueStore with initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: RwLock::new(HashMap::with_capacity(capacity)),
            thread_safe_enabled: AtomicBool::new(false),
            read_count: AtomicU64::new(0),
            write_count: AtomicU64::new(0),
            serialization_count: AtomicU64::new(0),
        }
    }

    // =========================================================================
    // Value Management
    // =========================================================================

    /// Add a value with a key.
    ///
    /// If the key already exists, the value will be overwritten.
    pub fn add(&self, key: String, value: Arc<dyn Value>) {
        let mut values = self.values.write();
        values.insert(key, value);
        self.write_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get a value by key.
    ///
    /// Returns None if the key doesn't exist.
    pub fn get(&self, key: &str) -> Option<Arc<dyn Value>> {
        let values = self.values.read();
        let value = values.get(key).cloned();
        if value.is_some() {
            self.read_count.fetch_add(1, Ordering::Relaxed);
        }
        value
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        let values = self.values.read();
        values.contains_key(key)
    }

    /// Remove a value by key.
    ///
    /// Returns true if removed, false if not found.
    pub fn remove(&self, key: &str) -> bool {
        let mut values = self.values.write();
        values.remove(key).is_some()
    }

    /// Clear all values.
    pub fn clear(&self) {
        let mut values = self.values.write();
        values.clear();
    }

    /// Get number of stored values.
    pub fn size(&self) -> usize {
        let values = self.values.read();
        values.len()
    }

    /// Check if store is empty.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Get all keys.
    pub fn keys(&self) -> Vec<String> {
        let values = self.values.read();
        values.keys().cloned().collect()
    }

    /// Get all values.
    pub fn values(&self) -> Vec<Arc<dyn Value>> {
        let values = self.values.read();
        values.values().cloned().collect()
    }

    // =========================================================================
    // Serialization
    // =========================================================================

    /// Serialize to JSON string.
    pub fn serialize(&self) -> Result<String, ContainerError> {
        self.serialization_count.fetch_add(1, Ordering::Relaxed);

        let values = self.values.read();
        let mut result = serde_json::Map::new();

        for (key, value) in values.iter() {
            let value_obj = serde_json::json!({
                "name": value.name(),
                "type": format!("{:?}", value.value_type()),
                "data": value.to_string()
            });
            result.insert(key.clone(), value_obj);
        }

        serde_json::to_string_pretty(&result).map_err(|e| {
            ContainerError::SerializationError(format!("JSON serialization failed: {}", e))
        })
    }

    /// Serialize to binary format.
    ///
    /// Binary format:
    /// - Version byte (1)
    /// - Number of entries (4 bytes, uint32, little-endian)
    /// - For each entry:
    ///   - Key length (4 bytes, uint32, little-endian)
    ///   - Key data (UTF-8)
    ///   - Value type (1 byte)
    ///   - Value length (4 bytes, uint32, little-endian)
    ///   - Value data
    pub fn serialize_binary(&self) -> Result<Vec<u8>, ContainerError> {
        self.serialization_count.fetch_add(1, Ordering::Relaxed);

        let values = self.values.read();
        let mut result = Vec::new();

        // Version byte
        result.push(BINARY_VERSION);

        // Number of entries
        let count = values.len() as u32;
        result.extend_from_slice(&count.to_le_bytes());

        // Serialize each key-value pair
        for (key, value) in values.iter() {
            // Key length and key
            let key_bytes = key.as_bytes();
            let key_len = key_bytes.len() as u32;
            result.extend_from_slice(&key_len.to_le_bytes());
            result.extend_from_slice(key_bytes);

            // Value type
            result.push(value.value_type() as u8);

            // Value data - use to_bytes() for binary representation
            let value_data = value.to_bytes();
            let value_len = value_data.len() as u32;
            result.extend_from_slice(&value_len.to_le_bytes());
            result.extend_from_slice(&value_data);
        }

        Ok(result)
    }

    /// Deserialize from binary format.
    ///
    /// Note: Requires a factory function to create values from type and data.
    pub fn deserialize_binary<F>(data: &[u8], factory: F) -> Result<Self, ContainerError>
    where
        F: Fn(&str, ValueType, &[u8]) -> Option<Arc<dyn Value>>,
    {
        if data.len() < 5 {
            return Err(ContainerError::DeserializationError(
                "Invalid data: too small".to_string(),
            ));
        }

        let mut offset = 0;

        // Read version
        let version = data[offset];
        offset += 1;

        if version != BINARY_VERSION {
            return Err(ContainerError::DeserializationError(format!(
                "Unsupported binary version: {}",
                version
            )));
        }

        // Read count
        let count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let store = Self::new();

        // Read each key-value pair
        for i in 0..count {
            if offset + 4 > data.len() {
                return Err(ContainerError::DeserializationError(format!(
                    "Truncated data at entry {}",
                    i
                )));
            }

            // Read key length
            let key_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;

            if offset + key_len + 5 > data.len() {
                return Err(ContainerError::DeserializationError(
                    "Truncated key data".to_string(),
                ));
            }

            // Read key
            let key = std::str::from_utf8(&data[offset..offset + key_len])
                .map_err(|e| {
                    ContainerError::DeserializationError(format!("Invalid UTF-8 in key: {}", e))
                })?
                .to_string();
            offset += key_len;

            // Read value type
            let value_type = value_type_from_u8(data[offset]);
            offset += 1;

            // Read value length
            let value_len =
                u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;

            if offset + value_len > data.len() {
                return Err(ContainerError::DeserializationError(
                    "Truncated value data".to_string(),
                ));
            }

            // Read value data
            let value_data = &data[offset..offset + value_len];
            offset += value_len;

            // Create value using factory
            if let Some(value) = factory(&key, value_type, value_data) {
                store.values.write().insert(key, value);
            }
        }

        Ok(store)
    }

    /// Convert to JSON format (alias for serialize).
    pub fn to_json(&self) -> Result<String, ContainerError> {
        self.serialize()
    }

    // =========================================================================
    // Thread Safety
    // =========================================================================

    /// Enable thread-safe operations.
    pub fn enable_thread_safety(&self) {
        self.thread_safe_enabled.store(true, Ordering::SeqCst);
    }

    /// Disable thread-safe operations.
    pub fn disable_thread_safety(&self) {
        self.thread_safe_enabled.store(false, Ordering::SeqCst);
    }

    /// Check if thread safety is enabled.
    pub fn is_thread_safe(&self) -> bool {
        self.thread_safe_enabled.load(Ordering::SeqCst)
    }

    // =========================================================================
    // Statistics
    // =========================================================================

    /// Get current statistics.
    pub fn get_stats(&self) -> ValueStoreStats {
        ValueStoreStats {
            read_count: self.read_count.load(Ordering::Relaxed),
            write_count: self.write_count.load(Ordering::Relaxed),
            serialization_count: self.serialization_count.load(Ordering::Relaxed),
        }
    }

    /// Get number of read operations.
    pub fn get_read_count(&self) -> u64 {
        self.read_count.load(Ordering::Relaxed)
    }

    /// Get number of write operations.
    pub fn get_write_count(&self) -> u64 {
        self.write_count.load(Ordering::Relaxed)
    }

    /// Get number of serialization operations.
    pub fn get_serialization_count(&self) -> u64 {
        self.serialization_count.load(Ordering::Relaxed)
    }

    /// Reset all statistics to zero.
    pub fn reset_statistics(&self) {
        self.read_count.store(0, Ordering::Relaxed);
        self.write_count.store(0, Ordering::Relaxed);
        self.serialization_count.store(0, Ordering::Relaxed);
    }

    // =========================================================================
    // Iteration Support
    // =========================================================================

    /// Iterate over all key-value pairs.
    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&str, &Arc<dyn Value>),
    {
        let values = self.values.read();
        for (key, value) in values.iter() {
            callback(key, value);
        }
    }
}

impl Clone for ValueStore {
    fn clone(&self) -> Self {
        let values = self.values.read();
        let new_values: HashMap<String, Arc<dyn Value>> = values.clone();

        Self {
            values: RwLock::new(new_values),
            thread_safe_enabled: AtomicBool::new(self.thread_safe_enabled.load(Ordering::SeqCst)),
            read_count: AtomicU64::new(0),
            write_count: AtomicU64::new(0),
            serialization_count: AtomicU64::new(0),
        }
    }
}

impl std::fmt::Debug for ValueStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = self.values.read();
        f.debug_struct("ValueStore")
            .field("size", &values.len())
            .field("keys", &values.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::*;

    #[test]
    fn test_create_empty_store() {
        let store = ValueStore::new();
        assert_eq!(store.size(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn test_add_and_get_value() {
        let store = ValueStore::new();
        let value = Arc::new(IntValue::new("count", 42)) as Arc<dyn Value>;
        store.add("count".to_string(), value);

        assert_eq!(store.size(), 1);
        assert!(!store.is_empty());

        let retrieved = store.get("count");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().to_int().unwrap(), 42);
    }

    #[test]
    fn test_contains() {
        let store = ValueStore::new();
        store.add(
            "name".to_string(),
            Arc::new(StringValue::new("name", "test")) as Arc<dyn Value>,
        );

        assert!(store.contains("name"));
        assert!(!store.contains("missing"));
    }

    #[test]
    fn test_remove() {
        let store = ValueStore::new();
        store.add(
            "key1".to_string(),
            Arc::new(IntValue::new("key1", 1)) as Arc<dyn Value>,
        );
        store.add(
            "key2".to_string(),
            Arc::new(IntValue::new("key2", 2)) as Arc<dyn Value>,
        );

        assert_eq!(store.size(), 2);
        assert!(store.remove("key1"));
        assert_eq!(store.size(), 1);
        assert!(!store.contains("key1"));
        assert!(!store.remove("nonexistent"));
    }

    #[test]
    fn test_clear() {
        let store = ValueStore::new();
        store.add(
            "a".to_string(),
            Arc::new(IntValue::new("a", 1)) as Arc<dyn Value>,
        );
        store.add(
            "b".to_string(),
            Arc::new(IntValue::new("b", 2)) as Arc<dyn Value>,
        );

        assert_eq!(store.size(), 2);
        store.clear();
        assert_eq!(store.size(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn test_statistics() {
        let store = ValueStore::new();

        assert_eq!(store.get_read_count(), 0);
        assert_eq!(store.get_write_count(), 0);

        store.add(
            "a".to_string(),
            Arc::new(IntValue::new("a", 1)) as Arc<dyn Value>,
        );
        assert_eq!(store.get_write_count(), 1);

        store.get("a");
        assert_eq!(store.get_read_count(), 1);

        store.reset_statistics();
        assert_eq!(store.get_read_count(), 0);
        assert_eq!(store.get_write_count(), 0);
    }

    #[test]
    fn test_serialization() {
        let store = ValueStore::new();
        store.add(
            "name".to_string(),
            Arc::new(StringValue::new("name", "test")) as Arc<dyn Value>,
        );

        let json = store.serialize();
        assert!(json.is_ok());
        assert!(!json.unwrap().is_empty());
    }

    #[test]
    fn test_binary_serialization() {
        let store = ValueStore::new();
        store.add(
            "count".to_string(),
            Arc::new(IntValue::new("count", 42)) as Arc<dyn Value>,
        );

        let binary = store.serialize_binary();
        assert!(binary.is_ok());

        let data = binary.unwrap();
        assert!(!data.is_empty());
        assert_eq!(data[0], BINARY_VERSION);
    }
}
