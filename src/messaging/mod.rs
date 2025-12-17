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

//! Messaging module providing builder patterns for container construction.
//!
//! This module provides the `MessagingContainerBuilder` pattern aligned with
//! the C++ container_system architecture, offering a fluent API for creating
//! `ValueContainer` instances with standardized header configurations.
//!
//! ## Example
//!
//! ```rust
//! use rust_container_system::messaging::MessagingContainerBuilder;
//!
//! let container = MessagingContainerBuilder::new()
//!     .with_source("client_app", "session_123")
//!     .with_target("server_app", "main")
//!     .with_type("user_request")
//!     .with_max_values(1000)
//!     .build();
//!
//! assert_eq!(container.source_id(), "client_app");
//! assert_eq!(container.message_type(), "user_request");
//! ```

/// Builder module for fluent container construction
pub mod builder;

/// Re-export MessagingContainerBuilder for convenient access
pub use builder::MessagingContainerBuilder;
