# Troubleshooting Guide

> **Version**: 0.1.0
> **Last Updated**: 2025-11-26

Solutions to common issues when using the Rust Container System.

## Table of Contents

- [Compilation Errors](#compilation-errors)
- [Runtime Errors](#runtime-errors)
- [Serialization Issues](#serialization-issues)
- [Performance Problems](#performance-problems)
- [Cross-Language Issues](#cross-language-issues)
- [Thread Safety Issues](#thread-safety-issues)
- [Memory Issues](#memory-issues)

---

## Compilation Errors

### Error: "expected trait Value, found struct IntValue"

**Problem:**
```rust
let mut container = ValueContainer::new();
container.add_value(IntValue::new("count", 42)); // Error!
```

**Solution:** Wrap values in `Arc`:
```rust
use std::sync::Arc;

container.add_value(Arc::new(IntValue::new("count", 42)))?;
```

**Why:** The container stores `Arc<dyn Value>` for thread safety and shared ownership.

---

### Error: "the trait `Value` cannot be made into an object"

**Problem:** Trying to use a type that doesn't fully implement `Value`.

**Solution:** Ensure your type implements all required trait methods, especially:
- `clone_value(&self) -> Arc<dyn Value>`
- `as_any(&self) -> &dyn Any`

---

### Error: "cannot borrow as mutable"

**Problem:**
```rust
let container = ValueContainer::new();
container.add_value(...)?; // Error: cannot borrow as mutable
```

**Solution:** Declare as `mut`:
```rust
let mut container = ValueContainer::new();
container.add_value(Arc::new(IntValue::new("key", 1)))?;
```

---

### Error: "the trait bound `i32: Into<String>` is not satisfied"

**Problem:** Wrong argument types for methods expecting strings.

**Solution:** Ensure string arguments are string types:
```rust
// Wrong
container.set_source(123, 456);

// Correct
container.set_source("123", "456");
// Or
container.set_source(id.to_string(), sub_id.to_string());
```

---

## Runtime Errors

### Error: "Container value limit reached"

**Problem:** Trying to add more values than the container allows.

**Solution:**
1. Increase the limit:
   ```rust
   let container = ValueContainer::with_max_values(50000);
   ```

2. Or check before adding:
   ```rust
   if container.try_add_value(Arc::new(IntValue::new("key", 1))) {
       // Success
   } else {
       // Limit reached, handle accordingly
   }
   ```

3. Or remove old values:
   ```rust
   container.remove_value("old_key");
   container.add_value(Arc::new(IntValue::new("new_key", 1)))?;
   ```

---

### Error: "ValueNotFound"

**Problem:** Trying to access a value that doesn't exist.

**Solution:** Use `Option` handling:
```rust
// Don't do this
let value = container.get_value("key").unwrap(); // Panics if not found

// Do this instead
if let Some(value) = container.get_value("key") {
    println!("Found: {}", value.to_string());
} else {
    println!("Key not found");
}

// Or with Result
let value = container.get_value("key")
    .ok_or_else(|| ContainerError::ValueNotFound("key".to_string()))?;
```

---

### Error: "InvalidTypeConversion"

**Problem:** Trying to convert a value to an incompatible type.

**Solution:**
1. Check the type first:
   ```rust
   let value = container.get_value("data").unwrap();

   if value.is_numeric() {
       let num = value.to_long()?;
   } else if value.is_string() {
       let text = value.to_string();
   }
   ```

2. Or handle the error:
   ```rust
   match value.to_int() {
       Ok(n) => println!("Number: {}", n),
       Err(_) => println!("Not a number, using string: {}", value.to_string()),
   }
   ```

---

### Error: "ValueOutOfRange"

**Problem:** Creating `LongValue` or `ULongValue` with out-of-range values.

**Solution:**
```rust
// LongValue is range-checked for C++ compatibility
// Use LLongValue for full i64 range
let long = LongValue::new("val", huge_number)?; // May fail

// Or use LLongValue which accepts full range
let llong = LLongValue::new("val", i64::MAX); // Always succeeds
```

---

## Serialization Issues

### JSON Deprecation Warnings

**Problem:**
```
WARNING: to_json() is deprecated and will be removed in v2.0.0
```

**Solution:** Migrate to wire protocol or JSON v2.0:
```rust
// Instead of deprecated JSON
#[allow(deprecated)]
let json = container.to_json()?;

// Use wire protocol
let wire = container.serialize_cpp_wire()?;

// Or JSON v2.0 adapter
let json = JsonV2Adapter::serialize(&container)?;
```

---

### Error: "Unknown value type code"

**Problem:** Deserializing JSON with invalid type codes.

**Solution:**
1. Ensure the type code is valid (0-15)
2. Check the JSON format matches expected structure:
   ```json
   {
     "values": [
       {
         "name": "count",
         "type": "4",
         "value": { "type": "int", "value": 42 }
       }
     ]
   }
   ```

---

### Error: "Failed to decode base64"

**Problem:** BytesValue deserialization fails.

**Solution:** Ensure bytes are properly base64 encoded:
```rust
use base64::{Engine as _, engine::general_purpose};

// Encoding
let encoded = general_purpose::STANDARD.encode(&bytes);

// The JSON should contain valid base64
// "AAEC/v8=" not "[0, 1, 2, 254, 255]"
```

---

### Wire Protocol Parse Errors

**Problem:** Can't parse wire protocol data from C++.

**Solution:**
1. Verify format:
   ```
   @header={{[3,source];[5,type];[6,version];}};@data={{[name,type,value];...}};
   ```

2. Check for special characters that need escaping
3. Ensure data isn't truncated

---

## Performance Problems

### Slow Value Access

**Problem:** Reading values is slower than expected.

**Solution:** Use zero-copy methods:
```rust
// Slow (clones)
let values = container.values();
for v in values { ... }

// Fast (no clone)
container.with_values(|values| {
    for v in values { ... }
});
```

---

### High Memory Usage

**Problem:** Container uses too much memory.

**Solutions:**
1. Set value limits:
   ```rust
   let container = ValueContainer::with_max_values(1000);
   ```

2. Clear values when done:
   ```rust
   container.clear_values();
   ```

3. Use smaller types:
   ```rust
   // Use IntValue instead of LongValue when possible
   // Use ShortValue for small numbers
   ```

---

### Slow Serialization

**Problem:** JSON serialization is slow.

**Solution:**
1. Use XML (3x faster):
   ```rust
   #[allow(deprecated)]
   let xml = container.to_xml()?;
   ```

2. Use wire protocol for maximum speed
3. Reduce number of values
4. Pre-allocate capacity

---

## Cross-Language Issues

### Data Not Readable by C++

**Problem:** C++ can't read data serialized by Rust.

**Solution:**
1. Use `serialize_cpp_wire()`:
   ```rust
   let wire = container.serialize_cpp_wire()?;
   ```

2. Verify type codes match:
   | Type | Rust Code | C++ Code |
   |------|-----------|----------|
   | Int | 4 | 4 |
   | Long | 6 | 6 |
   | String | 12 | 12 |

---

### Nested Container Issues

**Problem:** Nested containers fail with wire protocol.

**Known Issue:** Wire protocol has 2 failing tests for nested structures.

**Workaround:** Use JSON v2.0 for nested data:
```rust
// For nested structures, use JSON v2.0
let json = JsonV2Adapter::serialize(&container)?;
```

---

### Long/ULong Range Differences

**Problem:** Values work in Rust but fail in C++.

**Solution:** Use range-checked types:
```rust
// LongValue enforces C++ compatible range
let long = LongValue::new("val", value)?;

// If you need full i64 range (Rust-only)
let llong = LLongValue::new("val", value);
```

---

## Thread Safety Issues

### Deadlock

**Problem:** Program hangs when accessing container.

**Possible Causes:**
1. Holding lock while calling method that needs lock
2. Circular lock dependencies

**Solution:**
```rust
// Don't do nested access like this
let value = container.get_value("a"); // Holds read lock
container.add_value(...)?; // Needs write lock - deadlock!

// Do this instead
let value = container.get_value("a");
drop(value); // Release lock
container.add_value(...)?;

// Or use with_values for read-only access
container.with_values(|values| {
    // Process values here
});
```

---

### Race Condition

**Problem:** Unexpected behavior with concurrent access.

**Solution:** Use `Arc<Mutex>` for shared mutable access:
```rust
use std::sync::{Arc, Mutex};

let container = Arc::new(Mutex::new(ValueContainer::new()));

// Thread 1
{
    let mut c = container.lock().unwrap();
    c.add_value(Arc::new(IntValue::new("key", 1)))?;
}

// Thread 2
{
    let c = container.lock().unwrap();
    let val = c.get_value("key");
}
```

---

## Memory Issues

### Memory Leak

**Problem:** Memory usage grows over time.

**Solutions:**
1. Clear containers when done:
   ```rust
   container.clear_values();
   ```

2. Drop containers explicitly:
   ```rust
   drop(container);
   ```

3. Check for Arc cycles (rare):
   ```rust
   // Avoid storing Arc<Container> inside ContainerValue
   // that's stored in the same container
   ```

---

### Stack Overflow with Deep Nesting

**Problem:** Very deep nested containers cause stack overflow.

**Solution:** Limit nesting depth or use iterative processing:
```rust
// Check depth before processing
fn check_depth(container: &ValueContainer, max_depth: usize) -> bool {
    // Implement depth checking logic
}
```

---

## Still Having Issues?

1. **Check the tests**: Look at `tests/` for working examples
2. **Run with debug output**:
   ```bash
   RUST_BACKTRACE=1 cargo test
   ```
3. **Check documentation**:
   - [API Reference](../API_REFERENCE.md)
   - [FAQ](FAQ.md)
   - [Best Practices](BEST_PRACTICES.md)
4. **Open an issue**: [GitHub Issues](https://github.com/kcenon/rust_container_system/issues)

---

*Last updated: 2025-11-26*
