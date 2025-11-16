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

//! Container value implementation for nested structures.

use crate::core::error::{ContainerError, Result};
use crate::core::value::Value;
use crate::core::value_types::ValueType;
use std::any::Any;
use std::fmt;
use std::sync::Arc;

/// A value that contains other values (nested container)
///
/// Allows hierarchical data structures similar to JSON objects.
///
/// # Example
/// ```
/// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
/// use std::sync::Arc;
///
/// // Create child values
/// let child1 = Arc::new(IntValue::new("id", 123));
/// let child2 = Arc::new(StringValue::new("name", "Alice"));
///
/// // Create container with children
/// let container = ContainerValue::new("user_data", vec![child1, child2]);
///
/// assert_eq!(container.child_count(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct ContainerValue {
    name: String,
    children: Vec<Arc<dyn Value>>,
}

impl ContainerValue {
    /// Create a new empty container value
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let container = ContainerValue::new("my_container", vec![]);
    /// assert_eq!(container.child_count(), 0);
    /// ```
    pub fn new(name: impl Into<String>, children: Vec<Arc<dyn Value>>) -> Self {
        Self {
            name: name.into(),
            children,
        }
    }

    /// Create a container with reserved capacity
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let container = ContainerValue::with_capacity("data", 10);
    /// assert_eq!(container.child_count(), 0);
    /// ```
    pub fn with_capacity(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            children: Vec::with_capacity(capacity),
        }
    }

    /// Get the number of child values
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let child = Arc::new(IntValue::new("num", 42));
    /// let container = ContainerValue::new("data", vec![child]);
    /// assert_eq!(container.child_count(), 1);
    /// ```
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Get all children
    pub fn children(&self) -> &[Arc<dyn Value>] {
        &self.children
    }

    /// Add a child value
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let mut container = ContainerValue::new("data", vec![]);
    /// container.add_child(Arc::new(IntValue::new("id", 123)));
    /// assert_eq!(container.child_count(), 1);
    /// ```
    pub fn add_child(&mut self, child: Arc<dyn Value>) {
        self.children.push(child);
    }

    /// Get child by name and index
    ///
    /// Returns the nth occurrence of a child with the given name.
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let child1 = Arc::new(IntValue::new("value", 1));
    /// let child2 = Arc::new(IntValue::new("value", 2));
    /// let container = ContainerValue::new("data", vec![child1, child2]);
    ///
    /// let first = container.get_child("value", 0).unwrap();
    /// assert_eq!(first.to_int().unwrap(), 1);
    ///
    /// let second = container.get_child("value", 1).unwrap();
    /// assert_eq!(second.to_int().unwrap(), 2);
    /// ```
    pub fn get_child(&self, name: &str, index: usize) -> Option<Arc<dyn Value>> {
        let mut count = 0;
        for child in &self.children {
            if child.name() == name {
                if count == index {
                    return Some(Arc::clone(child));
                }
                count += 1;
            }
        }
        None
    }

    /// Get all children with the given name
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let child1 = Arc::new(IntValue::new("tag", 1));
    /// let child2 = Arc::new(IntValue::new("tag", 2));
    /// let child3 = Arc::new(IntValue::new("other", 3));
    /// let container = ContainerValue::new("data", vec![child1, child2, child3]);
    ///
    /// let tags = container.get_children("tag");
    /// assert_eq!(tags.len(), 2);
    /// ```
    pub fn get_children(&self, name: &str) -> Vec<Arc<dyn Value>> {
        self.children
            .iter()
            .filter(|child| child.name() == name)
            .map(Arc::clone)
            .collect()
    }

    /// Remove all children with the given name
    ///
    /// Returns true if any children were removed.
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let child1 = Arc::new(IntValue::new("temp", 1));
    /// let child2 = Arc::new(IntValue::new("keep", 2));
    /// let mut container = ContainerValue::new("data", vec![child1, child2]);
    ///
    /// assert!(container.remove_child("temp"));
    /// assert_eq!(container.child_count(), 1);
    /// assert!(!container.remove_child("temp"));
    /// ```
    pub fn remove_child(&mut self, name: &str) -> bool {
        let original_len = self.children.len();
        self.children.retain(|child| child.name() != name);
        self.children.len() < original_len
    }

    /// Remove all children
    ///
    /// # Example
    /// ```
    /// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
    ///
    /// let child = Arc::new(IntValue::new("data", 42));
    /// let mut container = ContainerValue::new("test", vec![child]);
    /// assert_eq!(container.child_count(), 1);
    ///
    /// container.clear_children();
    /// assert_eq!(container.child_count(), 0);
    /// ```
    pub fn clear_children(&mut self) {
        self.children.clear();
    }
}

