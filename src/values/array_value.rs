//! Array value implementation for lists/arrays.

use crate::core::error::Result;
use crate::core::value::Value;
use crate::core::value_types::ValueType;
use std::any::Any;
use std::fmt;
use std::sync::Arc;

/// A value that contains an ordered list of elements
///
/// ArrayValue (type 15) is an extension to support homogeneous or heterogeneous
/// collections of values, similar to JSON arrays. This enables cross-language
/// compatibility with array structures.
///
/// # Wire format (binary)
/// `[type:1=15][name_len:4 LE][name:UTF-8][value_size:4 LE][count:4 LE][values...]`
///
/// # Text format
/// `[name,15,count];[element1][element2]...`
///
/// # Example
/// ```
/// use rust_container_system::values::{ArrayValue, IntValue, StringValue};
/// use std::sync::Arc;
///
/// // Create elements
/// let elem1 = Arc::new(IntValue::new("", 10));
/// let elem2 = Arc::new(IntValue::new("", 20));
///
/// // Create array with elements
/// let array = ArrayValue::new("numbers", vec![elem1, elem2]);
///
/// assert_eq!(array.count(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct ArrayValue {
    name: String,
    elements: Vec<Arc<dyn Value>>,
}

impl ArrayValue {
    /// Create a new array value
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ArrayValue, IntValue};
    ///
    /// let array = ArrayValue::new("my_array", vec![]);
    /// assert_eq!(array.count(), 0);
    /// ```
    pub fn new(name: impl Into<String>, elements: Vec<Arc<dyn Value>>) -> Self {
        Self {
            name: name.into(),
            elements,
        }
    }

    /// Create an array with reserved capacity
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::ArrayValue;
    ///
    /// let array = ArrayValue::with_capacity("data", 10);
    /// assert_eq!(array.count(), 0);
    /// ```
    pub fn with_capacity(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            elements: Vec::with_capacity(capacity),
        }
    }

    /// Get the number of elements
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ArrayValue, IntValue};
    /// use std::sync::Arc;
    ///
    /// let elem = Arc::new(IntValue::new("", 42));
    /// let array = ArrayValue::new("data", vec![elem]);
    /// assert_eq!(array.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.elements.len()
    }

    /// Check if the array is empty
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::ArrayValue;
    ///
    /// let array = ArrayValue::new("data", vec![]);
    /// assert!(array.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get all elements
    pub fn elements(&self) -> &[Arc<dyn Value>] {
        &self.elements
    }

    /// Add an element to the end of the array
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ArrayValue, IntValue};
    /// use std::sync::Arc;
    ///
    /// let mut array = ArrayValue::new("data", vec![]);
    /// array.push(Arc::new(IntValue::new("", 123)));
    /// assert_eq!(array.count(), 1);
    /// ```
    pub fn push(&mut self, element: Arc<dyn Value>) {
        self.elements.push(element);
    }

    /// Add an element (C++ compatibility name)
    pub fn push_back(&mut self, element: Arc<dyn Value>) {
        self.push(element);
    }

    /// Get element at index
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ArrayValue, IntValue};
    /// use std::sync::Arc;
    ///
    /// let elem1 = Arc::new(IntValue::new("", 1));
    /// let elem2 = Arc::new(IntValue::new("", 2));
    /// let array = ArrayValue::new("data", vec![elem1, elem2]);
    ///
    /// let first = array.at(0).unwrap();
    /// assert_eq!(first.to_int().unwrap(), 1);
    /// ```
    pub fn at(&self, index: usize) -> Option<Arc<dyn Value>> {
        self.elements.get(index).cloned()
    }

    /// Clear all elements
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ArrayValue, IntValue};
    /// use std::sync::Arc;
    ///
    /// let mut array = ArrayValue::new("data", vec![Arc::new(IntValue::new("", 1))]);
    /// assert_eq!(array.count(), 1);
    ///
    /// array.clear();
    /// assert_eq!(array.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Serialize to text format
    fn serialize_text(&self) -> String {
        let mut result = format!("[{},{},{}];", self.name, self.value_type().to_str(), self.count());

        for element in &self.elements {
            result.push_str(&element.to_string());
        }

        result
    }

    /// Serialize to complete binary format with header
    ///
    /// Binary format (little-endian):
    /// `[type:1=15][name_len:4 LE][name:UTF-8][value_size:4 LE][count:4 LE][element1_bytes][element2_bytes]...`
    ///
    /// This produces byte-for-byte compatible output with C++ ArrayValue::serialize()
    pub fn to_binary_bytes(&self) -> Vec<u8> {
        // Serialize all elements first to calculate total size
        let mut serialized_elements: Vec<Vec<u8>> = Vec::with_capacity(self.elements.len());
        let mut total_elements_size = 0usize;

        for element in &self.elements {
            let elem_bytes = element.to_bytes();
            total_elements_size += elem_bytes.len();
            serialized_elements.push(elem_bytes);
        }

        // value_size = count(4) + all element bytes
        let value_size = 4 + total_elements_size;

        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len() as u32;

        // Calculate total capacity
        // type(1) + name_len(4) + name + value_size(4) + count(4) + elements
        let total_capacity = 1 + 4 + name_bytes.len() + 4 + 4 + total_elements_size;
        let mut result = Vec::with_capacity(total_capacity);

        // Type (1 byte) - ArrayValue = 15
        result.push(ValueType::Array as u8);

        // Name length (4 bytes, little-endian)
        result.extend_from_slice(&name_len.to_le_bytes());

        // Name (UTF-8 bytes)
        result.extend_from_slice(name_bytes);

        // Value size (4 bytes, little-endian)
        result.extend_from_slice(&(value_size as u32).to_le_bytes());

        // Element count (4 bytes, little-endian)
        let count = self.elements.len() as u32;
        result.extend_from_slice(&count.to_le_bytes());

        // Append all serialized elements
        for elem_bytes in serialized_elements {
            result.extend_from_slice(&elem_bytes);
        }

        result
    }

    /// Deserialize ArrayValue from complete binary format
    ///
    /// Binary format:
    /// `[type:1=15][name_len:4 LE][name:UTF-8][value_size:4 LE][count:4 LE][element1][element2]...`
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Data is too short
    /// - Type byte is not 15 (ArrayValue)
    /// - Element deserialization fails
    pub fn deserialize_binary(data: &[u8]) -> Result<Self> {
        use crate::core::error::ContainerError;

        if data.len() < 13 {
            // type(1) + name_len(4) + value_size(4) + count(4)
            return Err(ContainerError::InvalidDataFormat(
                format!("ArrayValue binary data too short: {} bytes", data.len())
            ));
        }

        let mut offset = 0;

        // Read type (1 byte)
        let type_id = data[offset];
        offset += 1;

        if type_id != ValueType::Array as u8 {
            return Err(ContainerError::InvalidDataFormat(
                format!("Expected ArrayValue type (15), got {}", type_id)
            ));
        }

        // Read name length (4 bytes, little-endian)
        let name_len = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;

        // Read name
        if offset + name_len > data.len() {
            return Err(ContainerError::InvalidDataFormat(
                format!("Name length {} exceeds data bounds", name_len)
            ));
        }
        let name = String::from_utf8(data[offset..offset + name_len].to_vec())
            .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid UTF-8 in name: {}", e)))?;
        offset += name_len;

        // Read value size (4 bytes, little-endian)
        if offset + 4 > data.len() {
            return Err(ContainerError::InvalidDataFormat(
                "Insufficient data for value_size".to_string()
            ));
        }
        let _value_size = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        // Read element count (4 bytes, little-endian)
        if offset + 4 > data.len() {
            return Err(ContainerError::InvalidDataFormat(
                "Insufficient data for element count".to_string()
            ));
        }
        let count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;

        // Deserialize all elements
        let mut elements = Vec::with_capacity(count);

        for i in 0..count {
            if offset >= data.len() {
                return Err(ContainerError::InvalidDataFormat(
                    format!("Unexpected end of data while reading element {}/{}", i + 1, count)
                ));
            }

            // Extract remaining data for element deserialization
            let element_data = &data[offset..];

            // Deserialize element using factory
            // TODO: Implement full value factory for all types
            let (element, bytes_read) = Self::deserialize_value(element_data)?;

            elements.push(element);
            offset += bytes_read;
        }

        Ok(ArrayValue::new(name, elements))
    }

    /// Helper function to deserialize a single value from binary data
    ///
    /// Returns the deserialized value and the number of bytes consumed.
    ///
    /// # TODO
    /// This is a minimal implementation. Full Value factory pattern needed for all types.
    fn deserialize_value(data: &[u8]) -> Result<(Arc<dyn Value>, usize)> {
        use crate::core::error::ContainerError;
        use crate::values::{IntValue, LongValue, StringValue};

        if data.is_empty() {
            return Err(ContainerError::InvalidDataFormat(
                "Empty data for value deserialization".to_string()
            ));
        }

        // Read type ID - manually map u8 to ValueType
        let type_byte = data[0];
        let type_id = match type_byte {
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
                return Err(ContainerError::InvalidDataFormat(
                    format!("Unknown value type: {}", type_byte)
                ));
            }
        };

        match type_id {
            ValueType::Int => {
                // Deserialize IntValue (type 3)
                // Format: [type:1][name_len:4][name][value_size:4][value:4]
                if data.len() < 13 {
                    return Err(ContainerError::InvalidDataFormat(
                        "Insufficient data for IntValue".to_string()
                    ));
                }

                let mut offset = 1;

                // Name length
                let name_len = u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
                offset += 4;

                // Name
                let name = String::from_utf8(data[offset..offset + name_len].to_vec())
                    .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid UTF-8: {}", e)))?;
                offset += name_len;

                // Skip value_size (4 bytes)
                offset += 4;

                // Read int value (4 bytes, little-endian)
                let value = i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
                offset += 4;

                Ok((Arc::new(IntValue::new(name, value)), offset))
            }

            ValueType::Long | ValueType::LLong => {
                // Deserialize LongValue (type 5 or 7)
                // Format: [type:1][name_len:4][name][value_size:4][value:8]
                if data.len() < 17 {
                    return Err(ContainerError::InvalidDataFormat(
                        "Insufficient data for LongValue".to_string()
                    ));
                }

                let mut offset = 1;

                // Name length
                let name_len = u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
                offset += 4;

                // Name
                let name = String::from_utf8(data[offset..offset + name_len].to_vec())
                    .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid UTF-8: {}", e)))?;
                offset += name_len;

                // Skip value_size (4 bytes)
                offset += 4;

                // Read long value (8 bytes, little-endian)
                let value = i64::from_le_bytes([
                    data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
                    data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7],
                ]);
                offset += 8;

                Ok((Arc::new(LongValue::new(name, value)?), offset))
            }

            ValueType::String => {
                // Deserialize StringValue (type 12)
                // Format: [type:1][name_len:4][name][value_size:4][string_bytes]
                if data.len() < 13 {
                    return Err(ContainerError::InvalidDataFormat(
                        "Insufficient data for StringValue".to_string()
                    ));
                }

                let mut offset = 1;

                // Name length
                let name_len = u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
                offset += 4;

                // Name
                let name = String::from_utf8(data[offset..offset + name_len].to_vec())
                    .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid UTF-8: {}", e)))?;
                offset += name_len;

                // Value size
                let value_size = u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
                offset += 4;

                // String bytes
                let str_value = String::from_utf8(data[offset..offset + value_size].to_vec())
                    .map_err(|e| ContainerError::InvalidDataFormat(format!("Invalid UTF-8 in string value: {}", e)))?;
                offset += value_size;

                Ok((Arc::new(StringValue::new(name, str_value)), offset))
            }

            _ => {
                Err(ContainerError::InvalidDataFormat(
                    format!("Unsupported value type for deserialization: {:?}", type_id)
                ))
            }
        }
    }
}

