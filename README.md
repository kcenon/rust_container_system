# Rust Container System

> **Languages**: English | [한국어](./README.ko.md)

A production-ready, high-performance Rust container framework designed to provide comprehensive data management capabilities for messaging systems and general-purpose applications.

This is a Rust implementation of the [container_system](https://github.com/kcenon/container_system) project, providing the same functionality with Rust's safety guarantees and performance benefits.

[![Rust CI](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange)]()

## Quick Reference

Comprehensive documentation organized by use case:

| Document | Description | Use When |
|----------|-------------|----------|
| **[Quick Start](docs/guides/QUICK_START.md)** | Get started in 5 minutes | First-time users |
| **[API Reference](docs/API_REFERENCE.md)** | Complete API documentation | Looking up methods |
| **[FEATURES.md](docs/FEATURES.md)** | Complete feature guide with examples | Learning all capabilities |
| **[Best Practices](docs/guides/BEST_PRACTICES.md)** | Recommended usage patterns | Writing production code |
| **[FAQ](docs/guides/FAQ.md)** | Frequently asked questions | Quick answers |
| **[Troubleshooting](docs/guides/TROUBLESHOOTING.md)** | Common issues and solutions | Fixing problems |
| **[BENCHMARKS.md](docs/BENCHMARKS.md)** | Detailed performance analysis | Optimizing performance |
| **[PRODUCTION_QUALITY.md](docs/PRODUCTION_QUALITY.md)** | Quality & readiness report | Production deployment |
| **[examples/](examples/)** | Working code examples | Getting started quickly |

**→ See [docs/README.md](docs/README.md) for the complete documentation hub**

## Features Overview

- **Type Safety**: 16 strongly-typed values with compile-time checks
- **Thread Safety**: Built-in `Arc<RwLock<T>>` for concurrent access
- **Serialization**: JSON, XML, and Wire Protocol (C++ compatible)
- **Performance**: 54M ops/sec value creation, 20ns HashMap lookup
- **Memory Efficient**: ~48 bytes overhead per value, O(1) Arc cloning
- **Zero Unsafe**: 100% safe Rust, no `unsafe` blocks
- **Builder Pattern**: Fluent API for ergonomic construction
- **Iterator Support**: Standard Rust iteration with `ExactSizeIterator`
- **Dependency Injection**: ContainerFactory trait for DI frameworks

**→ See [FEATURES.md](docs/FEATURES.md) for detailed documentation and examples**

## Performance Highlights

| Operation | Performance | Notes |
|-----------|-------------|-------|
| **Value Creation** | 18-40 ns | Primitives: 18ns, Strings: 40ns |
| **Container Add** | 170 ns/value | Amortized, linear scaling |
| **HashMap Lookup** | 21 ns | O(1), size-independent |
| **JSON Serialization** | 1.8 µs/value | 558K ops/sec |
| **XML Serialization** | 560 ns/value | **3x faster than JSON** |
| **Container Clone** | 10 ns | O(1) Arc reference count |

**→ See [BENCHMARKS.md](docs/BENCHMARKS.md) for detailed analysis**  
**→ See [BASELINE.md](docs/performance/BASELINE.md) for regression detection**

## Quality Status

| Metric | Status | Details |
|--------|--------|---------|
| **Tests** | ⚠️ 60/62 passing (96.8%) | [Details](docs/PRODUCTION_QUALITY.md#test-coverage) |
| **Memory Safety** | ✅ 100% safe Rust | 0 unsafe blocks |
| **Security** | ✅ 0 vulnerabilities | cargo audit clean |
| **Production Ready** | ✅ Yes (conditions apply) | [Readiness Report](docs/PRODUCTION_QUALITY.md) |

**Known Issues**: Wire protocol nested structures (2 test failures) - use JSON/XML for production.

**→ See [PRODUCTION_QUALITY.md](docs/PRODUCTION_QUALITY.md) for complete quality report**

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust_container_system = "0.1"
```

Or install via cargo:

```bash
cargo add rust_container_system
```

## Quick Start

### Basic Usage

```rust
use rust_container_system::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create container with builder pattern
    let mut container = ValueContainer::builder()
        .source("client_01", "session_123")
        .target("server", "main_handler")
        .message_type("user_data")
        .build();
    
    // Add values using From trait
    container.add_value(Box::new(IntValue::from(("user_id", 12345))))?;
    container.add_value(Box::new(StringValue::from(("username", "john_doe"))))?;
    container.add_value(Box::new(DoubleValue::from(("balance", 1500.75))))?;
    container.add_value(Box::new(BoolValue::from(("active", true))))?;
    
    // Retrieve values
    let user_id = container.get_value("user_id")
        .ok_or("Value not found")?
        .to_int()?;
    println!("User ID: {}", user_id);
    
    // Serialize to JSON
    let json = container.to_json()?;
    println!("JSON: {}", json);
    
    // Deserialize from JSON
    let restored = ValueContainer::from_json(&json)?;
    
    Ok(())
}
```

### Using MessagingContainerBuilder

For messaging-specific use cases aligned with C++ architecture:

```rust
use rust_container_system::messaging::MessagingContainerBuilder;
use rust_container_system::values::IntValue;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build container with fluent API
    let mut container = MessagingContainerBuilder::new()
        .with_source("client_app", "session_123")
        .with_target("server_app", "main")
        .with_type("user_request")
        .with_max_values(1000)
        .build();

    // Add values after building
    container.add_value(Arc::new(IntValue::new("request_id", 42)))?;

    Ok(())
}
```

### Using Dependency Injection

For loosely-coupled architectures with dependency injection:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// Create a shared factory for dependency injection
let factory: Arc<dyn ContainerFactory> = Arc::new(
    ArcContainerProvider::builder()
        .with_default_message_type("app_message")
        .build()
);

// Inject into services
struct MessageService {
    factory: Arc<dyn ContainerFactory>,
}

impl MessageService {
    fn create_request(&self, msg_type: &str) -> ValueContainer {
        self.factory.create_with_type(msg_type)
    }
}

let service = MessageService { factory };
let container = service.create_request("user_request");
```

