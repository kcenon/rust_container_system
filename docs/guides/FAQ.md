# Frequently Asked Questions (FAQ)

> **Version**: 0.1.0
> **Last Updated**: 2025-11-26

Quick answers to common questions about the Rust Container System.

## Table of Contents

- [General Questions](#general-questions)
- [Installation & Setup](#installation--setup)
- [Container Basics](#container-basics)
- [Value Types](#value-types)
- [Serialization](#serialization)
- [Performance](#performance)
- [Thread Safety](#thread-safety)
- [Cross-Language Compatibility](#cross-language-compatibility)
- [Error Handling](#error-handling)
- [Advanced Topics](#advanced-topics)

---

## General Questions

### Q: What is the Rust Container System?

**A:** It's a high-performance, type-safe data container library for Rust. It provides:
- 16 strongly-typed value types
- Thread-safe operations
- Multiple serialization formats (JSON, XML, Wire Protocol)
- Cross-language compatibility with C++, Python, Go, Node.js, and .NET implementations

### Q: When should I use this library?

**A:** Use it when you need:
- Structured data exchange between systems
- Type-safe message passing
- Cross-language data serialization
- Configuration or settings management
- Message queuing payloads

### Q: How does it compare to the C++ version?

**A:** The Rust version provides:
- **Comparable performance** (within 10-20% of C++)
- **Compile-time safety** vs runtime checks
- **No unsafe code** (100% safe Rust)
- **Ergonomic API** with builder patterns and traits
- **Wire protocol compatibility** for data exchange with C++

---

## Installation & Setup

### Q: How do I add this to my project?

**A:** Add to your `Cargo.toml`:

```toml
[dependencies]
rust_container_system = "0.1"
```

Or use cargo:

```bash
cargo add rust_container_system
```

### Q: What's the minimum Rust version?

**A:** Rust 1.90.0 or later.

### Q: What dependencies does it bring?

**A:** Core dependencies:
- `serde` / `serde_json` - JSON serialization
- `quick-xml` - XML serialization
- `parking_lot` - High-performance locks
- `thiserror` - Error handling
- `base64` - Binary encoding
- `indexmap` - Ordered maps

---

## Container Basics

### Q: How do I create a container?

**A:** Three ways:

```rust
use rust_container_system::prelude::*;

// 1. Default constructor
let container = ValueContainer::new();

// 2. With message type
let container = ValueContainer::with_message_type("user_data");

// 3. Builder pattern (recommended)
let container = ValueContainer::builder()
    .source("client", "session_1")
    .target("server", "handler")
    .message_type("request")
    .build();
```

### Q: What are source and target?

**A:** Routing metadata for message-based systems:
- **source_id / source_sub_id**: Sender identification
- **target_id / target_sub_id**: Receiver identification
- **message_type**: Message classification

These are optional but useful for routing and debugging.

### Q: How do I add values?

**A:** Wrap values in `Arc` and add them:

```rust
use std::sync::Arc;

let mut container = ValueContainer::new();
container.add_value(Arc::new(IntValue::new("count", 42)))?;
container.add_value(Arc::new(StringValue::new("name", "Alice")))?;
```

### Q: Can I have multiple values with the same name?

**A:** Yes! Use `get_value_array()` to retrieve all:

```rust
container.add_value(Arc::new(IntValue::new("tag", 1)))?;
container.add_value(Arc::new(IntValue::new("tag", 2)))?;
container.add_value(Arc::new(IntValue::new("tag", 3)))?;

// Get first only
let first = container.get_value("tag");

// Get all
let all_tags = container.get_value_array("tag");
assert_eq!(all_tags.len(), 3);
```

### Q: Is there a limit on values?

**A:** Yes, to prevent memory exhaustion:
- Default: 10,000 values
- Maximum: 100,000 values (absolute cap)

```rust
// Custom limit
let container = ValueContainer::with_max_values(500);

// Or via builder
let container = ValueContainer::builder()
    .max_values(1000)
    .build();
```

---

## Value Types

### Q: What value types are supported?

**A:** 16 types:

| Category | Types |
|----------|-------|
| Boolean | `BoolValue` |
| Integers | `ShortValue`, `UShortValue`, `IntValue`, `UIntValue`, `LongValue`, `ULongValue`, `LLongValue`, `ULLongValue` |
| Floating | `FloatValue`, `DoubleValue` |
| Text | `StringValue` |
| Binary | `BytesValue` |
| Complex | `ContainerValue`, `ArrayValue` |
| Special | Null (via `BaseValue::null()`) |

### Q: What's the difference between Long and LLong?

**A:**
- **LongValue/ULongValue**: Range-checked for C++ compatibility. Returns `Result` on construction.
- **LLongValue/ULLongValue**: Full Rust i64/u64 range. Always succeeds.

```rust
// Range-checked (returns Result)
let long = LongValue::new("val", 1_000_000)?;

// Full range (always succeeds)
let llong = LLongValue::new("val", i64::MAX);
```

### Q: How do I convert between types?

**A:** Use the `to_*` methods:

```rust
let value = container.get_value("count").unwrap();

// Type conversions (return Result)
let as_int: i32 = value.to_int()?;
let as_long: i64 = value.to_long()?;
let as_double: f64 = value.to_double()?;
let as_string: String = value.to_string(); // Always succeeds

// Type checking
if value.is_numeric() {
    println!("It's a number: {}", value.to_long()?);
}
```

### Q: How do I create nested structures?

**A:** Use `ContainerValue`:

```rust
let child1 = Arc::new(IntValue::new("id", 123));
let child2 = Arc::new(StringValue::new("name", "Alice"));
let nested = Arc::new(ContainerValue::new("user", vec![child1, child2]));

container.add_value(nested)?;
```

---

## Serialization

### Q: What serialization formats are supported?

**A:**
| Format | Method | Status | Use Case |
|--------|--------|--------|----------|
| Wire Protocol | `serialize_cpp_wire()` | **Recommended** | Cross-language |
| JSON v2.0 | `JsonV2Adapter` | Recommended | Human-readable cross-language |
| JSON | `to_json()` | Deprecated | Legacy |
| XML | `to_xml()` | Deprecated | Legacy |

### Q: Why is JSON deprecated?

**A:** The native JSON format wasn't cross-language compatible. Use:
- `serialize_cpp_wire()` for binary format
- `JsonV2Adapter` for JSON that works across languages

### Q: How do I serialize for cross-language use?

**A:**
```rust
// Wire protocol (recommended for C++ interop)
let wire_data = container.serialize_cpp_wire()?;
let restored = ValueContainer::deserialize_cpp_wire(&wire_data)?;

// JSON v2.0 (for human-readable format)
let json = JsonV2Adapter::serialize(&container)?;
let restored = JsonV2Adapter::deserialize(&json)?;
```

### Q: How are bytes encoded in JSON?

**A:** As base64 strings:

```rust
let bytes = BytesValue::new("data", vec![0xFF, 0xFE, 0xFD]);
let json = bytes.to_json()?;
// Contains: "//79" (base64 of [0xFF, 0xFE, 0xFD])
```

---

## Performance

### Q: What's the performance like?

**A:** Key benchmarks:

| Operation | Time |
|-----------|------|
| Value Creation | 18-40 ns |
| Container Add | 170 ns |
| HashMap Lookup | 21 ns |
| Container Clone | 10 ns (Arc) |
| JSON Serialize | 1.8 µs/value |
| XML Serialize | 560 ns/value |

### Q: How do I avoid unnecessary allocations?

**A:** Use the callback-based methods:

```rust
// Instead of this (allocates String):
let source = container.source_id();

// Use this (no allocation):
container.with_source_id(|id| {
    println!("Source: {}", id);
});

// Same for values:
container.with_values(|values| {
    for v in values {
        // Process without cloning Vec
    }
});
```

### Q: How do I run benchmarks?

**A:**
```bash
cargo bench
```

Results are in `target/criterion/`.

---

## Thread Safety

### Q: Is it thread-safe?

**A:** Yes! `ValueContainer` uses `Arc<RwLock<T>>` internally:
- Multiple readers can access simultaneously
- Writers get exclusive access
- `clone()` is O(1) (just increments ref count)

### Q: How do I share between threads?

**A:**
```rust
use std::sync::Arc;
use std::thread;

let container = Arc::new(ValueContainer::new());
let container_clone = Arc::clone(&container);

thread::spawn(move || {
    // Safe to read
    if let Some(val) = container_clone.get_value("key") {
        println!("{}", val.to_string());
    }
});
```

### Q: What about writes from multiple threads?

**A:** For writes, you need mutable access. Consider:
- Using a Mutex wrapper for shared mutable access
- Having a single writer thread
- Creating separate containers per thread

---

## Cross-Language Compatibility

### Q: Can I exchange data with C++?

**A:** Yes, use `serialize_cpp_wire()`:

```rust
// Rust
let wire = container.serialize_cpp_wire()?;
// Send `wire` to C++ application

// C++ can read it with container_system library
```

### Q: What about Python, Go, Node.js, .NET?

**A:** All implementations support the wire protocol:

```rust
// Serialize in Rust
let data = container.serialize_cpp_wire()?;

// Can be read by any language implementation
```

### Q: Are there any compatibility issues?

**A:** Known issues:
- Wire protocol nested containers have 2 test failures
- Use JSON/XML for nested structures in cross-language scenarios

---

## Error Handling

### Q: What errors can occur?

**A:** Main error types:

```rust
pub enum ContainerError {
    ValueNotFound(String),              // Value not in container
    InvalidTypeConversion { ... },      // Wrong type conversion
    InvalidDataFormat(String),          // Bad serialization data
    ValueOutOfRange { ... },            // Number out of range
    JsonError(serde_json::Error),       // JSON parsing error
    XmlError(String),                   // XML error
    IoError(std::io::Error),            // I/O error
}
```

### Q: How should I handle errors?

**A:**
```rust
use rust_container_system::prelude::*;

fn process(container: &ValueContainer) -> Result<()> {
    let value = container.get_value("count")
        .ok_or(ContainerError::ValueNotFound("count".to_string()))?;

    let count = value.to_int()?;
    println!("Count: {}", count);
    Ok(())
}

// Usage
match process(&container) {
    Ok(()) => println!("Success"),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Advanced Topics

### Q: How do I implement a custom value type?

**A:** Implement the `Value` trait:

```rust
use rust_container_system::core::{Value, ValueType, Result};
use std::sync::Arc;
use std::any::Any;

#[derive(Debug, Clone)]
struct MyValue {
    name: String,
    data: MyData,
}

impl Value for MyValue {
    fn name(&self) -> &str { &self.name }
    fn value_type(&self) -> ValueType { ValueType::Bytes } // Or appropriate type
    fn size(&self) -> usize { /* size calculation */ }
    fn to_string(&self) -> String { /* string representation */ }
    fn to_bytes(&self) -> Vec<u8> { /* byte representation */ }
    fn to_json(&self) -> Result<String> { /* JSON serialization */ }
    fn to_xml(&self) -> Result<String> { /* XML serialization */ }
    fn clone_value(&self) -> Arc<dyn Value> { Arc::new(self.clone()) }
    fn as_any(&self) -> &dyn Any { self }
}
```

### Q: How do I extend serialization?

**A:** The `JsonV2Adapter` shows how to create custom serialization:

```rust
use rust_container_system::core::json_v2_adapter::JsonV2Adapter;

// Custom format based on JSON v2.0
let json = JsonV2Adapter::serialize(&container)?;
```

### Q: Can I use this in `no_std` environments?

**A:** Not currently. The library requires `std` for:
- `String` and `Vec`
- `Arc` and `RwLock`
- Serialization libraries

---

## Still Have Questions?

- Check [Troubleshooting](TROUBLESHOOTING.md) for common issues
- Read the [API Reference](../API_REFERENCE.md) for detailed docs
- Open an issue on [GitHub](https://github.com/kcenon/rust_container_system/issues)

---

*This FAQ is updated regularly. Last update: 2025-11-26*