impl Value for ContainerValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Container
    }

    fn size(&self) -> usize {
        // Size is the sum of all children sizes plus overhead
        std::mem::size_of::<Self>()
            + self
                .children
                .iter()
                .map(|child| child.size())
                .sum::<usize>()
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_bytes(&self) -> bool {
        false
    }

    fn is_string(&self) -> bool {
        false
    }

    fn is_container(&self) -> bool {
        true
    }

    fn to_bool(&self) -> Result<bool> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "bool".to_string(),
        })
    }

    fn to_short(&self) -> Result<i16> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "i16".to_string(),
        })
    }

    fn to_ushort(&self) -> Result<u16> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "u16".to_string(),
        })
    }

    fn to_int(&self) -> Result<i32> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "i32".to_string(),
        })
    }

    fn to_uint(&self) -> Result<u32> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "u32".to_string(),
        })
    }

    fn to_long(&self) -> Result<i64> {
        // Return child count like C++ version
        Ok(self.children.len() as i64)
    }

    fn to_ulong(&self) -> Result<u64> {
        Ok(self.children.len() as u64)
    }

    fn to_float(&self) -> Result<f32> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "f32".to_string(),
        })
    }

    fn to_double(&self) -> Result<f64> {
        Err(ContainerError::InvalidTypeConversion {
            from: "container".to_string(),
            to: "f64".to_string(),
        })
    }

    fn to_string(&self) -> String {
        format!("[Container '{}' with {} children]", self.name, self.children.len())
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Serialize child count as bytes
        (self.children.len() as i64).to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        let mut json = String::from("{\n");
        json.push_str(&format!("  \"name\": \"{}\",\n", self.name));
        json.push_str(&format!("  \"type\": \"{}\",\n", self.value_type()));
        json.push_str(&format!("  \"child_count\": {},\n", self.children.len()));
        json.push_str("  \"children\": [\n");

        for (i, child) in self.children.iter().enumerate() {
            let child_json = child.to_json()?;
            json.push_str("    ");
            json.push_str(&child_json);
            if i < self.children.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("  ]\n");
        json.push('}');
        Ok(json)
    }

    fn to_xml(&self) -> Result<String> {
        let mut xml = String::from("<container_value>\n");
        xml.push_str(&format!("  <name>{}</name>\n", self.name));
        xml.push_str(&format!("  <type>{}</type>\n", self.value_type()));
        xml.push_str(&format!("  <child_count>{}</child_count>\n", self.children.len()));
        xml.push_str("  <children>\n");

        for child in &self.children {
            let child_xml = child.to_xml()?;
            for line in child_xml.lines() {
                xml.push_str("    ");
                xml.push_str(line);
                xml.push('\n');
            }
        }

        xml.push_str("  </children>\n");
        xml.push_str("</container_value>");
        Ok(xml)
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for ContainerValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ContainerValue({:?} with {} children)",
            self.name,
            self.children.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{IntValue, StringValue};

    #[test]
    fn test_container_value_creation() {
        let container = ContainerValue::new("test", vec![]);
        assert_eq!(container.name(), "test");
        assert_eq!(container.value_type(), ValueType::Container);
        assert_eq!(container.child_count(), 0);
        assert!(container.is_container());
    }

    #[test]
    fn test_container_with_children() {
        let child1 = Arc::new(IntValue::new("id", 123));
        let child2 = Arc::new(StringValue::new("name", "Alice"));

        let container = ContainerValue::new("user", vec![child1, child2]);
        assert_eq!(container.child_count(), 2);
    }

    #[test]
    fn test_add_child() {
        let mut container = ContainerValue::new("data", vec![]);
        assert_eq!(container.child_count(), 0);

        container.add_child(Arc::new(IntValue::new("value", 42)));
        assert_eq!(container.child_count(), 1);
    }

    #[test]
    fn test_get_child() {
        let child1 = Arc::new(IntValue::new("num", 1));
        let child2 = Arc::new(IntValue::new("num", 2));
        let child3 = Arc::new(StringValue::new("text", "hello"));

        let container = ContainerValue::new("data", vec![child1, child2, child3]);

        // Get first "num"
        let first = container.get_child("num", 0).unwrap();
        assert_eq!(first.to_int().unwrap(), 1);

        // Get second "num"
        let second = container.get_child("num", 1).unwrap();
        assert_eq!(second.to_int().unwrap(), 2);

        // Get "text"
        let text = container.get_child("text", 0).unwrap();
        assert_eq!(text.to_string(), "hello");

        // Non-existent
        assert!(container.get_child("missing", 0).is_none());
        assert!(container.get_child("num", 2).is_none());
    }

    #[test]
    fn test_get_children() {
        let child1 = Arc::new(IntValue::new("tag", 1));
        let child2 = Arc::new(IntValue::new("tag", 2));
        let child3 = Arc::new(IntValue::new("other", 3));

        let container = ContainerValue::new("data", vec![child1, child2, child3]);

        let tags = container.get_children("tag");
        assert_eq!(tags.len(), 2);

        let others = container.get_children("other");
        assert_eq!(others.len(), 1);

        let missing = container.get_children("none");
        assert_eq!(missing.len(), 0);
    }

    #[test]
    fn test_remove_child() {
        let child1 = Arc::new(IntValue::new("temp", 1));
        let child2 = Arc::new(IntValue::new("keep", 2));
        let child3 = Arc::new(IntValue::new("temp", 3));

        let mut container = ContainerValue::new("data", vec![child1, child2, child3]);
        assert_eq!(container.child_count(), 3);

        // Remove all "temp"
        assert!(container.remove_child("temp"));
        assert_eq!(container.child_count(), 1);

        // Try to remove again
        assert!(!container.remove_child("temp"));
        assert_eq!(container.child_count(), 1);

        // Only "keep" should remain
        let keep = container.get_child("keep", 0).unwrap();
        assert_eq!(keep.to_int().unwrap(), 2);
    }

    #[test]
    fn test_clear_children() {
        let child = Arc::new(IntValue::new("data", 42));
        let mut container = ContainerValue::new("test", vec![child]);
        assert_eq!(container.child_count(), 1);

        container.clear_children();
        assert_eq!(container.child_count(), 0);
    }

    #[test]
    fn test_to_long_returns_child_count() {
        let child1 = Arc::new(IntValue::new("a", 1));
        let child2 = Arc::new(IntValue::new("b", 2));
        let container = ContainerValue::new("test", vec![child1, child2]);

        assert_eq!(container.to_long().unwrap(), 2);
    }

    #[test]
    fn test_json_serialization() {
        let child1 = Arc::new(IntValue::new("id", 123));
        let child2 = Arc::new(StringValue::new("name", "Bob"));
        let container = ContainerValue::new("user", vec![child1, child2]);

        let json = container.to_json().unwrap();
        assert!(json.contains("\"name\": \"user\""));
        assert!(json.contains("\"child_count\": 2"));
        assert!(json.contains("\"children\""));
    }

    #[test]
    fn test_xml_serialization() {
        let child = Arc::new(IntValue::new("value", 42));
        let container = ContainerValue::new("data", vec![child]);

        let xml = container.to_xml().unwrap();
        assert!(xml.contains("<container_value>"));
        assert!(xml.contains("<name>data</name>"));
        assert!(xml.contains("<child_count>1</child_count>"));
        assert!(xml.contains("<children>"));
    }

    #[test]
    fn test_nested_containers() {
        // Create nested structure
        let inner_child = Arc::new(IntValue::new("deep_value", 99));
        let inner_container = Arc::new(ContainerValue::new("inner", vec![inner_child]));

        let outer_child = Arc::new(StringValue::new("text", "outer"));
        let outer_container =
            ContainerValue::new("outer", vec![inner_container.clone(), outer_child]);

        assert_eq!(outer_container.child_count(), 2);

        // Access nested container
        let nested = outer_container.get_child("inner", 0).unwrap();
        assert!(nested.is_container());

        // Downcast to ContainerValue
        if let Some(nested_cv) = nested.as_any().downcast_ref::<ContainerValue>() {
            assert_eq!(nested_cv.child_count(), 1);
            let deep = nested_cv.get_child("deep_value", 0).unwrap();
            assert_eq!(deep.to_int().unwrap(), 99);
        } else {
            panic!("Failed to downcast to ContainerValue");
        }
    }
}
