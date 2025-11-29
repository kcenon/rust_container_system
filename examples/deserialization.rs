// BSD 3-Clause License
//
// Copyright (c) 2021-2025, 🍀☀🌕🌥 🌊
// See LICENSE file for full license text.

//! Deserialization Example
//!
//! This example demonstrates how to deserialize data from various formats
//! including C++ wire protocol and JSON v2.0 format.

use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    println!("=== Deserialization Examples ===\n");

    // Example 1: Wire protocol roundtrip
    wire_protocol_roundtrip();

    // Example 2: JSON v2.0 format roundtrip
    json_v2_roundtrip();

    // Example 3: Deserializing from C++ generated data
    cpp_interop_example();

    // Example 4: Format detection
    format_detection_example();

    println!("\n=== All deserialization examples completed! ===");
}

/// Demonstrates wire protocol serialization and deserialization
fn wire_protocol_roundtrip() {
    println!("--- Wire Protocol Roundtrip ---");

    // Create original container
    let mut original = ValueContainer::new();
    original.set_source("rust_client", "session_1");
    original.set_target("cpp_server", "handler");
    original.set_message_type("user_data");

    original
        .add_value(Arc::new(IntValue::new("user_id", 12345)))
        .unwrap();
    original
        .add_value(Arc::new(StringValue::new("username", "alice")))
        .unwrap();
    original
        .add_value(Arc::new(BoolValue::new("verified", true)))
        .unwrap();
    original
        .add_value(Arc::new(DoubleValue::new("balance", 1500.75)))
        .unwrap();

    // Serialize to wire format
    let wire_data = original.serialize_cpp_wire().expect("Serialization failed");
    println!("Serialized wire data ({} bytes):", wire_data.len());
    println!("  {}", &wire_data[..wire_data.len().min(100)]);
    if wire_data.len() > 100 {
        println!("  ...");
    }

    // Deserialize back
    let restored =
        ValueContainer::deserialize_cpp_wire(&wire_data).expect("Deserialization failed");

    // Verify metadata
    println!("\nRestored container:");
    println!(
        "  Source: {}:{}",
        restored.source_id(),
        restored.source_sub_id()
    );
    println!(
        "  Target: {}:{}",
        restored.target_id(),
        restored.target_sub_id()
    );
    println!("  Type: {}", restored.message_type());
    println!("  Values: {}", restored.value_count());

    // Verify values
    println!("\nValues:");
    for name in ["user_id", "username", "verified", "balance"] {
        if let Some(value) = restored.get_value(name) {
            println!(
                "  {} ({:?}): {}",
                name,
                value.value_type(),
                value.to_string()
            );
        }
    }

    println!();
}

/// Demonstrates JSON v2.0 format serialization and deserialization
fn json_v2_roundtrip() {
    println!("--- JSON v2.0 Format Roundtrip ---");

    // Create container
    let mut container = ValueContainer::new();
    container.set_source("app", "worker_1");
    container.set_message_type("sensor_data");

    container
        .add_value(Arc::new(IntValue::new("sensor_id", 42)))
        .unwrap();
    container
        .add_value(Arc::new(DoubleValue::new("temperature", 23.5)))
        .unwrap();
    container
        .add_value(Arc::new(DoubleValue::new("humidity", 65.2)))
        .unwrap();
    container
        .add_value(Arc::new(BytesValue::new(
            "raw_data",
            vec![0xDE, 0xAD, 0xBE, 0xEF],
        )))
        .unwrap();

    // Serialize to JSON v2.0 (pretty-printed)
    let json = JsonV2Adapter::to_v2_json(&container, true).expect("JSON serialization failed");
    println!("JSON v2.0 output:");
    println!("{}", json);

    // Deserialize back
    let restored = JsonV2Adapter::from_v2_json(&json).expect("JSON deserialization failed");

    println!("\nRestored from JSON:");
    println!("  Source: {}", restored.source_id());
    println!("  Message type: {}", restored.message_type());
    println!("  Value count: {}", restored.value_count());

    // Verify bytes value was preserved
    if let Some(raw) = restored.get_value("raw_data") {
        println!("  raw_data type: {:?}", raw.value_type());
    }

    println!();
}

/// Demonstrates deserializing data that might come from C++
fn cpp_interop_example() {
    println!("--- C++ Interoperability ---");

    // Simulated C++ wire protocol data
    // This format matches what C++ container_system produces
    let cpp_data = "@header={{[3,cpp_service];[4,worker_0];[1,rust_handler];[2,main];[5,process_request];[6,1.0.0.0];}};@data={{[request_id,int_value,98765];[operation,string_value,calculate];[priority,int_value,1];[timeout_ms,int_value,5000];}};";

    println!("Parsing C++ generated wire data...");
    println!("  Input: {}...", &cpp_data[..cpp_data.len().min(60)]);

    match ValueContainer::deserialize_cpp_wire(cpp_data) {
        Ok(container) => {
            println!("\nSuccessfully parsed C++ data:");
            println!(
                "  Source: {} ({})",
                container.source_id(),
                container.source_sub_id()
            );
            println!(
                "  Target: {} ({})",
                container.target_id(),
                container.target_sub_id()
            );
            println!("  Message type: {}", container.message_type());

            println!("\n  Values:");
            container.with_values(|values| {
                for value in values {
                    println!(
                        "    {} ({:?}) = {}",
                        value.name(),
                        value.value_type(),
                        value.to_string()
                    );
                }
            });

            // Process specific values
            let request_id = container
                .get_value("request_id")
                .and_then(|v| v.to_int().ok())
                .unwrap_or(0);
            let operation = container
                .get_value("operation")
                .map(|v| v.to_string())
                .unwrap_or_default();

            println!("\n  Processing request #{}: {}", request_id, operation);
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }

    println!();
}

/// Demonstrates format detection and conversion
fn format_detection_example() {
    println!("--- Format Detection ---");

    // Sample data in different formats
    let samples = vec![
        (
            "Wire Protocol",
            "@header={{[3,src];[4,sub];[1,tgt];[2,sub];[5,msg];[6,1.0.0.0];}};@data={{[x,int_value,1];}};",
        ),
        (
            "JSON v2.0",
            r#"{"format":"json_v2","version":"2.0","values":[{"name":"x","type":4,"value":1}]}"#,
        ),
        (
            "C++ JSON",
            r#"{"source_id":"src","target_id":"tgt","message_type":"msg","values":[{"name":"x","type":"int_value","value":1}]}"#,
        ),
    ];

    for (name, data) in samples {
        let format = JsonV2Adapter::detect_format(data);
        println!("  {} -> {:?}", name, format);
    }

    // Convert between formats
    println!("\nFormat conversion:");
    let wire_data = "@header={{[3,client];[4,];[1,server];[2,];[5,test];[6,1.0.0.0];}};@data={{[count,int_value,42];}};";

    if let Ok(container) = ValueContainer::deserialize_cpp_wire(wire_data) {
        // Convert to JSON v2.0
        if let Ok(json) = JsonV2Adapter::to_v2_json(&container, false) {
            println!("  Wire -> JSON v2.0: {}", json);
        }

        // Convert to C++ JSON
        if let Ok(cpp_json) = JsonV2Adapter::to_cpp_json(&container, false) {
            println!("  Wire -> C++ JSON: {}", cpp_json);
        }
    }

    println!();
}
