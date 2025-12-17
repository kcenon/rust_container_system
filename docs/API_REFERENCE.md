# API Reference

> **Version**: 0.1.0
> **Last Updated**: 2025-12-17

This document provides a complete reference for the Rust Container System API.

## Table of Contents

- [Quick Start](#quick-start)
- [Module Structure](#module-structure)
- [Core Types](#core-types)
  - [ValueContainer](#valuecontainer)
  - [ValueContainerBuilder](#valuecontainerbuilder)
  - [Value Trait](#value-trait)
  - [ValueType Enum](#valuetype-enum)
  - [ContainerError](#containererror)
- [Value Types](#value-types)
  - [Primitive Types](#primitive-types)
  - [StringValue](#stringvalue)
  - [BytesValue](#bytesvalue)
  - [ContainerValue](#containervalue)
  - [ArrayValue](#arrayvalue)
- [Dependency Injection (kcenon)](#dependency-injection-kcenon)
  - [ContainerFactory Trait](#containerfactory-trait)
  - [DefaultContainerFactory](#defaultcontainerfactory)
  - [ArcContainerProvider](#arccontainerprovider)
- [Messaging Module](#messaging-module)
  - [MessagingContainerBuilder](#messagingcontainerbuilder)
- [Serialization](#serialization)
- [Thread Safety](#thread-safety)
- [Constants](#constants)

---

## Quick Start

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a container
    let mut container = ValueContainer::builder()
        .source("client", "session_1")
        .target("server", "handler")
        .message_type("user_data")
        .build();

    // Add values
    container.add_value(Arc::new(IntValue::new("user_id", 12345)))?;
    container.add_value(Arc::new(StringValue::new("username", "alice")))?;

    // Retrieve values
    if let Some(user_id) = container.get_value("user_id") {
        println!("User ID: {}", user_id.to_int()?);
    }

    // Serialize
    let wire_data = container.serialize_cpp_wire()?;

    Ok(())
}
```

---

## Module Structure

```rust
// Prelude import (recommended)
use rust_container_system::prelude::*;

// Or explicit imports
use rust_container_system::core::{
    ValueContainer,
    ValueContainerBuilder,
    Value,
    ValueType,
    ContainerError,
    Result,
};

use rust_container_system::values::{
    BoolValue, ShortValue, UShortValue, IntValue, UIntValue,
    LongValue, ULongValue, LLongValue, ULLongValue,
    FloatValue, DoubleValue, StringValue, BytesValue,
    ContainerValue, ArrayValue,
};

// Dependency Injection (kcenon module)
use rust_container_system::kcenon::{
    ContainerFactory,
    DefaultContainerFactory,
    DefaultContainerFactoryBuilder,
    ArcContainerProvider,
    ArcContainerProviderBuilder,
};

// Messaging module
use rust_container_system::messaging::MessagingContainerBuilder;
```

---

## Core Types

### ValueContainer

The main container type for storing and managing values.

```rust
pub struct ValueContainer {
    // Internal: Arc<RwLock<ContainerInner>>
}
```

#### Constructors

| Method | Description | Example |
|--------|-------------|---------|
| `new()` | Create empty container | `ValueContainer::new()` |
| `with_max_values(n)` | Create with value limit | `ValueContainer::with_max_values(1000)` |
| `with_message_type(s)` | Create with message type | `ValueContainer::with_message_type("event")` |
| `builder()` | Create builder | `ValueContainer::builder()` |

```rust
// Default constructor
let container = ValueContainer::new();

// With value limit (prevents memory exhaustion)
let limited = ValueContainer::with_max_values(1000);

// With message type
let typed = ValueContainer::with_message_type("user_request");

// Builder pattern
let built = ValueContainer::builder()
    .source("client", "session")
    .target("server", "handler")
    .message_type("data")
    .max_values(500)
    .build();
```

#### Header Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `set_source(id, sub_id)` | `()` | Set source information |
| `set_target(id, sub_id)` | `()` | Set target information |
| `set_message_type(type)` | `()` | Set message type |
| `source_id()` | `String` | Get source ID (clones) |
| `source_sub_id()` | `String` | Get source sub ID (clones) |
| `target_id()` | `String` | Get target ID (clones) |
| `target_sub_id()` | `String` | Get target sub ID (clones) |
| `message_type()` | `String` | Get message type (clones) |
| `version()` | `String` | Get version (clones) |
| `swap_header()` | `()` | Swap source and target |

```rust
let mut container = ValueContainer::new();

// Set header info
container.set_source("client_01", "session_abc");
container.set_target("server_main", "handler_1");
container.set_message_type("user_request");

// Get header info
println!("From: {}/{}", container.source_id(), container.source_sub_id());
println!("To: {}/{}", container.target_id(), container.target_sub_id());

// Swap for response
container.swap_header();
```

#### Zero-Copy Header Access

For performance-critical code, use callback-based methods to avoid string cloning:

```rust
// Avoid cloning with callback pattern
container.with_source_id(|id| {
    println!("Source: {}", id);
});

container.with_message_type_ref(|msg_type| {
    if msg_type == "urgent" {
        // Handle urgent message
    }
});
```

| Method | Description |
|--------|-------------|
| `with_source_id(f)` | Access source_id without cloning |
| `with_source_sub_id(f)` | Access source_sub_id without cloning |
| `with_target_id(f)` | Access target_id without cloning |
| `with_target_sub_id(f)` | Access target_sub_id without cloning |
| `with_message_type_ref(f)` | Access message_type without cloning |
| `with_version_ref(f)` | Access version without cloning |

#### Value Operations

| Method | Returns | Description |
|--------|---------|-------------|
| `add_value(value)` | `Result<()>` | Add a value |
| `try_add_value(value)` | `bool` | Add value, returns false if limit reached |
| `get_value(name)` | `Option<Arc<dyn Value>>` | Get first value by name |
| `get_value_array(name)` | `Vec<Arc<dyn Value>>` | Get all values by name |
| `values()` | `Vec<Arc<dyn Value>>` | Get all values (clones Vec) |
| `remove_value(name)` | `bool` | Remove all values with name |
| `clear_values()` | `()` | Remove all values |
| `value_count()` | `usize` | Get number of values |
| `is_empty()` | `bool` | Check if container has no values |

```rust
let mut container = ValueContainer::new();

// Add values
container.add_value(Arc::new(IntValue::new("count", 42)))?;
container.add_value(Arc::new(StringValue::new("name", "Alice")))?;

// Multiple values with same name
container.add_value(Arc::new(IntValue::new("tag", 1)))?;
container.add_value(Arc::new(IntValue::new("tag", 2)))?;
container.add_value(Arc::new(IntValue::new("tag", 3)))?;

// Get single value
if let Some(count) = container.get_value("count") {
    println!("Count: {}", count.to_int()?);
}

// Get all values with same name
let tags = container.get_value_array("tag");
for tag in tags {
    println!("Tag: {}", tag.to_int()?);
}

// Remove values
container.remove_value("tag"); // Removes all "tag" values
```

#### Zero-Copy Value Access

```rust
// Access values without cloning the Vec
let total = container.with_values(|values| {
    values.iter()
        .filter_map(|v| v.to_int().ok())
        .sum::<i32>()
});

// Access specific name's values without cloning
let tag_count = container.with_value_array("tag", |values| values.len())
    .unwrap_or(0);
```

#### Copying

```rust
// Copy header only
let header_only = container.copy(false);

// Full copy including values
let full_copy = container.copy(true);
```

#### Iteration

```rust
// For loop iteration
for value in &container {
    println!("{}: {}", value.name(), value.to_string());
}

// Iterator methods
let names: Vec<String> = (&container).into_iter()
    .map(|v| v.name().to_string())
    .collect();
```

---

### ValueContainerBuilder

Fluent builder for constructing containers.

```rust
let container = ValueContainer::builder()
    .source("sender", "session_1")
    .target("receiver", "main")
    .message_type("event_data")
    .max_values(1000)
    .build();
```

| Method | Description |
|--------|-------------|
| `source(id, sub_id)` | Set source information |
| `target(id, sub_id)` | Set target information |
| `message_type(type)` | Set message type |
| `max_values(n)` | Set maximum values limit |
| `build()` | Create the container |

---

### Value Trait

Common interface for all value types.

```rust
pub trait Value: Debug + Send + Sync {
    // Identity
    fn name(&self) -> &str;
    fn value_type(&self) -> ValueType;
    fn size(&self) -> usize;

    // Type checking
    fn is_null(&self) -> bool;
    fn is_bytes(&self) -> bool;
    fn is_boolean(&self) -> bool;
    fn is_numeric(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_container(&self) -> bool;

    // Type conversion (returns Result)
    fn to_bool(&self) -> Result<bool>;
    fn to_short(&self) -> Result<i16>;
    fn to_ushort(&self) -> Result<u16>;
    fn to_int(&self) -> Result<i32>;
    fn to_uint(&self) -> Result<u32>;
    fn to_long(&self) -> Result<i64>;
    fn to_ulong(&self) -> Result<u64>;
    fn to_float(&self) -> Result<f32>;
    fn to_double(&self) -> Result<f64>;

    // Serialization
    fn to_string(&self) -> String;
    fn to_bytes(&self) -> Vec<u8>;
    fn to_json(&self) -> Result<String>;
    fn to_xml(&self) -> Result<String>;

    // Cloning
    fn clone_value(&self) -> Arc<dyn Value>;
    fn as_any(&self) -> &dyn Any;
}
```

#### Usage Example

```rust
fn process_value(value: &dyn Value) {
    println!("Name: {}", value.name());
    println!("Type: {}", value.value_type());

    if value.is_numeric() {
        if let Ok(n) = value.to_long() {
            println!("Numeric value: {}", n);
        }
    } else if value.is_string() {
        println!("String value: {}", value.to_string());
    }
}
```

---

### ValueType Enum

Enumeration of all supported value types.

```rust
pub enum ValueType {
    Null,       // Type code: 0
    Bool,       // Type code: 1
    Short,      // Type code: 2
    UShort,     // Type code: 3
    Int,        // Type code: 4
    UInt,       // Type code: 5
    Long,       // Type code: 6
    ULong,      // Type code: 7
    LLong,      // Type code: 8
    ULLong,     // Type code: 9
    Float,      // Type code: 10
    Double,     // Type code: 11
    String,     // Type code: 12
    Bytes,      // Type code: 13
    Container,  // Type code: 14
    Array,      // Type code: 15
}
```

#### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `to_str()` | `&'static str` | Get type code as string ("0", "1", etc.) |
| `from_type_code(s)` | `Option<ValueType>` | Parse from type code string |
| `is_numeric()` | `bool` | Check if numeric type |

```rust
let vtype = ValueType::Int;
println!("Type code: {}", vtype.to_str()); // "4"
println!("Is numeric: {}", vtype.is_numeric()); // true

// Parse from string
let parsed = ValueType::from_type_code("4"); // Some(ValueType::Int)
```

---

### ContainerError

Error types for container operations.

```rust
pub enum ContainerError {
    /// Value not found by name
    ValueNotFound(String),

    /// Invalid type conversion attempted
    InvalidTypeConversion { from: String, to: String },

    /// Invalid data format (serialization/deserialization)
    InvalidDataFormat(String),

    /// Numeric value out of range
    ValueOutOfRange { type_name: String, value: String, min: String, max: String },

    /// JSON serialization error
    JsonError(serde_json::Error),

    /// XML serialization error
    XmlError(String),

    /// I/O error
    IoError(std::io::Error),
}
```

#### Error Handling

```rust
use rust_container_system::prelude::*;

fn process_container(container: &ValueContainer) -> Result<i32> {
    let value = container.get_value("count")
        .ok_or_else(|| ContainerError::ValueNotFound("count".to_string()))?;

    value.to_int()
}

// Usage
match process_container(&container) {
    Ok(count) => println!("Count: {}", count),
    Err(ContainerError::ValueNotFound(name)) => {
        eprintln!("Missing value: {}", name);
    }
    Err(ContainerError::InvalidTypeConversion { from, to }) => {
        eprintln!("Cannot convert {} to {}", from, to);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Value Types

### Type Summary

| Type | Rust Type | Size | Range | Type Code |
|------|-----------|------|-------|-----------|
| BoolValue | `bool` | 1 byte | true/false | 1 |
| ShortValue | `i16` | 2 bytes | -32,768 to 32,767 | 2 |
| UShortValue | `u16` | 2 bytes | 0 to 65,535 | 3 |
| IntValue | `i32` | 4 bytes | -2³¹ to 2³¹-1 | 4 |
| UIntValue | `u32` | 4 bytes | 0 to 2³²-1 | 5 |
| LongValue | `i64` | 8 bytes | C++ compatible range | 6 |
| ULongValue | `u64` | 8 bytes | C++ compatible range | 7 |
| LLongValue | `i64` | 8 bytes | Full i64 range | 8 |
| ULLongValue | `u64` | 8 bytes | Full u64 range | 9 |
| FloatValue | `f32` | 4 bytes | IEEE 754 | 10 |
| DoubleValue | `f64` | 8 bytes | IEEE 754 | 11 |
| StringValue | `String` | Variable | UTF-8 | 12 |
| BytesValue | `Vec<u8>` | Variable | Binary | 13 |
| ContainerValue | Nested | Variable | Nested values | 14 |
| ArrayValue | `Vec<Arc<dyn Value>>` | Variable | Heterogeneous | 15 |

### Primitive Types

#### BoolValue

```rust
// Construction
let bool_val = BoolValue::new("flag", true);
let bool_val = BoolValue::from(("flag", true));

// Access
assert!(bool_val.to_bool()?);
assert_eq!(bool_val.to_string(), "true");
```

#### Integer Types

```rust
// ShortValue (i16)
let short_val = ShortValue::new("temperature", -10);

// UShortValue (u16)
let ushort_val = UShortValue::new("port", 8080);

// IntValue (i32)
let int_val = IntValue::new("count", 42);
let int_val = IntValue::from(("count", 42));

// UIntValue (u32)
let uint_val = UIntValue::new("size", 1024);

// LongValue (i64, C++ compatible range)
let long_val = LongValue::new("timestamp", 1234567890)?; // Returns Result

// ULongValue (u64, C++ compatible range)
let ulong_val = ULongValue::new("file_size", 1_000_000)?; // Returns Result

// LLongValue (full i64 range)
let llong_val = LLongValue::new("big_num", i64::MAX);

// ULLongValue (full u64 range)
let ullong_val = ULLongValue::new("huge_num", u64::MAX);
```

**Note**: `LongValue` and `ULongValue` return `Result` because they enforce C++ compatible ranges for cross-language interoperability.

#### Floating Point Types

```rust
// FloatValue (f32)
let float_val = FloatValue::new("temperature", 36.5);

// DoubleValue (f64)
let double_val = DoubleValue::new("pi", std::f64::consts::PI);
let double_val = DoubleValue::from(("pi", std::f64::consts::PI));
```

### StringValue

```rust
// Construction
let string_val = StringValue::new("name", "Alice");
let string_val = StringValue::from(("name", "Alice"));

// Access
assert_eq!(string_val.to_string(), "Alice");
assert_eq!(string_val.size(), 5); // UTF-8 byte length
```

### BytesValue

```rust
// Construction
let bytes_val = BytesValue::new("data", vec![0xFF, 0xFE, 0xFD]);
let bytes_val = BytesValue::from(("data", vec![0x00, 0x01, 0x02]));

// Access
assert_eq!(bytes_val.size(), 3);
let bytes = bytes_val.to_bytes();

// JSON serialization uses base64 encoding
let json = bytes_val.to_json()?; // Contains base64 string
```

### ContainerValue

Nested container for hierarchical data.

```rust
// Create child values
let child1 = Arc::new(IntValue::new("id", 123));
let child2 = Arc::new(StringValue::new("name", "Alice"));

// Create nested container
let nested = ContainerValue::new("user", vec![child1, child2]);

// Access children
assert_eq!(nested.child_count(), 2);
if let Some(child) = nested.get_child("id") {
    println!("ID: {}", child.to_int()?);
}
```

### ArrayValue

Heterogeneous array of values.

```rust
// Create array elements
let elem1 = Arc::new(IntValue::new("", 10));
let elem2 = Arc::new(IntValue::new("", 20));
let elem3 = Arc::new(IntValue::new("", 30));

// Create array
let array = ArrayValue::new("numbers", vec![elem1, elem2, elem3]);

// Access elements
assert_eq!(array.count(), 3);
for elem in array.elements() {
    println!("Element: {}", elem.to_int()?);
}
```

**See also**: [ARRAY_VALUE_GUIDE.md](ARRAY_VALUE_GUIDE.md) for detailed array documentation.

---

## Dependency Injection (kcenon)

The `kcenon` module provides Dependency Injection support for `ValueContainer` components, aligned with the C++ Kcenon architecture while embracing Rust idioms.

### ContainerFactory Trait

The core abstraction for container creation, enabling dependency injection and testability.

```rust
pub trait ContainerFactory: Send + Sync {
    /// Create a new ValueContainer with default settings
    fn create(&self) -> ValueContainer;

    /// Create a new ValueContainer with specified message type
    fn create_with_type(&self, message_type: &str) -> ValueContainer;

    /// Create a new ValueContainer with full header configuration
    fn create_with_header(
        &self,
        source_id: &str,
        source_sub_id: &str,
        target_id: &str,
        target_sub_id: &str,
        message_type: &str,
    ) -> ValueContainer;
}
```

#### Custom Factory Implementation

```rust
use rust_container_system::kcenon::ContainerFactory;
use rust_container_system::core::ValueContainer;

struct CustomFactory {
    prefix: String,
}

impl ContainerFactory for CustomFactory {
    fn create(&self) -> ValueContainer {
        let mut container = ValueContainer::new();
        container.set_message_type(format!("{}_message", self.prefix));
        container
    }

    fn create_with_type(&self, message_type: &str) -> ValueContainer {
        let mut container = ValueContainer::new();
        container.set_message_type(format!("{}_{}", self.prefix, message_type));
        container
    }
}

let factory = CustomFactory { prefix: "app".to_string() };
let container = factory.create();
assert_eq!(container.message_type(), "app_message");
```

---

### DefaultContainerFactory

Basic implementation with configurable defaults for message type and maximum values.

#### Constructors

| Method | Description |
|--------|-------------|
| `new()` | Create with default settings (message_type: "data_container", max_values: 10,000) |
| `builder()` | Create a builder for custom configuration |
| `default_message_type()` | Get the configured default message type |
| `default_max_values()` | Get the configured default max values |

```rust
use rust_container_system::kcenon::{ContainerFactory, DefaultContainerFactory};

// Basic usage
let factory = DefaultContainerFactory::new();
let container = factory.create();
assert_eq!(container.message_type(), "data_container");

// With builder
let factory = DefaultContainerFactory::builder()
    .with_default_message_type("custom_type")
    .with_default_max_values(500)
    .build();

let container = factory.create();
assert_eq!(container.message_type(), "custom_type");
```

#### DefaultContainerFactoryBuilder

| Method | Description |
|--------|-------------|
| `new()` | Create new builder with defaults |
| `with_default_message_type(type)` | Set default message type |
| `with_default_max_values(n)` | Set default maximum values |
| `build()` | Build the factory |

---

### ArcContainerProvider

A thread-safe container provider designed for `Arc`-based dependency injection. Suitable for scenarios where factory instances need to be shared across threads.

#### Constructors

| Method | Description |
|--------|-------------|
| `new()` | Create with default settings |
| `with_factory(factory)` | Create with custom DefaultContainerFactory |
| `builder()` | Create a builder for custom configuration |

```rust
use rust_container_system::kcenon::{ContainerFactory, ArcContainerProvider};
use std::sync::Arc;

// Create a shared provider
let provider: Arc<dyn ContainerFactory> = Arc::new(ArcContainerProvider::new());

// Share across threads
let provider_for_thread = Arc::clone(&provider);
std::thread::spawn(move || {
    let container = provider_for_thread.create();
    // Use container...
});

// Use in main thread
let container = provider.create();
```

#### ArcContainerProviderBuilder

| Method | Description |
|--------|-------------|
| `new()` | Create new builder |
| `with_default_message_type(type)` | Set default message type |
| `with_default_max_values(n)` | Set default maximum values |
| `build()` | Build the provider |

```rust
use rust_container_system::kcenon::{ContainerFactory, ArcContainerProvider};

let provider = ArcContainerProvider::builder()
    .with_default_message_type("service_message")
    .with_default_max_values(100)
    .build();

let container = provider.create();
assert_eq!(container.message_type(), "service_message");
```

---

## Messaging Module

The `messaging` module provides builder patterns aligned with the C++ container_system architecture for creating `ValueContainer` instances with messaging-specific header configurations.

### MessagingContainerBuilder

A fluent builder for constructing `ValueContainer` instances with messaging headers.

#### Methods

| Method | Description |
|--------|-------------|
| `new()` | Create a new builder with default values |
| `with_source(id, sub_id)` | Set source (sender) information |
| `with_target(id, sub_id)` | Set target (receiver) information |
| `with_type(type_name)` | Set the message type |
| `with_max_values(count)` | Set maximum values limit |
| `build()` | Build the ValueContainer |

#### Default Values

- Source ID/Sub-ID: empty strings
- Target ID/Sub-ID: empty strings
- Message type: "data_container"
- Max values: DEFAULT_MAX_VALUES (10,000)

#### Basic Usage

```rust
use rust_container_system::messaging::MessagingContainerBuilder;

let container = MessagingContainerBuilder::new()
    .with_source("client", "session_1")
    .with_target("server", "main")
    .with_type("request")
    .with_max_values(500)
    .build();

assert_eq!(container.source_id(), "client");
assert_eq!(container.source_sub_id(), "session_1");
assert_eq!(container.target_id(), "server");
assert_eq!(container.target_sub_id(), "main");
assert_eq!(container.message_type(), "request");
```

#### Adding Values After Build

```rust
use rust_container_system::messaging::MessagingContainerBuilder;
use rust_container_system::values::{IntValue, StringValue};
use std::sync::Arc;

let mut container = MessagingContainerBuilder::new()
    .with_source("app", "instance_1")
    .with_type("data_transfer")
    .build();

container.add_value(Arc::new(IntValue::new("count", 42)))?;
container.add_value(Arc::new(StringValue::new("name", "test_value")))?;

assert_eq!(container.value_count(), 2);
```

---

## Serialization

### JSON Serialization (Deprecated)

> **Note**: JSON/XML methods are deprecated. Use `serialize_cpp_wire()` for cross-language compatibility.

```rust
#[allow(deprecated)]
let json = container.to_json()?;

#[allow(deprecated)]
let restored = ValueContainer::from_json(&json)?;
```

### XML Serialization (Deprecated)

```rust
#[allow(deprecated)]
let xml = container.to_xml()?;
```

### Wire Protocol (Recommended)

Cross-language compatible binary protocol:

```rust
// Serialize
let wire_data = container.serialize_cpp_wire()?;

// Deserialize
let restored = ValueContainer::deserialize_cpp_wire(&wire_data)?;
```

### JSON v2 Adapter

For JSON v2.0 format compatibility:

```rust
use rust_container_system::prelude::*;

// Serialize to JSON v2.0
let json = JsonV2Adapter::serialize(&container)?;

// Deserialize from JSON v2.0
let restored = JsonV2Adapter::deserialize(&json)?;
```

---

## Thread Safety

`ValueContainer` uses `Arc<RwLock<T>>` for thread-safe access:

```rust
use std::thread;
use std::sync::Arc;

let container = Arc::new(ValueContainer::new());

// Clone for thread (O(1) operation)
let container_clone = Arc::clone(&container);

// Reader thread
let reader = thread::spawn(move || {
    if let Some(value) = container_clone.get_value("data") {
        println!("Value: {}", value.to_string());
    }
});

// Writer thread can use another clone
// Note: For writes, you need mutable access pattern

reader.join().unwrap();
```

### Performance Characteristics

| Operation | Lock Type | Time Complexity |
|-----------|-----------|-----------------|
| `get_value()` | Read | O(1) |
| `add_value()` | Write | O(1) amortized |
| `remove_value()` | Write | O(n) |
| `clone()` | None | O(1) (Arc ref count) |
| `copy(true)` | Read | O(n) (deep copy) |

---

## Constants

```rust
/// Default maximum values per container (10,000)
pub const DEFAULT_MAX_VALUES: usize = 10_000;

/// Absolute maximum values (100,000)
pub const ABSOLUTE_MAX_VALUES: usize = 100_000;
```

---

## See Also

- [Features Guide](FEATURES.md) - Detailed feature documentation
- [Best Practices](guides/BEST_PRACTICES.md) - Recommended usage patterns
- [Troubleshooting](guides/TROUBLESHOOTING.md) - Common issues
- [Benchmarks](BENCHMARKS.md) - Performance data

---

*For the latest API documentation, run `cargo doc --open`.*
