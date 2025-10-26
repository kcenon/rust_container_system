//! # Rust Container System
//!
//! A production-ready, high-performance container framework for messaging systems
//! and general-purpose applications.
//!
//! ## Features
//!
//! - Type-safe value system with compile-time checks
//! - Thread-safe operations using parking_lot
//! - Efficient memory management with Arc and RwLock
//! - JSON and XML serialization support
//! - Zero-cost abstractions
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_container_system::prelude::*;
//! use std::sync::Arc;
//!
//! let mut container = ValueContainer::new();
//! container.set_message_type("user_data");
//! container.add_value(Arc::new(IntValue::new("user_id", 12345)));
//! container.add_value(Arc::new(StringValue::new("username", "john_doe")));
//!
//! let json = container.to_json().unwrap();
//! println!("{}", json);
//! ```

/// Core types and traits
pub mod core;

/// Value implementations
pub mod values;

/// Prelude for convenient imports
///
/// ```rust
/// use rust_container_system::prelude::*;
/// use std::sync::Arc;
///
/// let mut container = ValueContainer::new();
/// let value = Arc::new(IntValue::new("test", 42));
/// ```
pub mod prelude {
    pub use crate::core::{ContainerError, Result, Value, ValueContainer, ValueContainerBuilder, ValueIter, ValueType};
    pub use crate::values::{BoolValue, BytesValue, ContainerValue, DoubleValue, FloatValue, IntValue, LongValue, ShortValue, StringValue, UIntValue, ULongValue, UShortValue};
}

/// Re-export core types at root level
pub use core::{ContainerError, Result, Value, ValueContainer, ValueType};

/// Re-export value types at root level
pub use values::{BoolValue, BytesValue, DoubleValue, IntValue, LongValue, StringValue};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_basic_container_operations() {
        let mut container = ValueContainer::new();
        container.set_message_type("test_message");

        let value = Arc::new(values::StringValue::new("test", "value"));
        container.add_value(value).expect("Failed to add value");

        assert_eq!(container.value_count(), 1);
        assert!(!container.is_empty());

        let retrieved = container.get_value("test").unwrap();
        assert_eq!(retrieved.name(), "test");
        assert_eq!(retrieved.to_string(), "value");
    }

    #[test]
    fn test_container_serialization() {
        let mut container = ValueContainer::new();
        container.set_source("source1", "sub1");
        container.set_target("target1", "sub2");
        container.set_message_type("test_type");

        container.add_value(Arc::new(values::IntValue::new("count", 42))).expect("Failed to add value");
        container.add_value(Arc::new(values::StringValue::new("name", "test"))).expect("Failed to add value");

        let json = container.to_json();
        assert!(json.is_ok());

        let xml = container.to_xml();
        assert!(xml.is_ok());
    }
}
