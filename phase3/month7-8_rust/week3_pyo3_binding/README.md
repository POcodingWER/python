# Week 3: PyO3 Python 바인딩 🦀🐍

> **목표**: Rust 코드를 Python에서 호출할 수 있게 만들기

---

## 🎯 프로젝트 개요

### **Rust + Python 하이브리드 시스템**

```
┌─────────────┐      PyO3      ┌─────────────┐
│   Python    │ ←──────────→   │    Rust     │
│  (프론트)   │   바인딩       │  (백엔드)   │
└─────────────┘                └─────────────┘
    빠른 개발                      고성능 실행
    풍부한 라이브러리               메모리 안전성
```

---

## 📊 성능 결과

### **1. 벡터 내적 (100만 차원)**

```
Rust (PyO3):   28.43ms  ⚡ 1.2배 빠름
Python (순수): 35.35ms  기준
```

### **2. Linear Regression (100만 회)**

```
Rust (PyO3):   124.72ms
Python (순수): 108.89ms

⚠️ PyO3 오버헤드: 함수 호출 비용
💡 해결: 배치 처리로 오버헤드 상쇄
```

---

## 🔄 Rust → Python 변환 과정

### **전체 흐름도**

```
┌─────────────────────────────────────────────────────────┐
│ ① src/lib.rs (Rust 코드)                                │
│    #[pymodule]                                          │
│    fn zkml_py() { ... }                                 │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│ ② maturin develop --release                             │
│    - Rust 컴파일 (cargo build)                          │
│    - Python 확장 모듈 생성 (.so)                        │
│    - 가상환경에 설치                                     │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│ ③ zkml_py.cpython-39-darwin.so (564KB)                  │
│    - 컴파일된 Rust 코드 (기계어)                        │
│    - Python이 직접 로드 가능                            │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│ ④ Python에서 import                                     │
│    import zkml_py                                       │
│    model = zkml_py.LinearModel(2.0, 1.0)  ← Rust 실행! │
└─────────────────────────────────────────────────────────┘
```

### **단계별 설명**

#### **① Rust 코드 작성**

```rust
// src/lib.rs
#[pymodule]
fn zkml_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinearModel>()?;  // Rust 구조체 → Python 클래스
    m.add_function(wrap_pyfunction!(add, m)?)?;  // Rust 함수 → Python 함수
    Ok(())
}
```

#### **② maturin 빌드**

```bash
maturin develop --release
```

- Rust 코드를 컴파일 (`cargo build`)
- Python 확장 모듈로 변환 (`.so` 파일 생성)
- 가상환경에 자동 설치

#### **③ 컴파일 결과**

```
.venv/lib/python3.9/site-packages/zkml_py/
├── __init__.py                          # Python 모듈 진입점
└── zkml_py.cpython-39-darwin.so         # 컴파일된 Rust 코드 (564KB)
```

**`.so` 파일**: Python이 직접 로드할 수 있는 네이티브 확장 모듈 (기계어)

#### **④ Python에서 사용**

```python
import zkml_py  # .so 파일 로드

# Rust 코드가 Python 함수/클래스처럼 보임!
model = zkml_py.LinearModel(2.0, 1.0)  # 실제로는 Rust 실행
y = model.predict(3.0)  # Rust의 성능 그대로
```

---

## 🚀 설치 & 실행

### **1. 의존성 설치**

```bash
pip install maturin
```

### **2. 빌드 & 설치**

```bash
cd phase3/month7-8_rust/week3_pyo3_binding

# 개발 모드 (빠른 빌드)
maturin develop

# 릴리즈 모드 (최적화)
maturin develop --release
```

### **3. 테스트**

```bash
python test_zkml_py.py
```

### **4. 패키지 빌드**

```bash
# Wheel 생성
maturin build --release

# PyPI 업로드 (선택)
maturin publish
```

---

## 💻 사용 예제

### **Python에서 Rust 함수 호출**

```python
import zkml_py

# 1. 간단한 함수
result = zkml_py.add(10.0, 20.0)
print(result)  # 30.0

# 2. Rust 클래스 사용
model = zkml_py.LinearModel(2.0, 1.0)
print(model)  # LinearModel(weight=2, bias=1)

# 3. 예측
y = model.predict(3.0)
print(y)  # 7.0

# 4. 배치 예측
outputs = model.predict_batch([1.0, 2.0, 3.0])
print(outputs)  # [3.0, 5.0, 7.0]

# 5. 벡터 내적
a = [1.0, 2.0, 3.0]
b = [4.0, 5.0, 6.0]
result = zkml_py.dot_product(a, b)
print(result)  # 32.0
```

---

## 🔍 코드 구조

### **Rust 코드 (src/lib.rs)**

