// BSD 3-Clause License
//
// Copyright (c) 2021-2025, 🍀☀🌕🌥 🌊
// See LICENSE file for full license text.

//! Concurrency Example
//!
//! This example demonstrates thread-safe usage of the container system,
//! including sharing containers across threads and concurrent access patterns.

use rust_container_system::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Concurrency Examples ===\n");

    // Example 1: Sharing containers across threads
    shared_container_example();

    // Example 2: Producer-consumer pattern
    producer_consumer_example();

    // Example 3: Read-heavy workload
    read_heavy_example();

    println!("\n=== All concurrency examples completed! ===");
}

/// Demonstrates sharing a container across multiple threads
fn shared_container_example() {
    println!("--- Shared Container Across Threads ---");

    // Create a container with initial data
    let mut container = ValueContainer::new();
    container.set_source("main_thread", "example");
    container.set_message_type("shared_data");
    container
        .add_value(Arc::new(IntValue::new("counter", 0)))
        .unwrap();

    // Wrap in Arc for thread-safe sharing
    // Note: ValueContainer uses Arc<RwLock<>> internally
    let shared = Arc::new(container);

    // Spawn multiple reader threads
    let mut handles = vec![];

    for i in 0..3 {
        let container_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Simulate work
            thread::sleep(Duration::from_millis(10 * i as u64));

            // Read from container (thread-safe)
            if let Some(value) = container_clone.get_value("counter") {
                println!(
                    "  Thread {} read counter: {}",
                    i,
                    value.to_int().unwrap_or(-1)
                );
            }

            // Access metadata (thread-safe)
            println!(
                "  Thread {} sees source: {}",
                i,
                container_clone.source_id()
            );
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!();
}

/// Demonstrates producer-consumer pattern with containers
fn producer_consumer_example() {
    println!("--- Producer-Consumer Pattern ---");

    use std::sync::mpsc;

    // Channel for passing containers between threads
    let (tx, rx) = mpsc::channel::<ValueContainer>();

    // Producer thread: creates and sends containers
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let mut container = ValueContainer::new();
            container.set_source("producer", &format!("batch_{}", i));
            container.set_message_type("data_packet");

            // Add values
            container
                .add_value(Arc::new(IntValue::new("sequence", i)))
                .unwrap();
            container
                .add_value(Arc::new(StringValue::new("data", format!("payload_{}", i))))
                .unwrap();
            container
                .add_value(Arc::new(DoubleValue::new("timestamp", i as f64 * 1.5)))
                .unwrap();

            println!("  Producer: sent packet {}", i);
            tx.send(container).unwrap();

            thread::sleep(Duration::from_millis(20));
        }
    });

    // Consumer thread: receives and processes containers
    let consumer = thread::spawn(move || {
        let mut count = 0;
        while let Ok(container) = rx.recv_timeout(Duration::from_millis(200)) {
            let seq = container
                .get_value("sequence")
                .map(|v| v.to_int().unwrap_or(-1))
                .unwrap_or(-1);

            let data = container
                .get_value("data")
                .map(|v| v.to_string())
                .unwrap_or_default();

            println!("  Consumer: received packet {} with data '{}'", seq, data);
            count += 1;
        }
        println!("  Consumer: processed {} packets total", count);
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    println!();
}

/// Demonstrates read-heavy concurrent access
fn read_heavy_example() {
    println!("--- Read-Heavy Workload ---");

    // Create container with test data
    let mut container = ValueContainer::new();
    container.set_message_type("read_test");

    for i in 0..100 {
        container
            .add_value(Arc::new(IntValue::new(format!("value_{}", i), i)))
            .unwrap();
    }

    let shared = Arc::new(container);
    let num_readers = 4;
    let reads_per_thread = 1000;

    // Spawn multiple reader threads
    let start = std::time::Instant::now();
    let handles: Vec<_> = (0..num_readers)
        .map(|thread_id| {
            let container = Arc::clone(&shared);
            thread::spawn(move || {
                let mut sum = 0i64;
                for i in 0..reads_per_thread {
                    let key = format!("value_{}", i % 100);
                    if let Some(value) = container.get_value(&key) {
                        sum += value.to_long().unwrap_or(0);
                    }
                }
                println!(
                    "  Reader {} completed {} reads, sum = {}",
                    thread_id, reads_per_thread, sum
                );
                sum
            })
        })
        .collect();

    // Collect results
    let total: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    let elapsed = start.elapsed();
    let total_reads = num_readers * reads_per_thread;

    println!(
        "  Total: {} reads in {:?} ({:.0} reads/sec)",
        total_reads,
        elapsed,
        total_reads as f64 / elapsed.as_secs_f64()
    );
    println!("  Combined sum: {}", total);

    println!();
}
