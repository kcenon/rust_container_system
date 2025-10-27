//! Test suite for long/ulong type range checking policy
//!
//! Verifies that:
//! 1. LongValue enforces 32-bit signed range [-2^31, 2^31-1]
//! 2. ULongValue enforces 32-bit unsigned range [0, 2^32-1]
//! 3. Values exceeding these ranges return errors with helpful messages
//! 4. Serialization always produces 4 bytes (platform-independent)
//! 5. LLongValue/ULLongValue accept full 64-bit ranges

use rust_container_system::prelude::*;

// =============================================================================
// LongValue (type 6) Tests - Signed 32-bit Range
// =============================================================================

#[test]
fn test_long_value_accepts_valid_positive() {
    let lv = LongValue::new("test", 1_000_000).expect("Should accept 1 million");
    assert_eq!(lv.value(), 1_000_000);
}

#[test]
fn test_long_value_accepts_valid_negative() {
    let lv = LongValue::new("test", -1_000_000).expect("Should accept -1 million");
    assert_eq!(lv.value(), -1_000_000);
}

#[test]
fn test_long_value_accepts_zero() {
    let lv = LongValue::new("test", 0).expect("Should accept 0");
    assert_eq!(lv.value(), 0);
}

#[test]
fn test_long_value_accepts_int32_max() {
    let lv = LongValue::new("test", i32::MAX as i64).expect("Should accept i32::MAX");
    assert_eq!(lv.value(), i32::MAX);
}

#[test]
fn test_long_value_accepts_int32_min() {
    let lv = LongValue::new("test", i32::MIN as i64).expect("Should accept i32::MIN");
    assert_eq!(lv.value(), i32::MIN);
}

#[test]
fn test_long_value_rejects_int32_max_plus_one() {
    let result = LongValue::new("test", (i32::MAX as i64) + 1);
    assert!(result.is_err(), "Should reject i32::MAX + 1");
}

#[test]
fn test_long_value_rejects_int32_min_minus_one() {
    let result = LongValue::new("test", (i32::MIN as i64) - 1);
    assert!(result.is_err(), "Should reject i32::MIN - 1");
}

#[test]
fn test_long_value_rejects_large_positive() {
    let result = LongValue::new("test", 5_000_000_000);
    assert!(result.is_err(), "Should reject 5 billion");
}

#[test]
fn test_long_value_rejects_large_negative() {
    let result = LongValue::new("test", -5_000_000_000);
    assert!(result.is_err(), "Should reject -5 billion");
}

// =============================================================================
// ULongValue (type 7) Tests - Unsigned 32-bit Range
// =============================================================================

#[test]
fn test_ulong_value_accepts_valid_value() {
    let ulv = ULongValue::new("test", 1_000_000).expect("Should accept 1 million");
    assert_eq!(ulv.value(), 1_000_000);
}

#[test]
fn test_ulong_value_accepts_zero() {
    let ulv = ULongValue::new("test", 0).expect("Should accept 0");
    assert_eq!(ulv.value(), 0);
}

#[test]
fn test_ulong_value_accepts_uint32_max() {
    let ulv = ULongValue::new("test", u32::MAX as u64).expect("Should accept u32::MAX");
    assert_eq!(ulv.value(), u32::MAX);
}

#[test]
fn test_ulong_value_rejects_uint32_max_plus_one() {
    let result = ULongValue::new("test", (u32::MAX as u64) + 1);
    assert!(result.is_err(), "Should reject u32::MAX + 1");
}

#[test]
fn test_ulong_value_rejects_large_value() {
    let result = ULongValue::new("test", 10_000_000_000);
    assert!(result.is_err(), "Should reject 10 billion");
}

// =============================================================================
// Serialization Tests - Data Size Verification
// =============================================================================

#[test]
fn test_long_value_serializes_as_4_bytes() {
    let lv = LongValue::new("test", 12345).expect("Should create value");
    let bytes = lv.to_bytes();
    assert_eq!(bytes.len(), 4, "Should serialize as 4 bytes");
}

