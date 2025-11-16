# Features Documentation

**Version:** 0.1.0
**Last Updated:** 2025-11-16

This document provides comprehensive documentation of all features in the Rust Container System.

---

## Table of Contents

1. [Core Features](#1-core-features)
   - [Type System](#11-type-system)
   - [Serialization](#12-serialization)
   - [Thread Safety](#13-thread-safety)
2. [Advanced Features](#2-advanced-features)
   - [Nested Structures](#21-nested-structures)
   - [Builder Pattern](#22-builder-pattern)
   - [Iterator Support](#23-iterator-support)
3. [Real-World Examples](#3-real-world-examples)
   - [Messaging System](#31-messaging-system)
   - [Configuration Management](#32-configuration-management)
   - [RPC Communication](#33-rpc-communication)
4. [Cross-Language Compatibility](#4-cross-language-compatibility)

---

## 1. Core Features

### 1.1 Type System

The container system supports **16 strongly-typed value types** with compile-time type safety.

#### 1.1.1 Available Types

| Category | Type | Rust Type | Size | Range/Notes |
|----------|------|-----------|------|-------------|
| **Integers** | Short | `i16` | 2 bytes | -32,768 to 32,767 |
| | UShort | `u16` | 2 bytes | 0 to 65,535 |
| | Int | `i32` | 4 bytes | -2.1B to 2.1B |
| | UInt | `u32` | 4 bytes | 0 to 4.3B |
| | Long | `i64` | 8 bytes | -9.2E18 to 9.2E18 |
| | ULong | `u64` | 8 bytes | 0 to 1.8E19 |
| | LLong | `i64` | 8 bytes | Alias for Long |
| | ULLong | `u64` | 8 bytes | Alias for ULong |
| **Floating Point** | Float | `f32` | 4 bytes | IEEE 754 single |
| | Double | `f64` | 8 bytes | IEEE 754 double |
| **Boolean** | Bool | `bool` | 1 byte | true/false |
| **Text** | String | `String` | Variable | UTF-8 encoded |
| **Binary** | Bytes | `Vec<u8>` | Variable | Raw binary data |
| **Complex** | Container | `ValueContainer` | Variable | Nested container |
| | Array | `Vec<Box<dyn Value>>` | Variable | Heterogeneous array |
| **Special** | Null | - | 0 bytes | Represents absence |

#### 1.1.2 Type Creation Examples

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// Integers
let int_val = Arc::new(IntValue::new("count", 42));
let uint_val = Arc::new(UIntValue::new("id", 12345));
let long_val = Arc::new(LongValue::new("timestamp", 1234567890)?);

// Floating point
let float_val = Arc::new(FloatValue::new("ratio", 3.14));
let double_val = Arc::new(DoubleValue::new("price", 99.99));

// Boolean
let bool_val = Arc::new(BoolValue::new("active", true));

// Text
let string_val = Arc::new(StringValue::new("name", "Alice"));

// Binary
let bytes_val = Arc::new(BytesValue::new("data", vec![0x01, 0x02, 0x03]));

// Null
let null_val = Arc::new(NullValue::new("optional"));
```

#### 1.1.3 Type Conversion and Validation

```rust
// Type checking
if value.value_type() == ValueType::Int {
    let int_val = value.to_int()?;
    println!("Integer value: {}", int_val);
}

// Pattern matching
match value.value_type() {
    ValueType::Int => {
        println!("Found integer: {}", value.to_int()?);
    }
    ValueType::String => {
        println!("Found string: {}", value.to_string());
    }
    ValueType::Container => {
        println!("Found nested container");
    }
    _ => println!("Other type"),
}

// Dynamic downcasting
use std::any::Any;

if let Some(int_val) = value.as_any().downcast_ref::<IntValue>() {
    println!("Direct access to IntValue: {}", int_val.get());
}
```

---

### 1.2 Serialization

Support for multiple serialization formats with automatic format detection.

#### 1.2.1 Supported Formats

| Format | Performance | Use Case | Status |
|--------|-------------|----------|--------|
| **JSON** | 558K ops/s | Web APIs, config files | ⚠️ Deprecated |
| **XML** | 1.79M ops/s | Legacy systems, SOAP | ⚠️ Deprecated |
| **Wire Protocol** | TBD | C++ interoperability | ✅ Recommended |

**Note:** JSON and XML are deprecated in favor of the Wire Protocol for cross-language compatibility.

#### 1.2.2 JSON Serialization (Deprecated)

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();
container.set_message_type("user_data");
container.add_value(Arc::new(IntValue::new("id", 123)))?;
container.add_value(Arc::new(StringValue::new("name", "Alice")))?;

// Serialize to JSON
let json = container.to_json()?;
println!("JSON: {}", json);

// Deserialize from JSON
let restored = ValueContainer::from_json(&json)?;
assert_eq!(restored.get_value("id").unwrap().to_int()?, 123);
```

**Output:**
```json
{
  "header": {
    "message_type": "user_data"
  },
  "values": {
    "id": 123,
    "name": "Alice"
  }
}
```

#### 1.2.3 XML Serialization (Deprecated)

```rust
// Serialize to XML (3x faster than JSON)
let xml = container.to_xml()?;
println!("XML: {}", xml);

// Deserialize from XML
let restored = ValueContainer::from_xml(&xml)?;
```

**Output:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<container>
  <header>
    <message_type>user_data</message_type>
  </header>
  <values>
    <value name="id" type="int">123</value>
    <value name="name" type="string">Alice</value>
  </values>
</container>
```

#### 1.2.4 Wire Protocol (Recommended)

The Wire Protocol is a compact binary format designed for C++ interoperability.

```rust
// Serialize to Wire Protocol
let wire_data = container.to_wire_protocol()?;

// Deserialize from Wire Protocol
let restored = ValueContainer::from_wire_protocol(&wire_data)?;

// Automatic format detection
let data = get_data_from_network(); // Could be JSON, XML, or Wire
let container = ValueContainer::deserialize(&data)?;
```

**Format:** `@header={...};@data={...}`

**Advantages:**
- Compact binary representation
- Compatible with C++ container_system
- Preserves type information
- Faster parsing than text formats

---

### 1.3 Thread Safety

Built-in thread safety using `Arc<RwLock<ContainerInner>>`.

#### 1.3.1 Architecture

```rust
pub struct ValueContainer {
    inner: Arc<RwLock<ContainerInner>>,
}
```

**Benefits:**
- **Compile-time safety:** Rust's type system prevents data races
- **Efficient sharing:** `Arc` provides O(1) cloning
- **Concurrent reads:** Multiple readers can access simultaneously
- **Automatic cleanup:** Reference counting prevents leaks

#### 1.3.2 Concurrent Access Pattern

```rust
use std::thread;
use std::sync::Arc;

let container = ValueContainer::new();
container.add_value(Arc::new(IntValue::new("counter", 0)))?;

// Clone container (cheap Arc clone, ~10ns)
let container_clone1 = container.clone();
let container_clone2 = container.clone();

// Spawn reader threads
let handle1 = thread::spawn(move || {
    // Read access (multiple readers allowed)
    if let Some(value) = container_clone1.get_value("counter") {
        println!("Thread 1: {}", value.to_int().unwrap());
    }
});

let handle2 = thread::spawn(move || {
    // Read access (concurrent with thread 1)
    if let Some(value) = container_clone2.get_value("counter") {
        println!("Thread 2: {}", value.to_int().unwrap());
    }
});

// Wait for threads
handle1.join().unwrap();
handle2.join().unwrap();
```

#### 1.3.3 Performance Characteristics

| Operation | Single Thread | 2 Threads | 4 Threads | 8 Threads |
|-----------|--------------|-----------|-----------|-----------|
| Read | Baseline (50ns) | 1.9× | 3.7× | 7.0× |
| Write | 180ns/value | Sequential | Sequential | Sequential |
| Clone | 10ns (O(1)) | 10ns | 10ns | 10ns |

**Notes:**
- Reads scale linearly (RwLock allows parallel reads)
- Writes are exclusive (only one writer at a time)
- Container cloning is extremely cheap (Arc reference count)

---

## 2. Advanced Features

### 2.1 Nested Structures

Support for unlimited nesting of containers and arrays.

#### 2.1.1 Nested Containers

```rust
// Create inner container
let mut inner_container = ValueContainer::new();
inner_container.set_message_type("inner_data");
inner_container.add_value(Arc::new(IntValue::new("inner_id", 456)))?;

// Create outer container
let mut outer_container = ValueContainer::new();
outer_container.set_message_type("outer_data");
outer_container.add_value(Arc::new(IntValue::new("outer_id", 123)))?;

// Add nested container
outer_container.add_value(Arc::new(
    ContainerValue::new("nested", inner_container)
))?;

// Access nested values
if let Some(nested) = outer_container.get_value("nested") {
    if let Some(container_val) = nested.as_any().downcast_ref::<ContainerValue>() {
        let inner = container_val.get_container();
        let inner_id = inner.get_value("inner_id").unwrap().to_int()?;
        println!("Inner ID: {}", inner_id);
    }
}
```

#### 2.1.2 Heterogeneous Arrays

Arrays can contain values of different types.

```rust
let mut array = ArrayValue::new("mixed_array");

// Add different types to array
array.push(Arc::new(IntValue::new("", 42)))?;
array.push(Arc::new(StringValue::new("", "hello")))?;
array.push(Arc::new(BoolValue::new("", true)))?;

// Nested arrays
let mut inner_array = ArrayValue::new("");
inner_array.push(Arc::new(IntValue::new("", 1)))?;
inner_array.push(Arc::new(IntValue::new("", 2)))?;
array.push(Arc::new(inner_array))?;

container.add_value(Arc::new(array))?;

// Access array elements
if let Some(arr) = container.get_value("mixed_array") {
    if let Some(array_val) = arr.as_any().downcast_ref::<ArrayValue>() {
        println!("Array length: {}", array_val.len());
        
        for (i, element) in array_val.iter().enumerate() {
            println!("Element {}: {}", i, element.to_string());
        }
    }
}
```

#### 2.1.3 Nesting Depth Limits

**Current Limits:**
- Maximum nesting depth: 100 levels (configurable)
- Protection against infinite recursion
- Circular reference detection (planned)

---

### 2.2 Builder Pattern

Fluent API for ergonomic container construction.

#### 2.2.1 Basic Builder Usage

```rust
let container = ValueContainer::builder()
    .source("client_01", "session_123")
    .target("server", "main_handler")
    .message_type("user_data")
    .build();
```

#### 2.2.2 Builder with Values

```rust
let container = ValueContainer::builder()
    .message_type("notification")
    .with_value(Arc::new(StringValue::new("title", "Alert")))
    .with_value(Arc::new(StringValue::new("message", "System update required")))
    .with_value(Arc::new(IntValue::new("priority", 1)))
    .build();
```

#### 2.2.3 Header Management

```rust
// Set header values
container.set_header("custom_field", "custom_value");
container.set_source("client_01", "session_123");
container.set_target("server", "handler");
container.set_message_type("request");

// Get header values
let source = container.source_id();
let message_type = container.message_type();
let custom = container.get_header("custom_field");

// Remove header
container.remove_header("custom_field");

// Swap headers between containers
container1.swap_header(&mut container2);
```

---

### 2.3 Iterator Support

Standard Rust iteration with `ExactSizeIterator` implementation.

#### 2.3.1 Iterating Over Values

```rust
// Iterate over all values
for value in container.iter() {
    println!("{}: {}", value.name(), value.to_string());
}

// Collect into vector
let values: Vec<_> = container.iter().collect();

// Filter values
let integers: Vec<_> = container.iter()
    .filter(|v| v.value_type() == ValueType::Int)
    .collect();

// Map values
let names: Vec<String> = container.iter()
    .map(|v| v.name().to_string())
    .collect();
```

#### 2.3.2 ExactSizeIterator Features

```rust
let iter = container.iter();

// Know exact size
println!("Container has {} values", iter.len());

// Size hint
let (min, max) = iter.size_hint();
println!("Size: {} to {:?}", min, max);

// Enumerate
for (i, value) in container.iter().enumerate() {
    println!("{}: {}", i, value.name());
}
```

---

## 3. Real-World Examples

### 3.1 Messaging System

#### Request/Response Pattern

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// Create request message
fn create_request(user_id: i32, action: &str) -> ValueContainer {
    ValueContainer::builder()
        .source("client", &format!("user_{}", user_id))
        .target("server", "api_handler")
        .message_type("request")
        .with_value(Arc::new(StringValue::new("action", action)))
        .with_value(Arc::new(IntValue::new("user_id", user_id)))
        .with_value(Arc::new(LongValue::new("timestamp", get_timestamp())?))
        .build()
}

// Process request and create response
fn process_request(request: &ValueContainer) -> ValueContainer {
    let user_id = request.get_value("user_id")
        .and_then(|v| v.to_int().ok())
        .unwrap_or(0);
    
    let mut response = ValueContainer::builder()
        .source("server", "api_handler")
        .target("client", &format!("user_{}", user_id))
        .message_type("response")
        .build();
    
    response.add_value(Arc::new(IntValue::new("status", 200))).ok();
    response.add_value(Arc::new(StringValue::new("message", "Success"))).ok();
    
    response
}

// Send over network (example)
fn send_message(container: &ValueContainer) -> Result<(), Box<dyn std::error::Error>> {
    let wire_data = container.to_wire_protocol()?;
    // send_to_network(wire_data);
    Ok(())
}

// Receive from network (example)
fn receive_message(data: &[u8]) -> Result<ValueContainer, Box<dyn std::error::Error>> {
    ValueContainer::from_wire_protocol(data)
}
```

### 3.2 Configuration Management

#### Hierarchical Configuration

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

struct AppConfig {
    container: ValueContainer,
}

impl AppConfig {
    fn new() -> Self {
        let mut container = ValueContainer::new();
        container.set_message_type("config");
        
        // Database config
        let mut db_config = ValueContainer::new();
        db_config.add_value(Arc::new(StringValue::new("host", "localhost"))).ok();
        db_config.add_value(Arc::new(IntValue::new("port", 5432))).ok();
        db_config.add_value(Arc::new(StringValue::new("database", "myapp"))).ok();
        
        container.add_value(Arc::new(
            ContainerValue::new("database", db_config)
        )).ok();
        
        // Server config
        let mut server_config = ValueContainer::new();
        server_config.add_value(Arc::new(IntValue::new("port", 8080))).ok();
        server_config.add_value(Arc::new(IntValue::new("workers", 4))).ok();
        
        container.add_value(Arc::new(
            ContainerValue::new("server", server_config)
        )).ok();
        
        Self { container }
    }
    
    fn get_db_host(&self) -> Option<String> {
        self.container
            .get_value("database")
            .and_then(|v| v.as_any().downcast_ref::<ContainerValue>())
            .and_then(|cv| cv.get_container().get_value("host"))
            .map(|v| v.to_string())
    }
    
    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.container.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let container = ValueContainer::from_json(&json)?;
        Ok(Self { container })
    }
}
```

### 3.3 RPC Communication

#### Remote Procedure Call

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// RPC request
fn create_rpc_request(method: &str, params: Vec<(&str, i32)>) -> ValueContainer {
    let mut container = ValueContainer::builder()
        .message_type("rpc_request")
        .with_value(Arc::new(StringValue::new("method", method)))
        .build();
    
    // Add parameters as array
    let mut params_array = ArrayValue::new("params");
    for (name, value) in params {
        params_array.push(Arc::new(IntValue::new(name, value))).ok();
    }
    container.add_value(Arc::new(params_array)).ok();
    
    container
}

// RPC response
fn create_rpc_response(result: i32, error: Option<&str>) -> ValueContainer {
    let mut container = ValueContainer::builder()
        .message_type("rpc_response")
        .build();
    
    if let Some(err) = error {
        container.add_value(Arc::new(StringValue::new("error", err))).ok();
    } else {
        container.add_value(Arc::new(IntValue::new("result", result))).ok();
    }
    
    container
}

// Execute RPC
fn execute_rpc(request: &ValueContainer) -> ValueContainer {
    let method = request.get_value("method")
        .map(|v| v.to_string())
        .unwrap_or_default();
    
    match method.as_str() {
        "add" => {
            // Extract parameters and compute result
            create_rpc_response(42, None)
        }
        _ => {
            create_rpc_response(0, Some("Unknown method"))
        }
    }
}
```

---

## 4. Cross-Language Compatibility

### 4.1 C++ Interoperability

The Wire Protocol format is designed for seamless C++ interoperability.

#### Rust → C++

```rust
// Rust code
let container = ValueContainer::builder()
    .message_type("data")
    .with_value(Arc::new(IntValue::new("id", 123)))
    .build();

let wire_data = container.to_wire_protocol()?;
// Send wire_data to C++
```

```cpp
// C++ code
#include "container.hpp"

// Receive wire_data from Rust
auto container = value_container();
container.deserialize_cpp_wire(wire_data);

auto id = container.get_value("id")->to_int();
std::cout << "ID: " << id << std::endl;
```

#### C++ → Rust

```cpp
// C++ code
auto container = value_container();
container.set_message_type("data");
container.add_value(make_int_value("id", 456));

auto wire_data = container.serialize_cpp_wire();
// Send wire_data to Rust
```

```rust
// Rust code
// Receive wire_data from C++
let container = ValueContainer::from_wire_protocol(&wire_data)?;
let id = container.get_value("id").unwrap().to_int()?;
println!("ID: {}", id);
```

### 4.2 Type Mapping

| Rust Type | C++ Type | Wire Protocol ID |
|-----------|----------|------------------|
| `i16` (Short) | `int16_t` | 0 |
| `u16` (UShort) | `uint16_t` | 1 |
| `i32` (Int) | `int32_t` | 2 |
| `u32` (UInt) | `uint32_t` | 3 |
| `i64` (Long) | `int64_t` | 4 |
| `u64` (ULong) | `uint64_t` | 5 |
| `f32` (Float) | `float` | 8 |
| `f64` (Double) | `double` | 9 |
| `bool` (Bool) | `bool` | 10 |
| `String` | `std::string` | 11 |
| `Vec<u8>` (Bytes) | `std::vector<uint8_t>` | 12 |
| `ValueContainer` | `value_container` | 14 |
| `ArrayValue` | `array_value` | 15 |

### 4.3 Known Limitations

**Current Issues:**
- Nested array deserialization incomplete (TODO in wire_protocol.rs:445)
- Nested container deserialization incomplete (TODO in wire_protocol.rs:451)
- 2 test failures related to binary compatibility

**Workaround:**
- Use JSON or XML for production until Wire Protocol is fully stable
- Avoid deeply nested structures in cross-language scenarios

**Planned Improvements:**
- Complete nested structure support
- Add comprehensive interoperability tests
- CI/CD integration for cross-language testing

---

## 5. Migration Guide

### 5.1 From JSON/XML to Wire Protocol

**Before (JSON, Deprecated):**
```rust
let json = container.to_json()?;
let restored = ValueContainer::from_json(&json)?;
```

**After (Wire Protocol, Recommended):**
```rust
let wire = container.to_wire_protocol()?;
let restored = ValueContainer::from_wire_protocol(&wire)?;
```

**Benefits:**
- Faster serialization (~3x)
- Smaller payload size
- C++ compatibility
- Type preservation

### 5.2 From Arc::new to Box::new

**Note:** The examples above use `Arc::new` for compatibility, but `Box::new` is also supported.

```rust
// Both are valid
container.add_value(Arc::new(IntValue::new("id", 123)))?;
container.add_value(Box::new(IntValue::new("id", 123)))?;
```

---

## References

- [README.md](../README.md) - Quick start guide
- [BENCHMARKS.md](BENCHMARKS.md) - Performance analysis
- [BASELINE.md](performance/BASELINE.md) - Baseline metrics
- [PRODUCTION_QUALITY.md](PRODUCTION_QUALITY.md) - Quality report
- [C++ container_system](https://github.com/kcenon/container_system) - Original implementation

---

**Document Version:** 1.0
**Last Updated:** 2025-11-16
**Next Review:** 2025-12-16
