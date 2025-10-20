//! Core module providing foundation types and traits for the container system.
//!
//! ## Modules
//!
//! - `error`: Error types and Result alias
//! - `value`: Value trait definition
//! - `value_types`: ValueType enum (15 value types)
//! - `container`: ValueContainer implementation
//!
//! ## Re-export Pattern
//!
//! ```rust
//! // Simplified imports via re-exports
//! use rust_container_system::core::{ContainerError, Value};
//!
//! // Or use prelude:
//! use rust_container_system::prelude::*;
//!
//! // Instead of deep paths:
//! // use rust_container_system::core::error::ContainerError;
//! // use rust_container_system::core::value::Value;
//! ```

/// Error handling module
pub mod error;

/// Value trait module
pub mod value;

/// Value types enum
pub mod value_types;

/// Container implementation
pub mod container;

/// Re-export error types
///
/// ```rust
/// use rust_container_system::core::{ContainerError, Result};
///
/// fn process_data() -> Result<()> {
///     Err(ContainerError::ValueNotFound("key".to_string()))
/// }
/// ```
pub use error::{ContainerError, Result};

/// Re-export Value trait and BaseValue
///
/// ```rust
/// use rust_container_system::core::{Value, BaseValue, ValueType};
/// use rust_container_system::values::IntValue;
/// use std::sync::Arc;
///
/// // BaseValue is a generic value store
/// let base_value = BaseValue::new("test", ValueType::Int, vec![1, 2, 3, 4]);
///
/// // IntValue implements Value trait
/// let int_value: Arc<dyn Value> = Arc::new(IntValue::new("count", 42));
/// assert_eq!(int_value.name(), "count");
/// ```
pub use value::{BaseValue, Value};

/// Re-export ValueType enum
///
/// ```rust
/// use rust_container_system::core::ValueType;
///
/// let vtype = ValueType::String;
/// assert!(vtype.is_numeric() == false);
/// // to_str() returns the numeric string representation
/// assert_eq!(vtype.to_str(), "13");
/// ```
pub use value_types::ValueType;

/// Re-export ValueContainer, Builder and constants
///
/// ```rust
/// use rust_container_system::core::{ValueContainer, ValueContainerBuilder, DEFAULT_MAX_VALUES};
/// use rust_container_system::values::IntValue;
/// use std::sync::Arc;
///
/// let mut container = ValueContainer::new();
/// container.set_message_type("test");
/// container.add_value(Arc::new(IntValue::new("count", 42)));
///
/// // Or use builder pattern
/// let built = ValueContainer::builder()
///     .source("sender", "1")
///     .target("receiver", "2")
///     .message_type("event")
///     .build();
///
/// // Or create with custom limit
/// let limited = ValueContainer::with_max_values(1000);
/// ```
pub use container::{ValueContainer, ValueContainerBuilder, ValueIter, DEFAULT_MAX_VALUES, ABSOLUTE_MAX_VALUES};

/// XML escape utility function to prevent XML injection attacks
///
/// This function escapes XML special characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&apos;`
///
/// # Examples
///
/// ```rust
/// use rust_container_system::core::xml_escape;
///
/// let safe = xml_escape("<script>alert('xss')</script>");
/// assert_eq!(safe, "&lt;script&gt;alert(&apos;xss&apos;)&lt;/script&gt;");
/// ```
pub fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
