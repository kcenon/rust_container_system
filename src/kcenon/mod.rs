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

//! Kcenon module providing Dependency Injection (DI) support for container components.
//!
//! This module provides integration adapters for Dependency Injection frameworks,
//! allowing `ValueContainer` components to be easily registered and retrieved in
//! Rust DI ecosystems.
//!
//! ## Features
//!
//! - `ContainerFactory` trait for abstracting container creation
//! - `DefaultContainerFactory` implementation for standard use cases
//! - `ArcContainerProvider` for `Arc`-based dependency injection
//! - Thread-safe factory implementations
//!
//! ## Example
//!
//! ```rust
//! use rust_container_system::kcenon::{ContainerFactory, DefaultContainerFactory};
//!
//! let factory = DefaultContainerFactory::new();
//! let container = factory.create();
//!
//! assert!(container.is_empty());
//! ```
//!
//! ## Integration with DI Frameworks
//!
//! The `ContainerFactory` trait can be implemented for integration with various
//! Rust DI frameworks:
//!
//! ```rust
//! use rust_container_system::kcenon::{ContainerFactory, ArcContainerProvider};
//! use std::sync::Arc;
//!
//! // Create a shared provider for dependency injection
//! let provider: Arc<dyn ContainerFactory> = Arc::new(ArcContainerProvider::new());
//!
//! // Use in application components
//! let container = provider.create();
//! ```

/// Dependency Injection support module
pub mod di;

/// Re-export main DI types for convenient access
pub use di::{ArcContainerProvider, ContainerFactory, DefaultContainerFactory};
