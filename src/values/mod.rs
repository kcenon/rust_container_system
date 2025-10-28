//! Value implementations module.
//!
//! Provides concrete implementations of the Value trait for different data types.
//!
//! ## Modules
//!
//! - `primitive_values`: Bool, Int, Long, Double implementations
//! - `string_value`: UTF-8 string implementation
//! - `bytes_value`: Binary data implementation
//! - `container_value`: Nested container implementation

/// Primitive type implementations (Bool, Short, UShort, Int, UInt, Long, ULong, Float, Double)
pub mod primitive_values;

/// String value implementation
pub mod string_value;

/// Bytes value implementation
pub mod bytes_value;

/// Container value implementation (nested containers)
pub mod container_value;

/// Array value implementation (arrays/lists)
pub mod array_value;

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
    BoolValue, DoubleValue, FloatValue, IntValue, LLongValue, LongValue, ShortValue, UIntValue,
    ULLongValue, ULongValue, UShortValue,
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

/// Re-export container type
///
/// ```rust
/// use rust_container_system::values::{ContainerValue, IntValue, StringValue};
/// use rust_container_system::core::Value;
/// use std::sync::Arc;
///
/// // Create nested structure
/// let child1 = Arc::new(IntValue::new("id", 123));
/// let child2 = Arc::new(StringValue::new("name", "Alice"));
/// let container = Arc::new(ContainerValue::new("user_data", vec![child1, child2]));
///
/// println!("Container has {} children", container.child_count());
/// ```
pub use container_value::ContainerValue;

/// Re-export array type
///
/// ```rust
/// use rust_container_system::values::{ArrayValue, IntValue, StringValue};
/// use rust_container_system::core::Value;
/// use std::sync::Arc;
///
/// // Create array structure
/// let elem1 = Arc::new(IntValue::new("", 10));
/// let elem2 = Arc::new(IntValue::new("", 20));
/// let array = Arc::new(ArrayValue::new("numbers", vec![elem1, elem2]));
///
/// println!("Array has {} elements", array.count());
/// ```
pub use array_value::ArrayValue;
