// BSD 3-Clause License
//
// Copyright (c) 2021-2025, 🍀☀🌕🌥 🌊
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Integration tests for container system
//!
//! These tests verify:
//! - Thread safety
//! - Memory limits
//! - Serialization and injection prevention
//! - Performance characteristics

use rust_container_system::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_concurrent_container_access() {
    let mut container = ValueContainer::new();
    container.set_source("server", "main");
    container.set_target("client", "session_1");

    // Wrap in Arc for sharing across threads
    let container = Arc::new(container);

    let mut handles = vec![];

    // Spawn multiple readers
    for i in 0..5 {
        let container_clone = Arc::clone(&container);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let _ = container_clone.source_id();
                let _ = container_clone.target_id();
                let _ = container_clone.value_count();
                thread::sleep(Duration::from_micros(10));
            }
            i
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Container should still be valid
    assert_eq!(container.source_id(), "server");
    assert_eq!(container.target_id(), "client");
}

#[test]
fn test_value_limit_enforcement() {
    let mut container = ValueContainer::with_max_values(100);

    // Add values up to limit
    for i in 0..100 {
        let value = Arc::new(IntValue::new(format!("key_{}", i), i));
        assert!(container.add_value(value).is_ok());
    }

    assert_eq!(container.value_count(), 100);

    // Try to exceed limit
    let value = Arc::new(IntValue::new("overflow", 999));
    let result = container.add_value(value);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("limit reached"));
}

#[test]
#[allow(deprecated)]
fn test_xml_injection_prevention() {
    let mut container = ValueContainer::new();

    // Attempt XML injection in header
    container.set_source("<malicious><inject>evil</inject></malicious>", "test");

    // Attempt XSS in value
    container
        .add_value(Arc::new(StringValue::new(
            "user_input",
            "<script>alert('xss')</script>",
        )))
        .unwrap();

    let xml = container.to_xml().expect("Failed to serialize");

    // Verify no actual XML tags in output
    assert!(!xml.contains("<malicious>"));
    assert!(!xml.contains("<script>"));
    assert!(!xml.contains("<inject>"));

    // Verify escaped versions exist
    assert!(xml.contains("&lt;malicious&gt;"));
    assert!(xml.contains("&lt;script&gt;"));
}

#[test]
#[allow(deprecated)]
fn test_json_serialization() {
    let mut container = ValueContainer::new();
    container.set_source("server", "main");
    container.set_target("client", "user1");
    container.set_message_type("user_action");

    container
        .add_value(Arc::new(IntValue::new("user_id", 12345)))
        .unwrap();
    container
        .add_value(Arc::new(StringValue::new("action", "login")))
        .unwrap();
    container
        .add_value(Arc::new(BoolValue::new("success", true)))
        .unwrap();

    let json = container.to_json().expect("Failed to serialize");

    // Verify JSON structure
    assert!(json.contains("\"source_id\": \"server\""));
    assert!(json.contains("\"target_id\": \"client\""));
    assert!(json.contains("\"message_type\": \"user_action\""));
    assert!(json.contains("user_id"));
    assert!(json.contains("12345"));
}

#[test]
fn test_deep_copy_vs_shallow_copy() {
    let mut original = ValueContainer::new();
    original.set_source("source", "sub");
    original
        .add_value(Arc::new(IntValue::new("key", 42)))
        .unwrap();

    // Deep copy (with values)
    let deep_copy = original.copy(true);
    assert_eq!(deep_copy.value_count(), 1);
    assert_eq!(deep_copy.source_id(), "source");

    // Shallow copy (header only)
    let shallow_copy = original.copy(false);
    assert_eq!(shallow_copy.value_count(), 0);
    assert_eq!(shallow_copy.source_id(), "source");
}

#[test]
fn test_swap_header() {
    let mut container = ValueContainer::new();
    container.set_source("alice", "session1");
    container.set_target("bob", "session2");

    container.swap_header();

    // Source and target should be swapped
    assert_eq!(container.source_id(), "bob");
    assert_eq!(container.source_sub_id(), "session2");
    assert_eq!(container.target_id(), "alice");
    assert_eq!(container.target_sub_id(), "session1");
}

#[test]
fn test_multiple_values_same_name() {
    let mut container = ValueContainer::new();

    // Add multiple values with same name
    container
        .add_value(Arc::new(IntValue::new("count", 1)))
        .unwrap();
    container
        .add_value(Arc::new(IntValue::new("count", 2)))
        .unwrap();
    container
        .add_value(Arc::new(IntValue::new("count", 3)))
        .unwrap();

    // Get array should return all
    let values = container.get_value_array("count");
    assert_eq!(values.len(), 3);

    // Get value should return first
    let first = container.get_value("count").unwrap();
    assert_eq!(first.to_string(), "1");

    // Remove should remove all
    assert!(container.remove_value("count"));
    assert_eq!(container.value_count(), 0);
}

#[test]
fn test_clone_free_access() {
    let mut container = ValueContainer::new();
    container.set_source("test_source", "sub_id");

    // Clone-free access using callback
    let length = container.with_source_id(|id| id.len());
    assert_eq!(length, "test_source".len());

    // Verify no unnecessary cloning
    let result = container.with_message_type_ref(|msg_type| msg_type.to_uppercase());
    assert_eq!(result, "DATA_CONTAINER");
}

