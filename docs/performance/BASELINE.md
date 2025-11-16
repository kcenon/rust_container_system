# Performance Baseline

**Version:** 0.1.0
**Date:** 2025-11-16
**Purpose:** Document performance baselines and quality metrics to enable regression detection

## 1. System Information

### 1.1 Hardware Platform

| Component | Specification |
|-----------|--------------|
| **Platform** | macOS (Darwin 25.2.0) |
| **CPU** | Apple M1 (ARM64) |
| **Architecture** | arm64 (RELEASE_ARM64_T8103) |
| **Kernel** | Darwin Kernel Version 25.2.0 |

### 1.2 Software Environment

| Component | Version |
|-----------|---------|
| **Rust** | 1.90.0 (1159e78c4 2025-09-14) |
| **Build Profile** | Release (--release) |
| **Optimization** | Full optimizations enabled |

### 1.3 Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| serde | 1.0.228 | Serialization framework |
| serde_json | 1.0.145 | JSON serialization |
| quick-xml | 0.31.0 | XML serialization |
| parking_lot | 0.12.5 | High-performance synchronization |
| base64 | 0.22.1 | Binary encoding |

## 2. Performance Metrics (Criterion Benchmarks)

### 2.1 Value Creation Performance

Time to create individual value objects:

| Value Type | Mean Time | Throughput | Range |
|------------|-----------|------------|-------|
| **Bool** | 19.11 ns | 52.34 Melem/s | 19.06 - 19.16 ns |
| **Int** | 18.44 ns | 54.23 Melem/s | 18.39 - 18.50 ns |
| **Long** | 19.40 ns | 51.54 Melem/s | 19.05 - 20.09 ns |
| **Double** | 18.80 ns | 53.19 Melem/s | 18.48 - 19.38 ns |
| **String** | 39.00 ns | 25.64 Melem/s | 38.31 - 40.28 ns |
| **Bytes** | 36.93 ns | 27.08 Melem/s | 36.64 - 37.28 ns |

**Analysis:**
- Primitive types (bool, int, long, double): ~18-20 ns
- Heap-allocated types (string, bytes): ~37-40 ns (~2x slower due to allocation)
- Overall: 50+ million operations/second for primitives

### 2.2 Container Operations

#### Adding Values

| Container Size | Mean Time | Throughput | Per-Value Time |
|----------------|-----------|------------|----------------|
| **10 values** | 1.76 µs | 5.67 Melem/s | 176 ns/value |
| **100 values** | 15.68 µs | 6.38 Melem/s | 157 ns/value |
| **1000 values** | 183.02 µs | 5.46 Melem/s | 183 ns/value |

**Analysis:**
- Amortized cost: ~160-180 ns per value
- Scales linearly with container size
- HashMap insertion + locking overhead

#### Getting Values

| Container Size | Position | Mean Time | Notes |
|----------------|----------|-----------|-------|
| **10 values** | First | 21.63 ns | HashMap lookup |
| **10 values** | Middle | 48.06 ns | +Vec iteration |
| **10 values** | Last | 48.36 ns | +Vec iteration |
| **100 values** | First | 20.99 ns | HashMap lookup |
| **100 values** | Middle | 49.42 ns | +Vec iteration |
| **100 values** | Last | 49.55 ns | +Vec iteration |
| **1000 values** | First | 20.72 ns | HashMap lookup |
| **1000 values** | Middle | 52.62 ns | +Vec iteration |
| **1000 values** | Last | 52.33 ns | +Vec iteration |

**Analysis:**
- HashMap lookup: ~20-22 ns (O(1), size-independent)
- Vec index access: +28-32 ns overhead
- Scales with position within Vec, not container size

### 2.3 Serialization Performance

#### JSON Serialization

| Value Count | Mean Time | Throughput | Per-Value Time |
|-------------|-----------|------------|----------------|
| **10 values** | 36.70 µs | 272.46 Kelem/s | 3.67 µs/value |
| **50 values** | 94.99 µs | 526.38 Kelem/s | 1.90 µs/value |
| **100 values** | 179.10 µs | 558.33 Kelem/s | 1.79 µs/value |

**Analysis:**
- Fixed overhead: ~18 µs (container setup)
- Amortized cost: ~1.6 µs per value
- Throughput improves with larger containers

