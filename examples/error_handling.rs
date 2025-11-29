// BSD 3-Clause License
//
// Copyright (c) 2021-2025, 🍀☀🌕🌥 🌊
// See LICENSE file for full license text.

//! Error Handling Example
//!
//! This example demonstrates proper error handling patterns when working with
//! the container system, including type conversion errors, capacity limits,
//! and validation.

use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    println!("=== Error Handling Examples ===\n");

    // Example 1: Type conversion errors
    type_conversion_errors();

    // Example 2: Container capacity limits
    capacity_limit_errors();

    // Example 3: Value not found errors
    value_not_found_errors();

    // Example 4: Using Result combinators
    result_combinators();

    println!("\n=== All examples completed! ===");
}

/// Demonstrates handling type conversion errors
fn type_conversion_errors() {
    println!("--- Type Conversion Errors ---");

    // Create a string value
    let text = StringValue::new("message", "Hello");

    // Attempting to convert string to int will fail
    match text.to_int() {
        Ok(n) => println!("Converted to int: {}", n),
        Err(e) => println!("Expected error: {}", e),
    }

    // Create a large number that won't fit in i32
    let big_number = LLongValue::new("big", i64::MAX);

    match big_number.to_int() {
        Ok(n) => println!("Converted to int: {}", n),
        Err(e) => println!("Expected error (overflow): {}", e),
    }

    // Float to int conversion with out-of-range value
    let huge_float = DoubleValue::new("huge", 1e100);

    match huge_float.to_int() {
        Ok(n) => println!("Converted to int: {}", n),
        Err(e) => println!("Expected error (out of range): {}", e),
    }

    // NaN cannot be converted to integer
    let nan_value = DoubleValue::new("nan", f64::NAN);

    match nan_value.to_int() {
        Ok(n) => println!("Converted NaN to int: {}", n),
        Err(e) => println!("Expected error (NaN): {}", e),
    }

    println!();
}

/// Demonstrates container capacity limit handling
fn capacity_limit_errors() {
    println!("--- Container Capacity Limits ---");

    // Create a container with limited capacity
    let mut container = ValueContainer::with_max_values(3);

    // Add values up to the limit
    for i in 0..5 {
        let value = Arc::new(IntValue::new(format!("val_{}", i), i));
        match container.add_value(value) {
            Ok(()) => println!("Added value {}", i),
            Err(e) => println!("Failed to add value {}: {}", i, e),
        }
    }

    println!("Container has {} values (max: 3)", container.value_count());
    println!();
}

/// Demonstrates handling missing values
fn value_not_found_errors() {
    println!("--- Value Not Found Handling ---");

    let mut container = ValueContainer::new();
    container
        .add_value(Arc::new(IntValue::new("existing", 42)))
        .unwrap();

    // Try to get a value that exists
    match container.get_value("existing") {
        Some(v) => println!("Found 'existing': {}", v.to_string()),
        None => println!("'existing' not found"),
    }

    // Try to get a value that doesn't exist
    match container.get_value("missing") {
        Some(v) => println!("Found 'missing': {}", v.to_string()),
        None => println!("'missing' not found (expected)"),
    }

    // Pattern: Use Option methods for cleaner code
    let value = container
        .get_value("missing")
        .map(|v| v.to_int().unwrap_or(0))
        .unwrap_or(-1);
    println!("Value with default: {}", value);

    println!();
}

/// Demonstrates using Result combinators for cleaner error handling
fn result_combinators() {
    println!("--- Result Combinators ---");

    let mut container = ValueContainer::new();
    container
        .add_value(Arc::new(IntValue::new("count", 42)))
        .unwrap();
    container
        .add_value(Arc::new(StringValue::new("name", "Alice")))
        .unwrap();
    container
        .add_value(Arc::new(DoubleValue::new("score", 95.5)))
        .unwrap();

    // Using map_or for default values
    let count = container
        .get_value("count")
        .map(|v| v.to_int().unwrap_or(0))
        .unwrap_or(0);
    println!("Count (with defaults): {}", count);

    // Using and_then for chained operations
    let result: Option<i32> = container.get_value("score").and_then(|v| v.to_int().ok());
    println!("Score as int: {:?}", result);

    // Processing multiple values with error collection
    let keys = ["count", "missing", "score"];
    let results: Vec<_> = keys
        .iter()
        .map(|&key| {
            container
                .get_value(key)
                .ok_or_else(|| format!("'{}' not found", key))
                .and_then(|v| {
                    v.to_int()
                        .map_err(|e| format!("'{}' conversion failed: {}", key, e))
                })
        })
        .collect();

    for (key, result) in keys.iter().zip(results.iter()) {
        match result {
            Ok(n) => println!("  {}: {}", key, n),
            Err(e) => println!("  {}: Error - {}", key, e),
        }
    }

    // Serialize with error handling
    match container.serialize_cpp_wire() {
        Ok(wire) => println!("\nSerialized ({} bytes)", wire.len()),
        Err(e) => println!("\nSerialization failed: {}", e),
    }

    println!();
}
