//! UTF-8 string value implementation.

use crate::core::{Result, Value, ValueType};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;

/// UTF-8 encoded string value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringValue {
    name: String,
    value: String,
}

impl StringValue {
    /// Create a new string value
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Get string value as &str
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
        self.value.as_bytes().to_vec()
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
        Ok(format!("<string>{}</string>", crate::core::xml_escape(&self.value)))
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