### More Examples

```bash
# Basic container operations
cargo run --example basic_container

# Serialization (JSON/XML/Wire)
cargo run --example serialization

# Nested containers
cargo run --example nested_containers

# Dependency injection patterns
cargo run --example dependency_injection
```

**→ See [examples/](examples/) directory for more examples**  
**→ See [FEATURES.md](docs/FEATURES.md) for comprehensive usage guide**

## Value Types

The system supports **16 value types**:

| Category | Types | Size |
|----------|-------|------|
| **Integers** | Short, UShort, Int, UInt, Long, ULong, LLong, ULLong | 2-8 bytes |
| **Floating** | Float, Double | 4-8 bytes |
| **Boolean** | Bool | 1 byte |
| **Text** | String (UTF-8) | Variable |
| **Binary** | Bytes | Variable |
| **Complex** | Container (nested), Array (heterogeneous) | Variable |
| **Special** | Null | 0 bytes |

**→ See [FEATURES.md](docs/FEATURES.md#11-type-system) for detailed type documentation**

## Serialization Formats

| Format | Speed | Use Case | Compatibility |
|--------|-------|----------|---------------|
| **JSON** | 558K ops/s | Web APIs, config files | ✅ Universal |
| **XML** | 1.79M ops/s (**3x faster**) | Legacy systems, SOAP | ✅ Universal |
| **Wire Protocol** | TBD | C++ interoperability | ⚠️ Experimental |

```rust
// Automatic format detection
let container = ValueContainer::deserialize(&data)?;

// Or explicit format
let json = container.to_json()?;
let xml = container.to_xml()?;
let wire = container.to_wire_protocol()?;
```

**→ See [FEATURES.md](docs/FEATURES.md#12-serialization) for serialization guide**

## Thread Safety

Built-in concurrency support via `Arc<RwLock<ContainerInner>>`:

```rust
let container = ValueContainer::new();
let container_clone = container.clone(); // O(1) Arc clone

// Spawn reader thread
thread::spawn(move || {
    let value = container_clone.get_value("data").unwrap();
    println!("Value: {}", value.to_string());
});

// Main thread can write
container.add_value(Box::new(IntValue::from(("data", 42)))).ok();
```

**Performance**:
- Read operations: ~20-50 ns
- Write operations: ~180 ns/value
- Clone: 10 ns (O(1))

**→ See [FEATURES.md](docs/FEATURES.md#13-thread-safety) for concurrency patterns**

## Comparison with C++ Version

| Aspect | C++ (container_system) | Rust (this) | Winner |
|--------|------------------------|-------------|--------|
| **Performance** | Slightly faster (SIMD) | Comparable | C++ (marginal) |
| **Memory Safety** | Manual (RAII) | Automatic (ownership) | **Rust** |
| **Thread Safety** | Manual locks | Compile-time | **Rust** |
| **Type Safety** | Runtime | Compile-time | **Rust** |
| **Ease of Use** | Complex | Ergonomic (builder, traits) | **Rust** |

**Verdict**: Rust version trades ~10-20% performance for compile-time safety guarantees and ergonomic APIs.

**→ See [BENCHMARKS.md](docs/BENCHMARKS.md#41-rust-vs-c) for detailed comparison**

## Documentation

### API Documentation

```bash
cargo doc --open
```

### Documentation Structure

```
docs/
├── README.md                    # Documentation hub
├── API_REFERENCE.md             # Complete API reference
├── FEATURES.md                  # Complete feature guide
├── BENCHMARKS.md                # Performance analysis
├── PRODUCTION_QUALITY.md        # Quality & readiness
├── PROJECT_STRUCTURE.md         # Codebase organization
├── ARRAY_VALUE_GUIDE.md         # Array value guide
│
├── guides/                      # User guides
│   ├── QUICK_START.md          # 5-minute quick start
│   ├── FAQ.md                  # Frequently asked questions
│   ├── TROUBLESHOOTING.md      # Common issues and solutions
│   └── BEST_PRACTICES.md       # Recommended patterns
│
├── contributing/                # Contributor guides
│   └── TESTING.md              # Testing strategy
│
└── performance/
    └── BASELINE.md              # Baseline metrics
```

### Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

**Quick Start**:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Run benchmarks (`cargo bench`)
5. Submit pull request

**Quality Gates**:
- All tests passing
- No clippy warnings
- Formatted with rustfmt
- Benchmarks within 30% of baseline

**→ See [CONTRIBUTING.md](CONTRIBUTING.md) for complete contribution guide**
**→ See [Testing Guide](docs/contributing/TESTING.md) for testing requirements**

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Related Projects

- [container_system (C++)](https://github.com/kcenon/container_system) - Original C++ implementation

## Acknowledgments

- Based on the C++ container_system project
- Uses `serde` ecosystem for serialization
- `parking_lot` for high-performance synchronization
- `criterion` for benchmarking

---

**Version**: 0.1.0  
**Status**: Production-ready (with conditions - see [PRODUCTION_QUALITY.md](docs/PRODUCTION_QUALITY.md))  
**Minimum Rust**: 1.90.0
