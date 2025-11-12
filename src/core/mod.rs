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

//! Core module providing foundation types and traits for the container system.
//!
//! ## Modules
//!
//! - `error`: Error types and Result alias
//! - `value`: Value trait definition
//! - `value_types`: ValueType enum (15 value types)
//! - `container`: ValueContainer implementation
//!
//! ## Re-export Pattern
//!
//! ```rust
//! // Simplified imports via re-exports
//! use rust_container_system::core::{ContainerError, Value};
//!
//! // Or use prelude:
//! use rust_container_system::prelude::*;
//!
//! // Instead of deep paths:
//! // use rust_container_system::core::error::ContainerError;
//! // use rust_container_system::core::value::Value;
//! ```

/// Error handling module
pub mod error;

/// Value trait module
pub mod value;

/// Value types enum
pub mod value_types;

/// Container implementation
pub mod container;

/// C++ wire protocol implementation for cross-language compatibility
pub mod wire_protocol;

/// Re-export error types
///
/// ```rust
/// use rust_container_system::core::{ContainerError, Result};
///
/// fn process_data() -> Result<()> {
///     Err(ContainerError::ValueNotFound("key".to_string()))
/// }
/// ```
pub use error::{ContainerError, Result};

/// Re-export Value trait and BaseValue
///
/// ```rust
/// use rust_container_system::core::{Value, BaseValue, ValueType};
/// use rust_container_system::values::IntValue;
/// use std::sync::Arc;
///
/// // BaseValue is a generic value store
/// let base_value = BaseValue::new("test", ValueType::Int, vec![1, 2, 3, 4]);
///
/// // IntValue implements Value trait
/// let int_value: Arc<dyn Value> = Arc::new(IntValue::new("count", 42));
/// assert_eq!(int_value.name(), "count");
/// ```
pub use value::{BaseValue, Value};

/// Re-export ValueType enum
///
/// ```rust
/// use rust_container_system::core::ValueType;
///
/// let vtype = ValueType::String;
/// assert!(vtype.is_numeric() == false);
/// // to_str() returns the numeric string representation
/// assert_eq!(vtype.to_str(), "13");
/// ```
pub use value_types::ValueType;

/// Re-export ValueContainer, Builder and constants
///
/// ```rust
/// use rust_container_system::core::{ValueContainer, ValueContainerBuilder, DEFAULT_MAX_VALUES};
/// use rust_container_system::values::IntValue;
/// use std::sync::Arc;
///
/// let mut container = ValueContainer::new();
/// container.set_message_type("test");
/// container.add_value(Arc::new(IntValue::new("count", 42)));
///
/// // Or use builder pattern
/// let built = ValueContainer::builder()
///     .source("sender", "1")
///     .target("receiver", "2")
///     .message_type("event")
///     .build();
///
/// // Or create with custom limit
/// let limited = ValueContainer::with_max_values(1000);
/// ```
pub use container::{ValueContainer, ValueContainerBuilder, ValueIter, DEFAULT_MAX_VALUES, ABSOLUTE_MAX_VALUES};

/// XML escape utility function to prevent XML injection attacks
///
/// This function escapes XML special characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&apos;`
///
/// # Examples
///
/// ```rust
/// use rust_container_system::core::xml_escape;
///
/// let safe = xml_escape("<script>alert('xss')</script>");
/// assert_eq!(safe, "&lt;script&gt;alert(&apos;xss&apos;)&lt;/script&gt;");
/// ```
pub fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
