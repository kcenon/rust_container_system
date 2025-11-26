# Quick Start Guide

> **Time to complete**: 5 minutes
> **Prerequisites**: Rust 1.90+, Cargo

Get up and running with the Rust Container System in just a few steps.

## Table of Contents

1. [Installation](#1-installation)
2. [Create Your First Container](#2-create-your-first-container)
3. [Add Values](#3-add-values)
4. [Retrieve Values](#4-retrieve-values)
5. [Serialize Data](#5-serialize-data)
6. [Next Steps](#6-next-steps)

---

## 1. Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust_container_system = "0.1"
```

Or use cargo:

```bash
cargo add rust_container_system
```

---

## 2. Create Your First Container

```rust
use rust_container_system::prelude::*;

fn main() {
    // Method 1: Simple constructor
    let container = ValueContainer::new();

    // Method 2: Builder pattern (recommended)
    let container = ValueContainer::builder()
        .source("my_app", "session_1")
        .target("server", "handler")
        .message_type("user_request")
        .build();

    println!("Container created with type: {}", container.message_type());
}
```

**Output:**
```
Container created with type: user_request
```

---

## 3. Add Values

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut container = ValueContainer::new();

    // Add different value types
    container.add_value(Arc::new(IntValue::new("user_id", 12345)))?;
    container.add_value(Arc::new(StringValue::new("username", "alice")))?;
    container.add_value(Arc::new(BoolValue::new("active", true)))?;
    container.add_value(Arc::new(DoubleValue::new("balance", 150.75)))?;

    println!("Container has {} values", container.value_count());
    Ok(())
}
```

**Output:**
```
Container has 4 values
```

---

## 4. Retrieve Values

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut container = ValueContainer::new();
    container.add_value(Arc::new(IntValue::new("user_id", 12345)))?;
    container.add_value(Arc::new(StringValue::new("username", "alice")))?;

    // Get a single value
    if let Some(user_id) = container.get_value("user_id") {
        println!("User ID: {}", user_id.to_int()?);
    }

    // Iterate over all values
    for value in &container {
        println!("{}: {} (type: {})",
            value.name(),
            value.to_string(),
            value.value_type()
        );
    }

    Ok(())
}
```

**Output:**
```
User ID: 12345
user_id: 12345 (type: Int)
username: alice (type: String)
```

---

## 5. Serialize Data

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and populate container
    let mut container = ValueContainer::builder()
        .source("client", "session_1")
        .message_type("user_data")
        .build();

    container.add_value(Arc::new(IntValue::new("id", 1)))?;
    container.add_value(Arc::new(StringValue::new("name", "Alice")))?;

    // Serialize to wire protocol (cross-language compatible)
    let wire_data = container.serialize_cpp_wire()?;
    println!("Wire format:\n{}\n", wire_data);

    // Deserialize
    let restored = ValueContainer::deserialize_cpp_wire(&wire_data)?;
    println!("Restored {} values", restored.value_count());

    // Verify
    if let Some(name) = restored.get_value("name") {
        println!("Name: {}", name.to_string());
    }

    Ok(())
}
```

**Output:**
```
Wire format:
@header={{[3,client];[4,session_1];[5,user_data];[6,1.0.0.0];}};@data={{[id,int_value,1];[name,string_value,Alice];}};

Restored 2 values
Name: Alice
```

---

## 6. Next Steps

### Run the Examples

```bash
# Basic operations
cargo run --example basic_container

# Serialization
cargo run --example serialization

# Nested containers
cargo run --example nested_containers

# Thread safety
cargo run --example concurrency
```

### Learn More

| Topic | Documentation |
|-------|---------------|
| All features | [Features Guide](../FEATURES.md) |
| Full API | [API Reference](../API_REFERENCE.md) |
| Best practices | [Best Practices](BEST_PRACTICES.md) |
| Common issues | [Troubleshooting](TROUBLESHOOTING.md) |
| Questions | [FAQ](FAQ.md) |

### Complete Example

Here's a complete example combining everything:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create container with builder
    let mut request = ValueContainer::builder()
        .source("web_client", "browser_session")
        .target("api_server", "user_handler")
        .message_type("create_user")
        .build();

    // 2. Add user data
    request.add_value(Arc::new(StringValue::new("username", "alice")))?;
    request.add_value(Arc::new(StringValue::new("email", "alice@example.com")))?;
    request.add_value(Arc::new(IntValue::new("age", 28)))?;
    request.add_value(Arc::new(BoolValue::new("newsletter", true)))?;

    // 3. Serialize for transmission
    let wire_data = request.serialize_cpp_wire()?;

    // 4. [Simulated] Server receives and deserializes
    let received = ValueContainer::deserialize_cpp_wire(&wire_data)?;

    // 5. Process on server
    println!("Received request: {}", received.message_type());
    println!("From: {}/{}", received.source_id(), received.source_sub_id());
    println!("To: {}/{}", received.target_id(), received.target_sub_id());
    println!("\nUser data:");
    for value in &received {
        println!("  {}: {}", value.name(), value.to_string());
    }

    // 6. Create response (swap source/target)
    let mut response = received.copy(false); // Header only
    response.swap_header();
    response.set_message_type("create_user_response");
    response.add_value(Arc::new(BoolValue::new("success", true)))?;
    response.add_value(Arc::new(IntValue::new("user_id", 12345)))?;

    println!("\nResponse:");
    println!("  success: {}", response.get_value("success").unwrap().to_bool()?);
    println!("  user_id: {}", response.get_value("user_id").unwrap().to_int()?);

    Ok(())
}
```

**Output:**
```
Received request: create_user
From: web_client/browser_session
To: api_server/user_handler

User data:
  username: alice
  email: alice@example.com
  age: 28
  newsletter: true

Response:
  success: true
  user_id: 12345
```

---

## Summary

| Step | Code |
|------|------|
| Import | `use rust_container_system::prelude::*;` |
| Create | `ValueContainer::builder()...build()` |
| Add | `container.add_value(Arc::new(IntValue::new("key", 1)))?` |
| Get | `container.get_value("key")` |
| Serialize | `container.serialize_cpp_wire()?` |
| Deserialize | `ValueContainer::deserialize_cpp_wire(&data)?` |

---

*Ready for more? Check out the [Features Guide](../FEATURES.md) for complete documentation.*
