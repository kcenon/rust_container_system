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

//! Dependency Injection support for ValueContainer components.
//!
//! This module provides traits and implementations for integrating `ValueContainer`
//! into Dependency Injection (DI) frameworks. The design aligns with the C++ Kcenon
//! architecture while embracing Rust idioms.
//!
//! ## Core Concepts
//!
//! - **ContainerFactory**: A trait defining the contract for creating containers
//! - **DefaultContainerFactory**: A basic implementation with configurable defaults
//! - **ArcContainerProvider**: A thread-safe provider suitable for shared ownership
//!
//! ## Usage Patterns
//!
//! ### Basic Factory Usage
//!
//! ```rust
//! use rust_container_system::kcenon::{ContainerFactory, DefaultContainerFactory};
//!
//! let factory = DefaultContainerFactory::new();
//! let container = factory.create();
//! ```
//!
//! ### Configured Factory
//!
//! ```rust
//! use rust_container_system::kcenon::{ContainerFactory, DefaultContainerFactory};
//!
//! let factory = DefaultContainerFactory::builder()
//!     .with_default_message_type("custom_type")
//!     .with_default_max_values(500)
//!     .build();
//!
//! let container = factory.create();
//! assert_eq!(container.message_type(), "custom_type");
//! ```
//!
//! ### Arc-based Injection
//!
//! ```rust
//! use rust_container_system::kcenon::{ContainerFactory, ArcContainerProvider};
//! use std::sync::Arc;
//!
//! // Create shared provider
//! let provider: Arc<dyn ContainerFactory> = Arc::new(ArcContainerProvider::new());
//!
//! // Clone for multiple consumers
//! let provider_clone = Arc::clone(&provider);
//!
//! // Use in different parts of application
//! let container1 = provider.create();
//! let container2 = provider_clone.create();
//! ```

use crate::core::{ValueContainer, DEFAULT_MAX_VALUES};

/// Trait for creating `ValueContainer` instances.
///
/// This trait provides the abstraction for container creation, enabling
/// dependency injection and testability. Implementations can provide
/// different container configurations or wrap container creation with
/// additional logic.
///
/// # Thread Safety
///
/// Implementations should be thread-safe (`Send + Sync`) to support
/// concurrent access in multi-threaded applications.
///
/// # Example
///
/// ```rust
/// use rust_container_system::kcenon::ContainerFactory;
/// use rust_container_system::core::ValueContainer;
///
/// struct CustomFactory {
///     prefix: String,
/// }
///
/// impl ContainerFactory for CustomFactory {
///     fn create(&self) -> ValueContainer {
///         let mut container = ValueContainer::new();
///         container.set_message_type(format!("{}_message", self.prefix));
///         container
///     }
///
///     fn create_with_type(&self, message_type: &str) -> ValueContainer {
///         let mut container = ValueContainer::new();
///         container.set_message_type(format!("{}_{}", self.prefix, message_type));
///         container
///     }
/// }
///
/// let factory = CustomFactory { prefix: "app".to_string() };
/// let container = factory.create();
/// assert_eq!(container.message_type(), "app_message");
/// ```
pub trait ContainerFactory: Send + Sync {
    /// Create a new `ValueContainer` with default settings.
    ///
    /// Returns a fresh container instance configured with the factory's
    /// default parameters.
    fn create(&self) -> ValueContainer;

    /// Create a new `ValueContainer` with specified message type.
    ///
    /// # Arguments
    ///
    /// * `message_type` - The message type to set on the container
    fn create_with_type(&self, message_type: &str) -> ValueContainer;

    /// Create a new `ValueContainer` with full header configuration.
    ///
    /// # Arguments
    ///
    /// * `source_id` - Source identifier
    /// * `source_sub_id` - Source sub-identifier
    /// * `target_id` - Target identifier
    /// * `target_sub_id` - Target sub-identifier
    /// * `message_type` - Message type
    fn create_with_header(
        &self,
        source_id: &str,
        source_sub_id: &str,
        target_id: &str,
        target_sub_id: &str,
        message_type: &str,
    ) -> ValueContainer {
        let mut container = self.create_with_type(message_type);
        container.set_source(source_id, source_sub_id);
        container.set_target(target_id, target_sub_id);
        container
    }
}

/// Default implementation of `ContainerFactory`.
///
/// Provides basic container creation with configurable defaults for
/// message type and maximum values. This is suitable for most use cases.
///
/// # Example
///
/// ```rust
/// use rust_container_system::kcenon::{ContainerFactory, DefaultContainerFactory};
///
/// let factory = DefaultContainerFactory::new();
/// let container = factory.create();
///
/// assert_eq!(container.message_type(), "data_container");
/// ```
#[derive(Debug, Clone)]
pub struct DefaultContainerFactory {
    default_message_type: String,
    default_max_values: usize,
}