#[test]
fn test_ulong_value_serializes_as_4_bytes() {
    let ulv = ULongValue::new("test", 12345).expect("Should create value");
    let bytes = ulv.to_bytes();
    assert_eq!(bytes.len(), 4, "Should serialize as 4 bytes");
}

#[test]
fn test_long_value_size_is_4() {
    let lv = LongValue::new("test", 12345).expect("Should create value");
    assert_eq!(lv.size(), 4, "Size should be 4 bytes");
}

#[test]
fn test_ulong_value_size_is_4() {
    let ulv = ULongValue::new("test", 12345).expect("Should create value");
    assert_eq!(ulv.size(), 4, "Size should be 4 bytes");
}

// =============================================================================
// Type Verification Tests
// =============================================================================

#[test]
fn test_long_value_returns_correct_type() {
    let lv = LongValue::new("test", 12345).expect("Should create value");
    assert_eq!(lv.value_type(), ValueType::Long, "Should be type Long (6)");
}

#[test]
fn test_ulong_value_returns_correct_type() {
    let ulv = ULongValue::new("test", 12345).expect("Should create value");
    assert_eq!(ulv.value_type(), ValueType::ULong, "Should be type ULong (7)");
}

#[test]
fn test_llong_value_returns_correct_type() {
    let llv = LLongValue::new("test", 5_000_000_000);
    assert_eq!(llv.value_type(), ValueType::LLong, "Should be type LLong (8)");
}

#[test]
fn test_ullong_value_returns_correct_type() {
    let ullv = ULLongValue::new("test", 10_000_000_000);
    assert_eq!(ullv.value_type(), ValueType::ULLong, "Should be type ULLong (9)");
}

// =============================================================================
// Error Message Validation Tests
// =============================================================================

#[test]
fn test_long_value_error_message_is_descriptive() {
    let result = LongValue::new("test", 5_000_000_000);
    assert!(result.is_err(), "Should fail");
    let err_msg = format!("{:?}", result.unwrap_err());
    // Check that error contains type information
    assert!(err_msg.contains("i32") || err_msg.contains("long_value") || err_msg.contains("type 6"),
        "Error should mention target type: {}", err_msg);
}

#[test]
fn test_ulong_value_error_message_is_descriptive() {
    let result = ULongValue::new("test", 10_000_000_000);
    assert!(result.is_err(), "Should fail");
    let err_msg = format!("{:?}", result.unwrap_err());
    // Check that error contains type information
    assert!(err_msg.contains("u32") || err_msg.contains("ulong_value") || err_msg.contains("type 7"),
        "Error should mention target type: {}", err_msg);
}

// =============================================================================
// LLongValue/ULLongValue Tests - 64-bit Range (Should Always Succeed)
// =============================================================================

#[test]
fn test_llong_value_accepts_large_positive() {
    let llv = LLongValue::new("test", 5_000_000_000);
    assert_eq!(llv.value(), 5_000_000_000);
}

#[test]
fn test_llong_value_accepts_large_negative() {
    let llv = LLongValue::new("test", -5_000_000_000);
    assert_eq!(llv.value(), -5_000_000_000);
}

#[test]
fn test_llong_value_accepts_int64_max() {
    let llv = LLongValue::new("test", i64::MAX);
    assert_eq!(llv.value(), i64::MAX);
}

#[test]
fn test_llong_value_accepts_int64_min() {
    let llv = LLongValue::new("test", i64::MIN);
    assert_eq!(llv.value(), i64::MIN);
}

#[test]
fn test_ullong_value_accepts_large_value() {
    let ullv = ULLongValue::new("test", 10_000_000_000);
    assert_eq!(ullv.value(), 10_000_000_000);
}

#[test]
fn test_ullong_value_accepts_uint64_max() {
    let ullv = ULLongValue::new("test", u64::MAX);
    assert_eq!(ullv.value(), u64::MAX);
}

