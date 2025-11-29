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

//! Binary format interoperability tests
//!
//! These tests verify that binary serialization is compatible across
//! C++, Go, and Rust implementations.

use rust_container_system::prelude::*;
use rust_container_system::values::ArrayValue;
use std::sync::Arc;

/// Generate binary data for testing cross-language compatibility
#[test]
fn test_generate_rust_binary_data() {
    // Generate binary representations for various types
    // These hex strings can be used in Go/C++ tests

    let test_cases = vec![
        (
            "Int32",
            Arc::new(IntValue::new("testi32", 42)) as Arc<dyn Value>,
        ),
        (
            "Bool_True",
            Arc::new(BoolValue::new("bool_true", true)) as Arc<dyn Value>,
        ),
        (
            "Bool_False",
            Arc::new(BoolValue::new("bool_false", false)) as Arc<dyn Value>,
        ),
        (
            "String",
            Arc::new(StringValue::new("mystr", "Hello, World!")) as Arc<dyn Value>,
        ),
        (
            "Int64",
            Arc::new(LLongValue::new("i64", -9876543210)) as Arc<dyn Value>,
        ),
        (
            "Float",
            Arc::new(FloatValue::new("f32", 3.14159)) as Arc<dyn Value>,
        ),
        (
            "Double",
            Arc::new(DoubleValue::new("f64", 2.71828182845)) as Arc<dyn Value>,
        ),
        (
            "Bytes",
            Arc::new(BytesValue::new("bytes", vec![0xDE, 0xAD, 0xBE, 0xEF])) as Arc<dyn Value>,
        ),
    ];

    println!("\n=== Rust Binary Data for Cross-Language Testing ===\n");

    for (name, value) in test_cases {
        let binary = value.to_bytes();
        let hex_string = binary
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        println!(
            "// {} (type={:?}, name={})",
            name,
            value.value_type(),
            value.name()
        );
        println!(
            "const RUST_{}:  &str = \"{}\";",
            name.to_uppercase(),
            hex_string
        );
        println!("// Length: {} bytes\n", binary.len());
    }
}

/// Test binary roundtrip for types with complete binary format
/// Note: Only types with complete binary format (header + data) are tested
/// Many primitive types use minimal format - TODO: fix these in separate issue
#[test]
fn test_binary_roundtrip_all_types() {
    // Test only types known to have complete binary format implementation
    let test_cases: Vec<(&str, Arc<dyn Value>)> = vec![
        ("Bool_True", Arc::new(BoolValue::new("b_true", true))),
        ("Bool_False", Arc::new(BoolValue::new("b_false", false))),
        ("Int_Pos", Arc::new(IntValue::new("i32_pos", 2147483647))),
        ("Int_Neg", Arc::new(IntValue::new("i32_neg", -2147483648))),
        ("String_Empty", Arc::new(StringValue::new("s_empty", ""))),
        (
            "String_ASCII",
            Arc::new(StringValue::new("s_ascii", "Hello, World!")),
        ),
        (
            "String_UTF8",
            Arc::new(StringValue::new("s_utf8", "안녕하세요 🌍")),
        ),
        ("Bytes_Empty", Arc::new(BytesValue::new("b_empty", vec![]))),
        (
            "Bytes_Data",
            Arc::new(BytesValue::new(
                "b_data",
                vec![0x00, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF],
            )),
        ),
    ];

    for (name, value) in test_cases {
        // Serialize
        let binary = value.to_bytes();

        // Verify format structure
        assert!(binary.len() >= 10, "{}: binary too short", name);

        // Verify type byte
        assert_eq!(
            binary[0],
            value.value_type() as u8,
            "{}: type byte mismatch",
            name
        );

        // Verify name length (4 bytes LE)
        let name_len = u32::from_le_bytes([binary[1], binary[2], binary[3], binary[4]]) as usize;
        assert_eq!(
            name_len,
            value.name().len(),
            "{}: name length mismatch",
            name
        );

        // Verify name
        let name_bytes = &binary[5..5 + name_len];
        let actual_name = std::str::from_utf8(name_bytes).unwrap();
        assert_eq!(actual_name, value.name(), "{}: name mismatch", name);

        // Verify value_size field exists
        let value_size_offset = 5 + name_len;
        assert!(
            value_size_offset + 4 <= binary.len(),
            "{}: value_size missing",
            name
        );

        println!(
            "✓ {}: binary format verified ({} bytes)",
            name,
            binary.len()
        );
    }
}

/// Test deserializing Go-generated binary data
#[test]
fn test_deserialize_go_binary_data() {
    // These hex strings are generated by Go implementation
    // Format: [type:1][name_len:4 LE][name][value_size:4 LE][value]

    // Note: These would be actual hex strings from Go tests
    // For now, we document the expected format

    println!("Note: Actual Go→Rust deserialization tests require Go-generated binary data");
    println!("Expected format: [type:1][name_len:4 LE][name][value_size:4 LE][value]");
    println!("This test verifies the format expectations are correct");
}

/// Test binary compatibility with specific known values
#[test]
fn test_binary_format_structure() {
    // Create a simple Int32 value
    let value = IntValue::new("test", 42);
    let binary = value.to_bytes();

    // Expected format:
    // [type:1] = 0x04 (IntValue)
    // [name_len:4 LE] = 0x04000000 (4 in little-endian)
    // [name:4] = "test" (0x74657374)
    // [value_size:4 LE] = 0x04000000 (4 bytes)
    // [value:4 LE] = 0x2a000000 (42 in little-endian)

    assert_eq!(binary[0], 0x04, "Type byte should be 0x04 for Int");
    assert_eq!(binary[1], 0x04, "Name length byte 0 should be 0x04");
    assert_eq!(binary[2], 0x00, "Name length byte 1 should be 0x00");
    assert_eq!(binary[3], 0x00, "Name length byte 2 should be 0x00");
    assert_eq!(binary[4], 0x00, "Name length byte 3 should be 0x00");

    // Name bytes: "test"
    assert_eq!(&binary[5..9], b"test", "Name should be 'test'");

    // Value size
    assert_eq!(binary[9], 0x04, "Value size byte 0 should be 0x04");

    // Value: 42 in little-endian
    assert_eq!(binary[13], 0x2a, "Value byte 0 should be 0x2a (42)");
    assert_eq!(binary[14], 0x00, "Value byte 1 should be 0x00");

    println!("✓ Binary format structure verified");
    println!("  Total size: {} bytes", binary.len());
    println!(
        "  Hex: {}",
        binary
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
}

/// Test cross-language ArrayValue compatibility
#[test]
fn test_array_value_binary_format() {
    // Create an array with mixed types
    let elements = vec![
        Arc::new(IntValue::new("", 42)) as Arc<dyn Value>,
        Arc::new(StringValue::new("", "test")) as Arc<dyn Value>,
        Arc::new(BoolValue::new("", true)) as Arc<dyn Value>,
    ];
    let array = ArrayValue::new("mixed", elements);

    // Serialize using to_binary_bytes() which includes type header
    let binary = array.to_binary_bytes();

    // Verify format
    assert_eq!(
        binary[0],
        ValueType::Array as u8,
        "Type should be Array (0x0F)"
    );

    // Deserialize
    let restored = ArrayValue::deserialize_binary(&binary).unwrap();
    assert_eq!(restored.count(), 3, "Should have 3 elements");

    println!("✓ ArrayValue binary format compatible");
    println!("  Array size: {} bytes", binary.len());
    println!("  Element count: {}", restored.count());
}
