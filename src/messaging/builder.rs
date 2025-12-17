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

//! Messaging container builder implementation.
//!
//! This module provides the `MessagingContainerBuilder` which offers a fluent API
//! for creating `ValueContainer` instances with messaging-specific header configurations.
//! The design aligns with the C++ container_system architecture.

use crate::core::{ValueContainer, DEFAULT_MAX_VALUES};

/// Builder for constructing `ValueContainer` instances with a fluent API.
///
/// `MessagingContainerBuilder` provides methods for setting messaging headers
/// (source, target, type) in a chainable manner, aligned with the C++ architecture.
///
/// # Example
///
/// ```rust
/// use rust_container_system::messaging::MessagingContainerBuilder;
///
/// let container = MessagingContainerBuilder::new()
///     .with_source("client", "session_1")
///     .with_target("server", "main")
///     .with_type("request")
///     .with_max_values(500)
///     .build();
///
/// assert_eq!(container.source_id(), "client");
/// assert_eq!(container.source_sub_id(), "session_1");
/// assert_eq!(container.target_id(), "server");
/// assert_eq!(container.target_sub_id(), "main");
/// assert_eq!(container.message_type(), "request");
/// ```
///
/// # Default Values
///
/// - Source ID/Sub-ID: empty strings
/// - Target ID/Sub-ID: empty strings
/// - Message type: "data_container"
/// - Max values: DEFAULT_MAX_VALUES (10,000)
#[derive(Debug, Clone)]
pub struct MessagingContainerBuilder {
    source_id: String,
    source_sub_id: String,
    target_id: String,
    target_sub_id: String,
    message_type: String,
    max_values: usize,
}

impl MessagingContainerBuilder {
    /// Create a new builder with default values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::messaging::MessagingContainerBuilder;
    ///
    /// let builder = MessagingContainerBuilder::new();
    /// let container = builder.build();
    ///
    /// assert_eq!(container.message_type(), "data_container");
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            source_id: String::new(),
            source_sub_id: String::new(),
            target_id: String::new(),
            target_sub_id: String::new(),
            message_type: "data_container".to_string(),
            max_values: DEFAULT_MAX_VALUES,
        }
    }

    /// Set the source (sender) information.
    ///
    /// # Arguments
    ///
    /// * `id` - The source identifier (e.g., application name, client ID)
    /// * `sub_id` - The source sub-identifier (e.g., session ID, instance ID)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::messaging::MessagingContainerBuilder;
    ///
    /// let container = MessagingContainerBuilder::new()
    ///     .with_source("my_app", "session_abc123")
    ///     .build();
    ///
    /// assert_eq!(container.source_id(), "my_app");
    /// assert_eq!(container.source_sub_id(), "session_abc123");
    /// ```
    #[must_use = "builder methods return self for chaining"]
    pub fn with_source(mut self, id: impl Into<String>, sub_id: impl Into<String>) -> Self {
        self.source_id = id.into();
        self.source_sub_id = sub_id.into();
        self
    }

    /// Set the target (receiver) information.
    ///
    /// # Arguments
    ///
    /// * `id` - The target identifier (e.g., destination service, receiver ID)
    /// * `sub_id` - The target sub-identifier (e.g., channel, endpoint)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::messaging::MessagingContainerBuilder;
    ///
    /// let container = MessagingContainerBuilder::new()
    ///     .with_target("backend_service", "api_endpoint")
    ///     .build();
    ///
    /// assert_eq!(container.target_id(), "backend_service");
    /// assert_eq!(container.target_sub_id(), "api_endpoint");
    /// ```
    #[must_use = "builder methods return self for chaining"]
    pub fn with_target(mut self, id: impl Into<String>, sub_id: impl Into<String>) -> Self {
        self.target_id = id.into();
        self.target_sub_id = sub_id.into();
        self
    }

    /// Set the message type.
    ///
    /// The message type identifies the kind of message being sent,
    /// allowing receivers to process messages appropriately.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The message type identifier
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::messaging::MessagingContainerBuilder;
    ///
    /// let container = MessagingContainerBuilder::new()
    ///     .with_type("user_authentication")
    ///     .build();
    ///
    /// assert_eq!(container.message_type(), "user_authentication");
    /// ```
    #[must_use = "builder methods return self for chaining"]
    pub fn with_type(mut self, type_name: impl Into<String>) -> Self {
        self.message_type = type_name.into();
        self
    }

    /// Set the maximum number of values allowed in the container.
    ///
    /// This limit prevents memory exhaustion attacks by capping the number
    /// of values that can be added to the container.
    ///
    /// # Arguments
    ///
    /// * `count` - Maximum number of values (will be capped at ABSOLUTE_MAX_VALUES)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::messaging::MessagingContainerBuilder;
    /// use rust_container_system::values::IntValue;
    /// use std::sync::Arc;
    ///
    /// let mut container = MessagingContainerBuilder::new()
    ///     .with_max_values(2)
    ///     .build();
    ///
    /// assert!(container.add_value(Arc::new(IntValue::new("a", 1))).is_ok());
    /// assert!(container.add_value(Arc::new(IntValue::new("b", 2))).is_ok());
    /// assert!(container.add_value(Arc::new(IntValue::new("c", 3))).is_err());
    /// ```
    #[must_use = "builder methods return self for chaining"]
    pub fn with_max_values(mut self, count: usize) -> Self {
        self.max_values = count;
        self
    }

    /// Build the `ValueContainer` with the configured settings.
    ///
    /// This method consumes the builder and returns a new `ValueContainer`
    /// with all header fields set according to the builder configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::messaging::MessagingContainerBuilder;
    ///
    /// let container = MessagingContainerBuilder::new()
    ///     .with_source("client", "1")
    ///     .with_target("server", "2")
    ///     .with_type("ping")
    ///     .build();
    ///
    /// assert_eq!(container.source_id(), "client");
    /// assert_eq!(container.target_id(), "server");
    /// assert_eq!(container.message_type(), "ping");
    /// ```
    pub fn build(self) -> ValueContainer {
        let mut container = ValueContainer::with_max_values(self.max_values);
        container.set_source(&self.source_id, &self.source_sub_id);
        container.set_target(&self.target_id, &self.target_sub_id);
        container.set_message_type(&self.message_type);
        container
    }
}