#[test]
fn test_llong_value_serializes_as_8_bytes() {
    let llv = LLongValue::new("test", 5_000_000_000);
    let bytes = llv.to_bytes();
    assert_eq!(bytes.len(), 8, "Should serialize as 8 bytes");
}

#[test]
fn test_ullong_value_serializes_as_8_bytes() {
    let ullv = ULLongValue::new("test", 10_000_000_000);
    let bytes = ullv.to_bytes();
    assert_eq!(bytes.len(), 8, "Should serialize as 8 bytes");
}

// =============================================================================
// Cross-type Conversion Tests
// =============================================================================

#[test]
fn test_long_value_name() {
    let lv = LongValue::new("test_name", 123).expect("Should create value");
    assert_eq!(lv.name(), "test_name");
}

#[test]
fn test_ulong_value_name() {
    let ulv = ULongValue::new("test_name", 123).expect("Should create value");
    assert_eq!(ulv.name(), "test_name");
}

#[test]
fn test_llong_value_name() {
    let llv = LLongValue::new("test_name", 5_000_000_000);
    assert_eq!(llv.name(), "test_name");
}

#[test]
fn test_ullong_value_name() {
    let ullv = ULLongValue::new("test_name", 10_000_000_000);
    assert_eq!(ullv.name(), "test_name");
}

// =============================================================================
// Little-Endian Serialization Tests
// =============================================================================

#[test]
fn test_long_value_little_endian_serialization() {
    let lv = LongValue::new("test", 0x12345678).expect("Should create value");
    let bytes = lv.to_bytes();
    assert_eq!(bytes, vec![0x78, 0x56, 0x34, 0x12], "Should be little-endian");
}

#[test]
fn test_ulong_value_little_endian_serialization() {
    let ulv = ULongValue::new("test", 0x12345678).expect("Should create value");
    let bytes = ulv.to_bytes();
    assert_eq!(bytes, vec![0x78, 0x56, 0x34, 0x12], "Should be little-endian");
}

#[test]
fn test_long_value_negative_little_endian() {
    let lv = LongValue::new("test", -1).expect("Should create value");
    let bytes = lv.to_bytes();
    assert_eq!(bytes, vec![0xFF, 0xFF, 0xFF, 0xFF], "Should be little-endian two's complement");
}

// =============================================================================
// Boundary Value Tests
// =============================================================================

#[test]
fn test_long_value_boundary_values() {
    let test_cases = vec![
        (i32::MIN as i64, true, "i32::MIN"),
        (-1_000_000, true, "-1 million"),
        (-1, true, "-1"),
        (0, true, "0"),
        (1, true, "1"),
        (1_000_000, true, "1 million"),
        (i32::MAX as i64, true, "i32::MAX"),
        ((i32::MIN as i64) - 1, false, "i32::MIN - 1"),
        ((i32::MAX as i64) + 1, false, "i32::MAX + 1"),
        (-5_000_000_000, false, "-5 billion"),
        (5_000_000_000, false, "5 billion"),
    ];

    for (value, should_succeed, description) in test_cases {
        let result = LongValue::new("test", value);
        assert_eq!(
            result.is_ok(),
            should_succeed,
            "LongValue({}) [{}] should {}",
            value,
            description,
            if should_succeed { "succeed" } else { "fail" }
        );
    }
}

#[test]
fn test_ulong_value_boundary_values() {
    let test_cases = vec![
        (0, true, "0"),
        (1, true, "1"),
        (1_000_000, true, "1 million"),
        (u32::MAX as u64, true, "u32::MAX"),
        ((u32::MAX as u64) + 1, false, "u32::MAX + 1"),
        (10_000_000_000, false, "10 billion"),
    ];

    for (value, should_succeed, description) in test_cases {
        let result = ULongValue::new("test", value);
        assert_eq!(
            result.is_ok(),
            should_succeed,
            "ULongValue({}) [{}] should {}",
            value,
            description,
            if should_succeed { "succeed" } else { "fail" }
        );
    }
}
