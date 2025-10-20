//! Value implementations module.
//!
//! Provides concrete implementations of the Value trait for different data types.
//!
//! ## Modules
//!
//! - `primitive_values`: Bool, Int, Long, Double implementations
//! - `string_value`: UTF-8 string implementation
//! - `bytes_value`: Binary data implementation

/// Primitive type implementations (Bool, Short, UShort, Int, UInt, Long, ULong, Float, Double)
pub mod primitive_values;

/// String value implementation
pub mod string_value;

/// Bytes value implementation
pub mod bytes_value;

/// Re-export primitive types
///
/// ```rust
/// use rust_container_system::values::{BoolValue, IntValue, LongValue, DoubleValue};
/// use rust_container_system::core::Value;
/// use std::sync::Arc;
///
/// let values: Vec<Arc<dyn Value>> = vec![
///     Arc::new(BoolValue::new("flag", true)),
///     Arc::new(IntValue::new("count", 10)),
///     Arc::new(LongValue::new("big_num", 1_000_000)),
///     Arc::new(DoubleValue::new("ratio", 3.14)),
/// ];
/// ```
pub use primitive_values::{
    BoolValue, DoubleValue, FloatValue, IntValue, LongValue, ShortValue, UIntValue, ULongValue,
    UShortValue,
};

/// Re-export string type
///
/// ```rust
/// use rust_container_system::values::StringValue;
/// use rust_container_system::core::Value;
/// use std::sync::Arc;
///
/// let name = Arc::new(StringValue::new("username", "alice"));
/// println!("Name: {}", name.to_string());
/// ```
pub use string_value::StringValue;

/// Re-export bytes type
///
/// ```rust
/// use rust_container_system::values::BytesValue;
/// use rust_container_system::core::Value;
/// use std::sync::Arc;
///
/// let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
/// let bytes = Arc::new(BytesValue::new("image", image_data));
/// println!("Size: {} bytes", bytes.size());
/// ```
pub use bytes_value::BytesValue;
