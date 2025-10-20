# Rust Container System - 종합 문서
# Rust Container System - Comprehensive Documentation

---

## 목차 (Table of Contents)

1. [프로젝트 개요](#1-프로젝트-개요)
2. [아키텍처](#2-아키텍처)
3. [Core 모듈](#3-core-모듈)
   - 3.1 [Error 처리](#31-error-처리)
   - 3.2 [Value Types](#32-value-types)
   - 3.3 [Value Trait](#33-value-trait)
   - 3.4 [Container](#34-container)
4. [Values 모듈](#4-values-모듈)
   - 4.1 [Primitive Values](#41-primitive-values)
   - 4.2 [String Value](#42-string-value)
   - 4.3 [Bytes Value](#43-bytes-value)
5. [학습 가이드](#5-학습-가이드)
6. [사용 예제](#6-사용-예제)

---

## 1. 프로젝트 개요

### Rust Container System

Rust로 구현된 프로덕션급 고성능 컨테이너 프레임워크입니다.
메시징 시스템과 범용 애플리케이션을 위한 포괄적인 데이터 관리 기능을 제공합니다.

A production-ready, high-performance Rust container framework designed to provide
comprehensive data management capabilities for messaging systems and general-purpose applications.

### 주요 특징 (Features)

- **타입 안전성 (Type Safety)**: 컴파일 타임 타입 체크가 있는 강타입 값 시스템
- **스레드 안전성 (Thread Safety)**: parking_lot을 사용한 내장 스레드 안전 작업
- **메모리 효율성 (Memory Efficiency)**: Arc와 RwLock을 사용한 효율적인 메모리 관리
- **직렬화 (Serialization)**: JSON 및 XML 직렬화 지원
- **성능 (Performance)**: 제로 비용 추상화와 최소 오버헤드

### 학습 목표 (Learning Objectives)

이 라이브러리를 통해 배울 수 있는 것:

1. **모듈 시스템**: Rust의 모듈 구조와 가시성
2. **Trait 시스템**: 다형성과 공통 인터페이스
3. **스마트 포인터**: Arc, Box의 사용법
4. **동시성**: RwLock을 이용한 스레드 안전성
5. **에러 처리**: Result 타입과 ? 연산자
6. **제네릭**: 타입 파라미터와 제약 조건

---

## 2. 아키텍처

### 모듈 구조

컨테이너 시스템은 다음과 같은 모듈로 구성됩니다:

```
rust_container_system/
├── core/                    # 핵심 모듈
│   ├── error.rs            # 에러 타입 정의
│   ├── value_types.rs      # 15가지 값 타입 열거형
│   ├── value.rs            # Value trait 정의
│   ├── container.rs        # ValueContainer 구현
│   └── mod.rs              # Core 모듈 진입점
├── values/                  # 구체적인 값 구현
│   ├── primitive_values.rs # Bool, Int, Long, Double
│   ├── string_value.rs     # UTF-8 문자열
│   ├── bytes_value.rs      # 바이너리 데이터
│   └── mod.rs              # Values 모듈 진입점
└── lib.rs                   # 라이브러리 루트
```

### Re-export 패턴

#### pub use의 장점

```rust
// Re-export로 단순화된 경우:
use rust_container_system::core::{ContainerError, Value};

// 또는 prelude 사용:
use rust_container_system::prelude::*;

// 이렇게 하면 다음처럼 깊은 경로를 쓰지 않아도 됩니다:
// use rust_container_system::core::error::ContainerError;
// use rust_container_system::core::value::Value;
```

#### 설계 원칙

- **API 단순화**: 사용자가 깊은 경로를 알 필요 없음
- **유연성 유지**: 내부 구조 변경 시 사용자 코드 영향 최소화
- **명확성**: 가장 자주 사용되는 타입만 re-export

---

## 3. Core 모듈

Core 모듈은 컨테이너 시스템의 기초를 제공합니다.

### 3.1 Error 처리

#### ContainerError

시스템의 모든 에러를 표현하는 열거형입니다.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("Value not found: {0}")]
    ValueNotFound(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Invalid type conversion from {from} to {to}")]
    InvalidTypeConversion { from: String, to: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
```

#### 에러 처리 패턴

**thiserror 크레이트의 장점:**
- `#[error("...")]`로 에러 메시지 자동 생성
- `#[from]`으로 자동 변환 구현
- Display trait 자동 구현

**사용 예제:**

```rust
fn process_value(name: &str) -> Result<Arc<dyn Value>> {
    container.get_value(name)
        .ok_or_else(|| ContainerError::ValueNotFound(name.to_string()))?;
    // ...
}
```

---

### 3.2 Value Types

#### ValueType Enum

15가지 값 타입을 정의하는 열거형입니다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    Null = 0,        // 빈 값
    Bool = 1,        // Boolean (true/false)
    Char = 2,        // 8비트 문자
    Short = 3,       // 16비트 정수
    Int = 4,         // 32비트 정수
    Float = 5,       // 32비트 부동소수점
    LLong = 6,       // 64비트 정수
    Double = 7,      // 64비트 부동소수점
    Bytes = 8,       // 원시 바이트 배열
    String = 13,     // UTF-8 문자열
    Array = 14,      // 값 배열
    Container = 15,  // 중첩 컨테이너

    // 예약된 타입 (향후 사용)
    UChar = 9,
    UShort = 10,
    UInt = 11,
    ULLong = 12,
}
```

#### 주요 메서드

**1. to_str() - 타입 이름 반환**

```rust
let vtype = ValueType::String;
assert_eq!(vtype.to_str(), "13");  // 숫자 ID 반환
```

**2. from_str() - 문자열에서 변환**

```rust
let vtype = ValueType::from_str("13");
assert_eq!(vtype, Some(ValueType::String));
```

**3. size_bytes() - 타입 크기**

```rust
assert_eq!(ValueType::Int.size_bytes(), 4);
assert_eq!(ValueType::Double.size_bytes(), 8);
```

**4. is_numeric() - 숫자 타입 체크**

```rust
assert!(ValueType::Int.is_numeric());
assert!(!ValueType::String.is_numeric());
```

**5. is_integer() - 정수 타입 체크**

```rust
assert!(ValueType::Int.is_integer());
assert!(!ValueType::Double.is_integer());
```

**6. is_float() - 부동소수점 타입 체크**

```rust
assert!(ValueType::Double.is_float());
assert!(!ValueType::Int.is_float());
```

#### 타입 시스템 설계

**왜 15가지 타입인가?**

1. **다양성**: 다양한 데이터 표현 지원
2. **효율성**: 적절한 크기의 타입 선택 가능
3. **호환성**: 다른 시스템과의 데이터 교환
4. **확장성**: 향후 타입 추가 가능

---

### 3.3 Value Trait

모든 값 타입이 구현해야 하는 공통 인터페이스입니다.

#### Trait 정의

```rust
pub trait Value: Send + Sync {
    // 기본 정보
    fn name(&self) -> &str;
    fn value_type(&self) -> ValueType;
    fn size(&self) -> usize;

    // 타입 변환
    fn to_bool(&self) -> Result<bool>;
    fn to_int(&self) -> Result<i32>;
    fn to_long(&self) -> Result<i64>;
    fn to_float(&self) -> Result<f32>;
    fn to_double(&self) -> Result<f64>;
    fn to_string(&self) -> String;
    fn to_bytes(&self) -> Vec<u8>;

    // 직렬화
    fn to_json(&self) -> Result<String>;
    fn to_xml(&self) -> Result<String>;

    // 유틸리티
    fn clone_value(&self) -> Arc<dyn Value>;
    fn as_any(&self) -> &dyn Any;
}
```

#### BaseValue

범용 값 저장소로, 타입과 바이트 데이터를 직접 저장합니다.

```rust
pub struct BaseValue {
    name: String,
    value_type: ValueType,
    data: Vec<u8>,
}
```

**사용 사례:**
- 타입이 런타임에 결정되는 경우
- 외부 시스템에서 받은 데이터 저장
- 직렬화/역직렬화 중간 단계

---

### 3.4 Container

#### ValueContainer

값들을 저장하고 관리하는 메인 컨테이너입니다.

```rust
pub struct ValueContainer {
    inner: Arc<RwLock<ContainerInner>>,
}

struct ContainerInner {
    source: String,
    sub_source: String,
    target: String,
    sub_target: String,
    message_type: String,
    values: HashMap<String, Arc<dyn Value>>,
}
```

#### 주요 기능

**1. 헤더 관리**

```rust
// 발신자 정보
container.set_source("client", "session_123");

// 수신자 정보
container.set_target("server", "handler");

// 메시지 타입
container.set_message_type("user_data");
```

**2. 값 관리**

```rust
// 값 추가
container.add_value(Arc::new(IntValue::new("id", 123)));

// 값 조회
if let Some(value) = container.get_value("id") {
    println!("ID: {}", value.to_int().unwrap());
}

// 값 제거
container.remove_value("id");

// 모든 값 제거
container.clear_values();
```

**3. 직렬화**

```rust
// JSON 직렬화
let json = container.to_json()?;

// XML 직렬화
let xml = container.to_xml()?;
```

**4. 컨테이너 복사**

```rust
// 깊은 복사 (모든 값 복제)
let cloned = container.copy();
```

#### 스레드 안전성

**Arc + RwLock 패턴:**

- **Arc (Atomic Reference Counting)**: 여러 소유자 간 공유
- **RwLock (Read-Write Lock)**: 동시 읽기, 배타적 쓰기
- **parking_lot**: 표준 라이브러리보다 빠른 성능

**동작 방식:**

```rust
// 읽기: 여러 스레드가 동시에 가능
let inner = self.inner.read();
let value = inner.values.get(name);

// 쓰기: 하나의 스레드만 가능
let mut inner = self.inner.write();
inner.values.insert(name, value);
```

---

## 4. Values 모듈

Value trait의 구체적인 구현을 제공합니다.

### 4.1 Primitive Values

기본 타입 구현: Boolean과 숫자 타입입니다.

#### BoolValue

**설명:**
- true 또는 false 값을 저장
- 크기: 1 바이트
- 사용처: 플래그, 상태 표시

**예제:**

```rust
let flag = BoolValue::new("is_active", true);
assert_eq!(flag.value(), true);
assert_eq!(flag.to_bytes(), vec![1]);
```

#### IntValue (i32)

**설명:**
- 32비트 부호 있는 정수
- 범위: -2,147,483,648 ~ 2,147,483,647
- 크기: 4 바이트

**타입 변환:**

```rust
let value = IntValue::new("count", 100);

// 안전한 확장
assert_eq!(value.to_long().unwrap(), 100i64);
assert_eq!(value.to_double().unwrap(), 100.0);

// 바이트 변환 (Little-Endian)
let bytes = value.to_bytes();
// [100, 0, 0, 0] (little-endian)
```

#### LongValue (i64)

**설명:**
- 64비트 부호 있는 정수
- 범위: -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807
- 크기: 8 바이트
- 사용처: 타임스탬프, 큰 ID, 파일 크기

**타입 변환 (범위 체크):**

```rust
let large = LongValue::new("big", 5_000_000_000);

// 범위 초과 시 에러
assert!(large.to_int().is_err());

// 안전한 범위
let small = LongValue::new("small", 100);
assert_eq!(small.to_int().unwrap(), 100);
```

#### DoubleValue (f64)

**설명:**
- 64비트 배정밀도 부동소수점
- IEEE 754 표준
- 크기: 8 바이트
- 정밀도: 약 15-17 자리 10진수

**특수 값:**

```rust
let nan = DoubleValue::new("invalid", f64::NAN);
let inf = DoubleValue::new("inf", f64::INFINITY);
let neg_zero = DoubleValue::new("nz", -0.0);
```

**부동소수점 비교:**

```rust
// 직접 비교는 위험
let x = 0.1 + 0.2;
let y = 0.3;
assert!(x != y);  // true! 부동소수점 오차

// Epsilon 비교 사용
const EPSILON: f64 = 1e-10;
assert!((x - y).abs() < EPSILON);
```

#### 숫자 타입 선택 가이드

**i32 (IntValue) 사용 시기:**
- 일반적인 정수 값 (-20억 ~ 20억)
- 카운터, 인덱스, 나이, 수량 등
- 메모리 효율이 중요한 경우

**i64 (LongValue) 사용 시기:**
- 큰 범위의 정수 필요 (±900경)
- 타임스탬프 (밀리초 단위)
- 데이터베이스 ID
- 파일 크기, 메모리 주소

**f64 (DoubleValue) 사용 시기:**
- 실수 계산이 필요한 경우
- 과학적 계산 (물리, 화학, 공학)
- 금융 데이터 (단, 정밀도 주의)
- 좌표, GPS 데이터

**사용 주의:**
- 정확한 10진수 계산 필요 시: Decimal 타입 사용 (rust_decimal 크레이트)
- 돈 계산: 센트 단위 정수 또는 Decimal 사용
- 부동소수점 오차 존재: 0.1 + 0.2 ≠ 0.3

#### 타입 변환 (Type Conversion)

**as 연산자:**
- 빠르지만 안전하지 않음
- 오버플로우 체크 없음
- 정밀도 손실 무시

```rust
let x: i32 = 100;
let y: i64 = x as i64;  // 항상 안전 (범위 확장)

let a: i64 = 5_000_000_000;
let b: i32 = a as i32;  // 오버플로우! 정의되지 않은 값
```

**try_into() 메서드:**
- 안전한 변환
- 범위 체크 수행
- Result 반환 (성공/실패)

```rust
let x: i64 = 100;
let y: i32 = x.try_into().unwrap();  // Ok(100)

let a: i64 = 5_000_000_000;
let b: Result<i32, _> = a.try_into();  // Err (범위 초과)
```

#### Little-Endian vs Big-Endian

**Little-Endian (작은 쪽이 먼저):**
- 하위 바이트(LSB)가 낮은 주소에 저장
- x86, x86-64, ARM (대부분) 사용
- 0x12345678 → [78, 56, 34, 12]

**Big-Endian (큰 쪽이 먼저):**
- 상위 바이트(MSB)가 낮은 주소에 저장
- 네트워크 바이트 순서
- 0x12345678 → [12, 34, 56, 78]

**Rust의 바이트 변환:**

```rust
let value: i32 = 0x12345678;

// Little-Endian으로 변환
let le = value.to_le_bytes();  // [78, 56, 34, 12]

// Big-Endian으로 변환
let be = value.to_be_bytes();  // [12, 34, 56, 78]

// Native Endian (플랫폼 기본값)
let ne = value.to_ne_bytes();

// 복원
let restored = i32::from_le_bytes(le);
```

#### IEEE 754 부동소수점 표준

**f64 구조 (64비트):**

```
[부호 1비트][지수 11비트][가수 52비트]
```

**특수 값:**
- **NaN** (Not a Number): 0.0 / 0.0, sqrt(-1.0)
- **+Infinity**: 1.0 / 0.0
- **-Infinity**: -1.0 / 0.0
- **-0.0**: 음수 영 (일부 연산에서 +0.0과 다르게 동작)

**정밀도 한계:**

```rust
let x = 0.1 + 0.2;
let y = 0.3;
assert!(x != y);  // true! 부동소수점 오차
assert!((x - y).abs() < 1e-10);  // 대신 epsilon 비교 사용
```

**안전한 비교:**

```rust
const EPSILON: f64 = 1e-10;

fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
```

---

### 4.2 String Value

UTF-8 인코딩된 문자열을 저장합니다.

#### StringValue

```rust
pub struct StringValue {
    name: String,
    value: String,
}
```

#### String vs &str

**String:**
- **소유권**: 문자열 데이터를 소유
- **할당**: 힙에 할당
- **가변성**: 수정 가능 (push_str, pop 등)
- **크기**: 실행 시간에 변경 가능
- **사용처**: 문자열을 소유하고 수정해야 할 때

**&str:**
- **소유권**: 빌림 (참조)
- **할당**: 스택에 포인터 + 길이 저장, 데이터는 다른 곳에
- **가변성**: 불변 (읽기 전용)
- **크기**: 컴파일 타임에 결정 (문자열 리터럴) 또는 슬라이스
- **사용처**: 문자열을 읽기만 할 때

**변환:**

```rust
let s = String::from("hello");  // String
let slice: &str = &s;           // String → &str (deref)
let owned: String = slice.to_string(); // &str → String (복사)
```

#### UTF-8 인코딩

**특징:**
- 가변 길이 인코딩 (1~4 바이트)
- ASCII와 호환 (1바이트 ASCII 그대로 사용)
- 전 세계 모든 문자 표현 가능

**바이트 길이:**
- ASCII (a-z, 0-9): 1바이트
- 유럽 문자 (é, ñ): 2바이트
- 한글, 한자, 아랍어: 3바이트
- 이모지: 4바이트

**예제:**

```rust
let s = "Hello 안녕 🦀";
// "Hello " = 6 bytes (공백 포함)
// "안녕" = 6 bytes (3 × 2)
// "🦀" = 4 bytes
// 총 = 17 bytes
assert_eq!(s.len(), 17);
assert_eq!(s.chars().count(), 9); // 문자 개수
```

#### 주요 메서드

**1. 생성:**

```rust
// &str로 생성
let val1 = StringValue::new("key1", "value1");

// String으로 생성
let val2 = StringValue::new(
    String::from("key2"),
    String::from("value2")
);
```

**2. 값 조회:**

```rust
let val = StringValue::new("greeting", "Hello, World!");
let text = val.value();
assert_eq!(text, "Hello, World!");
assert_eq!(text.len(), 13);
```

**3. 크기 (바이트):**

```rust
let ascii = StringValue::new("ascii", "Hello");
assert_eq!(ascii.size(), 5);

let korean = StringValue::new("korean", "안녕");
assert_eq!(korean.size(), 6); // 3바이트 × 2글자
```

**4. 바이트 변환:**

```rust
let val = StringValue::new("msg", "ABC");
let bytes = val.to_bytes();
assert_eq!(bytes, vec![65, 66, 67]); // ASCII codes
```

---

### 4.3 Bytes Value

원시 바이너리 데이터를 저장합니다.

#### BytesValue

```rust
pub struct BytesValue {
    name: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}
```

#### Vec<u8>란?

- **u8**: 8비트 부호 없는 정수 (0~255)
- **Vec**: 크기가 가변적인 동적 배열
- **임의의 바이너리 데이터**를 저장하는 표준 방식

#### 사용 사례

**1. 이미지 데이터:**

```rust
// JPEG 이미지 시그니처
let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
let image = BytesValue::new("image", jpeg_header);
```

**2. 파일 내용:**

```rust
use std::fs;

let file_data = fs::read("document.pdf").unwrap();
let file = BytesValue::new("document", file_data);
```

**3. 암호화된 데이터:**

```rust
let plain_text = "secret message".as_bytes();
let encrypted = encrypt(plain_text);
let secure_data = BytesValue::new("encrypted", encrypted);
```

**4. 해시 값:**

```rust
let hash = vec![0xAB, 0xCD, 0xEF, 0x12];
let hash_value = BytesValue::new("sha256", hash);
```

#### Base64 인코딩

바이너리 데이터를 텍스트로 표현하는 인코딩 방식입니다.

**특징:**
- 64개의 ASCII 문자만 사용 (A-Z, a-z, 0-9, +, /)
- 3바이트 → 4문자로 변환 (33% 크기 증가)
- JSON, XML, 이메일 등에서 바이너리 전송 시 사용

**동작 원리:**

1. 입력: 3바이트 (24비트) 단위로 처리
2. 출력: 4개의 6비트 그룹으로 분할
3. 각 6비트를 Base64 알파벳으로 변환

**예제: "Hi"를 인코딩**

```text
입력: "Hi" = [0x48, 0x69] = [01001000, 01101001]

3바이트로 패딩: [01001000, 01101001, 00000000]

6비트 그룹: [010010, 000110, 100100, 000000]
             = [18, 6, 36, 0]

Base64 알파벳:
  18 → S
  6  → G
  36 → k
  0  → A (패딩)

패딩 처리: 마지막 2개를 = 로 교체
최종 결과: "SGk=" (4자)
```

**크기 계산:**

```
원본 n 바이트 → ceil(n / 3) × 4 자
예: 100 bytes → 136 bytes (36% 증가)
```

**JSON 직렬화:**

```rust
let data = BytesValue::new("data", vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
let json = data.to_json().unwrap();
// JSON: "SGVsbG8="
```

#### Serde 커스터마이징

**#[serde(with = "serde_bytes")]**

이 속성은 Vec<u8>의 커스텀 직렬화/역직렬화를 제공합니다.

**왜 필요한가?**
- 바이트 배열의 효율적인 처리
- 특정 포맷에 맞는 직렬화
- 성능 최적화

**구현:**

```rust
mod serde_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bytes.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<u8>::deserialize(deserializer)
    }
}
```

#### &[u8] vs Vec<u8>

**&[u8] (바이트 슬라이스):**
- 빌린 바이트 슬라이스 (읽기 전용)
- 소유권 없음
- 스택에 포인터 + 길이만 저장

**Vec<u8> (바이트 벡터):**
- 소유권을 가진 동적 배열
- 힙에 할당
- 크기 변경 가능

**변환:**

```rust
let array: [u8; 4] = [10, 20, 30, 40];
let slice: &[u8] = &array;           // 배열 → 슬라이스
let vec: Vec<u8> = slice.to_vec();   // 슬라이스 → Vec (복사)
```

---

## 5. 학습 가이드

### 5.1 Rust 기초 개념

#### 소유권 (Ownership)

**규칙:**
1. 각 값은 하나의 소유자를 가짐
2. 소유자가 스코프를 벗어나면 값은 drop됨
3. 한 번에 하나의 소유자만 존재

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1의 소유권이 s2로 이동
// println!("{}", s1);  // 에러! s1은 더 이상 유효하지 않음
```

#### 빌림 (Borrowing)

**불변 참조:**

```rust
let s = String::from("hello");
let len = calculate_length(&s);  // s를 빌림
println!("{}", s);  // s는 여전히 유효
```

**가변 참조:**

```rust
let mut s = String::from("hello");
change(&mut s);  // 가변 빌림

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**규칙:**
- 불변 참조는 여러 개 가능
- 가변 참조는 하나만 가능
- 가변 참조와 불변 참조는 동시에 불가

#### Arc와 RwLock

**Arc (Atomic Reference Counting):**

```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = data.clone();  // 참조 카운트 증가

// 두 변수 모두 같은 데이터를 가리킴
```

**RwLock (Read-Write Lock):**

```rust
use parking_lot::RwLock;
use std::sync::Arc;

let data = Arc::new(RwLock::new(vec![1, 2, 3]));

// 읽기 (여러 스레드 동시 가능)
{
    let read_guard = data.read();
    println!("{:?}", *read_guard);
}

// 쓰기 (배타적 접근)
{
    let mut write_guard = data.write();
    write_guard.push(4);
}
```

### 5.2 Trait 시스템

#### Trait 정의

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

#### Trait 구현

```rust
struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}
```

#### Trait 객체

```rust
// 정적 디스패치
fn print_summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

// 동적 디스패치
fn print_summary_dyn(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

### 5.3 에러 처리

#### Result 타입

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### ? 연산자

```rust
fn read_file(path: &str) -> Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

// 위 코드는 다음과 같음:
fn read_file_expanded(path: &str) -> Result<String> {
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };
    Ok(contents)
}
```

#### 에러 변환

```rust
impl From<std::io::Error> for ContainerError {
    fn from(err: std::io::Error) -> Self {
        ContainerError::IoError(err)
    }
}
```

---

## 6. 사용 예제

### 6.1 기본 사용법

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn main() -> Result<()> {
    // 1. 컨테이너 생성
    let mut container = ValueContainer::new();

    // 2. 헤더 설정
    container.set_source("client_01", "session_123");
    container.set_target("server", "main_handler");
    container.set_message_type("user_data");

    // 3. 값 추가
    container.add_value(Arc::new(IntValue::new("user_id", 12345)));
    container.add_value(Arc::new(StringValue::new("username", "john_doe")));
    container.add_value(Arc::new(DoubleValue::new("balance", 1500.75)));
    container.add_value(Arc::new(BoolValue::new("active", true)));

    // 4. 값 조회
    if let Some(user_id) = container.get_value("user_id") {
        println!("User ID: {}", user_id.to_int()?);
    }

    // 5. JSON 직렬화
    let json = container.to_json()?;
    println!("JSON: {}", json);

    Ok(())
}
```

### 6.2 타입 변환

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn process_numeric_value(value: Arc<dyn Value>) -> Result<()> {
    // 타입에 따라 다르게 처리
    match value.value_type() {
        ValueType::Int => {
            let num = value.to_int()?;
            println!("Integer: {}", num);
        },
        ValueType::LLong => {
            let num = value.to_long()?;
            println!("Long: {}", num);
        },
        ValueType::Double => {
            let num = value.to_double()?;
            println!("Double: {:.2}", num);
        },
        _ => {
            println!("Not a numeric type");
        }
    }

    Ok(())
}
```

### 6.3 컨테이너 직렬화/역직렬화

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn save_and_load() -> Result<()> {
    // 데이터 준비
    let mut container = ValueContainer::new();
    container.set_message_type("data");
    container.add_value(Arc::new(StringValue::new("key", "value")));

    // JSON으로 저장
    let json = container.to_json()?;
    std::fs::write("data.json", json)?;

    // 파일에서 로드
    let json = std::fs::read_to_string("data.json")?;
    println!("Loaded: {}", json);

    Ok(())
}
```

### 6.4 스레드 간 공유

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;
use std::thread;

fn share_container() {
    let container = Arc::new(ValueContainer::new());

    // 여러 스레드에서 읽기
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let container_clone = container.clone();
            thread::spawn(move || {
                println!("Thread {}: {}", i, container_clone.message_type());
            })
        })
        .collect();

    // 모든 스레드 대기
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### 6.5 에러 처리

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn safe_value_access(
    container: &ValueContainer,
    name: &str
) -> Result<i32> {
    // 값 조회
    let value = container.get_value(name)
        .ok_or_else(|| ContainerError::ValueNotFound(name.to_string()))?;

    // 타입 변환
    let num = value.to_int()
        .map_err(|e| ContainerError::TypeMismatch {
            expected: "Int".to_string(),
            actual: format!("{:?}", value.value_type()),
        })?;

    Ok(num)
}

fn main() {
    let container = ValueContainer::new();

    match safe_value_access(&container, "count") {
        Ok(num) => println!("Count: {}", num),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 6.6 파일 업로드 시나리오

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

fn create_file_upload(
    filename: &str,
    mime_type: &str,
    content: Vec<u8>
) -> Result<ValueContainer> {
    let mut upload = ValueContainer::new();
    upload.set_message_type("file_upload");

    // 메타데이터
    upload.add_value(Arc::new(StringValue::new("filename", filename)));
    upload.add_value(Arc::new(StringValue::new("mime_type", mime_type)));
    upload.add_value(Arc::new(IntValue::new("size", content.len() as i32)));

    // 파일 내용 (Base64로 인코딩됨)
    upload.add_value(Arc::new(BytesValue::new("content", content)));

    Ok(upload)
}

fn main() -> Result<()> {
    // 파일 읽기
    let file_content = std::fs::read("document.pdf")?;

    // 업로드 컨테이너 생성
    let upload = create_file_upload(
        "document.pdf",
        "application/pdf",
        file_content
    )?;

    // JSON으로 전송
    let json = upload.to_json()?;
    println!("Upload JSON: {}", json);

    Ok(())
}
```

### 6.7 메시지 교환

```rust
use rust_container_system::prelude::*;
use std::sync::Arc;

// 요청 생성
fn create_request(user_id: i32, action: &str) -> ValueContainer {
    let mut request = ValueContainer::new();
    request.set_message_type("request");
    request.set_source("client", "web_app");
    request.set_target("server", "api");

    request.add_value(Arc::new(IntValue::new("user_id", user_id)));
    request.add_value(Arc::new(StringValue::new("action", action)));
    request.add_value(Arc::new(IntValue::new(
        "timestamp",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32
    )));

    request
}

// 응답 생성
fn create_response(
    request: &ValueContainer,
    success: bool,
    data: Option<String>
) -> ValueContainer {
    let mut response = ValueContainer::new();
    response.set_message_type("response");

    // 헤더 교환 (발신자 ↔ 수신자)
    response.set_source(request.target(), request.sub_target());
    response.set_target(request.source(), request.sub_source());

    response.add_value(Arc::new(BoolValue::new("success", success)));

    if let Some(data) = data {
        response.add_value(Arc::new(StringValue::new("data", data)));
    }

    response
}

fn main() -> Result<()> {
    // 클라이언트: 요청 생성
    let request = create_request(123, "get_profile");
    println!("Request: {}", request.to_json()?);

    // 서버: 응답 생성
    let response = create_response(&request, true, Some("Profile data".to_string()));
    println!("Response: {}", response.to_json()?);

    Ok(())
}
```

---

## 부록 A: 타입 크기 참조표

| 타입 | Rust 타입 | 크기 (bytes) | 범위 |
|------|-----------|--------------|------|
| Null | - | 0 | - |
| Bool | bool | 1 | true/false |
| Char | i8 | 1 | -128 ~ 127 |
| Short | i16 | 2 | -32,768 ~ 32,767 |
| Int | i32 | 4 | -2,147,483,648 ~ 2,147,483,647 |
| Float | f32 | 4 | ±3.4e±38 (7자리) |
| LLong | i64 | 8 | -9.2e+18 ~ 9.2e+18 |
| Double | f64 | 8 | ±1.7e±308 (15자리) |
| Bytes | Vec<u8> | 가변 | 0 ~ 무한대 |
| String | String | 가변 | UTF-8 문자열 |

---

## 부록 B: 에러 코드 참조

| 에러 타입 | 설명 | 발생 상황 |
|----------|------|-----------|
| ValueNotFound | 값을 찾을 수 없음 | get_value()에서 존재하지 않는 키 |
| TypeMismatch | 타입 불일치 | 잘못된 타입으로 변환 시도 |
| InvalidTypeConversion | 변환 불가능 | 범위 초과 또는 호환되지 않는 타입 |
| IoError | I/O 에러 | 파일 읽기/쓰기 실패 |
| JsonError | JSON 에러 | JSON 직렬화/역직렬화 실패 |
| Utf8Error | UTF-8 에러 | 잘못된 UTF-8 바이트 시퀀스 |

---

## 부록 C: 성능 최적화 팁

### 1. Arc vs Box

**Arc 사용:**
- 여러 소유자가 필요한 경우
- 스레드 간 공유가 필요한 경우
- 참조 카운팅 오버헤드 존재

**Box 사용:**
- 단일 소유자만 필요한 경우
- 힙 할당만 필요한 경우
- 오버헤드가 적음

### 2. Clone 최소화

```rust
// 나쁜 예: 불필요한 clone
let value = container.get_value("key").unwrap().clone();

// 좋은 예: 참조 사용
if let Some(value) = container.get_value("key") {
    // value를 참조로 사용
}
```

### 3. 문자열 처리

```rust
// 나쁜 예: 반복적인 할당
let mut s = String::new();
for i in 0..100 {
    s = format!("{}{}", s, i);  // 매번 새로운 할당
}

// 좋은 예: push_str 사용
let mut s = String::with_capacity(200);
for i in 0..100 {
    s.push_str(&i.to_string());
}
```

### 4. 컨테이너 크기 예약

```rust
// 많은 값을 추가할 경우
let mut container = ValueContainer::new();
// HashMap에 예약 기능이 있다면 사용
// (현재 구현에는 없지만 향후 추가 가능)
```

---

## 부록 D: 참고 자료

### 공식 문서
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)

### 관련 크레이트
- [serde](https://serde.rs/) - 직렬화/역직렬화
- [thiserror](https://docs.rs/thiserror/) - 에러 처리
- [parking_lot](https://docs.rs/parking_lot/) - 동기화 primitives
- [base64](https://docs.rs/base64/) - Base64 인코딩

### 학습 리소스
- [Rust Programming Language](https://www.rust-lang.org/)
- [Rustlings](https://github.com/rust-lang/rustlings) - 연습 문제
- [Rust Playground](https://play.rust-lang.org/) - 온라인 실행 환경

---

## 변경 이력

### Version 0.1.0 (2025-01-XX)
- 초기 릴리스
- Core 모듈 구현
- Values 모듈 구현
- 종합 문서 작성

---

**문서 작성일:** 2025년 1월
**작성자:** Claude Code
**라이선스:** MIT (프로젝트에 따름)

이 문서는 Rust Container System의 모든 주석과 문서를 통합하여 작성되었습니다.
초보자부터 중급자까지 학습할 수 있도록 상세한 설명과 예제를 포함하고 있습니다.
