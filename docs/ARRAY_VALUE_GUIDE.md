# ArrayValue Implementation Guide (Rust)

## Overview

`ArrayValue` is a Rust implementation of type-15 array values that provides heterogeneous collections with full cross-language compatibility. It leverages Rust's type safety, ownership system, and trait-based polymorphism.

## Architecture

### Type System

```
trait Value (Arc<dyn Value>)
├── BoolValue (type 1)
├── Numeric types (2-11)
├── BytesValue (type 12)
├── StringValue (type 13)
└── ArrayValue (type 15) ← Dynamic heterogeneous collection
```

### Module Structure

```
rust_container_system/
├── src/
│   ├── core/
│   │   ├── value.rs         # Value trait definition
│   │   ├── value_types.rs   # ValueType enum with Array
│   │   ├── container.rs     # ValueContainer with ArrayValue support
│   │   └── wire_protocol.rs # Serialization with Array support
│   └── values/
│       ├── mod.rs           # Re-exports including ArrayValue
│       ├── array_value.rs   # ArrayValue implementation
│       ├── int_value.rs
│       ├── string_value.rs
│       └── ...
```

### Struct Diagram

```
┌─────────────────────────────────────────┐
│     trait Value                         │
├─────────────────────────────────────────┤
│ fn name(&self) -> &str                  │
│ fn value_type(&self) -> ValueType       │
│ fn size(&self) -> usize                 │
│ fn to_bytes(&self) -> Vec<u8>          │
│ fn to_json(&self) -> Result<String>    │
│ fn to_xml(&self) -> Result<String>     │
│ fn clone_value(&self) -> Arc<dyn Value>│
│ fn as_any(&self) -> &dyn Any           │
└─────────────────────────────────────────┘
                ▲
                │ implements
                │
┌─────────────────────────────────────────┐
│   struct ArrayValue                     │
├─────────────────────────────────────────┤
│ name: String                            │
│ elements: Vec<Arc<dyn Value>>          │
├─────────────────────────────────────────┤
│ new(name, elements) -> Self            │
│ with_capacity(name, cap) -> Self       │
│ count(&self) -> usize                  │
│ is_empty(&self) -> bool                │
│ elements(&self) -> &[Arc<dyn Value>]   │
│ push(&mut self, element)               │
│ push_back(&mut self, element)          │
│ at(&self, index) -> Option<Arc<...>>   │
│ clear(&mut self)                       │
└─────────────────────────────────────────┘
```

## Usage Examples

### Basic Creation

```rust
use rust_container_system::prelude::*;
use rust_container_system::values::{ArrayValue, IntValue, StringValue};
use std::sync::Arc;

// Create empty array
let mut numbers = ArrayValue::new("numbers", vec![]);

// Add elements
numbers.push(Arc::new(IntValue::new("", 10)));
numbers.push(Arc::new(IntValue::new("", 20)));
numbers.push(Arc::new(IntValue::new("", 30)));

println!("Array has {} elements", numbers.count());
```

### Pre-allocated Array

```rust
// Create with reserved capacity
let mut large_array = ArrayValue::with_capacity("data", 1000);

for i in 0..1000 {
    large_array.push(Arc::new(IntValue::new("", i)));
}
```

### Constructor with Initial Elements

```rust
// Create array with initial values
let elem1 = Arc::new(IntValue::new("", 100));
let elem2 = Arc::new(IntValue::new("", 200));
let elem3 = Arc::new(IntValue::new("", 300));

let array = ArrayValue::new("scores", vec![elem1, elem2, elem3]);
assert_eq!(array.count(), 3);
```

### Heterogeneous Collections

```rust
use rust_container_system::values::{IntValue, StringValue, DoubleValue, BoolValue};

// Mix different value types
let mut mixed = ArrayValue::new("mixed", vec![]);

mixed.push(Arc::new(IntValue::new("", 42)));
mixed.push(Arc::new(StringValue::new("", "hello")));
mixed.push(Arc::new(DoubleValue::new("", 3.14)));
mixed.push(Arc::new(BoolValue::new("", true)));

// Access elements
if let Some(first) = mixed.at(0) {
    println!("First element type: {:?}", first.value_type());
}
```

### Integration with ValueContainer

```rust
use rust_container_system::core::ValueContainer;

// Create container
let mut container = ValueContainer::new();
container.set_message_type("user_data");

// Create array
let mut colors = ArrayValue::new("colors", vec![]);
colors.push(Arc::new(StringValue::new("", "red")));
colors.push(Arc::new(StringValue::new("", "green")));
colors.push(Arc::new(StringValue::new("", "blue")));

// Add to container
container.add_value(Arc::new(colors))?;

// Serialize to C++ wire protocol
let wire_data = serialize_cpp_wire(&container)?;
println!("Wire format: {}", wire_data);
// Output: @header={{...}};@data={{[colors,array_value,3];}};
```

## Iteration and Access

