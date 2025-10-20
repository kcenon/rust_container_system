# Rust Container System

> **Languages**: [English](./README.md) | 한국어

메시징 시스템 및 범용 애플리케이션을 위한 포괄적인 데이터 관리 기능을 제공하도록 설계된 프로덕션 준비 완료, 고성능 Rust 컨테이너 프레임워크입니다.

이것은 [container_system](https://github.com/kcenon/container_system) 프로젝트의 Rust 구현으로, Rust의 안전성 보장 및 성능 이점과 함께 동일한 기능을 제공합니다.

## 기능

- **타입 안전성**: 컴파일 타임 검사를 포함한 강력한 타입의 값 시스템
- **스레드 안전성**: `parking_lot` RwLock을 사용하는 내장 스레드 안전 작업
- **메모리 효율성**: `Arc` 및 스마트 포인터를 사용한 효율적인 메모리 관리
- **직렬화**: JSON 및 XML 직렬화 지원
- **성능**: 제로 비용 추상화 및 최소 오버헤드
- **크로스 플랫폼**: Windows, Linux, macOS에서 작동

## 값 타입

컨테이너 시스템은 다음 값 타입을 지원합니다:

| 타입 | 설명 | 크기 |
|------|------|------|
| `Null` | Null/빈 값 | 0 바이트 |
| `Bool` | 부울 true/false | 1 바이트 |
| `Short` | 16비트 부호 있는 정수 | 2 바이트 |
| `UShort` | 16비트 부호 없는 정수 | 2 바이트 |
| `Int` | 32비트 부호 있는 정수 | 4 바이트 |
| `UInt` | 32비트 부호 없는 정수 | 4 바이트 |
| `Long` | 64비트 부호 있는 정수 | 8 바이트 |
| `ULong` | 64비트 부호 없는 정수 | 8 바이트 |
| `LLong` | 64비트 부호 있는 정수 | 8 바이트 |
| `ULLong` | 64비트 부호 없는 정수 | 8 바이트 |
| `Float` | 32비트 부동소수점 | 4 바이트 |
| `Double` | 64비트 부동소수점 | 8 바이트 |
| `Bytes` | 원시 바이트 배열 | 가변 |
| `String` | UTF-8 문자열 | 가변 |
| `Container` | 중첩 컨테이너 | 가변 |

## 빠른 시작

`Cargo.toml`에 추가:

```toml
[dependencies]
rust_container_system = "0.1"
```

### 기본 사용법

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() {
    // 새 컨테이너 생성
    let mut container = ValueContainer::new();
    container.set_source("client_01", "session_123");
    container.set_target("server", "main_handler");
    container.set_message_type("user_data");

    // 값 추가
    container.add_value(Arc::new(IntValue::new("user_id", 12345)));
    container.add_value(Arc::new(StringValue::new("username", "john_doe")));
    container.add_value(Arc::new(DoubleValue::new("balance", 1500.75)));
    container.add_value(Arc::new(BoolValue::new("active", true)));

    // 값 가져오기
    let user_id = container.get_value("user_id").unwrap();
    println!("User ID: {}", user_id.to_int().unwrap());

    // JSON으로 직렬화
    let json = container.to_json().unwrap();
    println!("JSON: {}", json);

    // XML로 직렬화
    let xml = container.to_xml().unwrap();
    println!("XML: {}", xml);
}
```

### 값 작업

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// 다양한 타입의 값 생성
let bool_val = Arc::new(BoolValue::new("is_active", true));
let int_val = Arc::new(IntValue::new("count", 42));
let long_val = Arc::new(LongValue::new("timestamp", 1234567890));
let double_val = Arc::new(DoubleValue::new("price", 99.99));
let string_val = Arc::new(StringValue::new("name", "John Doe"));
let bytes_val = Arc::new(BytesValue::new("data", vec![1, 2, 3, 4]));

// 컨테이너에 추가
let mut container = ValueContainer::new();
container.add_value(bool_val);
container.add_value(int_val);
container.add_value(long_val);
container.add_value(double_val);
container.add_value(string_val);
container.add_value(bytes_val);

// 값 검색 및 사용
if let Some(value) = container.get_value("price") {
    match value.to_double() {
        Ok(price) => println!("Price: ${:.2}", price),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 스레드 안전성

컨테이너는 기본적으로 `Arc<RwLock<...>>`를 사용하여 스레드 안전합니다:

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;
use std::thread;

let mut container = ValueContainer::new();
container.add_value(Arc::new(IntValue::new("counter", 0)));

// 스레드 안전성을 위한 클론
let container_clone = container.clone();

let handle = thread::spawn(move || {
    let value = container_clone.get_value("counter").unwrap();
    println!("Counter: {}", value.to_int().unwrap());
});

handle.join().unwrap();
```

## 프로젝트 구조

```
rust_container_system/
├── src/
│   ├── core/              # 핵심 타입 및 trait
│   │   ├── value_types.rs # 값 타입 enum
│   │   ├── value.rs       # Value trait
│   │   ├── container.rs   # ValueContainer
│   │   ├── error.rs       # 에러 타입
│   │   └── mod.rs
│   ├── values/            # 값 구현
│   │   ├── primitive_values.rs
│   │   ├── string_value.rs
│   │   ├── bytes_value.rs
│   │   └── mod.rs
│   └── lib.rs
├── examples/              # 예제 프로그램
├── tests/                 # 통합 테스트
├── benches/              # 벤치마크
├── Cargo.toml
└── README.md
```

## C++ 버전과의 비교

| 기능 | C++ 버전 | Rust 버전 |
|------|----------|-----------|
| 타입 안전성 | ✓ (C++20) | ✓ (Rust) |
| 스레드 안전성 | 수동 (mutex) | 자동 (Arc+RwLock) |
| 메모리 안전성 | 수동 (smart pointers) | 자동 (ownership) |
| 직렬화 | Binary, JSON, XML | JSON, XML |
| SIMD 지원 | ✓ (AVX2, NEON) | 계획 중 |
| 성능 | 높음 | 높음 |

## 빌드

### 전제 조건

- Rust 1.70 이상
- Cargo

### 빌드 명령어

```bash
# 프로젝트 빌드
cargo build

# 릴리스 최적화와 함께 빌드
cargo build --release

# 테스트 실행
cargo test

# 벤치마크 실행
cargo bench

# 문서 생성
cargo doc --open
```

## 예제

더 많은 예제는 `examples/` 디렉토리를 참조하세요:

```bash
# 기본 예제 실행
cargo run --example basic_container

# 직렬화 예제 실행
cargo run --example serialization

# 스레드 안전성 예제 실행
cargo run --example thread_safety
```

## 성능

Rust 구현은 C++ 버전과 비슷하거나 더 나은 성능을 제공합니다:

- **제로 비용 추상화**: 타입 안전성에 대한 런타임 오버헤드 없음
- **메모리 효율성**: Arc 및 RwLock의 효율적인 사용
- **스레드 안전성**: 가능한 경우 락 없는 읽기

## 기여

기여를 환영합니다! Pull Request를 자유롭게 제출해 주세요.

## 라이선스

이 프로젝트는 BSD 3-Clause License로 라이선스가 부여됩니다 - 자세한 내용은 LICENSE 파일을 참조하세요.

## 감사의 말

- 원본 C++ 구현: [container_system](https://github.com/kcenon/container_system)
- Rust의 훌륭한 생태계로 구축

## 관련 프로젝트

- **messaging_system**: 컨테이너 기능의 주요 사용자
- **network_system**: 컨테이너를 위한 네트워크 전송
- **database_system**: 컨테이너를 위한 영구 저장소

---

Made with ❤️ in Rust
