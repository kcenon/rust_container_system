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

//! Example demonstrating nested containers (ContainerValue)

#![allow(deprecated)]

use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    println!("=== Rust Container System - Nested Containers Example ===\n");

    // Example 1: Simple nested container
    simple_nested_example();

    // Example 2: Complex hierarchical structure
    complex_nested_example();

    // Example 3: Serialization of nested containers
    serialization_example();

    println!("\n=== All nested container examples completed! ===");
}

fn simple_nested_example() {
    println!("1. Simple Nested Container:");

    // Create child values (using Arc<dyn Value> for heterogeneous types)
    let child1: Arc<dyn Value> = Arc::new(IntValue::new("id", 123));
    let child2: Arc<dyn Value> = Arc::new(StringValue::new("name", "Alice"));
    let child3: Arc<dyn Value> = Arc::new(DoubleValue::new("balance", 1500.75));

    // Create container with children
    let user_data = Arc::new(ContainerValue::new(
        "user_data",
        vec![child1, child2, child3],
    ));

    println!(
        "   Created container '{}' with {} children",
        user_data.name(),
        user_data.child_count()
    );

    // Access children
    if let Some(id_value) = user_data.get_child("id", 0) {
        println!("   User ID: {}", id_value.to_int().unwrap());
    }

    if let Some(name_value) = user_data.get_child("name", 0) {
        println!("   User Name: {}", name_value.to_string());
    }

    if let Some(balance_value) = user_data.get_child("balance", 0) {
        println!("   Balance: ${:.2}", balance_value.to_double().unwrap());
    }

    println!();
}

fn complex_nested_example() {
    println!("2. Complex Hierarchical Structure:");

    // Create user profile
    let profile_name: Arc<dyn Value> = Arc::new(StringValue::new("name", "Bob Johnson"));
    let profile_age: Arc<dyn Value> = Arc::new(IntValue::new("age", 35));
    let profile_email: Arc<dyn Value> = Arc::new(StringValue::new("email", "bob@example.com"));
    let user_profile = Arc::new(ContainerValue::new(
        "profile",
        vec![profile_name, profile_age, profile_email],
    ));

    // Create user preferences
    let pref_theme: Arc<dyn Value> = Arc::new(StringValue::new("theme", "dark"));
    let pref_notifications: Arc<dyn Value> = Arc::new(BoolValue::new("notifications", true));
    let pref_language: Arc<dyn Value> = Arc::new(StringValue::new("language", "en"));
    let user_prefs = Arc::new(ContainerValue::new(
        "preferences",
        vec![pref_theme, pref_notifications, pref_language],
    ));

    // Create user statistics
    let stats_login: Arc<dyn Value> =
        Arc::new(LongValue::new("login_count", 150).expect("Value out of range"));
    let stats_messages: Arc<dyn Value> =
        Arc::new(LongValue::new("messages_sent", 1250).expect("Value out of range"));
    let stats_avg_time: Arc<dyn Value> = Arc::new(DoubleValue::new("avg_session_time", 23.5));
    let user_stats = Arc::new(ContainerValue::new(
        "statistics",
        vec![stats_login, stats_messages, stats_avg_time],
    ));

    // Create main user container with nested containers
    let user_container = Arc::new(ContainerValue::new(
        "user",
        vec![user_profile.clone(), user_prefs.clone(), user_stats.clone()],
    ));

    println!("   Created hierarchical user structure:");
    println!(
        "   - Main container: {} ({} children)",
        user_container.name(),
        user_container.child_count()
    );

    for child in user_container.children() {
        if child.is_container() {
            if let Some(nested) = child.as_any().downcast_ref::<ContainerValue>() {
                println!(
                    "     - Nested container '{}': {} children",
                    nested.name(),
                    nested.child_count()
                );
            }
        }
    }

    // Access nested data
    println!("\n   Accessing nested data:");
    if let Some(profile_nested) = user_container.get_child("profile", 0) {
        if let Some(profile_cv) = profile_nested.as_any().downcast_ref::<ContainerValue>() {
            if let Some(name) = profile_cv.get_child("name", 0) {
                println!("     Profile name: {}", name.to_string());
            }
            if let Some(age) = profile_cv.get_child("age", 0) {
                println!("     Profile age: {}", age.to_int().unwrap());
            }
        }
    }

    if let Some(prefs_nested) = user_container.get_child("preferences", 0) {
        if let Some(prefs_cv) = prefs_nested.as_any().downcast_ref::<ContainerValue>() {
            if let Some(theme) = prefs_cv.get_child("theme", 0) {
                println!("     Theme: {}", theme.to_string());
            }
        }
    }

    println!();
}

fn serialization_example() {
    println!("3. Nested Container Serialization:");

    // Create nested structure
    let inner_value1: Arc<dyn Value> = Arc::new(IntValue::new("deep_value", 99));
    let inner_value2: Arc<dyn Value> = Arc::new(StringValue::new("deep_text", "nested data"));
    let inner_container = Arc::new(ContainerValue::new(
        "inner",
        vec![inner_value1, inner_value2],
    ));

    let outer_value: Arc<dyn Value> = Arc::new(StringValue::new("description", "outer level"));
    let outer_container = ContainerValue::new("outer", vec![inner_container.clone(), outer_value]);

    // Serialize to JSON
    println!("   JSON Serialization:");
    match outer_container.to_json() {
        Ok(json) => {
            println!(
                "   {}",
                json.lines().take(10).collect::<Vec<_>>().join("\n   ")
            );
            println!("   ... (truncated for display)");
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Serialize to XML
    println!("\n   XML Serialization:");
    match outer_container.to_xml() {
        Ok(xml) => {
            println!(
                "   {}",
                xml.lines().take(10).collect::<Vec<_>>().join("\n   ")
            );
            println!("   ... (truncated for display)");
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Size comparison
    let json_size = outer_container.to_json().unwrap().len();
    let xml_size = outer_container.to_xml().unwrap().len();
    println!("\n   Size comparison:");
    println!("   - JSON: {} bytes", json_size);
    println!("   - XML:  {} bytes", xml_size);
    println!();
}
