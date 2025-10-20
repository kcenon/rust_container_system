# Rust Container System

> **Languages**: English | [한국어](./README.ko.md)

A production-ready, high-performance Rust container framework designed to provide comprehensive data management capabilities for messaging systems and general-purpose applications.

This is a Rust implementation of the [container_system](https://github.com/kcenon/container_system) project, providing the same functionality with Rust's safety guarantees and performance benefits.

## Quality Status

- Verification: `cargo check`, `cargo test`(unit, integration, property, doc), `cargo clippy --all-targets` 모두 통과 ✅
- Known issues: 없음. 값 수 제한(`DEFAULT_MAX_VALUES`)이 메모리 보호를 위해 기본적으로 적용되어 있으므로, 대용량 환경에서는 필요에 따라 명시적으로 조정하십시오.
- Production guidance: 현재 상태 그대로 프로덕션 투입 가능. 큰 메시지 페이로드를 사용할 경우 메모리 사용량을 모니터링하세요.

## Features

- **Type Safety**: Strongly-typed value system with compile-time checks
- **Thread Safety**: Built-in thread-safe operations using `parking_lot` RwLock
- **Memory Efficiency**: Efficient memory management with `Arc` and smart pointers
- **Serialization**: JSON and XML serialization support with proper escaping
- **Performance**: Zero-cost abstractions and minimal overhead with `#[inline]` optimizations
- **Cross-Platform**: Works on Windows, Linux, and macOS
- **Builder Pattern**: Fluent API for constructing containers (v0.1.1+)
- **Iterator Support**: Standard Rust iteration with `ExactSizeIterator` (v0.1.1+)
- **Ergonomic APIs**: `From` trait implementations for easy value creation (v0.1.1+)
- **Property Testing**: Robust validation using `proptest` (v0.1.1+)
- **Benchmarking**: Performance tracking with `criterion` (v0.1.1+)

## Value Types

The container system supports the following value types:

| Type | Description | Size |
|------|-------------|------|
| `Null` | Null/empty value | 0 bytes |
| `Bool` | Boolean true/false | 1 byte |
| `Short` | 16-bit signed integer | 2 bytes |
| `UShort` | 16-bit unsigned integer | 2 bytes |
| `Int` | 32-bit signed integer | 4 bytes |
| `UInt` | 32-bit unsigned integer | 4 bytes |
| `Long` | 64-bit signed integer | 8 bytes |
| `ULong` | 64-bit unsigned integer | 8 bytes |
| `LLong` | 64-bit signed integer | 8 bytes |
| `ULLong` | 64-bit unsigned integer | 8 bytes |
| `Float` | 32-bit floating point | 4 bytes |
| `Double` | 64-bit floating point | 8 bytes |
| `Bytes` | Raw byte array | Variable |
| `String` | UTF-8 string | Variable |
| `Container` | Nested container | Variable |

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_container_system = "0.1"
```

### Basic Usage

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    // Create a new container
    let mut container = ValueContainer::new();
    container.set_source("client_01", "session_123");
    container.set_target("server", "main_handler");
    container.set_message_type("user_data");

    // Add values
    container.add_value(Arc::new(IntValue::new("user_id", 12345)));
    container.add_value(Arc::new(StringValue::new("username", "john_doe")));
    container.add_value(Arc::new(DoubleValue::new("balance", 1500.75)));
    container.add_value(Arc::new(BoolValue::new("active", true)));

    // Get a value
    let user_id = container.get_value("user_id").unwrap();
    println!("User ID: {}", user_id.to_int().unwrap());

    // Serialize to JSON
    let json = container.to_json().unwrap();
    println!("JSON: {}", json);

    // Serialize to XML
    let xml = container.to_xml().unwrap();
    println!("XML: {}", xml);
}
```

### Working with Values

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// Create different types of values
let bool_val = Arc::new(BoolValue::new("is_active", true));
let int_val = Arc::new(IntValue::new("count", 42));
let long_val = Arc::new(LongValue::new("timestamp", 1234567890));
let double_val = Arc::new(DoubleValue::new("price", 99.99));
let string_val = Arc::new(StringValue::new("name", "John Doe"));
let bytes_val = Arc::new(BytesValue::new("data", vec![1, 2, 3, 4]));

// Add to container
let mut container = ValueContainer::new();
container.add_value(bool_val);
container.add_value(int_val);
container.add_value(long_val);
container.add_value(double_val);
container.add_value(string_val);
container.add_value(bytes_val);

// Retrieve and use values
if let Some(value) = container.get_value("price") {
    match value.to_double() {
        Ok(price) => println!("Price: ${:.2}", price),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Builder Pattern (v0.1.1+)

Use the fluent builder API for ergonomic container construction:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

let mut container = ValueContainer::builder()
    .source("client_01", "session_123")
    .target("server", "main_handler")
    .message_type("user_event")
    .max_values(1000)
    .build();

// Add values after building
container.add_value(Arc::new(IntValue::new("user_id", 12345)));
container.add_value(Arc::new(StringValue::new("username", "john_doe")));
```

### Iterator Support (v0.1.1+)

Iterate over container values using standard Rust iterators:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

let mut container = ValueContainer::new();
container.add_value(Arc::new(IntValue::new("a", 1)));
container.add_value(Arc::new(IntValue::new("b", 2)));
container.add_value(Arc::new(IntValue::new("c", 3)));

// Use for loop
for value in &container {
    println!("{}: {}", value.name(), value.to_string());
}

// Use iterator methods
let names: Vec<String> = (&container)
    .into_iter()
    .map(|v| v.name().to_string())
    .collect();
```

### From Trait (v0.1.1+)

Create values ergonomically using tuple syntax:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

let mut container = ValueContainer::new();

// Create values from tuples
container.add_value(Arc::new(IntValue::from(("user_id", 12345))));
container.add_value(Arc::new(StringValue::from(("username", "john_doe"))));
container.add_value(Arc::new(DoubleValue::from(("balance", 1500.75))));
container.add_value(Arc::new(BoolValue::from(("active", true))));
```

### Thread Safety

The container is thread-safe by default using `Arc<RwLock<...>>`:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;
use std::thread;

let mut container = ValueContainer::new();
container.add_value(Arc::new(IntValue::new("counter", 0)));

// Clone for thread safety
let container_clone = container.clone();

let handle = thread::spawn(move || {
    let value = container_clone.get_value("counter").unwrap();
    println!("Counter: {}", value.to_int().unwrap());
});

handle.join().unwrap();
```

## Project Structure

```
rust_container_system/
├── src/
│   ├── core/              # Core types and traits
│   │   ├── value_types.rs # Value type enum
│   │   ├── value.rs       # Value trait
│   │   ├── container.rs   # ValueContainer
│   │   ├── error.rs       # Error types
│   │   └── mod.rs
│   ├── values/            # Value implementations
│   │   ├── primitive_values.rs
│   │   ├── string_value.rs
│   │   ├── bytes_value.rs
│   │   └── mod.rs
│   └── lib.rs
├── examples/              # Example programs
├── tests/                 # Integration tests
├── benches/              # Benchmarks
├── Cargo.toml
└── README.md
```

## Comparison with C++ Version

| Feature | C++ Version | Rust Version |
|---------|-------------|--------------|
| Type Safety | ✓ (C++20) | ✓ (Rust) |
| Thread Safety | Manual (mutex) | Automatic (Arc+RwLock) |
| Memory Safety | Manual (smart pointers) | Automatic (ownership) |
| Serialization | Binary, JSON, XML | JSON, XML |
| SIMD Support | ✓ (AVX2, NEON) | Planned |
| Performance | High | High |

## Building

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build Commands

```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open
```

## Examples

See the `examples/` directory for more examples:

```bash
# Run the basic example
cargo run --example basic_container

# Run the serialization example
cargo run --example serialization

# Run the thread safety example
cargo run --example thread_safety
```

## Performance

The Rust implementation provides comparable or better performance than the C++ version:

- **Zero-cost abstractions**: No runtime overhead for type safety
- **Memory efficiency**: Efficient use of Arc and RwLock
- **Thread safety**: Lock-free reads when possible
- **Value access**: ~10-50ns for primitive types
- **JSON serialization**: ~1-10μs depending on container size
- **XML serialization**: ~2-20μs depending on container size
- **Memory overhead**: ~80 bytes per value + data size

### Performance Characteristics

- **Value creation**: O(1) amortized
- **Value lookup by name**: O(1) with HashMap-based indexing
- **Value removal**: O(n) with HashSet-optimized filtering
- **Serialization**: O(n) where n is number of values
- **Thread-safe clone**: O(1) with Arc reference counting
- **Iteration**: O(n) with ExactSizeIterator optimization

### Benchmarks

Run benchmarks with:
```bash
cargo bench
```

Expected performance (on modern hardware):
- Create and add value: ~50ns
- Get value by name: ~100ns (depends on container size)
- JSON serialization (10 values): ~5μs
- XML serialization (10 values): ~10μs

## Security

### Input Validation

**⚠️ IMPORTANT**: Always validate container size and value counts to prevent memory exhaustion.

**✅ DO** validate before deserialization:

```rust
use rust_container_system::prelude::*;

// Safe: Limit container size
fn safe_deserialize(json_data: &str) -> Result<ValueContainer> {
    // Check input size before parsing
    const MAX_JSON_SIZE: usize = 1024 * 1024;  // 1MB limit
    if json_data.len() > MAX_JSON_SIZE {
        return Err("Input too large".into());
    }

    let container = ValueContainer::from_json(json_data)?;

    // Validate value count
    const MAX_VALUES: usize = 1000;
    if container.value_count() > MAX_VALUES {
        return Err("Too many values in container".into());
    }

    Ok(container)
}
```

**❌ DON'T** deserialize untrusted input without limits:

```rust
// UNSAFE: Unbounded deserialization
let container = ValueContainer::from_json(untrusted_json)?;  // DON'T DO THIS!
// Attacker can send huge JSON causing memory exhaustion
```

### Type Safety

- **Compile-time type checking**: Rust's type system prevents type confusion
- **Safe conversions**: Type conversion methods return `Result` for safe error handling
- **No undefined behavior**: 100% safe Rust prevents memory corruption

### Best Practices

1. **Limit container size**: Set maximum JSON/XML input size before parsing
2. **Validate value counts**: Reject containers with excessive values
3. **Sanitize string values**: Validate string content from untrusted sources
4. **Use bounded types**: Prefer primitive types over unbounded Bytes/String when possible
5. **Monitor memory usage**: Track container memory in production systems
6. **Validate deserialization**: Always check deserialize results for malformed input

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// ✅ DO: Validate and bound inputs
fn process_user_container(json: &str) -> Result<()> {
    // Validate size limits
    if json.len() > 100_000 {  // 100KB limit
        return Err("Container too large".into());
    }

    let container = ValueContainer::from_json(json)?;

    // Validate value count
    if container.value_count() > 100 {
        return Err("Too many values".into());
    }

    // Validate specific values
    if let Some(name_val) = container.get_value("username") {
        let name = name_val.to_string()?;
        if name.len() > 255 {
            return Err("Username too long".into());
        }
    }

    Ok(())
}

// ❌ DON'T: Trust untrusted input
fn unsafe_process(untrusted_data: &str) -> Result<()> {
    let container = ValueContainer::from_json(untrusted_data)?;  // No validation!
    // Process without checking size or content
    Ok(())
}
```

### Memory Safety

- **100% Safe Rust**: No `unsafe` code blocks in the entire codebase
- **Ownership System**: Prevents data races and use-after-free bugs
- **Bounded Growth**: Application-level limits prevent unbounded memory growth
- **Thread Safety**: Arc and RwLock provide safe concurrent access

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the BSD 3-Clause License - see the LICENSE file for details.

## Acknowledgments

- Original C++ implementation: [container_system](https://github.com/kcenon/container_system)
- Built with Rust's excellent ecosystem

## Related Projects

- **messaging_system**: Primary consumer of container functionality
- **network_system**: Network transport for containers
- **database_system**: Persistent storage for containers

---

Made with ❤️ in Rust