```rust
use pyo3::prelude::*;

#[pyclass]
struct LinearModel {
    weight: f64,
    bias: f64,
}

#[pymethods]
impl LinearModel {
    #[new]
    fn new(weight: f64, bias: f64) -> Self {
        LinearModel { weight, bias }
    }

    fn predict(&self, x: f64) -> f64 {
        self.weight * x + self.bias
    }
}

#[pymodule]
fn zkml_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinearModel>()?;
    Ok(())
}
```

### **Python 코드**

```python
import zkml_py

model = zkml_py.LinearModel(2.0, 1.0)
y = model.predict(3.0)  # Rust 함수 호출!
```

---

## 💡 PyO3 핵심 개념

### **1. 데코레이터**

| 데코레이터      | 용도                        |
| --------------- | --------------------------- |
| `#[pyclass]`    | Rust 구조체 → Python 클래스 |
| `#[pymethods]`  | Rust 메서드 → Python 메서드 |
| `#[pyfunction]` | Rust 함수 → Python 함수     |
| `#[pymodule]`   | Rust 모듈 → Python 모듈     |

### **2. 타입 변환**

| Rust            | Python        |
| --------------- | ------------- |
| `f64`           | `float`       |
| `i32`           | `int`         |
| `String`        | `str`         |
| `Vec<T>`        | `list`        |
| `HashMap<K, V>` | `dict`        |
| `Option<T>`     | `None` or `T` |
| `Result<T, E>`  | Exception     |

### **3. 에러 처리**

```rust
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "0으로 나눌 수 없습니다"
        ))
    } else {
        Ok(a / b)
    }
}
```

```python
try:
    result = zkml_py.divide(10.0, 0.0)
except ValueError as e:
    print(e)  # "0으로 나눌 수 없습니다"
```

---

## 🎯 성능 최적화 팁

### **1. 배치 처리**

❌ **나쁜 예 (함수 호출 오버헤드)**

```python
for x in data:
    y = model.predict(x)  # 100만 번 호출
```

✅ **좋은 예 (배치 처리)**

```python
outputs = model.predict_batch(data)  # 1번 호출
```

### **2. NumPy 통합**

```rust
use numpy::PyArray1;

#[pyfunction]
fn process_array(arr: &PyArray1<f64>) -> Vec<f64> {
    arr.to_vec().unwrap()
        .iter()
        .map(|&x| x * 2.0)
        .collect()
}
```

### **3. 병렬 처리**

```rust
use rayon::prelude::*;

#[pyfunction]
fn parallel_process(data: Vec<f64>) -> Vec<f64> {
    data.par_iter()
        .map(|&x| expensive_computation(x))
        .collect()
}
```

---

## 📦 배포

### **1. Wheel 빌드**

```bash
# 현재 플랫폼용
maturin build --release

# 여러 플랫폼용 (CI/CD)
maturin build --release --target x86_64-unknown-linux-gnu
maturin build --release --target aarch64-apple-darwin
```

### **2. PyPI 업로드**

```bash
maturin publish
```

### **3. 사용자 설치**

```bash
pip install zkml-py
```

---

## 🔥 실전 활용 예시

### **1. 고성능 데이터 처리**

```python
import zkml_py
import pandas as pd

# 대용량 데이터 처리
df = pd.read_csv("big_data.csv")
results = zkml_py.process_batch(df["values"].tolist())
```

### **2. 암호화 연산**

```python
# Rust로 구현된 ZK 증명 생성
proof = zkml_py.generate_proof(witness, circuit)
is_valid = zkml_py.verify_proof(proof, public_inputs)
```

### **3. 이미지 처리**

```python
import numpy as np
from PIL import Image

img = np.array(Image.open("photo.jpg"))
processed = zkml_py.fast_filter(img.flatten().tolist())
```

---

## 💡 배운 점

### **✅ PyO3의 장점**

- Python의 편의성 + Rust의 성능
- 기존 Python 코드와 완벽한 호환
- 타입 안전성 보장
- 쉬운 배포 (Wheel)

### **⚠️ 주의사항**

- 함수 호출 오버헤드 존재
- 배치 처리로 오버헤드 상쇄 필요
- 복잡한 타입 변환 시 성능 저하
- GIL (Global Interpreter Lock) 고려

### **🎯 최적 사용 사례**

- CPU 집약적 연산
- 대용량 데이터 처리
- 암호화/보안 연산
- 병렬 처리

---

## 📚 참고 자료

- [PyO3 User Guide](https://pyo3.rs/)
- [Maturin Documentation](https://www.maturin.rs/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

---

**완료일**: 2025-10-20  
**성능**: Rust 1.2배 빠름 (벡터 내적)  
**다음 단계**: Month 8 - 프로덕션 ZK 시스템 🚀