impl DefaultContainerFactory {
    /// Create a new factory with default settings.
    ///
    /// Default message type is "data_container" and max values is 10,000.
    #[must_use]
    pub fn new() -> Self {
        Self {
            default_message_type: "data_container".to_string(),
            default_max_values: DEFAULT_MAX_VALUES,
        }
    }

    /// Create a builder for configuring the factory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::kcenon::{ContainerFactory, DefaultContainerFactory};
    ///
    /// let factory = DefaultContainerFactory::builder()
    ///     .with_default_message_type("custom")
    ///     .with_default_max_values(1000)
    ///     .build();
    ///
    /// let container = factory.create();
    /// assert_eq!(container.message_type(), "custom");
    /// ```
    #[must_use]
    pub fn builder() -> DefaultContainerFactoryBuilder {
        DefaultContainerFactoryBuilder::new()
    }

    /// Get the default message type.
    #[must_use]
    pub fn default_message_type(&self) -> &str {
        &self.default_message_type
    }

    /// Get the default max values.
    #[must_use]
    pub fn default_max_values(&self) -> usize {
        self.default_max_values
    }
}

impl Default for DefaultContainerFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerFactory for DefaultContainerFactory {
    fn create(&self) -> ValueContainer {
        let mut container = ValueContainer::with_max_values(self.default_max_values);
        container.set_message_type(&self.default_message_type);
        container
    }

    fn create_with_type(&self, message_type: &str) -> ValueContainer {
        let mut container = ValueContainer::with_max_values(self.default_max_values);
        container.set_message_type(message_type);
        container
    }
}

/// Builder for `DefaultContainerFactory`.
///
/// Provides a fluent API for configuring factory defaults.
#[derive(Debug, Clone)]
pub struct DefaultContainerFactoryBuilder {
    default_message_type: String,
    default_max_values: usize,
}

impl DefaultContainerFactoryBuilder {
    /// Create a new builder with default values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            default_message_type: "data_container".to_string(),
            default_max_values: DEFAULT_MAX_VALUES,
        }
    }

    /// Set the default message type for created containers.
    #[must_use]
    pub fn with_default_message_type(mut self, message_type: impl Into<String>) -> Self {
        self.default_message_type = message_type.into();
        self
    }

    /// Set the default maximum values for created containers.
    #[must_use]
    pub fn with_default_max_values(mut self, max_values: usize) -> Self {
        self.default_max_values = max_values;
        self
    }

    /// Build the factory with the configured settings.
    #[must_use]
    pub fn build(self) -> DefaultContainerFactory {
        DefaultContainerFactory {
            default_message_type: self.default_message_type,
            default_max_values: self.default_max_values,
        }
    }
}

impl Default for DefaultContainerFactoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A thread-safe container provider for `Arc`-based dependency injection.
///
/// `ArcContainerProvider` is designed for scenarios where factory instances
/// need to be shared across threads using `Arc`. It provides the same
/// functionality as `DefaultContainerFactory` but emphasizes its role
/// as a shared service provider.
///
/// # Example
///
/// ```rust
/// use rust_container_system::kcenon::{ContainerFactory, ArcContainerProvider};
/// use std::sync::Arc;
///
/// // Create a shared provider
/// let provider = Arc::new(ArcContainerProvider::new());
///
/// // Share across threads
/// let provider_for_thread = Arc::clone(&provider);
/// std::thread::spawn(move || {
///     let container = provider_for_thread.create();
///     // Use container...
/// });
///
/// // Use in main thread
/// let container = provider.create();
/// ```
#[derive(Debug, Clone)]
pub struct ArcContainerProvider {
    inner: DefaultContainerFactory,
}

impl ArcContainerProvider {
    /// Create a new provider with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: DefaultContainerFactory::new(),
        }
    }

    /// Create a new provider with custom factory configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_container_system::kcenon::{
    ///     ContainerFactory, ArcContainerProvider, DefaultContainerFactory
    /// };
    ///
    /// let factory = DefaultContainerFactory::builder()
    ///     .with_default_message_type("custom")
    ///     .build();
    ///
    /// let provider = ArcContainerProvider::with_factory(factory);
    /// let container = provider.create();
    /// assert_eq!(container.message_type(), "custom");
    /// ```
    #[must_use]
    pub fn with_factory(factory: DefaultContainerFactory) -> Self {
        Self { inner: factory }
    }

    /// Create a builder for configuring the provider.
    #[must_use]
    pub fn builder() -> ArcContainerProviderBuilder {
        ArcContainerProviderBuilder::new()
    }
}

impl Default for ArcContainerProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerFactory for ArcContainerProvider {
    fn create(&self) -> ValueContainer {
        self.inner.create()
    }

    fn create_with_type(&self, message_type: &str) -> ValueContainer {
        self.inner.create_with_type(message_type)
    }
}

/// Builder for `ArcContainerProvider`.
#[derive(Debug, Clone)]
pub struct ArcContainerProviderBuilder {
    factory_builder: DefaultContainerFactoryBuilder,
}