#### XML Serialization

| Value Count | Mean Time | Throughput | Per-Value Time |
|-------------|-----------|------------|----------------|
| **10 values** | 14.00 µs | 714.25 Kelem/s | 1.40 µs/value |
| **50 values** | 39.65 µs | 1.26 Melem/s | 793 ns/value |
| **100 values** | 55.98 µs | 1.79 Melem/s | 560 ns/value |

**Analysis:**
- **XML is 2-3x faster than JSON** (quick-xml efficiency)
- Fixed overhead: ~6 µs
- Amortized cost: ~500 ns per value

### 2.4 Container Cloning

| Container Size | Mean Time | Throughput | Notes |
|----------------|-----------|------------|-------|
| **10 values** | 9.91 ns | 1.01 Gelem/s | Arc clone |
| **100 values** | 9.91 ns | 10.09 Gelem/s | Arc clone |
| **1000 values** | 9.91 ns | 100.95 Gelem/s | Arc clone |

**Analysis:**
- **O(1) cloning** via Arc reference counting
- No data duplication (shallow clone)
- Throughput scales with element count (misleading metric)

### 2.5 Value Type Conversions

| Conversion | Mean Time | Throughput | Notes |
|------------|-----------|------------|-------|
| **int → long** | 3.55 ns | 281.47 Melem/s | Simple cast |
| **int → double** | 3.87 ns | 258.60 Melem/s | Simple cast |
| **int → string** | 19.18 ns | 52.15 Melem/s | Allocation |
| **string → bytes** | 27.08 ns | 36.93 Melem/s | UTF-8 copy |

**Analysis:**
- Primitive conversions: <4 ns (CPU instruction-level)
- String conversion: ~20 ns (allocation overhead)
- UTF-8 operations: ~27 ns (memory copy)

## 3. Memory Metrics

### 3.1 Type Sizes (Estimated)

```rust
// Value trait object:
Box<dyn Value> = 16 bytes (fat pointer: 8 bytes ptr + 8 bytes vtable)

// Actual value sizes:
BoolValue   ≈ 24 bytes (name: String + value: bool + padding)
IntValue    ≈ 32 bytes (name: String + value: i32 + padding)
StringValue ≈ 48 bytes (name: String + value: String)
```

### 3.2 Container Overhead

```rust
ValueContainer {
    inner: Arc<RwLock<ContainerInner>>  // 8 bytes (Arc pointer)
}

ContainerInner {
    header: HashMap<String, String>,    // ~48 bytes empty
    values: HashMap<String, Vec<Box<dyn Value>>>,  // ~48 bytes empty
}

Estimated empty container: ~104 bytes
```

### 3.3 Scalability Estimates

| Container Size | Estimated Memory | Per-Value Overhead |
|----------------|------------------|-------------------|
| **Empty** | ~100 bytes | - |
| **10 values** | ~600 bytes | ~50 bytes/value |
| **100 values** | ~5 KB | ~48 bytes/value |
| **1000 values** | ~48 KB | ~46 bytes/value |

**Note:** Actual measurements require memory profiling tools (e.g., `heaptrack`, `valgrind`)

## 4. Quality Metrics

### 4.1 Test Coverage

| Metric | Value | Source |
|--------|-------|--------|
| **Total Tests** | 44 | `cargo test` |
| **Pass Rate** | 100% | All tests passing |
| **Test Categories** | 5 | Unit, integration, property, interop, examples |
| **Code Coverage** | TBD | Requires `cargo tarpaulin` |

### 4.2 Static Analysis

| Tool | Result | Notes |
|------|--------|-------|
| **cargo check** | ✅ Pass | No compilation errors |
| **cargo clippy** | ⚠️ 3 warnings | Deprecated function warnings |
| **rustfmt** | ✅ Clean | Code formatted |
| **cargo build --release** | ✅ Success | Optimized build |

### 4.3 Security Audit

| Metric | Value | Notes |
|--------|-------|-------|
| **unsafe blocks** | 0 | 100% safe Rust |
| **cargo audit** | TBD | Requires installation |
| **Dependencies** | 20+ direct | Standard ecosystem crates |

## 5. Regression Detection Thresholds

### 5.1 Performance Thresholds

