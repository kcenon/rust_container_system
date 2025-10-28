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
}
