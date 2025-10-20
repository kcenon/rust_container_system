# Rust Container System - 개선 계획

> **Languages**: [English](./IMPROVEMENTS.md) | 한국어

## 개요

이 문서는 코드 분석을 바탕으로 식별된 Rust Container System의 약점과 제안된 개선사항을 설명합니다.

## 식별된 문제점

### 1. 직렬화 정확도 손실

**문제**: 직렬화가 모든 값을 `Value::to_string`을 통해 평면화하여 바이너리/타입 데이터가 내보낸 페이로드에서 정확도를 잃습니다.

**위치**: `src/core/container.rs:274`

**현재 구현**:
```rust
// 타입 정보와 바이너리 데이터 정확도 손실
fn to_json(&self) -> String {
    // 각 값이 문자열로 변환됨
    value.to_string()
}
```

**영향**:
- 바이너리 데이터가 읽을 수 없는 문자열이 됨
- 타입 정보 손실 (숫자가 문자열이 됨)
- 직렬화/역직렬화 왕복 불가

**제안된 해결책**:

```rust
// TODO: 타입 정보를 보존하는 적절한 직렬화 구현
// Value enum에 variant별 직렬화 추가
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Int(v) => serializer.serialize_i32(*v),
            Value::Long(v) => serializer.serialize_i64(*v),
            Value::String(v) => serializer.serialize_str(v),
            Value::Bytes(v) => serializer.serialize_bytes(v),
            // ... 각 variant를 적절히 처리
        }
    }
}
```

**우선순위**: 높음
**예상 작업량**: 중간

### 2. 이차 시간 복잡도 제거 성능

**문제**: 값 제거 시 `value_map` 전체를 재구성하여 빈번한 변경 시 이차 시간이 될 수 있습니다.

**위치**: `src/core/container.rs:187`

**현재 구현**:
```rust
pub fn remove_value(&mut self, key: &str) -> Option<Arc<dyn Value>> {
    let mut inner = self.inner.write();
    // value_map 전체 재구성
    inner.values.retain(|v| v.key() != key);
    inner.rebuild_value_map(); // 매 제거마다 O(n) 작업
}
```

**영향**:
- O(n) 제거 작업
- 빈번한 제거 시 낮은 성능
- 불필요한 할당

**제안된 해결책**:

```rust
// TODO: 전체 재구성을 피하도록 값 제거 최적화
pub fn remove_value(&mut self, key: &str) -> Option<Arc<dyn Value>> {
    let mut inner = self.inner.write();

    // 한 번의 패스로 찾기 및 제거
    if let Some(pos) = inner.values.iter().position(|v| v.key() == key) {
        let removed = inner.values.swap_remove(pos);

        // 전체 재구성 대신 점진적 업데이트
        inner.value_map.remove(key);

        // swap_remove를 사용했고 교체된 요소를 수정해야 하는 경우에만 재구성
        if pos < inner.values.len() {
            let swapped_key = inner.values[pos].key();
            inner.value_map.insert(swapped_key.to_string(), pos);
        }

        Some(removed)
    } else {
        None
    }
}
```

**대안**: 삽입 순서를 유지하고 O(1) 제거를 허용하는 `IndexMap` 사용:

```rust
// Cargo.toml에 추가:
// indexmap = "2.0"

use indexmap::IndexMap;

struct ContainerInner {
    // Vec + HashMap을 IndexMap으로 교체
    values: IndexMap<String, Arc<dyn Value>>,
    // ...
}
```

**우선순위**: 중간
**예상 작업량**: 중간

## 추가 개선사항

### 3. 값 접근 최적화

**제안**: 락 경합을 줄이기 위한 대량 작업 추가:

```rust
// TODO: 락 경합을 줄이기 위한 대량 작업 추가
pub fn get_values(&self, keys: &[&str]) -> HashMap<String, Arc<dyn Value>> {
    let inner = self.inner.read();
    keys.iter()
        .filter_map(|key| {
            inner.value_map.get(*key)
                .and_then(|&idx| inner.values.get(idx))
                .map(|v| (key.to_string(), Arc::clone(v)))
        })
        .collect()
}

pub fn set_values(&mut self, values: Vec<Arc<dyn Value>>) {
    let mut inner = self.inner.write();
    for value in values {
        inner.values.push(value);
    }
    inner.rebuild_value_map(); // 모든 값에 대해 한 번만
}
```

