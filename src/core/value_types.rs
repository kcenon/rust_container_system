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

//! Value type definitions for the container system.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Value types supported by the container system
///
/// Each variant corresponds to a specific data type (0-14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ValueType {
    /// Null or undefined value
    #[default]
    Null = 0,
    /// Boolean (true/false)
    Bool = 1,
    /// 16-bit signed integer
    Short = 2,
    /// 16-bit unsigned integer
    UShort = 3,
    /// 32-bit signed integer
    Int = 4,
    /// 32-bit unsigned integer
    UInt = 5,
    /// 64-bit signed integer (platform-dependent)
    Long = 6,
    /// 64-bit unsigned integer (platform-dependent)
    ULong = 7,
    /// 64-bit signed integer (long long)
    LLong = 8,
    /// 64-bit unsigned integer (unsigned long long)
    ULLong = 9,
    /// 32-bit floating point
    Float = 10,
    /// 64-bit floating point
    Double = 11,
    /// Raw byte array (binary data)
    Bytes = 12,
    /// UTF-8 encoded string
    String = 13,
    /// Nested container
    Container = 14,
    /// Array/list of values
    Array = 15,
}

impl ValueType {
    /// Convert type code string to ValueType
    ///
    /// # Example
    /// ```
    /// use rust_container_system::ValueType;
    ///
    /// assert_eq!(ValueType::from_type_code("4"), Some(ValueType::Int));
    /// assert_eq!(ValueType::from_type_code("99"), None);
    /// ```
    pub fn from_type_code(s: &str) -> Option<Self> {
        match s {
            "0" => Some(ValueType::Null),
            "1" => Some(ValueType::Bool),
            "2" => Some(ValueType::Short),
            "3" => Some(ValueType::UShort),
            "4" => Some(ValueType::Int),
            "5" => Some(ValueType::UInt),
            "6" => Some(ValueType::Long),
            "7" => Some(ValueType::ULong),
            "8" => Some(ValueType::LLong),
            "9" => Some(ValueType::ULLong),
            "10" => Some(ValueType::Float),
            "11" => Some(ValueType::Double),
            "12" => Some(ValueType::Bytes),
            "13" => Some(ValueType::String),
            "14" => Some(ValueType::Container),
            "15" => Some(ValueType::Array),
            _ => None,
        }
    }

