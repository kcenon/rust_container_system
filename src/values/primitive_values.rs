//! Primitive value implementations (Boolean and numeric types).

use crate::core::{ContainerError, Result, Value, ValueType};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;

/// Boolean value (true/false)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolValue {
    name: String,
    value: bool,
}

impl BoolValue {
    pub fn new(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl Value for BoolValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Bool
    }

    fn size(&self) -> usize {
        1
    }

    fn to_bool(&self) -> Result<bool> {
        Ok(self.value)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![if self.value { 1 } else { 0 }]
    }

    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information
        let tagged = serde_json::json!({
            "type": "bool",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<bool>{}</bool>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 32-bit signed integer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntValue {
    name: String,
    value: i32,
}

impl IntValue {
    pub fn new(name: impl Into<String>, value: i32) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl Value for IntValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Int
    }

    fn size(&self) -> usize {
        4
    }

    fn to_int(&self) -> Result<i32> {
        Ok(self.value)
    }

    fn to_long(&self) -> Result<i64> {
        Ok(self.value as i64)
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information (i32 vs i64)
        let tagged = serde_json::json!({
            "type": "int",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<int>{}</int>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 64-bit signed integer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongValue {
    name: String,
    value: i64,
}

impl LongValue {
    pub fn new(name: impl Into<String>, value: i64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

impl Value for LongValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::LLong
    }

    fn size(&self) -> usize {
        8
    }

    fn to_long(&self) -> Result<i64> {
        Ok(self.value)
    }

    fn to_int(&self) -> Result<i32> {
        self.value
            .try_into()
            .map_err(|_| ContainerError::InvalidTypeConversion {
                from: "i64".to_string(),
                to: "i32".to_string(),
            })
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information (i32 vs i64)
        let tagged = serde_json::json!({
            "type": "long",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<long>{}</long>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 16-bit signed integer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortValue {
    name: String,
    value: i16,
}

impl ShortValue {
    pub fn new(name: impl Into<String>, value: i16) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> i16 {
        self.value
    }
}

impl Value for ShortValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Short
    }

    fn size(&self) -> usize {
        2
    }

    fn to_int(&self) -> Result<i32> {
        Ok(self.value as i32)
    }

    fn to_long(&self) -> Result<i64> {
        Ok(self.value as i64)
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        let tagged = serde_json::json!({
            "type": "short",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<short>{}</short>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 16-bit unsigned integer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UShortValue {
    name: String,
    value: u16,
}

impl UShortValue {
    pub fn new(name: impl Into<String>, value: u16) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> u16 {
        self.value
    }
}

impl Value for UShortValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::UShort
    }

    fn size(&self) -> usize {
        2
    }

    fn to_int(&self) -> Result<i32> {
        Ok(self.value as i32)
    }

    fn to_long(&self) -> Result<i64> {
        Ok(self.value as i64)
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        let tagged = serde_json::json!({
            "type": "ushort",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<ushort>{}</ushort>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 32-bit unsigned integer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIntValue {
    name: String,
    value: u32,
}

impl UIntValue {
    pub fn new(name: impl Into<String>, value: u32) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl Value for UIntValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::UInt
    }

    fn size(&self) -> usize {
        4
    }

    fn to_int(&self) -> Result<i32> {
        self.value
            .try_into()
            .map_err(|_| ContainerError::InvalidTypeConversion {
                from: "u32".to_string(),
                to: "i32".to_string(),
            })
    }

    fn to_long(&self) -> Result<i64> {
        Ok(self.value as i64)
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        let tagged = serde_json::json!({
            "type": "uint",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<uint>{}</uint>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 64-bit unsigned integer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ULongValue {
    name: String,
    value: u64,
}

impl ULongValue {
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Value for ULongValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::ULLong
    }

    fn size(&self) -> usize {
        8
    }

    fn to_int(&self) -> Result<i32> {
        self.value
            .try_into()
            .map_err(|_| ContainerError::InvalidTypeConversion {
                from: "u64".to_string(),
                to: "i32".to_string(),
            })
    }

    fn to_long(&self) -> Result<i64> {
        self.value
            .try_into()
            .map_err(|_| ContainerError::InvalidTypeConversion {
                from: "u64".to_string(),
                to: "i64".to_string(),
            })
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        let tagged = serde_json::json!({
            "type": "ulong",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<ulong>{}</ulong>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 32-bit floating point (IEEE 754)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatValue {
    name: String,
    value: f32,
}

impl FloatValue {
    pub fn new(name: impl Into<String>, value: f32) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

impl Value for FloatValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Float
    }

    fn size(&self) -> usize {
        4
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value)
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value as f64)
    }

    fn to_int(&self) -> Result<i32> {
        if !self.value.is_finite() {
            return Err(ContainerError::InvalidTypeConversion {
                from: "f32 (non-finite)".to_string(),
                to: "i32".to_string(),
            });
        }

        if self.value > i32::MAX as f32 || self.value < i32::MIN as f32 {
            return Err(ContainerError::InvalidTypeConversion {
                from: format!("f32({})", self.value),
                to: "i32".to_string(),
            });
        }

        Ok(self.value as i32)
    }

    fn to_long(&self) -> Result<i64> {
        if !self.value.is_finite() {
            return Err(ContainerError::InvalidTypeConversion {
                from: "f32 (non-finite)".to_string(),
                to: "i64".to_string(),
            });
        }

        if self.value > i64::MAX as f32 || self.value < i64::MIN as f32 {
            return Err(ContainerError::InvalidTypeConversion {
                from: format!("f32({})", self.value),
                to: "i64".to_string(),
            });
        }

        Ok(self.value as i64)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        let tagged = serde_json::json!({
            "type": "float",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<float>{}</float>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 64-bit floating point (IEEE 754)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleValue {
    name: String,
    value: f64,
}

impl DoubleValue {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Value for DoubleValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn value_type(&self) -> ValueType {
        ValueType::Double
    }

    fn size(&self) -> usize {
        8
    }

    fn to_double(&self) -> Result<f64> {
        Ok(self.value)
    }

    fn to_float(&self) -> Result<f32> {
        Ok(self.value as f32)
    }

    fn to_long(&self) -> Result<i64> {
        // Validate float is finite (not NaN or Infinity)
        if !self.value.is_finite() {
            return Err(ContainerError::InvalidTypeConversion {
                from: "f64 (non-finite)".to_string(),
                to: "i64".to_string(),
            });
        }

        // Check range to prevent undefined behavior
        if self.value > i64::MAX as f64 || self.value < i64::MIN as f64 {
            return Err(ContainerError::InvalidTypeConversion {
                from: format!("f64({})", self.value),
                to: "i64".to_string(),
            });
        }

        Ok(self.value as i64)
    }

    fn to_int(&self) -> Result<i32> {
        // Validate float is finite (not NaN or Infinity)
        if !self.value.is_finite() {
            return Err(ContainerError::InvalidTypeConversion {
                from: "f64 (non-finite)".to_string(),
                to: "i32".to_string(),
            });
        }

        // Check range to prevent undefined behavior
        if self.value > i32::MAX as f64 || self.value < i32::MIN as f64 {
            return Err(ContainerError::InvalidTypeConversion {
                from: format!("f64({})", self.value),
                to: "i32".to_string(),
            });
        }

        Ok(self.value as i32)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    fn to_json(&self) -> Result<String> {
        // Use tagged format to preserve type information (f32 vs f64)
        let tagged = serde_json::json!({
            "type": "double",
            "value": self.value
        });
        serde_json::to_string(&tagged).map_err(Into::into)
    }

    fn to_xml(&self) -> Result<String> {
        Ok(format!("<double>{}</double>", crate::core::xml_escape(&self.value.to_string())))
    }

    fn clone_value(&self) -> Arc<dyn Value> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// From/TryFrom implementations for ergonomic value creation

impl From<(String, bool)> for BoolValue {
    fn from((name, value): (String, bool)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, bool)> for BoolValue {
    fn from((name, value): (&str, bool)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, i32)> for IntValue {
    fn from((name, value): (String, i32)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, i32)> for IntValue {
    fn from((name, value): (&str, i32)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, i64)> for LongValue {
    fn from((name, value): (String, i64)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, i64)> for LongValue {
    fn from((name, value): (&str, i64)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, f64)> for DoubleValue {
    fn from((name, value): (String, f64)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, f64)> for DoubleValue {
    fn from((name, value): (&str, f64)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, i16)> for ShortValue {
    fn from((name, value): (String, i16)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, i16)> for ShortValue {
    fn from((name, value): (&str, i16)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, u16)> for UShortValue {
    fn from((name, value): (String, u16)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, u16)> for UShortValue {
    fn from((name, value): (&str, u16)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, u32)> for UIntValue {
    fn from((name, value): (String, u32)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, u32)> for UIntValue {
    fn from((name, value): (&str, u32)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, u64)> for ULongValue {
    fn from((name, value): (String, u64)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, u64)> for ULongValue {
    fn from((name, value): (&str, u64)) -> Self {
        Self::new(name, value)
    }
}

impl From<(String, f32)> for FloatValue {
    fn from((name, value): (String, f32)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, f32)> for FloatValue {
    fn from((name, value): (&str, f32)) -> Self {
        Self::new(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_from_tuple() {
        let value1: BoolValue = ("test", true).into();
        assert_eq!(value1.name(), "test");
        assert!(value1.value());

        let value2: BoolValue = (String::from("test2"), false).into();
        assert_eq!(value2.name(), "test2");
        assert!(!value2.value());
    }

    #[test]
    fn test_int_from_tuple() {
        let value1: IntValue = ("count", 42).into();
        assert_eq!(value1.name(), "count");
        assert_eq!(value1.value(), 42);

        let value2: IntValue = (String::from("number"), -100).into();
        assert_eq!(value2.name(), "number");
        assert_eq!(value2.value(), -100);
    }

    #[test]
    fn test_long_from_tuple() {
        let value1: LongValue = ("large", 9223372036854775807i64).into();
        assert_eq!(value1.name(), "large");
        assert_eq!(value1.value(), 9223372036854775807i64);

        let value2: LongValue = (String::from("timestamp"), 1234567890i64).into();
        assert_eq!(value2.name(), "timestamp");
        assert_eq!(value2.value(), 1234567890i64);
    }

    #[test]
    fn test_double_from_tuple() {
        let value1: DoubleValue = ("pi", std::f64::consts::PI).into();
        assert_eq!(value1.name(), "pi");
        assert!((value1.value() - std::f64::consts::PI).abs() < f64::EPSILON);

        let value2: DoubleValue = (String::from("temp"), -40.5).into();
        assert_eq!(value2.name(), "temp");
        assert_eq!(value2.value(), -40.5);
    }
}
