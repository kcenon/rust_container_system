# Rust Container System Documentation

> **Version**: 0.1.0
> **Last Updated**: 2025-12-17
> **Status**: Production Ready (with conditions)

Welcome to the Rust Container System documentation! This guide will help you navigate the available resources.

## Quick Navigation

| I want to... | Go to |
|-------------|-------|
| Get started quickly | [Quick Start Guide](guides/QUICK_START.md) |
| Learn all features | [Features Guide](FEATURES.md) |
| See the full API | [API Reference](API_REFERENCE.md) |
| Understand the architecture | [Architecture](../ARCHITECTURE.md) |
| Check performance data | [Benchmarks](BENCHMARKS.md) |
| Fix an issue | [Troubleshooting](guides/TROUBLESHOOTING.md) |
| Find answers | [FAQ](guides/FAQ.md) |
| Follow best practices | [Best Practices](guides/BEST_PRACTICES.md) |
| Run or write tests | [Testing Guide](contributing/TESTING.md) |
| Contribute code | [Contributing](../CONTRIBUTING.md) |

## Documentation Structure

```
docs/
├── README.md                    # This file - Documentation hub
├── FEATURES.md                  # Complete feature documentation
├── API_REFERENCE.md             # Full API reference
├── BENCHMARKS.md                # Performance analysis
├── PRODUCTION_QUALITY.md        # Quality & readiness report
├── ARRAY_VALUE_GUIDE.md         # Array value implementation guide
├── PROJECT_STRUCTURE.md         # Codebase organization
├── IMPROVEMENT_PLAN.md          # Roadmap and planned features
│
├── guides/                      # User guides
│   ├── QUICK_START.md          # 5-minute quick start
│   ├── FAQ.md                  # Frequently asked questions
│   ├── TROUBLESHOOTING.md      # Common issues and solutions
│   └── BEST_PRACTICES.md       # Recommended patterns
│
├── contributing/                # Contributor guides
│   └── TESTING.md              # Testing strategy and guide
│
└── performance/                 # Performance documentation
    └── BASELINE.md             # Performance baselines
```

## Getting Started

### For New Users

1. **[Quick Start Guide](guides/QUICK_START.md)** - Get running in 5 minutes
2. **[Features Guide](FEATURES.md)** - Learn what the system can do
3. **[Examples](../examples/)** - See working code

### For Developers Integrating the Library

1. **[API Reference](API_REFERENCE.md)** - Complete API documentation
2. **[Best Practices](guides/BEST_PRACTICES.md)** - Recommended usage patterns
3. **[Architecture](../ARCHITECTURE.md)** - System design overview

### For Contributors

1. **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute
2. **[Testing Guide](contributing/TESTING.md)** - Testing requirements
3. **[Project Structure](PROJECT_STRUCTURE.md)** - Codebase organization

## Key Features

| Feature | Description | Documentation |
|---------|-------------|---------------|
| **16 Value Types** | Comprehensive type system | [Features](FEATURES.md#11-type-system) |
| **Thread Safety** | Built-in `Arc<RwLock<T>>` | [Features](FEATURES.md#13-thread-safety) |
| **Serialization** | JSON, XML, Wire Protocol | [Features](FEATURES.md#12-serialization) |
| **Builder Pattern** | Fluent API for construction | [API Reference](API_REFERENCE.md#valuecontainerbuilder) |
| **Dependency Injection** | ContainerFactory trait for DI patterns | [Features](FEATURES.md#24-dependency-injection-support) |
| **Messaging Builder** | MessagingContainerBuilder for messaging patterns | [API Reference](API_REFERENCE.md#messagingcontainerbuilder) |
| **Zero Unsafe** | 100% safe Rust | [Production Quality](PRODUCTION_QUALITY.md) |

## Performance Highlights

| Operation | Performance | Notes |
|-----------|-------------|-------|
| Value Creation | 18-40 ns | Primitives: 18ns, Strings: 40ns |
| Container Add | 170 ns/value | Amortized, linear scaling |
| HashMap Lookup | 21 ns | O(1), size-independent |
| JSON Serialization | 1.8 µs/value | 558K ops/sec |
| XML Serialization | 560 ns/value | **3x faster than JSON** |

**Full details**: [Benchmarks](BENCHMARKS.md)

## Cross-Language Compatibility

The Rust Container System is compatible with other language implementations:

| Language | Repository | Wire Protocol | JSON v2 |
|----------|-----------|---------------|---------|
| **C++** | [container_system](https://github.com/kcenon/container_system) | ✅ | ✅ |
| **Python** | [python_container_system](https://github.com/kcenon/python_container_system) | ✅ | ✅ |
| **Go** | [go_container_system](https://github.com/kcenon/go_container_system) | ✅ | ✅ |
| **Node.js** | [nodejs_container_system](https://github.com/kcenon/nodejs_container_system) | ✅ | ✅ |
| **.NET** | [dotnet_container_system](https://github.com/kcenon/dotnet_container_system) | ✅ | ✅ |

## Common Use Cases

### Web Services / APIs
```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// Create response container
let mut response = ValueContainer::builder()
    .source("api_server", "v1")
    .message_type("user_response")
    .build();

response.add_value(Arc::new(IntValue::new("status", 200)))?;
response.add_value(Arc::new(StringValue::new("message", "Success")))?;

let json = response.serialize_cpp_wire()?; // Cross-language compatible
```

### Message Queuing
```rust
// Create message with routing
let mut message = ValueContainer::builder()
    .source("producer", "queue_1")
    .target("consumer", "worker_pool")
    .message_type("task_request")
    .build();

message.add_value(Arc::new(StringValue::new("task_type", "process_image")))?;
message.add_value(Arc::new(BytesValue::new("payload", image_data)))?;
```

### Configuration Storage
```rust
let mut config = ValueContainer::new();
config.set_message_type("app_config");

config.add_value(Arc::new(IntValue::new("max_connections", 100)))?;
config.add_value(Arc::new(StringValue::new("log_level", "info")))?;
config.add_value(Arc::new(BoolValue::new("debug_mode", false)))?;
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/kcenon/rust_container_system/issues)
- **FAQ**: [Frequently Asked Questions](guides/FAQ.md)
- **Troubleshooting**: [Common Issues](guides/TROUBLESHOOTING.md)

## Related Projects

- [container_system (C++)](https://github.com/kcenon/container_system) - Original implementation
- [messaging_system](https://github.com/kcenon/messaging_system) - Network messaging framework
- [thread_system](https://github.com/kcenon/thread_system) - Thread pool utilities

---

*This documentation is maintained alongside the codebase. For the latest updates, check the [CHANGELOG](../CHANGELOG.md).*
