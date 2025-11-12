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

//! Criterion benchmarks for rust_container_system

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use rust_container_system::prelude::*;
use std::sync::Arc;

// ============================================================================
// Value Creation Benchmarks
// ============================================================================

fn bench_value_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("value_creation");
    group.throughput(Throughput::Elements(1));

    group.bench_function("bool", |b| {
        b.iter(|| {
            let value = BoolValue::new(black_box("test"), black_box(true));
            black_box(value)
        });
    });

    group.bench_function("int", |b| {
        b.iter(|| {
            let value = IntValue::new(black_box("test"), black_box(42));
            black_box(value)
        });
    });

    group.bench_function("long", |b| {
        b.iter(|| {
            let value = LongValue::new(black_box("test"), black_box(123456789i64));
            black_box(value)
        });
    });

    group.bench_function("double", |b| {
        b.iter(|| {
            let value = DoubleValue::new(black_box("test"), black_box(std::f64::consts::PI));
            black_box(value)
        });
    });

    group.bench_function("string", |b| {
        b.iter(|| {
            let value = StringValue::new(black_box("test"), black_box("Hello, World!"));
            black_box(value)
        });
    });

    group.bench_function("bytes", |b| {
        let data = vec![1u8, 2, 3, 4, 5];
        b.iter(|| {
            let value = BytesValue::new(black_box("test"), black_box(data.clone()));
            black_box(value)
        });
    });

    group.finish();
}

// ============================================================================
// Container Operations Benchmarks
// ============================================================================

fn bench_container_add_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("container_add_values");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut container = ValueContainer::new();
                for i in 0..size {
                    let value = Arc::new(IntValue::new(format!("key_{}", i), i));
                    container.add_value(value).unwrap();
                }
                black_box(container)
            });
        });
    }

    group.finish();
}

fn bench_container_get_value(c: &mut Criterion) {
    let mut group = c.benchmark_group("container_get_value");

    // Pre-populate containers with different sizes
    let sizes = [10, 100, 1000];

    for &size in sizes.iter() {
        let mut container = ValueContainer::new();
        for i in 0..size {
            container
                .add_value(Arc::new(IntValue::new(format!("key_{}", i), i)))
                .unwrap();
        }

        group.bench_with_input(
            BenchmarkId::new("first", size),
            &container,
            |b, container| {
                b.iter(|| {
                    let value = container.get_value(black_box("key_0"));
                    black_box(value)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("middle", size),
            &container,
            |b, container| {
                b.iter(|| {
                    let value = container.get_value(black_box(&format!("key_{}", size / 2)));
                    black_box(value)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("last", size),
            &container,
            |b, container| {
                b.iter(|| {
                    let value = container.get_value(black_box(&format!("key_{}", size - 1)));
                    black_box(value)
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Serialization Benchmarks
// ============================================================================

fn bench_json_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_serialization");

    for size in [10, 50, 100].iter() {
        let mut container = ValueContainer::new();
        container.set_source("client", "123");
        container.set_target("server", "main");
        container.set_message_type("test_message");

        for i in 0..*size {
            container
                .add_value(Arc::new(IntValue::new(format!("int_{}", i), i)))
                .unwrap();
            container
                .add_value(Arc::new(StringValue::new(
                    format!("str_{}", i),
                    format!("value_{}", i),
                )))
                .unwrap();
        }

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), &container, |b, container| {
            b.iter(|| {
                let json = container.to_json().unwrap();
                black_box(json)
            });
        });
    }

    group.finish();
}

fn bench_xml_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_serialization");

    for size in [10, 50, 100].iter() {
        let mut container = ValueContainer::new();
        container.set_source("client", "123");
        container.set_target("server", "main");
        container.set_message_type("test_message");

        for i in 0..*size {
            container
                .add_value(Arc::new(IntValue::new(format!("int_{}", i), i)))
                .unwrap();
        }

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), &container, |b, container| {
            b.iter(|| {
                let xml = container.to_xml().unwrap();
                black_box(xml)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Clone Benchmarks
// ============================================================================

fn bench_container_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("container_clone");

    for size in [10, 100, 1000].iter() {
        let mut container = ValueContainer::new();
        for i in 0..*size {
            container
                .add_value(Arc::new(IntValue::new(format!("key_{}", i), i)))
                .unwrap();
        }

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), &container, |b, container| {
            b.iter(|| {
                let cloned = container.clone();
                black_box(cloned)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Value Type Conversion Benchmarks
// ============================================================================

fn bench_value_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("value_conversions");
    group.throughput(Throughput::Elements(1));

    let int_val = IntValue::new("test", 42);

    group.bench_function("int_to_long", |b| {
        b.iter(|| {
            let result = int_val.to_long();
            black_box(result)
        });
    });

    group.bench_function("int_to_double", |b| {
        b.iter(|| {
            let result = int_val.to_double();
            black_box(result)
        });
    });

    group.bench_function("int_to_string", |b| {
        b.iter(|| {
            let result = int_val.to_string();
            black_box(result)
        });
    });

    let string_val = StringValue::new("test", "Hello, World!");

    group.bench_function("string_to_bytes", |b| {
        b.iter(|| {
            let result = string_val.to_bytes();
            black_box(result)
        });
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_value_creation,
    bench_container_add_values,
    bench_container_get_value,
    bench_json_serialization,
    bench_xml_serialization,
    bench_container_clone,
    bench_value_conversions
);

criterion_main!(benches);