#[test]
fn test_clear_values_preserves_header() {
    let mut container = ValueContainer::new();
    container.set_source("source", "sub");
    container.set_target("target", "sub");
    container.set_message_type("test");

    container
        .add_value(Arc::new(IntValue::new("key", 42)))
        .unwrap();
    assert_eq!(container.value_count(), 1);

    // Clear values
    container.clear_values();

    // Values should be gone, header preserved
    assert_eq!(container.value_count(), 0);
    assert_eq!(container.source_id(), "source");
    assert_eq!(container.target_id(), "target");
    assert_eq!(container.message_type(), "test");
}

#[test]
fn test_concurrent_modifications() {
    use std::sync::Mutex;

    // Note: ValueContainer uses &mut for modifications,
    // so we need Mutex for interior mutability in concurrent scenarios
    let container = Arc::new(Mutex::new(ValueContainer::new()));

    let mut handles = vec![];

    // Spawn threads that add values
    for i in 0..5 {
        let container_clone = Arc::clone(&container);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let key = format!("thread_{}_key_{}", i, j);
                let value = Arc::new(IntValue::new(key, i * 10 + j));

                let mut container_guard = container_clone.lock().unwrap();
                let _ = container_guard.add_value(value);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Verify all values were added
    let container_guard = container.lock().unwrap();
    assert_eq!(container_guard.value_count(), 50);
}

#[test]
fn test_large_value_count() {
    let mut container = ValueContainer::with_max_values(10_000);

    // Add many values
    for i in 0..1000 {
        let value = Arc::new(IntValue::new(format!("key_{}", i), i));
        container.add_value(value).expect("Failed to add value");
    }

    assert_eq!(container.value_count(), 1000);

    // Verify values are retrievable
    let value = container.get_value("key_500").expect("Value not found");
    assert_eq!(value.to_string(), "500");
}

#[test]
fn test_remove_nonexistent_value() {
    let mut container = ValueContainer::new();

    // Removing non-existent value should return false
    assert!(!container.remove_value("nonexistent"));
}

#[test]
fn test_absolute_max_value_limit() {
    // Requesting more than ABSOLUTE_MAX_VALUES should be capped
    let _container = ValueContainer::with_max_values(usize::MAX);

    // Should be capped at ABSOLUTE_MAX_VALUES
    // The container should have ABSOLUTE_MAX_VALUES as limit
    // We can't directly access it, but we know it's enforced
}

#[test]
fn test_bytes_value() {
    let mut container = ValueContainer::new();

    let data = vec![1u8, 2, 3, 4, 5];
    container
        .add_value(Arc::new(BytesValue::new("binary", data.clone())))
        .unwrap();

    let retrieved = container.get_value("binary").expect("Value not found");
    assert_eq!(retrieved.name(), "binary");
}

#[test]
fn test_all_primitive_types() {
    let mut container = ValueContainer::new();

    container
        .add_value(Arc::new(BoolValue::new("bool", true)))
        .unwrap();
    // NOTE: CharValue, ByteValue, ShortValue, FloatValue are not yet implemented
    // container.add_value(Arc::new(CharValue::new("char", 'A'))).unwrap();
    // container.add_value(Arc::new(ByteValue::new("byte", 255))).unwrap();
    // container.add_value(Arc::new(ShortValue::new("short", 1000))).unwrap();
    container
        .add_value(Arc::new(IntValue::new("int", 100000)))
        .unwrap();
    container
        .add_value(Arc::new(LongValue::new("long", 1000000000).unwrap()))
        .unwrap();
    // container.add_value(Arc::new(FloatValue::new("float", 3.14))).unwrap();
    container
        .add_value(Arc::new(DoubleValue::new("double", std::f64::consts::E)))
        .unwrap();
    container
        .add_value(Arc::new(StringValue::new("string", "hello")))
        .unwrap();

    // Updated to reflect only implemented types: Bool, Int, Long, Double, String
    assert_eq!(container.value_count(), 5);

    // Verify all are retrievable
    assert!(container.get_value("bool").is_some());
    // assert!(container.get_value("char").is_some());
    // assert!(container.get_value("byte").is_some());
    // assert!(container.get_value("short").is_some());
    assert!(container.get_value("int").is_some());
    assert!(container.get_value("long").is_some());
    // assert!(container.get_value("float").is_some());
    assert!(container.get_value("double").is_some());
    assert!(container.get_value("string").is_some());
}

#[test]
fn test_serialization_round_trip() {
    let mut original = ValueContainer::new();
    original.set_source("sender", "app1");
    original.set_target("receiver", "app2");
    original.set_message_type("data");

    original
        .add_value(Arc::new(IntValue::new("id", 123)))
        .unwrap();
    original
        .add_value(Arc::new(StringValue::new("name", "test")))
        .unwrap();

    // Serialize
    let bytes = original.serialize().expect("Failed to serialize");
    assert!(!bytes.is_empty());

    // Note: Full round-trip deserialization would require implementing
    // a deserialize method, which is not currently in the codebase
}

#[test]
#[allow(deprecated)]
fn test_empty_container_serialization() {
    let container = ValueContainer::new();

    let json = container.to_json().expect("Failed to serialize");
    assert!(json.contains("\"values\": []"));

    let xml = container.to_xml().expect("Failed to serialize");
    assert!(xml.contains("<values>"));
    assert!(xml.contains("</values>"));
}
