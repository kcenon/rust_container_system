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

//! Property-based tests using proptest

use proptest::prelude::*;
use rust_container_system::prelude::*;
use std::sync::Arc;

// ============================================================================
// Value Type Roundtrip Tests
// ============================================================================

proptest! {
    /// Test that Bool values roundtrip correctly
    #[test]
    fn test_bool_roundtrip(value in any::<bool>()) {
        let bool_val = BoolValue::new("test", value);
        assert_eq!(bool_val.to_bool().unwrap(), value);
        assert_eq!(bool_val.name(), "test");
    }

    /// Test that Int values roundtrip correctly
    #[test]
    fn test_int_roundtrip(value in any::<i32>()) {
        let int_val = IntValue::new("test", value);
        assert_eq!(int_val.to_int().unwrap(), value);
        assert_eq!(int_val.name(), "test");
    }

    /// Test that LLong values (64-bit) roundtrip correctly
    #[test]
    fn test_llong_roundtrip(value in any::<i64>()) {
        let llong_val = LLongValue::new("test", value);
        assert_eq!(llong_val.to_long().unwrap(), value);
        assert_eq!(llong_val.name(), "test");
    }

    /// Test that Long values (32-bit) roundtrip correctly for valid range
    #[test]
    fn test_long_roundtrip(value in (i32::MIN as i64)..(i32::MAX as i64)) {
        let long_val = LongValue::new("test", value).unwrap();
        assert_eq!(long_val.to_long().unwrap(), value);
        assert_eq!(long_val.name(), "test");
    }

    /// Test that Double values roundtrip correctly (excluding NaN)
    #[test]
    fn test_double_roundtrip(value in any::<f64>().prop_filter("not NaN", |v| !v.is_nan())) {
        let double_val = DoubleValue::new("test", value);
        let retrieved = double_val.to_double().unwrap();
        // Use approximate equality for floating point
        assert!((retrieved - value).abs() < 1e-10 || retrieved == value);
        assert_eq!(double_val.name(), "test");
    }

    /// Test that String values roundtrip correctly
    #[test]
    fn test_string_roundtrip(value in ".*") {
        let string_val = StringValue::new("test", value.clone());
        assert_eq!(string_val.to_string(), value);
        assert_eq!(string_val.name(), "test");
    }

    /// Test that Bytes values roundtrip correctly
    #[test]
    fn test_bytes_roundtrip(value in prop::collection::vec(any::<u8>(), 0..1000)) {
        let bytes_val = BytesValue::new("test", value.clone());
        // Use data() to get raw bytes (to_bytes() returns serialized format)
        assert_eq!(bytes_val.data(), &value);
        assert_eq!(bytes_val.name(), "test");
    }
}

// ============================================================================
// ValueContainer Tests
// ============================================================================

proptest! {
    /// Test that containers handle arbitrary numbers of values
    #[test]
    fn test_container_add_multiple_values(count in 0usize..100) {
        let mut container = ValueContainer::new();

        for i in 0..count {
            let value = Arc::new(IntValue::new(format!("val_{}", i), i as i32));
            container.add_value(value).unwrap();
        }

        assert_eq!(container.value_count(), count);
    }

    /// Test that containers enforce capacity limits
    #[test]
    fn test_container_capacity_limit(
        capacity in 1usize..50,
        attempt in 1usize..100
    ) {
        let mut container = ValueContainer::with_max_values(capacity);

        let mut successful = 0;
        for i in 0..attempt {
            let value = Arc::new(IntValue::new(format!("val_{}", i), i as i32));
            if container.add_value(value).is_ok() {
                successful += 1;
            }
        }

        assert_eq!(successful, capacity.min(attempt));
        assert_eq!(container.value_count(), capacity.min(attempt));
    }

    /// Test that value lookup by name works correctly
    #[test]
    fn test_container_value_lookup(names in prop::collection::hash_set("[a-z]{3,10}", 0..20)) {
        let mut container = ValueContainer::new();
        let names_vec: Vec<String> = names.into_iter().collect();

        // Add values with unique names
        for (i, name) in names_vec.iter().enumerate() {
            let value = Arc::new(IntValue::new(name, i as i32));
            container.add_value(value).unwrap();
        }

        // Verify all values can be found
        for (i, name) in names_vec.iter().enumerate() {
            let found = container.get_value(name);
            assert!(found.is_some(), "Failed to find value: {}", name);
            assert_eq!(found.unwrap().to_int().unwrap(), i as i32);
        }
    }
}

