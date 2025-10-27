//! Serialization example
//!
//! Demonstrates serializing containers to JSON, XML, and binary formats.
//!
//! Run with: cargo run --example serialization

use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    println!("=== Rust Container System - Serialization Example ===\n");

    // Create container with sample market data
    let mut container = ValueContainer::new();
    container.set_source("trading_engine", "session_001");
    container.set_target("risk_monitor", "main");
    container.set_message_type("market_data");

    // Add stock market data
    container.add_value(Arc::new(StringValue::new("symbol", "AAPL"))).expect("Failed to add symbol");
    container.add_value(Arc::new(DoubleValue::new("price", 175.50))).expect("Failed to add price");
    container.add_value(Arc::new(LongValue::new("volume", 1000000).expect("Value out of range"))).expect("Failed to add volume");
    container.add_value(Arc::new(BoolValue::new("is_active", true))).expect("Failed to add is_active");
    container.add_value(Arc::new(IntValue::new("timestamp", 1234567890))).expect("Failed to add timestamp");

    println!("Container created with {} values\n", container.value_count());

    // JSON serialization
    println!("=== JSON Serialization ===");
    match container.to_json() {
        Ok(json) => {
            println!("{}\n", json);
        }
        Err(e) => {
            eprintln!("JSON serialization error: {}", e);
        }
    }

    // XML serialization
    println!("=== XML Serialization ===");
    match container.to_xml() {
        Ok(xml) => {
            println!("{}\n", xml);
        }
        Err(e) => {
            eprintln!("XML serialization error: {}", e);
        }
    }

    // Binary serialization
    println!("=== Binary Serialization ===");
    match container.serialize() {
        Ok(binary) => {
            println!("Binary size: {} bytes", binary.len());
            println!("First 100 bytes: {:?}\n", &binary[..100.min(binary.len())]);
        }
        Err(e) => {
            eprintln!("Binary serialization error: {}", e);
        }
    }

    // BytesValue example with base64 encoding
    println!("=== Bytes Value Example ===");
    let mut bytes_container = ValueContainer::new();
    bytes_container.set_message_type("binary_data");

    // Sample binary data: "Hello" in ASCII
    let sample_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
    bytes_container.add_value(Arc::new(BytesValue::new("data", sample_data))).expect("Failed to add data");

    // Bytes are encoded as base64 in JSON
    match bytes_container.to_json() {
        Ok(json) => {
            println!("Bytes value in JSON:");
            println!("{}\n", json);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    println!("=== Example Complete ===");
}