| Operation | Baseline | Warning | Critical |
|-----------|----------|---------|----------|
| **Value creation (primitives)** | 19 ns | < 28 ns (1.5x) | < 38 ns (2x) |
| **Value creation (strings)** | 39 ns | < 59 ns (1.5x) | < 78 ns (2x) |
| **Container add (per value)** | 170 ns | < 255 ns (1.5x) | < 340 ns (2x) |
| **HashMap lookup** | 21 ns | < 32 ns (1.5x) | < 42 ns (2x) |
| **JSON serialization (per value)** | 1.8 µs | < 2.7 µs (1.5x) | < 3.6 µs (2x) |
| **XML serialization (per value)** | 700 ns | < 1.05 µs (1.5x) | < 1.4 µs (2x) |

### 5.2 Memory Thresholds

| Metric | Baseline | Warning | Critical |
|--------|----------|---------|----------|
| **Empty container** | ~100 bytes | < 150 bytes (1.5x) | < 200 bytes (2x) |
| **Per-value overhead** | ~48 bytes | < 72 bytes (1.5x) | < 96 bytes (2x) |

### 5.3 Quality Thresholds

| Metric | Minimum | Target |
|--------|---------|--------|
| **Test pass rate** | 100% | 100% |
| **Code coverage** | 70% | 85% |
| **Clippy warnings** | < 5 | 0 |
| **Security vulnerabilities** | 0 critical/high | 0 |

## 6. Comparison with C++ Implementation

### 6.1 Performance Comparison

| Operation | C++ (container_system) | Rust (this) | Ratio |
|-----------|------------------------|-------------|-------|
| **Value creation** | ~15-20 ns | ~19-39 ns | 1.0-2.0x slower |
| **Container operations** | Similar | Similar | ~1x |
| **Thread safety** | Lock-free (some ops) | RwLock | Varies |
| **Serialization** | Wire protocol only | JSON/XML/Wire | Different |

### 6.2 Safety Comparison

| Aspect | C++ | Rust | Winner |
|--------|-----|------|--------|
| **Memory safety** | Manual (RAII) | Automatic (ownership) | Rust |
| **Thread safety** | Runtime checks | Compile-time | Rust |
| **Type safety** | Runtime (virtual) | Compile-time (trait) | Rust |
| **RAII score** | 20/20 (A+) | Built-in | Rust |

## 7. Benchmark Methodology

### 7.1 Criterion Configuration

```rust
criterion = { version = "0.5", features = ["html_reports"] }
```

**Settings:**
- Warm-up time: 3 seconds
- Measurement time: 5 seconds
- Sample size: 100 iterations
- Confidence level: 95%

### 7.2 Test Data

All benchmarks use:
- **String values:** "test_value" (10 bytes)
- **Numeric values:** Integer range 0-1000
- **Container sizes:** 10, 50, 100, 1000 values

### 7.3 Reproducibility

To reproduce these benchmarks:

```bash
cd rust_container_system
cargo clean
cargo bench --bench container_benchmarks > baseline_results.txt
```

## 8. Known Limitations

### 8.1 Current Implementation

1. **Remove Operations:** O(n) complexity due to value_map rebuild (to be fixed with IndexMap)
2. **Serialization Fidelity:** Type information lost in JSON/XML (to be improved)
3. **Wire Protocol:** Nested structures not fully deserialized (TODO items exist)

### 8.2 Measurement Gaps

1. **Memory profiling:** Not yet measured (requires tooling)
2. **Concurrent performance:** Single-threaded benchmarks only
3. **Large-scale testing:** Max 1000 values tested

## 9. Future Work

### 9.1 Planned Improvements

1. **IndexMap migration:** Expected 10x improvement in remove operations
2. **Type-preserving serialization:** Enable round-trip guarantees
3. **Wire protocol completion:** Full nested structure support
4. **Concurrent benchmarks:** Multi-threaded read/write tests

### 9.2 Monitoring Recommendations

1. **Run benchmarks:** Before each release
2. **Track trends:** Plot performance over versions
3. **CI integration:** Automated regression detection
4. **Memory profiling:** Periodic large-scale testing

---

**Document Version:** 1.0
**Last Updated:** 2025-11-16
**Maintainer:** Development Team
**Review Date:** 2025-12-16 (1 month)