    /// Convert ValueType to string representation
    ///
    /// # Example
    /// ```
    /// use rust_container_system::ValueType;
    ///
    /// assert_eq!(ValueType::Int.to_str(), "4");
    /// assert_eq!(ValueType::String.to_str(), "13");
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            ValueType::Null => "0",
            ValueType::Bool => "1",
            ValueType::Short => "2",
            ValueType::UShort => "3",
            ValueType::Int => "4",
            ValueType::UInt => "5",
            ValueType::Long => "6",
            ValueType::ULong => "7",
            ValueType::LLong => "8",
            ValueType::ULLong => "9",
            ValueType::Float => "10",
            ValueType::Double => "11",
            ValueType::Bytes => "12",
            ValueType::String => "13",
            ValueType::Container => "14",
            ValueType::Array => "15",
        }
    }

    /// Check if this is a numeric type (integer or float)
    ///
    /// # Example
    /// ```
    /// use rust_container_system::ValueType;
    ///
    /// assert!(ValueType::Int.is_numeric());
    /// assert!(ValueType::Double.is_numeric());
    /// assert!(!ValueType::String.is_numeric());
    /// ```
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            ValueType::Short
                | ValueType::UShort
                | ValueType::Int
                | ValueType::UInt
                | ValueType::Long
                | ValueType::ULong
                | ValueType::LLong
                | ValueType::ULLong
                | ValueType::Float
                | ValueType::Double
        )
    }

    /// Check if this is an integer type
    ///
    /// # Example
    /// ```
    /// use rust_container_system::ValueType;
    ///
    /// assert!(ValueType::Int.is_integer());
    /// assert!(!ValueType::Double.is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            ValueType::Short
                | ValueType::UShort
                | ValueType::Int
                | ValueType::UInt
                | ValueType::Long
                | ValueType::ULong
                | ValueType::LLong
                | ValueType::ULLong
        )
    }

    /// Check if this is a floating point type
    ///
    /// # Example
    /// ```
    /// use rust_container_system::ValueType;
    ///
    /// assert!(ValueType::Float.is_float());
    /// assert!(ValueType::Double.is_float());
    /// assert!(!ValueType::Int.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        matches!(self, ValueType::Float | ValueType::Double)
    }

    /// Get size in bytes for fixed-size types
    ///
    /// Returns None for variable-size types (String, Bytes, Container).
    ///
    /// # Example
    /// ```
    /// use rust_container_system::ValueType;
    ///
    /// assert_eq!(ValueType::Bool.size_bytes(), Some(1));
    /// assert_eq!(ValueType::Int.size_bytes(), Some(4));
    /// assert_eq!(ValueType::String.size_bytes(), None);
    /// ```
    pub fn size_bytes(&self) -> Option<usize> {
        match self {
            ValueType::Null => Some(0),
            ValueType::Bool => Some(1),
            ValueType::Short | ValueType::UShort => Some(2),
            ValueType::Int | ValueType::UInt | ValueType::Float => Some(4),
            ValueType::Long
            | ValueType::ULong
            | ValueType::LLong
            | ValueType::ULLong
            | ValueType::Double => Some(8),
            ValueType::Bytes | ValueType::String | ValueType::Container | ValueType::Array => None,
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueType::Null => "null_value",
                ValueType::Bool => "bool_value",
                ValueType::Short => "short_value",
                ValueType::UShort => "ushort_value",
                ValueType::Int => "int_value",
                ValueType::UInt => "uint_value",
                ValueType::Long => "long_value",
                ValueType::ULong => "ulong_value",
                ValueType::LLong => "llong_value",
                ValueType::ULLong => "ullong_value",
                ValueType::Float => "float_value",
                ValueType::Double => "double_value",
                ValueType::Bytes => "bytes_value",
                ValueType::String => "string_value",
                ValueType::Container => "container_value",
                ValueType::Array => "array_value",
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_from_type_code() {
        assert_eq!(ValueType::from_type_code("0"), Some(ValueType::Null));
        assert_eq!(ValueType::from_type_code("1"), Some(ValueType::Bool));
        assert_eq!(ValueType::from_type_code("14"), Some(ValueType::Container));
        assert_eq!(ValueType::from_type_code("99"), None);
        assert_eq!(ValueType::from_type_code("abc"), None);
    }

    #[test]
    fn test_value_type_to_str() {
        assert_eq!(ValueType::Null.to_str(), "0");
        assert_eq!(ValueType::Bool.to_str(), "1");
        assert_eq!(ValueType::Int.to_str(), "4");
        assert_eq!(ValueType::Container.to_str(), "14");
    }

    #[test]
    fn test_is_numeric() {
        assert!(ValueType::Int.is_numeric());
        assert!(ValueType::Double.is_numeric());
        assert!(ValueType::Float.is_numeric());
        assert!(!ValueType::String.is_numeric());
        assert!(!ValueType::Bool.is_numeric());
        assert!(!ValueType::Bytes.is_numeric());
    }

    #[test]
    fn test_size_bytes() {
        assert_eq!(ValueType::Bool.size_bytes(), Some(1));
        assert_eq!(ValueType::Short.size_bytes(), Some(2));
        assert_eq!(ValueType::Int.size_bytes(), Some(4));
        assert_eq!(ValueType::Double.size_bytes(), Some(8));
        assert_eq!(ValueType::String.size_bytes(), None);
        assert_eq!(ValueType::Bytes.size_bytes(), None);
        assert_eq!(ValueType::Container.size_bytes(), None);
    }
}
