//! Basic container usage example
//!
//! Demonstrates creating containers, adding values, querying values,
//! swapping headers, and copying containers.
//!
//! Run with: cargo run --example basic_container

use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    println!("=== Rust Container System - Basic Example ===\n");

    // Create container and set header information
    let mut container = ValueContainer::new();
    container.set_source("client_01", "session_123");
    container.set_target("server", "main_handler");
    container.set_message_type("user_data");

    println!("Container created:");
    println!("  Source: {} / {}", container.source_id(), container.source_sub_id());
    println!("  Target: {} / {}", container.target_id(), container.target_sub_id());
    println!("  Message Type: {}\n", container.message_type());

    // Add various types of values
    println!("Adding values...");
    container.add_value(Arc::new(IntValue::new("user_id", 12345))).expect("Failed to add user_id");
    container.add_value(Arc::new(StringValue::new("username", "john_doe"))).expect("Failed to add username");
    container.add_value(Arc::new(DoubleValue::new("balance", 1500.75))).expect("Failed to add balance");
    container.add_value(Arc::new(BoolValue::new("active", true))).expect("Failed to add active");
    container.add_value(Arc::new(LongValue::new("timestamp", 1234567890))).expect("Failed to add timestamp");
    println!("  Added {} values\n", container.value_count());

    // Retrieve and display values
    println!("Retrieving values:");
    if let Some(user_id) = container.get_value("user_id") {
        println!("  User ID: {}", user_id.to_int().unwrap());
    }
    if let Some(username) = container.get_value("username") {
        println!("  Username: {}", username.to_string());
    }
    if let Some(balance) = container.get_value("balance") {
        println!("  Balance: ${:.2}", balance.to_double().unwrap());
    }
    if let Some(active) = container.get_value("active") {
        println!("  Active: {}", active.to_bool().unwrap());
    }
    if let Some(timestamp) = container.get_value("timestamp") {
        println!("  Timestamp: {}", timestamp.to_long().unwrap());
    }

    // Swap header (source <-> target)
    println!("\nSwapping header...");
    container.swap_header();
    println!("  Source: {} / {}", container.source_id(), container.source_sub_id());
    println!("  Target: {} / {}", container.target_id(), container.target_sub_id());

    // Copy container
    println!("\nCreating a copy...");
    let copy = container.copy(true);
    println!("  Copy has {} values", copy.value_count());

    let header_only = container.copy(false);
    println!("  Header-only copy has {} values", header_only.value_count());

    println!("\n=== Example Complete ===");
}