// ============================================================================
// Metadata Tests
// ============================================================================

proptest! {
    /// Test that container metadata is preserved
    #[test]
    fn test_container_metadata(
        source_id in "[a-zA-Z0-9_]{1,20}",
        target_id in "[a-zA-Z0-9_]{1,20}",
        msg_type in "[a-zA-Z0-9_]{1,20}"
    ) {
        let mut container = ValueContainer::new();
        container.set_source(&source_id, "sub");
        container.set_target(&target_id, "sub");
        container.set_message_type(&msg_type);

        assert_eq!(container.source_id(), source_id);
        assert_eq!(container.target_id(), target_id);
        assert_eq!(container.message_type(), msg_type);
    }

    /// Test that container source and target sub-IDs work
    #[test]
    fn test_container_sub_ids(
        source_id in "[a-z]{3,10}",
        source_sub_id in "[a-z]{3,10}",
        target_id in "[a-z]{3,10}",
        target_sub_id in "[a-z]{3,10}"
    ) {
        let mut container = ValueContainer::new();
        container.set_source(&source_id, &source_sub_id);
        container.set_target(&target_id, &target_sub_id);

        assert_eq!(container.source_id(), source_id);
        assert_eq!(container.source_sub_id(), source_sub_id);
        assert_eq!(container.target_id(), target_id);
        assert_eq!(container.target_sub_id(), target_sub_id);
    }
}

// ============================================================================
// Serialization Tests
// ============================================================================

#[allow(deprecated)]
proptest! {
    /// Test that JSON serialization doesn't panic on various inputs
    #[test]
    fn test_json_serialization_no_panic(
        int_values in prop::collection::vec(any::<i32>(), 0..20),
        string_values in prop::collection::vec(".*", 0..20)
    ) {
        let mut container = ValueContainer::new();

        for (i, val) in int_values.iter().enumerate() {
            container.add_value(Arc::new(IntValue::new(format!("int_{}", i), *val))).ok();
        }

        for (i, val) in string_values.iter().enumerate() {
            container.add_value(Arc::new(StringValue::new(format!("str_{}", i), val))).ok();
        }

        // Should not panic
        let json_result = container.to_json();
        assert!(json_result.is_ok());
    }

    /// Test that XML serialization doesn't panic on various inputs
    #[test]
    fn test_xml_serialization_no_panic(
        int_values in prop::collection::vec(any::<i32>(), 0..10)
    ) {
        let mut container = ValueContainer::new();

        for (i, val) in int_values.iter().enumerate() {
            container.add_value(Arc::new(IntValue::new(format!("val_{}", i), *val))).ok();
        }

        // Should not panic
        let xml_result = container.to_xml();
        assert!(xml_result.is_ok());
    }
}

// ============================================================================
// Type Conversion Tests
// ============================================================================

proptest! {
    /// Test that value type conversion never panics
    #[test]
    fn test_value_type_conversion_safety(value in any::<i32>()) {
        let int_val = IntValue::new("test", value);

        // All these conversions should be safe (return Option or Result)
        let _ = int_val.to_bool();
        let _ = int_val.to_int();
        let _ = int_val.to_long();
        let _ = int_val.to_double();
        let _ = int_val.to_string();
        let _ = int_val.to_bytes();
    }

    /// Test that value cloning works correctly
    #[test]
    fn test_value_clone(value in any::<i64>()) {
        let original = LLongValue::new("test", value);
        let cloned = original.clone();

        assert_eq!(original.name(), cloned.name());
        assert_eq!(original.to_long().unwrap(), cloned.to_long().unwrap());
        assert_eq!(original.to_string(), cloned.to_string());
    }
}

// ============================================================================
// Thread Safety Tests
// ============================================================================

proptest! {
    /// Test that containers can be safely cloned (Arc-based thread safety)
    #[test]
    fn test_container_clone_safety(count in 0usize..20) {
        let mut container = ValueContainer::new();

        for i in 0..count {
            container.add_value(Arc::new(IntValue::new(format!("val_{}", i), i as i32))).ok();
        }

        // Clone should work
        let cloned = container.clone();
        assert_eq!(container.value_count(), cloned.value_count());
        assert_eq!(container.source_id(), cloned.source_id());
    }
}