**우선순위**: 낮음
**예상 작업량**: 소

### 4. 직렬화 형식 버전 관리

**제안**: 직렬화된 컨테이너에 버전 정보 추가:

```rust
// TODO: 직렬화 형식에 버전 관리 추가
#[derive(Serialize, Deserialize)]
struct SerializedContainer {
    version: u32,
    source: String,
    target: String,
    timestamp: String,
    values: Vec<SerializedValue>,
}
```

**우선순위**: 낮음
**예상 작업량**: 소

## 테스트 요구사항

### 필요한 새 테스트:

1. **직렬화 왕복 테스트**:
   ```rust
   #[test]
   fn test_binary_serialization_roundtrip() {
       let mut container = ValueContainer::new();
       let binary_data = vec![0u8, 1, 2, 255];
       container.add_value(Arc::new(BytesValue::new("data", binary_data.clone())));

       let json = container.to_json();
       let restored = ValueContainer::from_json(&json);

       let value = restored.get_value("data").unwrap();
       assert_eq!(value.as_bytes(), Some(&binary_data[..]));
   }
   ```

2. **성능 테스트**:
   ```rust
   #[test]
   fn test_remove_performance() {
       let mut container = ValueContainer::new();

       // 10000개 값 추가
       for i in 0..10000 {
           container.add_value(Arc::new(IntValue::new(&format!("key_{}", i), i)));
       }

       // 격 값마다 제거 - 이차 시간이 아니어야 함
       let start = Instant::now();
       for i in (0..10000).step_by(2) {
           container.remove_value(&format!("key_{}", i));
       }
       let elapsed = start.elapsed();

       // 합리적인 시간 내에 완료되어야 함
       assert!(elapsed < Duration::from_secs(1));
   }
   ```

3. **동시성 테스트**:
   ```rust
   #[test]
   fn test_concurrent_modifications() {
       let container = Arc::new(RwLock::new(ValueContainer::new()));

       // 동시 작업을 수행하는 여러 스레드 생성
       let handles: Vec<_> = (0..10).map(|i| {
           let container = Arc::clone(&container);
           thread::spawn(move || {
               for j in 0..100 {
                   let mut c = container.write();
                   c.add_value(Arc::new(IntValue::new(&format!("t{}_k{}", i, j), j)));
               }
           })
       }).collect();

       for handle in handles {
           handle.join().unwrap();
       }

       let container = container.read();
       assert_eq!(container.value_count(), 1000);
   }
   ```

## 구현 로드맵

### 1단계: 중요 수정 (스프린트 1)
- [ ] 타입 보존 직렬화 구현
- [ ] 직렬화 왕복 테스트 추가
- [ ] 새 직렬화 형식으로 문서 업데이트

### 2단계: 성능 최적화 (스프린트 2)
- [ ] 제거 작업 최적화 (IndexMap 고려)
- [ ] 성능 벤치마크 추가
- [ ] 변경 중 메모리 사용량 프로파일링

### 3단계: API 개선 (스프린트 3)
- [ ] 대량 작업 메서드 추가
- [ ] 직렬화 버전 관리 추가
- [ ] 동시성 테스트 개선

## Breaking Changes

⚠️ **주의**: 직렬화 수정은 기존 JSON/XML 형식을 깨뜨립니다.

**마이그레이션 경로**:
1. 직렬화된 출력에 버전 필드 추가
2. 전환 기간 동안 이전 형식 읽기 지원
3. 다음 메이저 버전에서 이전 형식 deprecated
4. CHANGELOG에 마이그레이션 문서화

## 참고자료

- 코드 분석: Container System Review 2025-10-16
- 관련 이슈:
  - 직렬화 정확도 (#TODO)
  - 제거 성능 (#TODO)

---

*개선 계획 버전 1.0*
*최종 업데이트: 2025-10-17*