impl ArcContainerProviderBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            factory_builder: DefaultContainerFactoryBuilder::new(),
        }
    }

    /// Set the default message type.
    #[must_use]
    pub fn with_default_message_type(mut self, message_type: impl Into<String>) -> Self {
        self.factory_builder = self.factory_builder.with_default_message_type(message_type);
        self
    }

    /// Set the default maximum values.
    #[must_use]
    pub fn with_default_max_values(mut self, max_values: usize) -> Self {
        self.factory_builder = self.factory_builder.with_default_max_values(max_values);
        self
    }

    /// Build the provider.
    #[must_use]
    pub fn build(self) -> ArcContainerProvider {
        ArcContainerProvider {
            inner: self.factory_builder.build(),
        }
    }
}

impl Default for ArcContainerProviderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_default_factory_create() {
        let factory = DefaultContainerFactory::new();
        let container = factory.create();

        assert!(container.is_empty());
        assert_eq!(container.message_type(), "data_container");
    }

    #[test]
    fn test_default_factory_create_with_type() {
        let factory = DefaultContainerFactory::new();
        let container = factory.create_with_type("custom_message");

        assert_eq!(container.message_type(), "custom_message");
    }

    #[test]
    fn test_default_factory_create_with_header() {
        let factory = DefaultContainerFactory::new();
        let container =
            factory.create_with_header("source", "sub_source", "target", "sub_target", "request");

        assert_eq!(container.source_id(), "source");
        assert_eq!(container.source_sub_id(), "sub_source");
        assert_eq!(container.target_id(), "target");
        assert_eq!(container.target_sub_id(), "sub_target");
        assert_eq!(container.message_type(), "request");
    }

    #[test]
    fn test_factory_builder() {
        let factory = DefaultContainerFactory::builder()
            .with_default_message_type("configured_type")
            .with_default_max_values(500)
            .build();

        assert_eq!(factory.default_message_type(), "configured_type");
        assert_eq!(factory.default_max_values(), 500);

        let container = factory.create();
        assert_eq!(container.message_type(), "configured_type");
    }

    #[test]
    fn test_arc_container_provider() {
        let provider = ArcContainerProvider::new();
        let container = provider.create();

        assert!(container.is_empty());
        assert_eq!(container.message_type(), "data_container");
    }

    #[test]
    fn test_arc_provider_with_factory() {
        let factory = DefaultContainerFactory::builder()
            .with_default_message_type("injected")
            .build();

        let provider = ArcContainerProvider::with_factory(factory);
        let container = provider.create();

        assert_eq!(container.message_type(), "injected");
    }

    #[test]
    fn test_arc_provider_builder() {
        let provider = ArcContainerProvider::builder()
            .with_default_message_type("built")
            .with_default_max_values(100)
            .build();

        let container = provider.create();
        assert_eq!(container.message_type(), "built");
    }

    #[test]
    fn test_factory_trait_object() {
        let factory: Box<dyn ContainerFactory> = Box::new(DefaultContainerFactory::new());
        let container = factory.create();

        assert!(container.is_empty());
    }

    #[test]
    fn test_arc_factory_trait_object() {
        let factory: Arc<dyn ContainerFactory> = Arc::new(ArcContainerProvider::new());
        let container = factory.create();

        assert!(container.is_empty());
    }

    #[test]
    fn test_thread_safety() {
        let provider: Arc<dyn ContainerFactory> = Arc::new(ArcContainerProvider::new());
        let mut handles = vec![];

        for i in 0..4 {
            let provider_clone = Arc::clone(&provider);
            let handle = thread::spawn(move || {
                let container = provider_clone.create_with_type(&format!("thread_{}", i));
                assert_eq!(container.message_type(), format!("thread_{}", i));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_custom_factory_implementation() {
        struct PrefixedFactory {
            prefix: String,
        }

        impl ContainerFactory for PrefixedFactory {
            fn create(&self) -> ValueContainer {
                let mut container = ValueContainer::new();
                container.set_message_type(format!("{}_default", self.prefix));
                container
            }

            fn create_with_type(&self, message_type: &str) -> ValueContainer {
                let mut container = ValueContainer::new();
                container.set_message_type(format!("{}_{}", self.prefix, message_type));
                container
            }
        }

        let factory = PrefixedFactory {
            prefix: "app".to_string(),
        };

        assert_eq!(factory.create().message_type(), "app_default");
        assert_eq!(
            factory.create_with_type("request").message_type(),
            "app_request"
        );
    }

    #[test]
    fn test_factory_default_trait() {
        let factory = DefaultContainerFactory::default();
        assert_eq!(factory.default_message_type(), "data_container");

        let provider = ArcContainerProvider::default();
        let container = provider.create();
        assert_eq!(container.message_type(), "data_container");
    }
}