### Safe Element Access

```rust
let array = ArrayValue::new("data", vec![
    Arc::new(IntValue::new("", 10)),
    Arc::new(IntValue::new("", 20)),
    Arc::new(IntValue::new("", 30)),
]);

// Option-based access (safe)
match array.at(0) {
    Some(element) => println!("First: {}", element.to_string()),
    None => println!("Index out of bounds"),
}

// Out of bounds returns None (no panic)
assert!(array.at(100).is_none());
```

### Iterating Elements

```rust
let array = ArrayValue::new("items", vec![/* ... */]);

// Iterate through references
for (i, element) in array.elements().iter().enumerate() {
    println!("Element {}: {}", i, element.to_string());
}

// Count elements
let count = array.count();
println!("Total elements: {}", count);

// Check if empty
if array.is_empty() {
    println!("Array is empty");
}
```

### Mutable Operations

```rust
let mut array = ArrayValue::new("mutable", vec![]);

// Add elements
array.push(Arc::new(IntValue::new("", 1)));
array.push(Arc::new(IntValue::new("", 2)));

// Clear all elements
array.clear();
assert!(array.is_empty());
```

## Serialization

### Binary Format (Value trait)

```rust
use rust_container_system::core::Value;

let mut array = ArrayValue::new("data", vec![
    Arc::new(IntValue::new("", 42)),
    Arc::new(StringValue::new("", "test")),
]);

// Get total size
let size = array.size();
println!("Binary size: {} bytes", size);

// Serialize to bytes
let bytes = array.to_bytes();
println!("Serialized {} bytes", bytes.len());
```

### JSON Format

```rust
let array = ArrayValue::new("colors", vec![
    Arc::new(StringValue::new("", "red")),
    Arc::new(StringValue::new("", "blue")),
]);

let json = array.to_json()?;
println!("JSON: {}", json);
// Output: {"name":"colors","type":"array","elements":[...]}
```

### XML Format

```rust
let array = ArrayValue::new("scores", vec![
    Arc::new(IntValue::new("", 95)),
    Arc::new(IntValue::new("", 87)),
]);

let xml = array.to_xml()?;
println!("XML: {}", xml);
// Output: <array name="scores" count="2">...</array>
```

### Wire Protocol Format

```rust
use rust_container_system::core::wire_protocol;

let mut container = ValueContainer::new();
let array = ArrayValue::new("items", vec![
    Arc::new(IntValue::new("", 1)),
    Arc::new(IntValue::new("", 2)),
]);
container.add_value(Arc::new(array))?;

// Serialize to C++ compatible format
let wire_format = wire_protocol::serialize_cpp_wire(&container)?;

// Format: @header={{...}};@data={{[items,array_value,2];}};
```

## Type Safety and Downcasting

### Using as_any() for Downcasting

```rust
use std::any::Any;

let value: Arc<dyn Value> = Arc::new(ArrayValue::new("test", vec![]));

// Downcast to concrete type
if let Some(array) = value.as_any().downcast_ref::<ArrayValue>() {
    println!("It's an ArrayValue with {} elements", array.count());
}
```

### Type Checking

```rust
use rust_container_system::core::ValueType;

let element: Arc<dyn Value> = array.at(0).unwrap();

match element.value_type() {
    ValueType::Int => {
        if let Some(int_val) = element.as_any().downcast_ref::<IntValue>() {
            println!("Integer value: {}", int_val.value());
        }
    }
    ValueType::String => {
        if let Some(str_val) = element.as_any().downcast_ref::<StringValue>() {
            println!("String value: {}", str_val.value());
        }
    }
    ValueType::Array => {
        println!("Nested array detected");
    }
    _ => println!("Other type: {:?}", element.value_type()),
}
```

## Error Handling

```rust
use rust_container_system::core::{Result, ContainerError};

fn process_array(array: &ArrayValue) -> Result<()> {
    if array.is_empty() {
        return Err(ContainerError::InvalidDataFormat(
            "Array cannot be empty".to_string()
        ));
    }

    // Process elements
    for element in array.elements() {
        let json = element.to_json()?; // Propagate errors
        println!("Element JSON: {}", json);
    }

    Ok(())
}
```

## Cross-Language Interoperability

### Receiving from C++

```rust
use rust_container_system::core::wire_protocol;

// Receive wire format from C++
let cpp_wire_data = "@header={{[5,test];}};@data={{[nums,array_value,3];}};";

// Deserialize (note: full nested support TODO)
let container = wire_protocol::deserialize_cpp_wire(cpp_wire_data)?;

// Currently creates empty ArrayValue placeholder
if let Some(array_val) = container.values()
    .iter()
    .find(|v| v.name() == "nums")
    .and_then(|v| v.as_any().downcast_ref::<ArrayValue>())
{
    println!("Received array: {}", array_val.name());
}
```

### Sending to Python/Go

