# Testing Guide

> **Version**: 0.1.0
> **Last Updated**: 2025-11-26

Comprehensive guide to testing in the Rust Container System project.

## Table of Contents

- [Test Overview](#test-overview)
- [Running Tests](#running-tests)
- [Test Organization](#test-organization)
- [Writing Tests](#writing-tests)
- [Test Categories](#test-categories)
- [Benchmarking](#benchmarking)
- [Coverage](#coverage)
- [CI/CD Integration](#cicd-integration)

---

## Test Overview

The project maintains high test coverage across multiple test categories:

| Category | Location | Framework | Purpose |
|----------|----------|-----------|---------|
| Unit Tests | `src/**/*.rs` | Built-in | Per-module functionality |
| Integration Tests | `tests/` | Built-in | Cross-module behavior |
| Property Tests | `tests/property_tests.rs` | `proptest` | Randomized input testing |
| Interop Tests | `tests/interop_tests.rs` | Built-in | Cross-language compatibility |
| Benchmarks | `benches/` | `criterion` | Performance measurement |

### Current Status

- **Tests**: 60/62 passing (96.8%)
- **Known Issues**: 2 wire protocol nested structure tests
- **Coverage Target**: 80%+

---

## Running Tests

### Basic Commands

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test file
cargo test --test integration_tests

# Run tests matching pattern
cargo test test_container

# Run specific test
cargo test test_container_creation -- --exact
```

### Test Options

```bash
# Run tests in parallel (default)
cargo test

# Run tests sequentially
cargo test -- --test-threads=1

# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored

# Show test timing
cargo test -- --show-output
```

### Running Specific Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --tests

# Property tests
cargo test --test property_tests

# Interop tests
cargo test --test interop_tests

# Benchmarks
cargo bench
```

---

## Test Organization

### Directory Structure

```
rust_container_system/
├── src/
│   ├── lib.rs                          # Library tests
│   ├── core/
│   │   ├── container.rs                # Container unit tests
│   │   ├── value.rs                    # Value trait tests
│   │   ├── error.rs                    # Error type tests
│   │   └── wire_protocol.rs            # Wire protocol tests
│   └── values/
│       ├── primitive_values.rs         # Numeric type tests
│       ├── string_value.rs             # String tests
│       ├── bytes_value.rs              # Binary data tests
│       └── array_value.rs              # Array tests
│
├── tests/
│   ├── integration_tests.rs            # Full integration tests
│   ├── interop_tests.rs                # Cross-language tests
│   ├── binary_interop_tests.rs         # Binary format tests
│   ├── property_tests.rs               # Property-based tests
│   └── test_long_range_checking.rs     # Range validation tests
│
└── benches/
    └── container_benchmarks.rs         # Performance benchmarks
```

### Test Module Pattern

Each source file with significant logic includes a test module:

```rust
// src/core/container.rs

pub struct ValueContainer { ... }

impl ValueContainer { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let container = ValueContainer::new();
        assert!(container.is_empty());
    }

    #[test]
    fn test_add_value() {
        let mut container = ValueContainer::new();
        let value = Arc::new(IntValue::new("key", 42));
        assert!(container.add_value(value).is_ok());
    }
}
```

---

## Writing Tests

### Test Structure

Follow the Arrange-Act-Assert pattern:

```rust
#[test]
fn test_feature_name() {
    // Arrange
    let mut container = ValueContainer::new();
    let value = Arc::new(IntValue::new("count", 42));

    // Act
    let result = container.add_value(value);

    // Assert
    assert!(result.is_ok());
    assert_eq!(container.value_count(), 1);
}
```

### Naming Conventions

```rust
// Pattern: test_<feature>_<scenario>_<expected_result>

#[test]
fn test_container_add_value_succeeds() { ... }

#[test]
fn test_container_add_value_fails_when_limit_reached() { ... }

#[test]
fn test_serialization_json_roundtrip_preserves_data() { ... }
```

### Testing Errors

```rust
#[test]
fn test_value_limit_returns_error() {
    let mut container = ValueContainer::with_max_values(1);
    container.add_value(Arc::new(IntValue::new("a", 1))).unwrap();

    let result = container.add_value(Arc::new(IntValue::new("b", 2)));

    assert!(result.is_err());
    match result {
        Err(ContainerError::InvalidDataFormat(msg)) => {
            assert!(msg.contains("limit"));
        }
        _ => panic!("Expected InvalidDataFormat error"),
    }
}
```

### Testing with Results

```rust
#[test]
fn test_long_value_range() -> Result<(), Box<dyn std::error::Error>> {
    let long = LongValue::new("valid", 1_000_000)?;
    assert_eq!(long.to_long()?, 1_000_000);
    Ok(())
}
```

---

## Test Categories

### Unit Tests

Test individual functions and methods in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_is_numeric() {
        assert!(ValueType::Int.is_numeric());
        assert!(ValueType::Double.is_numeric());
        assert!(!ValueType::String.is_numeric());
        assert!(!ValueType::Bytes.is_numeric());
    }

    #[test]
    fn test_xml_escape() {
        let escaped = xml_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;script&gt;"));
    }
}
```

### Integration Tests

Test multiple components working together.

```rust
// tests/integration_tests.rs

use rust_container_system::prelude::*;
use std::sync::Arc;

#[test]
fn test_full_workflow() {
    // Create container
    let mut container = ValueContainer::builder()
        .source("test", "1")
        .message_type("integration_test")
        .build();

    // Add various value types
    container.add_value(Arc::new(IntValue::new("int", 42))).unwrap();
    container.add_value(Arc::new(StringValue::new("str", "hello"))).unwrap();
    container.add_value(Arc::new(BoolValue::new("bool", true))).unwrap();

    // Serialize and deserialize
    let wire = container.serialize_cpp_wire().unwrap();
    let restored = ValueContainer::deserialize_cpp_wire(&wire).unwrap();

    // Verify
    assert_eq!(restored.value_count(), 3);
    assert_eq!(restored.get_value("int").unwrap().to_int().unwrap(), 42);
}
```

### Property Tests

Test with randomized inputs using `proptest`.

```rust
// tests/property_tests.rs

use proptest::prelude::*;
use rust_container_system::prelude::*;
use std::sync::Arc;

proptest! {
    #[test]
    fn test_int_roundtrip(value: i32) {
        let int_val = IntValue::new("test", value);
        prop_assert_eq!(int_val.to_int().unwrap(), value);
    }

    #[test]
    fn test_string_roundtrip(s in "\\PC*") {
        let string_val = StringValue::new("test", &s);
        prop_assert_eq!(string_val.to_string(), s);
    }

    #[test]
    fn test_container_serialization(values in prop::collection::vec(any::<i32>(), 0..100)) {
        let mut container = ValueContainer::new();
        for (i, v) in values.iter().enumerate() {
            container.add_value(Arc::new(IntValue::new(format!("v{}", i), *v))).unwrap();
        }

        let wire = container.serialize_cpp_wire().unwrap();
        let restored = ValueContainer::deserialize_cpp_wire(&wire).unwrap();

        prop_assert_eq!(container.value_count(), restored.value_count());
    }
}
```

### Interoperability Tests

Test cross-language compatibility.

```rust
// tests/interop_tests.rs

#[test]
fn test_cpp_wire_format_compatibility() {
    // Wire format generated by C++ container_system
    let cpp_wire = "@header={{[3,client];[5,test];}};@data={{[count,int_value,42];}};";

    let container = ValueContainer::deserialize_cpp_wire(cpp_wire).unwrap();

    assert_eq!(container.source_id(), "client");
    assert_eq!(container.get_value("count").unwrap().to_int().unwrap(), 42);
}
```

---

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench container_creation

# Save baseline
cargo bench -- --save-baseline main

# Compare with baseline
cargo bench -- --baseline main
```

### Writing Benchmarks

```rust
// benches/container_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_container_system::prelude::*;
use std::sync::Arc;

fn benchmark_container_creation(c: &mut Criterion) {
    c.bench_function("container_new", |b| {
        b.iter(|| {
            black_box(ValueContainer::new())
        })
    });
}

fn benchmark_value_addition(c: &mut Criterion) {
    c.bench_function("add_int_value", |b| {
        let mut container = ValueContainer::new();
        b.iter(|| {
            container.add_value(Arc::new(IntValue::new("key", 42))).ok();
        })
    });
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut container = ValueContainer::new();
    for i in 0..100 {
        container.add_value(Arc::new(IntValue::new(format!("v{}", i), i))).unwrap();
    }

    c.bench_function("serialize_wire_100_values", |b| {
        b.iter(|| {
            black_box(container.serialize_cpp_wire().unwrap())
        })
    });
}

criterion_group!(
    benches,
    benchmark_container_creation,
    benchmark_value_addition,
    benchmark_serialization
);
criterion_main!(benches);
```

### Performance Targets

| Operation | Target | Acceptable |
|-----------|--------|------------|
| Value Creation | < 50 ns | < 100 ns |
| Container Add | < 200 ns | < 500 ns |
| HashMap Lookup | < 30 ns | < 100 ns |
| Wire Serialize (100 values) | < 50 µs | < 100 µs |

---

## Coverage

### Using cargo-tarpaulin

```bash
# Install
cargo install cargo-tarpaulin

# Run with HTML report
cargo tarpaulin --out Html

# Run with specific target
cargo tarpaulin --target-dir coverage --out Html

# Exclude tests from coverage
cargo tarpaulin --ignore-tests --out Html
```

### Coverage Targets

| Component | Target | Current |
|-----------|--------|---------|
| Core | 90% | ~85% |
| Values | 85% | ~80% |
| Integration | 80% | ~75% |
| Overall | 80% | ~80% |

### Coverage Report

The HTML report is generated at `tarpaulin-report.html`.

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/rust.yml

name: Rust CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Run tests
      run: cargo test --all-features

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Check formatting
      run: cargo fmt -- --check

  benchmark:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Run benchmarks
      run: cargo bench --no-run

  coverage:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin

    - name: Generate coverage
      run: cargo tarpaulin --out Xml

    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

### Pre-commit Checks

Before committing:

```bash
# Run all checks
cargo test && cargo clippy && cargo fmt --check

# Or use a script
./scripts/pre-commit.sh
```

---

## Test Quality Guidelines

### What to Test

- **Happy paths**: Normal use cases
- **Edge cases**: Empty containers, max values, boundary values
- **Error conditions**: Invalid inputs, type mismatches
- **Serialization**: Round-trip for all value types
- **Thread safety**: Concurrent access patterns

### What NOT to Test

- Trivial getters/setters without logic
- External library functionality
- Private implementation details that may change

### Test Independence

Each test should:
- Create its own test data
- Not depend on other tests
- Clean up after itself (if needed)
- Be able to run in any order

---

## Debugging Tests

### Enable Logging

```rust
#[test]
fn test_with_logging() {
    eprintln!("Debug: starting test");

    let container = ValueContainer::new();
    eprintln!("Debug: container created, count = {}", container.value_count());

    // ...
}
```

Run with:
```bash
cargo test -- --nocapture
```

### Using Debug Assertions

```rust
#[test]
fn test_with_debug() {
    let container = ValueContainer::new();

    debug_assert!(container.is_empty(), "Container should be empty");

    // ...
}
```

---

## Contributing Tests

When adding new features:

1. **Write tests first** (TDD recommended)
2. **Cover happy path** and error cases
3. **Add integration test** if feature spans modules
4. **Update benchmarks** if performance-critical
5. **Ensure all tests pass** before submitting PR

```bash
# Full check before PR
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt -- --check
cargo bench --no-run
```

---

*For more information, see [CONTRIBUTING.md](../../CONTRIBUTING.md) and [Best Practices](../guides/BEST_PRACTICES.md).*