impl Value for ArrayValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Array
    }

    fn to_string(&self) -> String {
        self.serialize_text()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn size(&self) -> usize {
        // Calculate total size: count (4 bytes) + all elements
        let mut total = 4; // count size
        for element in &self.elements {
            total += element.size();
        }
        total
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Serialize to binary format
        let mut bytes = Vec::new();

        // Add count (4 bytes, little-endian)
        let count = self.elements.len() as u32;
        bytes.extend_from_slice(&count.to_le_bytes());

        // Add all element bytes
        for element in &self.elements {
            bytes.extend_from_slice(&element.to_bytes());
        }

        bytes
    }

    fn to_json(&self) -> Result<String> {
        use serde_json::json;

        let elements_json: Vec<_> = self.elements
            .iter()
            .map(|e| e.to_json())
            .collect::<Result<Vec<_>>>()?;

        let obj = json!({
            "name": self.name,
            "type": "array",
            "elements": elements_json
        });

        Ok(obj.to_string())
    }

    fn to_xml(&self) -> Result<String> {
        let mut xml = format!(r#"<array name="{}" count="{}">"#, self.name, self.elements.len());

        for element in &self.elements {
            xml.push_str(&element.to_xml()?);
        }

        xml.push_str("</array>");
        Ok(xml)
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }
}

impl fmt::Display for ArrayValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array({} elements)", self.elements.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::IntValue;

    #[test]
    fn test_empty_array() {
        let array = ArrayValue::new("test", vec![]);
        assert_eq!(array.count(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_with_elements() {
        let elem1 = Arc::new(IntValue::new("", 10));
        let elem2 = Arc::new(IntValue::new("", 20));
        let array = ArrayValue::new("numbers", vec![elem1, elem2]);

        assert_eq!(array.count(), 2);
        assert!(!array.is_empty());
    }

    #[test]
    fn test_push() {
        let mut array = ArrayValue::new("test", vec![]);
        array.push(Arc::new(IntValue::new("", 42)));
        assert_eq!(array.count(), 1);
    }

    #[test]
    fn test_at() {
        let elem1 = Arc::new(IntValue::new("", 10));
        let elem2 = Arc::new(IntValue::new("", 20));
        let array = ArrayValue::new("test", vec![elem1, elem2]);

        assert!(array.at(0).is_some());
        assert!(array.at(1).is_some());
        assert!(array.at(2).is_none());
    }

    #[test]
    fn test_clear() {
        let mut array = ArrayValue::new("test", vec![Arc::new(IntValue::new("", 1))]);
        assert_eq!(array.count(), 1);

        array.clear();
        assert_eq!(array.count(), 0);
    }

    #[test]
    fn test_display() {
        let array = ArrayValue::new("test", vec![Arc::new(IntValue::new("", 1))]);
        let display = format!("{}", array);
        assert!(display.contains("Array(1 elements)"));
    }

    #[test]
    fn test_binary_serialization() {
        use crate::values::StringValue;

        // Create array with mixed types
        let mut array = ArrayValue::new("test_array", vec![]);
        array.push(Arc::new(IntValue::new("", 42)));
        array.push(Arc::new(IntValue::new("", 100)));
        array.push(Arc::new(StringValue::new("", "hello")));

        // Serialize to binary
        let binary_data = array.to_binary_bytes();
        assert!(!binary_data.is_empty());

        // Verify type byte (ArrayValue = 15)
        assert_eq!(binary_data[0], 15);

        // Deserialize
        let restored = ArrayValue::deserialize_binary(&binary_data).unwrap();

        // Verify name
        assert_eq!(restored.name(), "test_array");

        // Verify count
        assert_eq!(restored.count(), 3);

        // Verify elements
        let elem0 = restored.at(0).unwrap();
        assert_eq!(elem0.to_int().unwrap(), 42);

        let elem1 = restored.at(1).unwrap();
        assert_eq!(elem1.to_int().unwrap(), 100);

        let elem2 = restored.at(2).unwrap();
        assert_eq!(elem2.to_string(), "hello");
    }

    #[test]
    fn test_binary_roundtrip() {
        let elem1 = Arc::new(IntValue::new("", 1));
        let elem2 = Arc::new(IntValue::new("", 2));
        let elem3 = Arc::new(IntValue::new("", 3));

        let original = ArrayValue::new("numbers", vec![elem1, elem2, elem3]);

        // Serialize
        let data = original.to_binary_bytes();

        // Deserialize
        let restored = ArrayValue::deserialize_binary(&data).unwrap();

        // Verify
        assert_eq!(original.name(), restored.name());
        assert_eq!(original.count(), restored.count());

        for i in 0..original.count() {
            let orig_elem = original.at(i).unwrap();
            let rest_elem = restored.at(i).unwrap();
            assert_eq!(orig_elem.to_int().unwrap(), rest_elem.to_int().unwrap());
        }
    }

    #[test]
    fn test_binary_empty_array() {
        let empty = ArrayValue::new("empty", vec![]);

        // Serialize
        let data = empty.to_binary_bytes();

        // Deserialize
        let restored = ArrayValue::deserialize_binary(&data).unwrap();

        assert_eq!(restored.count(), 0);
        assert!(restored.is_empty());
    }

    #[test]
    fn test_binary_format_structure() {
        // Test that binary format matches C++ specification
        let array = ArrayValue::new("test", vec![Arc::new(IntValue::new("", 123))]);

        let data = array.to_binary_bytes();

        // Verify type byte (ArrayValue = 15)
        assert_eq!(data[0], 15);

        // Verify name length (4 bytes, little-endian)
        let name_len = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
        assert_eq!(name_len, 4); // "test" = 4 bytes

        // Verify name
        let name = String::from_utf8(data[5..9].to_vec()).unwrap();
        assert_eq!(name, "test");
    }

    #[test]
    fn test_binary_invalid_data() {
        // Test with insufficient data
        let too_short = vec![15, 0, 0, 0, 0]; // Just type + name_len
        let result = ArrayValue::deserialize_binary(&too_short);
        assert!(result.is_err());

        // Test with wrong type
        let mut wrong_type = vec![0u8; 20];
        wrong_type[0] = 3; // IntValue instead of ArrayValue
        let result = ArrayValue::deserialize_binary(&wrong_type);
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_large_array() {
        // Test with larger array to ensure offset calculations are correct
        const SIZE: usize = 100;

        let mut array = ArrayValue::new("large", vec![]);
        for i in 0..SIZE {
            array.push(Arc::new(IntValue::new("", i as i32)));
        }

        let data = array.to_binary_bytes();
        let restored = ArrayValue::deserialize_binary(&data).unwrap();

        assert_eq!(restored.count(), SIZE);

        // Spot check a few elements
        let check_indices = [0, SIZE / 4, SIZE / 2, SIZE * 3 / 4, SIZE - 1];
        for &idx in &check_indices {
            let elem = restored.at(idx).unwrap();
            assert_eq!(elem.to_int().unwrap(), idx as i32);
        }
    }

    #[test]
    fn test_binary_compatibility() {
        use crate::values::StringValue;

        // This test creates binary data that matches the C++ format exactly
        let mut array = ArrayValue::new("colors", vec![]);
        array.push(Arc::new(StringValue::new("", "red")));
        array.push(Arc::new(StringValue::new("", "blue")));

        // Serialize
        let data = array.to_binary_bytes();

        // Verify we can deserialize it back
        let restored = ArrayValue::deserialize_binary(&data).unwrap();

        // Verify structure
        assert_eq!(restored.name(), "colors");
        assert_eq!(restored.count(), 2);

        // Verify elements
        let elem0 = restored.at(0).unwrap();
        assert_eq!(elem0.to_string(), "red");

        let elem1 = restored.at(1).unwrap();
        assert_eq!(elem1.to_string(), "blue");
    }
}