impl Default for MessagingContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{IntValue, StringValue};
    use std::sync::Arc;

    #[test]
    fn test_builder_new() {
        let builder = MessagingContainerBuilder::new();
        let container = builder.build();

        assert_eq!(container.source_id(), "");
        assert_eq!(container.source_sub_id(), "");
        assert_eq!(container.target_id(), "");
        assert_eq!(container.target_sub_id(), "");
        assert_eq!(container.message_type(), "data_container");
        assert!(container.is_empty());
    }

    #[test]
    fn test_builder_with_source() {
        let container = MessagingContainerBuilder::new()
            .with_source("app_client", "session_001")
            .build();

        assert_eq!(container.source_id(), "app_client");
        assert_eq!(container.source_sub_id(), "session_001");
    }

    #[test]
    fn test_builder_with_target() {
        let container = MessagingContainerBuilder::new()
            .with_target("app_server", "main_handler")
            .build();

        assert_eq!(container.target_id(), "app_server");
        assert_eq!(container.target_sub_id(), "main_handler");
    }

    #[test]
    fn test_builder_with_type() {
        let container = MessagingContainerBuilder::new()
            .with_type("custom_message")
            .build();

        assert_eq!(container.message_type(), "custom_message");
    }

    #[test]
    fn test_builder_with_max_values() {
        let mut container = MessagingContainerBuilder::new().with_max_values(3).build();

        assert!(container.add_value(Arc::new(IntValue::new("a", 1))).is_ok());
        assert!(container.add_value(Arc::new(IntValue::new("b", 2))).is_ok());
        assert!(container.add_value(Arc::new(IntValue::new("c", 3))).is_ok());
        assert!(container
            .add_value(Arc::new(IntValue::new("d", 4)))
            .is_err());
    }

    #[test]
    fn test_builder_fluent_api() {
        let container = MessagingContainerBuilder::new()
            .with_source("client_app", "session_xyz")
            .with_target("server_app", "endpoint_1")
            .with_type("data_transfer")
            .with_max_values(1000)
            .build();

        assert_eq!(container.source_id(), "client_app");
        assert_eq!(container.source_sub_id(), "session_xyz");
        assert_eq!(container.target_id(), "server_app");
        assert_eq!(container.target_sub_id(), "endpoint_1");
        assert_eq!(container.message_type(), "data_transfer");
    }

    #[test]
    fn test_builder_with_string_types() {
        let source_id = String::from("dynamic_source");
        let target_id: &str = "static_target";

        let container = MessagingContainerBuilder::new()
            .with_source(source_id, "sub1")
            .with_target(target_id, String::from("sub2"))
            .with_type("test")
            .build();

        assert_eq!(container.source_id(), "dynamic_source");
        assert_eq!(container.target_id(), "static_target");
    }

    #[test]
    fn test_builder_default() {
        let builder = MessagingContainerBuilder::default();
        let container = builder.build();

        assert_eq!(container.message_type(), "data_container");
    }

    #[test]
    fn test_builder_clone() {
        let builder = MessagingContainerBuilder::new()
            .with_source("src", "sub")
            .with_type("msg");

        let cloned = builder.clone();
        let container1 = builder.build();
        let container2 = cloned.build();

        assert_eq!(container1.source_id(), container2.source_id());
        assert_eq!(container1.message_type(), container2.message_type());
    }

    #[test]
    fn test_builder_add_values_after_build() {
        let mut container = MessagingContainerBuilder::new()
            .with_source("test", "1")
            .with_type("data")
            .build();

        container
            .add_value(Arc::new(IntValue::new("count", 42)))
            .unwrap();
        container
            .add_value(Arc::new(StringValue::new("name", "test_value")))
            .unwrap();

        assert_eq!(container.value_count(), 2);
        assert_eq!(container.get_value("count").unwrap().to_int().unwrap(), 42);
        assert_eq!(
            container.get_value("name").unwrap().to_string(),
            "test_value"
        );
    }

    #[test]
    fn test_builder_empty_strings() {
        let container = MessagingContainerBuilder::new()
            .with_source("", "")
            .with_target("", "")
            .with_type("")
            .build();

        assert_eq!(container.source_id(), "");
        assert_eq!(container.target_id(), "");
        assert_eq!(container.message_type(), "");
    }

    #[test]
    fn test_builder_special_characters() {
        let container = MessagingContainerBuilder::new()
            .with_source("user@domain.com", "session/path")
            .with_target("서버", "端点")
            .with_type("<xml>type</xml>")
            .build();

        assert_eq!(container.source_id(), "user@domain.com");
        assert_eq!(container.source_sub_id(), "session/path");
        assert_eq!(container.target_id(), "서버");
        assert_eq!(container.target_sub_id(), "端点");
        assert_eq!(container.message_type(), "<xml>type</xml>");
    }
}
