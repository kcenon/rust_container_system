# Features Documentation

**Version:** 0.1.0
**Date:** 2025-11-16
**Purpose:** Comprehensive guide to all features in the Rust Container System

## Table of Contents

- [Core Features](#core-features)
- [Advanced Features](#advanced-features)
- [Real-World Use Cases](#real-world-use-cases)
- [Cross-Language Compatibility](#cross-language-compatibility)
- [Performance Characteristics](#performance-characteristics)

---

## Core Features

### 1.1 Type System

The container system provides **16 value types** with compile-time type safety and runtime type checking.

#### Primitive Types

| Type | Rust Type | Size | Range | Description |
|------|-----------|------|-------|-------------|
| **Null** | `()` | 0 bytes | N/A | Empty/null value |
| **Bool** | `bool` | 1 byte | true/false | Boolean value |
| **Short** | `i16` | 2 bytes | -32,768 to 32,767 | 16-bit signed integer |
| **UShort** | `u16` | 2 bytes | 0 to 65,535 | 16-bit unsigned integer |
| **Int** | `i32` | 4 bytes | -2B to 2B | 32-bit signed integer |
| **UInt** | `u32` | 4 bytes | 0 to 4B | 32-bit unsigned integer |
| **Long** | `i64` | 8 bytes | -9E to 9E | 64-bit signed integer |
| **ULong** | `u64` | 8 bytes | 0 to 18E | 64-bit unsigned integer |
| **LLong** | `i64` | 8 bytes | -9E to 9E | 64-bit signed integer (alias) |
| **ULLong** | `u64` | 8 bytes | 0 to 18E | 64-bit unsigned integer (alias) |
| **Float** | `f32` | 4 bytes | ±3.4E±38 | 32-bit floating point |
| **Double** | `f64` | 8 bytes | ±1.7E±308 | 64-bit floating point |

#### Complex Types

| Type | Rust Type | Description |
|------|-----------|-------------|
| **String** | `String` | UTF-8 encoded text |
| **Bytes** | `Vec<u8>` | Raw binary data |
| **Array** | `Vec<Box<dyn Value>>` | Heterogeneous array |
| **Container** | `ValueContainer` | Nested container |

#### Type Safety Example

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();

// Compile-time type safety
container.add_value(Box::new(IntValue::new("age".to_string(), 25))).unwrap();

// Runtime type checking
let value = container.get_value("age").unwrap();
match value.to_int() {
    Ok(age) => println!("Age: {}", age),
    Err(e) => eprintln!("Type mismatch: {}", e),
}

// Type conversion
let age_str = value.to_string().unwrap();  // "25"
let age_double = value.to_double().unwrap();  // 25.0
```

### 1.2 Serialization

Support for **three serialization formats** with automatic format detection and proper escaping.

#### Supported Formats

| Format | Library | Performance | Use Case |
|--------|---------|-------------|----------|
| **JSON** | `serde_json` | ~558K ops/s | Web APIs, configuration |
| **XML** | `quick-xml` | ~1.79M ops/s | Legacy systems, SOAP |
| **Wire Protocol** | Custom | TBD | C++ interoperability |

#### JSON Serialization

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();
container.set_header("message_type", "user_data");
container.add_value(Box::new(IntValue::new("user_id".to_string(), 12345))).unwrap();
container.add_value(Box::new(StringValue::new("name".to_string(), "John Doe"))).unwrap();

// Serialize to JSON
let json = container.to_json().unwrap();
println!("{}", json);

// Output:
// {
//   "header": {
//     "message_type": "user_data"
//   },
//   "values": {
//     "user_id": "12345",
//     "name": "John Doe"
//   }
// }

// Deserialize from JSON
let restored = ValueContainer::from_json(&json).unwrap();
```

#### XML Serialization

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();
container.add_value(Box::new(StringValue::new("title".to_string(), "Rust Book"))).unwrap();

// Serialize to XML
let xml = container.to_xml().unwrap();
println!("{}", xml);

// Output:
// <container>
//   <header/>
//   <values>
//     <string name="title">Rust Book</string>
//   </values>
// </container>

// Deserialize from XML
let restored = ValueContainer::from_xml(&xml).unwrap();
```

#### Wire Protocol (C++ Compatible)

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();
container.add_value(Box::new(IntValue::new("count".to_string(), 42))).unwrap();

// Serialize to wire protocol
let wire = container.to_wire_protocol().unwrap();
// Format: @header={};@data={[count,4,42];}

// Deserialize from wire protocol
let restored = ValueContainer::from_wire_protocol(&wire).unwrap();
```

#### Automatic Format Detection

```rust
use rust_container_system::prelude::*;

let json_data = r#"{"header":{},"values":{}}"#;
let xml_data = r#"<container><header/></container>"#;
let wire_data = r#"@header={};@data={};"#;

// Auto-detects format
let from_json = ValueContainer::deserialize(json_data).unwrap();
let from_xml = ValueContainer::deserialize(xml_data).unwrap();
let from_wire = ValueContainer::deserialize(wire_data).unwrap();
```

### 1.3 Thread Safety

Built-in thread-safe operations using **`Arc<RwLock<ContainerInner>>`** pattern.

#### Concurrency Guarantees

- **Data race freedom**: Rust's type system prevents data races at compile time
- **Deadlock prevention**: RwLock allows multiple readers or single writer
- **Send + Sync traits**: Container can be safely shared across threads

#### Thread Safety Example

```rust
use rust_container_system::prelude::*;
use std::thread;
use std::sync::Arc;

let mut container = ValueContainer::new();
container.add_value(Box::new(IntValue::new("counter".to_string(), 0))).unwrap();

// Clone for thread safety (Arc clone, not deep clone)
let container_clone = container.clone();

// Spawn reader thread
let reader = thread::spawn(move || {
    for _ in 0..1000 {
        let value = container_clone.get_value("counter").unwrap();
        println!("Counter: {}", value.to_int().unwrap());
    }
});

// Writer thread (main)
for i in 0..1000 {
    container.remove_value("counter").ok();
    container.add_value(Box::new(IntValue::new("counter".to_string(), i))).unwrap();
}

reader.join().unwrap();
```

#### Performance Characteristics

| Operation | Concurrency | Performance |
|-----------|-------------|-------------|
| **Read** | Multiple readers | ~20-50 ns |
| **Write** | Single writer | ~180 ns/value |
| **Clone** | O(1) Arc clone | ~10 ns |

---

## Advanced Features

### 2.1 Nested Structures

Support for **unlimited nesting depth** of containers and arrays.

#### Nested Containers

```rust
use rust_container_system::prelude::*;

// Create inner container
let mut inner = ValueContainer::new();
inner.set_header("type", "user_profile");
inner.add_value(Box::new(StringValue::new("name".to_string(), "John"))).unwrap();
inner.add_value(Box::new(IntValue::new("age".to_string(), 30))).unwrap();

// Create outer container
let mut outer = ValueContainer::new();
outer.set_header("type", "api_response");
outer.add_value(Box::new(ContainerValue::new("profile".to_string(), inner))).unwrap();
outer.add_value(Box::new(BoolValue::new("success".to_string(), true))).unwrap();

// Access nested data
let profile = outer.get_value("profile").unwrap();
let profile_container = profile.as_any().downcast_ref::<ContainerValue>().unwrap();
let name = profile_container.get_container().get_value("name").unwrap();
println!("Name: {}", name.to_string().unwrap());
```

#### Heterogeneous Arrays

```rust
use rust_container_system::prelude::*;

let mut array = ArrayValue::new("mixed_data".to_string());

// Add different types to same array
array.push(Box::new(IntValue::new("elem".to_string(), 42))).unwrap();
array.push(Box::new(StringValue::new("elem".to_string(), "hello"))).unwrap();
array.push(Box::new(DoubleValue::new("elem".to_string(), 3.14))).unwrap();

// Nested arrays
let mut inner_array = ArrayValue::new("inner".to_string());
inner_array.push(Box::new(IntValue::new("elem".to_string(), 1))).unwrap();
inner_array.push(Box::new(IntValue::new("elem".to_string(), 2))).unwrap();

array.push(Box::new(inner_array)).unwrap();

// Add to container
let mut container = ValueContainer::new();
container.add_value(Box::new(array)).unwrap();
```

### 2.2 Builder Pattern

Fluent API for **ergonomic container construction**.

```rust
use rust_container_system::prelude::*;

// Concise builder syntax
let mut container = ValueContainer::builder()
    .source("client_01", "session_123")
    .target("server", "main_handler")
    .message_type("user_event")
    .max_values(1000)
    .build();

// Add values after building
container.add_value(Box::new(IntValue::new("user_id".to_string(), 12345))).unwrap();
container.add_value(Box::new(StringValue::new("action".to_string(), "login"))).unwrap();

// Builder with initial values
let mut container2 = ValueContainer::builder()
    .message_type("notification")
    .build();
```

### 2.3 Iterator Support

Standard Rust iteration with **`ExactSizeIterator`** implementation.

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();
container.add_value(Box::new(IntValue::new("a".to_string(), 1))).unwrap();
container.add_value(Box::new(IntValue::new("b".to_string(), 2))).unwrap();
container.add_value(Box::new(IntValue::new("c".to_string(), 3))).unwrap();

// For loop
for value in &container {
    println!("{}: {}", value.name(), value.to_string());
}

// Iterator methods
let names: Vec<String> = (&container)
    .into_iter()
    .map(|v| v.name().to_string())
    .collect();

// ExactSizeIterator
let iter = (&container).into_iter();
println!("Total values: {}", iter.len());

// Filter and collect
let ints: Vec<_> = (&container)
    .into_iter()
    .filter(|v| v.value_type() == ValueType::Int)
    .collect();
```

### 2.4 Ergonomic Value Creation

**`From` trait** implementations for tuple syntax.

```rust
use rust_container_system::prelude::*;

let mut container = ValueContainer::new();

// Tuple syntax for value creation
container.add_value(Box::new(IntValue::from(("user_id", 12345)))).unwrap();
container.add_value(Box::new(StringValue::from(("username", "john_doe")))).unwrap();
container.add_value(Box::new(DoubleValue::from(("balance", 1500.75)))).unwrap();
container.add_value(Box::new(BoolValue::from(("active", true)))).unwrap();
container.add_value(Box::new(BytesValue::from(("data", vec![1, 2, 3])))).unwrap();

// More concise than explicit constructors
// Old way:
let old = IntValue::new("count".to_string(), 42);
// New way:
let new = IntValue::from(("count", 42));
```

---

## Real-World Use Cases

### 3.1 Messaging System

Container-based pub/sub messaging with routing and headers.

```rust
use rust_container_system::prelude::*;

// Publisher
fn publish_user_event(user_id: i32, action: &str) -> ValueContainer {
    let mut msg = ValueContainer::builder()
        .source("user_service", "event_publisher")
        .target("event_bus", "user_events")
        .message_type("user_action")
        .build();
    
    msg.set_header("event_id", &uuid::Uuid::new_v4().to_string());
    msg.set_header("timestamp", &chrono::Utc::now().to_rfc3339());
    
    msg.add_value(Box::new(IntValue::from(("user_id", user_id)))).unwrap();
    msg.add_value(Box::new(StringValue::from(("action", action)))).unwrap();
    
    msg
}

// Subscriber
fn handle_user_event(msg: &ValueContainer) {
    let user_id = msg.get_value("user_id").unwrap().to_int().unwrap();
    let action = msg.get_value("action").unwrap().to_string().unwrap();
    
    println!("User {} performed action: {}", user_id, action);
}

// Usage
let event = publish_user_event(12345, "login");
let serialized = event.to_wire_protocol().unwrap();
// Send over network...

// Receiver
let received = ValueContainer::from_wire_protocol(&serialized).unwrap();
handle_user_event(&received);
```

### 3.2 Configuration Management

Type-safe configuration with nested sections.

```rust
use rust_container_system::prelude::*;

// Load configuration
fn load_config() -> ValueContainer {
    let json = r#"
    {
        "header": {
            "config_version": "1.0"
        },
        "values": {
            "database_host": "localhost",
            "database_port": "5432",
            "max_connections": "100",
            "enable_ssl": "true",
            "timeout_ms": "5000"
        }
    }
    "#;
    
    ValueContainer::from_json(json).unwrap()
}

// Access configuration
fn get_database_config(config: &ValueContainer) -> (String, u16, i32, bool) {
    let host = config.get_value("database_host").unwrap().to_string().unwrap();
    let port = config.get_value("database_port").unwrap().to_int().unwrap() as u16;
    let max_conn = config.get_value("max_connections").unwrap().to_int().unwrap();
    let ssl = config.get_value("enable_ssl").unwrap().to_bool().unwrap();
    
    (host, port, max_conn, ssl)
}

// Usage
let config = load_config();
let (host, port, max_conn, ssl) = get_database_config(&config);
println!("Connecting to {}:{} (max: {}, SSL: {})", host, port, max_conn, ssl);
```

### 3.3 RPC/IPC Communication

Cross-process communication with serialization.

```rust
use rust_container_system::prelude::*;

// RPC Request
struct RpcRequest {
    method: String,
    params: ValueContainer,
}

impl RpcRequest {
    fn new(method: &str) -> Self {
        Self {
            method: method.to_string(),
            params: ValueContainer::new(),
        }
    }
    
    fn add_param(&mut self, name: &str, value: Box<dyn Value>) {
        self.params.add_value(value).ok();
    }
    
    fn serialize(&self) -> String {
        let mut container = ValueContainer::new();
        container.set_header("rpc_method", &self.method);
        container.set_header("rpc_version", "1.0");
        
        // Copy params
        for value in &self.params {
            container.add_value(value.clone_value()).ok();
        }
        
        container.to_wire_protocol().unwrap()
    }
}

// RPC Response
struct RpcResponse {
    result: ValueContainer,
    error: Option<String>,
}

impl RpcResponse {
    fn from_wire(data: &str) -> Result<Self, String> {
        let container = ValueContainer::from_wire_protocol(data)
            .map_err(|e| e.to_string())?;
        
        let error = container.get_header("error").map(|s| s.to_string());
        
        Ok(Self {
            result: container,
            error,
        })
    }
}

// Usage
let mut request = RpcRequest::new("get_user");
request.add_param("user_id", Box::new(IntValue::from(("user_id", 12345))));

let wire_data = request.serialize();
// Send via IPC/network...

// Receiver
let response = RpcResponse::from_wire(&wire_data).unwrap();
```

---

## Cross-Language Compatibility

### 4.1 Wire Protocol Specification

Binary-compatible protocol for C++/Python/Go interoperability.

#### Format Specification

```
Wire Protocol Format:
@header={key1=value1;key2=value2;...};@data={[name,type,data];[name,type,data];...}

Type IDs:
0  = Null
1  = Bool
2  = Short
3  = UShort
4  = Int
5  = UInt
6  = Long
7  = ULong
8  = LLong
9  = ULLong
10 = Float
11 = Double
12 = Bytes (base64 encoded)
13 = String
14 = Container (nested)
15 = Array
```

#### C++ Interoperability Example

```rust
use rust_container_system::prelude::*;

// Create container in Rust
let mut rust_container = ValueContainer::new();
rust_container.set_header("source", "rust_app");
rust_container.add_value(Box::new(IntValue::from(("count", 42)))).unwrap();
rust_container.add_value(Box::new(StringValue::from(("message", "Hello from Rust")))).unwrap();

// Serialize to wire protocol
let wire_data = rust_container.to_wire_protocol().unwrap();
// Wire: @header={source=rust_app;};@data={[count,4,42];[message,13,Hello from Rust];}

// This can be read by C++ container_system:
// auto cpp_container = value_container::from_wire_protocol(wire_data);
// int count = cpp_container.get_value<int>("count");
// std::string msg = cpp_container.get_value<std::string>("message");
```

#### Python Interoperability (via bindings)

```python
# Python side (using PyO3 bindings - future work)
from rust_container import ValueContainer

# Create in Python
container = ValueContainer()
container.set_header("source", "python_app")
container.add_int("count", 42)
container.add_string("message", "Hello from Python")

# Serialize
wire_data = container.to_wire_protocol()

# Send to Rust application...
```

### 4.2 Serialization Compatibility Matrix

| Format | Rust | C++ | Python | Go | JavaScript |
|--------|------|-----|--------|----|-----------| 
| **Wire Protocol** | ✅ | ✅ | ⏳ (WIP) | ⏳ (WIP) | ❌ |
| **JSON** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **XML** | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## Performance Characteristics

### 5.1 Operation Performance

Based on Apple M1 benchmarks (see [BASELINE.md](performance/BASELINE.md)):

| Operation | Performance | Notes |
|-----------|-------------|-------|
| **Value Creation** | 18-40 ns | Primitives: 18ns, Strings: 40ns |
| **Container Add** | 170 ns/value | Amortized cost |
| **Value Lookup** | 21 ns | O(1) HashMap lookup |
| **JSON Serialization** | 1.8 µs/value | serde_json |
| **XML Serialization** | 560 ns/value | **3x faster than JSON** |
| **Container Clone** | 10 ns | Arc reference count |

### 5.2 Memory Usage

| Structure | Memory | Overhead |
|-----------|--------|----------|
| **Empty Container** | ~100 bytes | Arc + RwLock + HashMaps |
| **Per Value** | ~48 bytes | Name + data + metadata |
| **1000 values** | ~48 KB | Linear scaling |

### 5.3 Scalability

- **Container size**: Tested up to 1000 values, linear scaling
- **Nesting depth**: Unlimited (recursion-safe)
- **Thread count**: RwLock scales with read concurrency
- **Memory**: No memory leaks (100% safe Rust)

---

## Migration from C++ container_system

### Key Differences

| Feature | C++ | Rust | Notes |
|---------|-----|------|-------|
| **Memory Safety** | Manual RAII | Automatic ownership | Compile-time guarantees |
| **Thread Safety** | Manual locks | Built-in RwLock | Send + Sync traits |
| **SIMD** | ✅ AVX2/NEON | ❌ Not yet | Future work |
| **Performance** | Slightly faster | Comparable | Rust: safer, C++: SIMD |

### API Comparison

```cpp
// C++
auto container = value_container();
container.add_value(make_int_value("count", 42));
auto value = container.get_value("count");
int count = value->to_int();
```

```rust
// Rust
let mut container = ValueContainer::new();
container.add_value(Box::new(IntValue::from(("count", 42)))).unwrap();
let value = container.get_value("count").unwrap();
let count = value.to_int().unwrap();
```

---

**Document Version:** 1.0
**Last Updated:** 2025-11-16
**See Also:**
- [BASELINE.md](performance/BASELINE.md) - Performance metrics
- [BENCHMARKS.md](BENCHMARKS.md) - Detailed benchmark results
- [README.md](../README.md) - Quick start guide