```rust
let mut container = ValueContainer::new();
let array = ArrayValue::new("data", vec![
    Arc::new(IntValue::new("", 100)),
    Arc::new(StringValue::new("", "test")),
]);
container.add_value(Arc::new(array))?;

// Serialize for cross-language transmission
let wire_data = wire_protocol::serialize_cpp_wire(&container)?;

// Send wire_data bytes over network/IPC
// Python/Go can deserialize: container = ValueContainer.from_string(wire_data)
```

## Clone and Copy Semantics

```rust
// ArrayValue implements Clone
let original = ArrayValue::new("original", vec![
    Arc::new(IntValue::new("", 42)),
]);

// Clone creates new ArrayValue with cloned Arc references
let cloned = original.clone();
assert_eq!(original.count(), cloned.count());

// Clone via Value trait
let value_clone: Arc<dyn Value> = original.clone_value();
```

## Best Practices

### 1. Ownership and Lifetimes

```rust
// Good: Use Arc for shared ownership
let element = Arc::new(IntValue::new("", 42));
array.push(element.clone()); // Share ownership

// Avoid: Don't create temporary values inline without Arc
// array.push(IntValue::new("", 42)); // Won't compile - needs Arc
```

### 2. Capacity Pre-allocation

```rust
// Good: Reserve capacity for known sizes
let mut large_array = ArrayValue::with_capacity("big_data", 10000);
for i in 0..10000 {
    large_array.push(Arc::new(IntValue::new("", i)));
}

// Avoid: Growing from empty for large arrays (multiple allocations)
```

### 3. Error Propagation

```rust
fn build_array() -> Result<ArrayValue> {
    let mut array = ArrayValue::new("result", vec![]);

    // Use ? operator for error propagation
    let json_data = fetch_data()?;
    array.push(Arc::new(StringValue::new("", json_data)));

    Ok(array)
}
```

### 4. Immutability When Possible

```rust
// Create immutable arrays when data doesn't change
let fixed_array = ArrayValue::new("constants", vec![
    Arc::new(IntValue::new("", 100)),
    Arc::new(IntValue::new("", 200)),
]);

// Pass as &ArrayValue instead of &mut
fn process_array(array: &ArrayValue) {
    // Read-only operations
    println!("Processing {} elements", array.count());
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let array = ArrayValue::new("test", vec![]);
        assert_eq!(array.count(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_heterogeneous_array() {
        let mut array = ArrayValue::new("mixed", vec![]);
        array.push(Arc::new(IntValue::new("", 42)));
        array.push(Arc::new(StringValue::new("", "hello")));

        assert_eq!(array.count(), 2);

        let first = array.at(0).unwrap();
        assert_eq!(first.value_type(), ValueType::Int);

        let second = array.at(1).unwrap();
        assert_eq!(second.value_type(), ValueType::String);
    }

    #[test]
    fn test_out_of_bounds() {
        let array = ArrayValue::new("test", vec![]);
        assert!(array.at(0).is_none());
    }
}
```

## Performance Considerations

- **Arc overhead**: Each element wrapped in `Arc<dyn Value>` has allocation and reference counting cost
- **Dynamic dispatch**: Trait object calls have small runtime cost
- **Clone behavior**: Cloning `ArrayValue` clones `Vec` and increments Arc counters (not deep copy of element data)
- **Capacity**: Use `with_capacity()` for known sizes to avoid reallocations

## Common Patterns

### Building from Iterator

```rust
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
let array = ArrayValue::new(
    "numbers",
    numbers.into_iter()
        .map(|n| Arc::new(IntValue::new("", n)) as Arc<dyn Value>)
        .collect()
);
```

### Filtering Elements

```rust
let filtered: Vec<Arc<dyn Value>> = array.elements()
    .iter()
    .filter(|elem| elem.value_type() == ValueType::Int)
    .cloned()
    .collect();

let filtered_array = ArrayValue::new("filtered", filtered);
```

### Transforming Elements

```rust
for element in array.elements() {
    if let Some(int_val) = element.as_any().downcast_ref::<IntValue>() {
        println!("Integer: {}", int_val.value() * 2);
    }
}
```

## Migration from Legacy Code

If using raw `Vec<Arc<dyn Value>>`:

### Before
```rust
let mut values: Vec<Arc<dyn Value>> = Vec::new();
values.push(Arc::new(IntValue::new("", 10)));
```

### After
```rust
let mut array = ArrayValue::new("values", vec![]);
array.push(Arc::new(IntValue::new("", 10)));
```

**Benefits:**
- Type-safe value type (ValueType::Array)
- Serialization support (to_bytes, to_json, to_xml)
- Cross-language compatibility
- Clear ownership semantics

## See Also

- [Rust Value Trait Documentation](../src/core/value.rs)
- [ValueType Enum](../src/core/value_types.rs)
- [Cross-Language Wire Protocol](WIRE_PROTOCOL.md)
- [Container Architecture](ARCHITECTURE.md)
