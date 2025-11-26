# Best Practices Guide

> **Version**: 0.1.0
> **Last Updated**: 2025-11-26

Recommended patterns and practices for using the Rust Container System effectively.

## Table of Contents

- [Container Creation](#container-creation)
- [Value Management](#value-management)
- [Serialization](#serialization)
- [Performance Optimization](#performance-optimization)
- [Error Handling](#error-handling)
- [Thread Safety](#thread-safety)
- [Cross-Language Compatibility](#cross-language-compatibility)
- [Testing](#testing)
- [Anti-Patterns](#anti-patterns)

---

## Container Creation

### Use the Builder Pattern

**Recommended:**
```rust
let container = ValueContainer::builder()
    .source("client_app", "session_123")
    .target("server", "handler")
    .message_type("user_request")
    .max_values(1000)
    .build();
```

**Why:**
- Clear, readable code
- Compile-time validation
- Fluent API
- Easy to maintain

### Set Appropriate Value Limits

**Recommended:**
```rust
// For known data sizes
let container = ValueContainer::with_max_values(expected_count + buffer);

// For user-provided data (prevent DoS)
let container = ValueContainer::with_max_values(100);
```

**Why:**
- Prevents memory exhaustion attacks
- Makes memory usage predictable
- Fails fast on malformed input

### Use Meaningful Message Types

**Recommended:**
```rust
container.set_message_type("user.create.request");
container.set_message_type("order.process.response");
container.set_message_type("auth.token.refresh");
```

**Avoid:**
```rust
container.set_message_type("msg1");
container.set_message_type("data");
```

---

## Value Management

### Choose the Right Value Type

| Use Case | Recommended Type |
|----------|-----------------|
| Small integers (-32K to 32K) | `ShortValue` |
| Standard integers | `IntValue` |
| Timestamps, file sizes | `LongValue` (range-checked) |
| Full 64-bit integers | `LLongValue` |
| Cross-language integers | `LongValue` / `ULongValue` |
| Currency (cents) | `LongValue` |
| Scientific values | `DoubleValue` |
| User text | `StringValue` |
| Binary data | `BytesValue` |
| Hierarchical data | `ContainerValue` |
| Lists | `ArrayValue` |

### Use Consistent Naming

**Recommended:**
```rust
// Use snake_case for names
container.add_value(Arc::new(IntValue::new("user_id", 123)))?;
container.add_value(Arc::new(StringValue::new("first_name", "Alice")))?;
container.add_value(Arc::new(StringValue::new("created_at", timestamp)))?;
```

**Avoid:**
```rust
container.add_value(Arc::new(IntValue::new("userId", 123)))?;
container.add_value(Arc::new(IntValue::new("UserID", 123)))?;
container.add_value(Arc::new(IntValue::new("user-id", 123)))?;
```

### Group Related Values with ContainerValue

**Recommended:**
```rust
// Group user data
let user = ContainerValue::new("user", vec![
    Arc::new(IntValue::new("id", 123)),
    Arc::new(StringValue::new("name", "Alice")),
    Arc::new(StringValue::new("email", "alice@example.com")),
]);

// Group address data
let address = ContainerValue::new("address", vec![
    Arc::new(StringValue::new("street", "123 Main St")),
    Arc::new(StringValue::new("city", "Boston")),
    Arc::new(StringValue::new("zip", "02101")),
]);

container.add_value(Arc::new(user))?;
container.add_value(Arc::new(address))?;
```

---

## Serialization

### Use Wire Protocol for Cross-Language

**Recommended:**
```rust
// For C++/Python/Go/Node.js/.NET interop
let wire_data = container.serialize_cpp_wire()?;
let restored = ValueContainer::deserialize_cpp_wire(&wire_data)?;
```

**Why:**
- Guaranteed compatibility across languages
- Efficient binary format
- Well-tested with other implementations

### Use JSON v2.0 for Human-Readable Format

**Recommended:**
```rust
use rust_container_system::core::json_v2_adapter::JsonV2Adapter;

// When human readability matters
let json = JsonV2Adapter::serialize(&container)?;
let restored = JsonV2Adapter::deserialize(&json)?;
```

### Avoid Deprecated Methods

**Avoid:**
```rust
#[allow(deprecated)]
let json = container.to_json()?;  // Deprecated
let xml = container.to_xml()?;    // Deprecated
```

**Use instead:**
```rust
let wire = container.serialize_cpp_wire()?;
// or
let json = JsonV2Adapter::serialize(&container)?;
```

---

## Performance Optimization

### Use Zero-Copy Access Methods

**Recommended:**
```rust
// Don't clone when reading
container.with_source_id(|id| {
    println!("Source: {}", id);
});

container.with_values(|values| {
    for v in values {
        // Process without cloning Vec
    }
});

container.with_value_array("tags", |tags| {
    tags.len()
}).unwrap_or(0);
```

**Avoid:**
```rust
// Unnecessary cloning
let source = container.source_id(); // Clones String
let values = container.values();    // Clones Vec
```

### Minimize Lock Contention

**Recommended:**
```rust
// Read once, process outside lock
let data = container.with_values(|values| {
    values.iter()
        .map(|v| (v.name().to_string(), v.to_string()))
        .collect::<Vec<_>>()
});

// Now process without holding lock
for (name, value) in data {
    expensive_operation(&name, &value);
}
```

**Avoid:**
```rust
// Holding lock during expensive operations
for value in &container {
    expensive_operation(value.name(), &value.to_string());
}
```

### Pre-allocate When Possible

**Recommended:**
```rust
// When you know the count ahead of time
let mut container = ValueContainer::with_max_values(expected_count);
```

### Use Appropriate Numeric Types

**Recommended:**
```rust
// Use smaller types when possible
let port = ShortValue::new("port", 8080);        // 2 bytes
let count = IntValue::new("count", 1000);         // 4 bytes
let timestamp = LongValue::new("time", ts)?;      // 8 bytes
```

---

## Error Handling

### Use the ? Operator

**Recommended:**
```rust
fn process_container(container: &ValueContainer) -> Result<i32> {
    let value = container.get_value("count")
        .ok_or_else(|| ContainerError::ValueNotFound("count".to_string()))?;

    let count = value.to_int()?;
    Ok(count)
}
```

### Handle Missing Values Gracefully

**Recommended:**
```rust
// With default
let count = container.get_value("count")
    .and_then(|v| v.to_int().ok())
    .unwrap_or(0);

// With explicit handling
match container.get_value("required_field") {
    Some(value) => process(value),
    None => return Err(MyError::MissingField("required_field")),
}
```

### Validate Early

**Recommended:**
```rust
fn create_user(container: &ValueContainer) -> Result<User> {
    // Validate all required fields first
    let name = container.get_value("name")
        .ok_or(ContainerError::ValueNotFound("name".to_string()))?;
    let email = container.get_value("email")
        .ok_or(ContainerError::ValueNotFound("email".to_string()))?;

    // Then process
    Ok(User {
        name: name.to_string(),
        email: email.to_string(),
    })
}
```

---

## Thread Safety

### Clone Arc, Not Container

**Recommended:**
```rust
use std::sync::Arc;
use std::thread;

let container = Arc::new(ValueContainer::new());

// O(1) - just increments reference count
let container_clone = Arc::clone(&container);

thread::spawn(move || {
    // Use container_clone
});
```

### Use Separate Containers for Writers

**Recommended:**
```rust
// Each writer has its own container
fn worker(id: usize) -> ValueContainer {
    let mut container = ValueContainer::new();
    container.add_value(Arc::new(IntValue::new("worker_id", id as i32))).ok();
    // ... add more values
    container
}

// Merge results later
let results: Vec<ValueContainer> = (0..4)
    .into_par_iter()
    .map(worker)
    .collect();
```

### Avoid Shared Mutable State

**Avoid:**
```rust
let container = Arc::new(Mutex::new(ValueContainer::new()));
// Multiple threads fighting for the lock
```

**Consider instead:**
- Message passing (channels)
- Per-thread containers
- Read-only shared containers

---

## Cross-Language Compatibility

### Use Range-Checked Types

**Recommended:**
```rust
// For C++ compatibility
let timestamp = LongValue::new("ts", value)?;  // Range checked
let size = ULongValue::new("size", value)?;    // Range checked
```

### Test Interoperability

**Recommended:**
```rust
#[test]
fn test_cpp_interop() {
    let mut container = ValueContainer::new();
    container.add_value(Arc::new(IntValue::new("test", 42))).unwrap();

    // Serialize
    let wire = container.serialize_cpp_wire().unwrap();

    // Verify it can be deserialized
    let restored = ValueContainer::deserialize_cpp_wire(&wire).unwrap();
    assert_eq!(restored.get_value("test").unwrap().to_int().unwrap(), 42);
}
```

### Document Type Mappings

```rust
// Type mapping documentation for cross-language projects
//
// | Rust Type    | C++ Type          | Python Type | Type Code |
// |--------------|-------------------|-------------|-----------|
// | IntValue     | int32_t           | int         | 4         |
// | LongValue    | int64_t (limited) | int         | 6         |
// | StringValue  | std::string       | str         | 12        |
```

---

## Testing

### Test Serialization Round-Trips

**Recommended:**
```rust
#[test]
fn test_serialization_roundtrip() {
    let mut original = ValueContainer::new();
    original.add_value(Arc::new(StringValue::new("key", "value"))).unwrap();

    let wire = original.serialize_cpp_wire().unwrap();
    let restored = ValueContainer::deserialize_cpp_wire(&wire).unwrap();

    assert_eq!(
        original.get_value("key").unwrap().to_string(),
        restored.get_value("key").unwrap().to_string()
    );
}
```

### Test Edge Cases

**Recommended:**
```rust
#[test]
fn test_empty_container() {
    let container = ValueContainer::new();
    assert!(container.is_empty());
    assert!(container.serialize_cpp_wire().is_ok());
}

#[test]
fn test_max_values() {
    let mut container = ValueContainer::with_max_values(2);
    assert!(container.add_value(Arc::new(IntValue::new("a", 1))).is_ok());
    assert!(container.add_value(Arc::new(IntValue::new("b", 2))).is_ok());
    assert!(container.add_value(Arc::new(IntValue::new("c", 3))).is_err());
}
```

---

## Anti-Patterns

### Don't Use String for Everything

**Avoid:**
```rust
container.add_value(Arc::new(StringValue::new("count", "42")))?;
container.add_value(Arc::new(StringValue::new("active", "true")))?;
container.add_value(Arc::new(StringValue::new("price", "19.99")))?;
```

**Use proper types:**
```rust
container.add_value(Arc::new(IntValue::new("count", 42)))?;
container.add_value(Arc::new(BoolValue::new("active", true)))?;
container.add_value(Arc::new(DoubleValue::new("price", 19.99)))?;
```

### Don't Ignore Errors

**Avoid:**
```rust
container.add_value(Arc::new(IntValue::new("key", 1))).ok(); // Silent failure
let _ = container.serialize_cpp_wire(); // Ignored error
```

**Handle them:**
```rust
container.add_value(Arc::new(IntValue::new("key", 1)))?;
let data = container.serialize_cpp_wire()?;
```

### Don't Over-Nest

**Avoid:**
```rust
// Too deep nesting
let level4 = ContainerValue::new("d", vec![...]);
let level3 = ContainerValue::new("c", vec![Arc::new(level4)]);
let level2 = ContainerValue::new("b", vec![Arc::new(level3)]);
let level1 = ContainerValue::new("a", vec![Arc::new(level2)]);
```

**Flatten when possible:**
```rust
// Use naming conventions instead
container.add_value(Arc::new(StringValue::new("user.address.street", "123 Main")))?;
container.add_value(Arc::new(StringValue::new("user.address.city", "Boston")))?;
```

### Don't Mix Deprecated and New APIs

**Avoid:**
```rust
#[allow(deprecated)]
let json = container.to_json()?;
let restored = JsonV2Adapter::deserialize(&json)?; // Format mismatch!
```

**Be consistent:**
```rust
let json = JsonV2Adapter::serialize(&container)?;
let restored = JsonV2Adapter::deserialize(&json)?;
```

---

## Summary Checklist

- [ ] Use builder pattern for container creation
- [ ] Set appropriate value limits
- [ ] Choose correct value types
- [ ] Use snake_case for value names
- [ ] Use wire protocol for cross-language
- [ ] Use zero-copy methods for performance
- [ ] Handle errors with ? operator
- [ ] Test serialization round-trips
- [ ] Avoid anti-patterns

---

*For more information, see the [API Reference](../API_REFERENCE.md) and [FAQ](FAQ.md).*
