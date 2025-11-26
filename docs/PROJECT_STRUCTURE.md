# Project Structure

> **Version**: 0.1.0
> **Last Updated**: 2025-11-26

This document describes the organization of the Rust Container System codebase.

## Table of Contents

- [Directory Overview](#directory-overview)
- [Source Code Structure](#source-code-structure)
- [Module Organization](#module-organization)
- [Test Organization](#test-organization)
- [Documentation Structure](#documentation-structure)
- [Build Configuration](#build-configuration)

---

## Directory Overview

```
rust_container_system/
├── Cargo.toml                 # Package manifest
├── Cargo.lock                 # Dependency lock file
├── LICENSE-MIT                # MIT License
├── LICENSE-APACHE             # Apache 2.0 License
│
├── README.md                  # Main project documentation
├── README.ko.md               # Korean documentation
├── ARCHITECTURE.md            # System architecture
├── CHANGELOG.md               # Version history
├── CONTRIBUTING.md            # Contribution guidelines
├── DOCUMENTATION.md           # Learning guide
├── MIGRATION_GUIDE.md         # Wire protocol migration
├── IMPROVEMENTS.md            # Improvement opportunities
├── COMPLETION_REPORT.md       # Implementation status
├── REVIEW.md                  # Code review notes
│
├── src/                       # Source code
│   ├── lib.rs                # Library entry point
│   ├── core/                 # Core types and traits
│   └── values/               # Value implementations
│
├── tests/                     # Integration tests
│   ├── integration_tests.rs
│   ├── interop_tests.rs
│   ├── binary_interop_tests.rs
│   ├── property_tests.rs
│   └── test_long_range_checking.rs
│
├── examples/                  # Usage examples
│   ├── basic_container.rs
│   ├── serialization.rs
│   ├── nested_containers.rs
│   ├── concurrency.rs
│   ├── error_handling.rs
│   └── deserialization.rs
│
├── benches/                   # Benchmarks
│   └── container_benchmarks.rs
│
├── docs/                      # Documentation
│   ├── README.md             # Documentation hub
│   ├── API_REFERENCE.md      # API documentation
│   ├── FEATURES.md           # Feature guide
│   ├── BENCHMARKS.md         # Performance analysis
│   ├── PRODUCTION_QUALITY.md # Quality report
│   ├── ARRAY_VALUE_GUIDE.md  # Array documentation
│   ├── PROJECT_STRUCTURE.md  # This file
│   ├── IMPROVEMENT_PLAN.md   # Roadmap
│   ├── guides/               # User guides
│   ├── contributing/         # Contributor guides
│   └── performance/          # Performance docs
│
└── target/                    # Build output (gitignored)
```

---

## Source Code Structure

### Library Entry Point

**`src/lib.rs`**

The main library entry point that:
- Declares public modules (`core`, `values`)
- Defines the `prelude` module for convenient imports
- Re-exports commonly used types at the root level

```rust
// Public modules
pub mod core;
pub mod values;

// Prelude for convenient imports
pub mod prelude {
    pub use crate::core::{...};
    pub use crate::values::{...};
}

// Root-level re-exports
pub use core::{ContainerError, Result, Value, ValueContainer, ValueType};
pub use values::{BoolValue, BytesValue, DoubleValue, IntValue, LongValue, StringValue};
```

### Core Module

**`src/core/`** - Foundation types and traits

| File | Purpose | Key Types |
|------|---------|-----------|
| `mod.rs` | Module declarations and re-exports | - |
| `error.rs` | Error types | `ContainerError`, `Result` |
| `value.rs` | Value trait definition | `Value`, `BaseValue` |
| `value_types.rs` | Type enumeration | `ValueType` |
| `container.rs` | Container implementation | `ValueContainer`, `ValueContainerBuilder`, `ValueIter` |
| `wire_protocol.rs` | C++ wire protocol | `serialize_cpp_wire()`, `deserialize_cpp_wire()` |
| `json_v2_adapter.rs` | JSON v2.0 compatibility | `JsonV2Adapter`, `SerializationFormat` |

#### Core Module Dependency Graph

```
core/
├── error.rs           ← No dependencies
├── value_types.rs     ← No dependencies
├── value.rs           ← error, value_types
├── container.rs       ← error, value, value_types
├── wire_protocol.rs   ← error, container, value, values/*
└── json_v2_adapter.rs ← error, container, value, value_types
```

### Values Module

**`src/values/`** - Concrete value type implementations

| File | Purpose | Key Types |
|------|---------|-----------|
| `mod.rs` | Module declarations and re-exports | - |
| `primitive_values.rs` | Numeric and boolean types | `BoolValue`, `ShortValue`, `IntValue`, `LongValue`, `FloatValue`, `DoubleValue`, etc. |
| `string_value.rs` | UTF-8 string type | `StringValue` |
| `bytes_value.rs` | Binary data type | `BytesValue` |
| `container_value.rs` | Nested containers | `ContainerValue` |
| `array_value.rs` | Heterogeneous arrays | `ArrayValue` |

#### Value Type Hierarchy

```
Value (trait)
├── BoolValue
├── ShortValue / UShortValue
├── IntValue / UIntValue
├── LongValue / ULongValue (range-checked)
├── LLongValue / ULLongValue (full range)
├── FloatValue / DoubleValue
├── StringValue
├── BytesValue
├── ContainerValue
└── ArrayValue
```

---

## Module Organization

### Public API Surface

The library exposes types through multiple paths:

```rust
// 1. Prelude (recommended)
use rust_container_system::prelude::*;

// 2. Core module
use rust_container_system::core::{ValueContainer, Value, ValueType, ContainerError, Result};

// 3. Values module
use rust_container_system::values::{IntValue, StringValue, ArrayValue};

// 4. Root re-exports (common types only)
use rust_container_system::{ValueContainer, IntValue, StringValue};
```

### Internal vs Public

| Visibility | Description | Example |
|------------|-------------|---------|
| `pub` | Public API | `ValueContainer`, `Value` |
| `pub(crate)` | Crate-internal | Helper functions |
| Private | Implementation detail | `ContainerInner` |

---

## Test Organization

### Test Types

```
tests/
├── integration_tests.rs          # Full system integration tests
├── interop_tests.rs              # Cross-language compatibility tests
├── binary_interop_tests.rs       # Binary wire protocol tests
├── property_tests.rs             # Property-based tests (proptest)
└── test_long_range_checking.rs   # Long/ULong range validation tests
```

### Test Categories

| Category | Location | Framework | Purpose |
|----------|----------|-----------|---------|
| Unit tests | `src/**/*.rs` | Built-in | Per-module functionality |
| Integration tests | `tests/` | Built-in | Cross-module behavior |
| Property tests | `tests/property_tests.rs` | `proptest` | Randomized input testing |
| Benchmarks | `benches/` | `criterion` | Performance measurement |

### Running Tests

```bash
# All tests
cargo test

# Specific test file
cargo test --test integration_tests

# Tests with output
cargo test -- --nocapture

# Property tests only
cargo test --test property_tests

# With coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

---

## Documentation Structure

### Documentation Files

```
docs/
├── README.md                    # Documentation hub
├── API_REFERENCE.md             # Complete API documentation
├── FEATURES.md                  # Feature guide with examples
├── BENCHMARKS.md                # Performance analysis
├── PRODUCTION_QUALITY.md        # Quality metrics and readiness
├── ARRAY_VALUE_GUIDE.md         # Array value deep dive
├── PROJECT_STRUCTURE.md         # This file
├── IMPROVEMENT_PLAN.md          # Future roadmap
│
├── guides/                      # User-focused guides
│   ├── QUICK_START.md          # Getting started in 5 minutes
│   ├── FAQ.md                  # Frequently asked questions
│   ├── TROUBLESHOOTING.md      # Common issues and solutions
│   └── BEST_PRACTICES.md       # Recommended patterns
│
├── contributing/                # Contributor-focused guides
│   └── TESTING.md              # Testing strategy and guidelines
│
└── performance/                 # Performance documentation
    └── BASELINE.md             # Performance baselines
```

### Root-Level Documentation

| File | Purpose |
|------|---------|
| `README.md` | Project overview and quick start |
| `README.ko.md` | Korean translation |
| `ARCHITECTURE.md` | System design and principles |
| `CHANGELOG.md` | Version history |
| `CONTRIBUTING.md` | How to contribute |
| `DOCUMENTATION.md` | Comprehensive learning guide |
| `MIGRATION_GUIDE.md` | Wire protocol migration |

---

## Build Configuration

### Cargo.toml

```toml
[package]
name = "rust_container_system"
version = "0.1.0"
edition = "2021"
license = "BSD-3-Clause"
description = "High-performance container framework for type-safe data management"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
quick-xml = { version = "0.31", features = ["serialize"] }
thiserror = "2.0"
parking_lot = "0.12"
base64 = "0.22"
regex = "1.10"
indexmap = { version = "2.1", features = ["serde"] }

[dev-dependencies]
criterion = "0.5"
proptest = "1.4"

[[bench]]
name = "container_benchmarks"
harness = false
```

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `serde` | Serialization framework |
| `serde_json` | JSON serialization |
| `quick-xml` | XML serialization |
| `thiserror` | Error type derivation |
| `parking_lot` | High-performance synchronization |
| `base64` | Binary data encoding |
| `regex` | Wire protocol parsing |
| `indexmap` | Insertion-order preserving map |

### Dev Dependencies

| Dependency | Purpose |
|------------|---------|
| `criterion` | Benchmarking framework |
| `proptest` | Property-based testing |

---

## Examples Directory

### Available Examples

| Example | Description | Run Command |
|---------|-------------|-------------|
| `basic_container.rs` | Basic operations | `cargo run --example basic_container` |
| `serialization.rs` | JSON/XML/Wire serialization | `cargo run --example serialization` |
| `nested_containers.rs` | Hierarchical data | `cargo run --example nested_containers` |
| `concurrency.rs` | Thread-safe operations | `cargo run --example concurrency` |
| `error_handling.rs` | Error patterns | `cargo run --example error_handling` |
| `deserialization.rs` | Parsing data | `cargo run --example deserialization` |

---

## Code Statistics

| Metric | Value |
|--------|-------|
| Source files | ~15 |
| Lines of Rust code | ~4,000 |
| Test files | 5 |
| Example files | 6 |
| Documentation files | 20+ |
| Dependencies | 8 |
| Dev dependencies | 2 |

---

## See Also

- [Architecture](../ARCHITECTURE.md) - System design details
- [Contributing](../CONTRIBUTING.md) - How to contribute
- [Testing Guide](contributing/TESTING.md) - Testing requirements

---

*This structure follows Rust conventions and enables efficient navigation for both users and contributors.*
