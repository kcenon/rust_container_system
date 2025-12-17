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

//! Dependency Injection example
//!
//! Demonstrates using ContainerFactory trait and implementations for
//! dependency injection patterns in Rust applications.
//!
//! Run with: cargo run --example dependency_injection

use rust_container_system::prelude::*;
use std::sync::Arc;

/// Example service that uses ContainerFactory for creating containers
struct MessageService {
    factory: Arc<dyn ContainerFactory>,
    service_id: String,
}

impl MessageService {
    fn new(factory: Arc<dyn ContainerFactory>, service_id: impl Into<String>) -> Self {
        Self {
            factory,
            service_id: service_id.into(),
        }
    }

    fn create_request(&self, target: &str, request_type: &str) -> ValueContainer {
        let mut container = self.factory.create_with_type(request_type);
        container.set_source(&self.service_id, "main");
        container.set_target(target, "handler");
        container
    }

    fn create_response(&self, request: &ValueContainer) -> ValueContainer {
        let mut response = self.factory.create_with_type("response");
        response.set_source(&self.service_id, "main");
        response.set_target(&request.source_id(), &request.source_sub_id());
        response
    }
}

/// Custom factory implementation example
struct PrefixedContainerFactory {
    prefix: String,
    max_values: usize,
}

impl PrefixedContainerFactory {
    fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            max_values: 1000,
        }
    }
}

impl ContainerFactory for PrefixedContainerFactory {
    fn create(&self) -> ValueContainer {
        let mut container = ValueContainer::with_max_values(self.max_values);
        container.set_message_type(format!("{}_message", self.prefix));
        container
    }

    fn create_with_type(&self, message_type: &str) -> ValueContainer {
        let mut container = ValueContainer::with_max_values(self.max_values);
        container.set_message_type(format!("{}_{}", self.prefix, message_type));
        container
    }
}

fn main() {
    println!("=== Rust Container System - Dependency Injection Example ===\n");

    // 1. Using DefaultContainerFactory
    println!("1. DefaultContainerFactory:");
    let default_factory = DefaultContainerFactory::new();
    let container = default_factory.create();
    println!("   Message type: {}", container.message_type());

    // 2. Configured factory using builder
    println!("\n2. Configured Factory (Builder Pattern):");
    let configured_factory = DefaultContainerFactory::builder()
        .with_default_message_type("app_message")
        .with_default_max_values(500)
        .build();
    let container = configured_factory.create();
    println!("   Message type: {}", container.message_type());
    println!(
        "   Max values: {}",
        configured_factory.default_max_values()
    );

    // 3. Arc-based provider for sharing across components
    println!("\n3. ArcContainerProvider (Shared Ownership):");
    let provider = Arc::new(
        ArcContainerProvider::builder()
            .with_default_message_type("shared_message")
            .build(),
    );

    // Clone for multiple services
    let provider_clone = Arc::clone(&provider);
    let container1 = provider.create();
    let container2 = provider_clone.create();
    println!("   Container 1 type: {}", container1.message_type());
    println!("   Container 2 type: {}", container2.message_type());

    // 4. Service injection example
    println!("\n4. Service Injection:");
    let service_factory: Arc<dyn ContainerFactory> = Arc::new(
        ArcContainerProvider::builder()
            .with_default_message_type("service_message")
            .build(),
    );

    let service = MessageService::new(Arc::clone(&service_factory), "auth_service");
    let request = service.create_request("user_service", "login_request");
    println!("   Request source: {}", request.source_id());
    println!("   Request target: {}", request.target_id());
    println!("   Request type: {}", request.message_type());

    let response = service.create_response(&request);
    println!("   Response target: {}", response.target_id());
    println!("   Response type: {}", response.message_type());

    // 5. Custom factory implementation
    println!("\n5. Custom Factory Implementation:");
    let custom_factory: Arc<dyn ContainerFactory> =
        Arc::new(PrefixedContainerFactory::new("custom"));
    let container = custom_factory.create();
    println!("   Default message type: {}", container.message_type());
    let typed_container = custom_factory.create_with_type("request");
    println!("   Typed message type: {}", typed_container.message_type());

    // 6. Thread-safe usage
    println!("\n6. Thread-safe Usage:");
    let shared_factory: Arc<dyn ContainerFactory> = Arc::new(ArcContainerProvider::new());
    let mut handles = vec![];

    for i in 0..3 {
        let factory_clone = Arc::clone(&shared_factory);
        let handle = std::thread::spawn(move || {
            let container = factory_clone.create_with_type(&format!("thread_{}_message", i));
            println!("   Thread {} created: {}", i, container.message_type());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 7. Factory with full header configuration
    println!("\n7. Full Header Configuration:");
    let factory = DefaultContainerFactory::new();
    let container = factory.create_with_header(
        "source_app",
        "source_session",
        "target_app",
        "target_session",
        "full_config_message",
    );
    println!("   Source: {} / {}", container.source_id(), container.source_sub_id());
    println!("   Target: {} / {}", container.target_id(), container.target_sub_id());
    println!("   Type: {}", container.message_type());

    println!("\n=== Example Complete ===");
}
