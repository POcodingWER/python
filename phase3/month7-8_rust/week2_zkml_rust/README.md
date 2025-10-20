# Week 2: Rust ZKML 실전 프로젝트 🦀⚡

> **목표**: Python ZKML 코드를 Rust로 포팅하고 성능 비교

---

## 🎯 프로젝트 개요

### **Python (Month 5-6) → Rust (Month 7-8)**

| 항목              | Python + EZKL | Rust + halo2        |
| ----------------- | ------------- | ------------------- |
| **언어**          | Python 3.11   | Rust 1.92           |
| **AI 프레임워크** | PyTorch       | 순수 수학           |
| **ZK 라이브러리** | EZKL          | halo2_proofs        |
| **개발 속도**     | 빠름 ⚡       | 중간                |
| **실행 속도**     | 기준          | **5,257배 빠름** 🚀 |
| **메모리 사용**   | 높음          | 낮음                |
| **타입 안전성**   | 약함          | 강함 ✅             |

---

## 📊 성능 벤치마크

### **1. Linear Regression 추론 (100만 회)**

```
┌─────────────────┬──────────────┬────────────┐
│ 구현            │ 실행 시간    │ 성능 비교  │
├─────────────────┼──────────────┼────────────┤
│ Rust (순수)     │ 1.23ms       │ 5,257배 ⚡ │
│ Python (PyTorch)│ 6,466ms      │ 기준       │
└─────────────────┴──────────────┴────────────┘
```

### **2. 메모리 사용량**

```
Rust:   ~2MB (컴파일 타임 최적화)
Python: ~50MB (PyTorch 런타임)

메모리 효율: 25배 개선 ✅
```

### **3. 바이너리 크기**

```
Rust Release:  ~5MB (정적 링크)
Python + deps: ~500MB (PyTorch, NumPy, EZKL)

배포 크기: 100배 감소 ✅
```

---

## 🚀 실행 방법

### **1. 빌드 & 실행**

```bash
cd phase3/month7-8_rust/week2_zkml_rust

# 개발 모드
cargo run

# 릴리즈 모드 (최적화)
cargo run --release

# 테스트
cargo test

# 벤치마크
cargo bench
```

### **2. Python 비교 실행**

```bash
cd ../../phase2/month5-6_zkml
source zkml_env/bin/activate
python week1_ezkl_basics/01_simple_linear_regression.py
```

---

## 📁 프로젝트 구조

```
week2_zkml_rust/
├── src/
│   ├── main.rs                 # 메인 프로그램
│   ├── linear_regression.rs    # 순수 Rust Linear Regression
│   └── zkml_circuit.rs         # halo2 ZK 회로
├── Cargo.toml                  # 의존성 설정
└── README.md                   # 이 파일
```

---

## 🔍 코드 비교

### **Python (PyTorch)**

```python
import torch
import torch.nn as nn

class SimpleLinearRegression(nn.Module):
    def __init__(self):
        super().__init__()
        self.linear = nn.Linear(1, 1)

    def forward(self, x):
        return self.linear(x)

model = SimpleLinearRegression()
with torch.no_grad():
    model.linear.weight.fill_(2.0)
    model.linear.bias.fill_(1.0)

# 예측
x = torch.tensor([[3.0]])
y = model(x)  # 7.0
```

### **Rust (순수 구현)**

```rust
struct LinearModel {
    weight: f64,
    bias: f64,
}

impl LinearModel {
    fn new(weight: f64, bias: f64) -> Self {
        LinearModel { weight, bias }
    }

    fn predict(&self, x: f64) -> f64 {
        self.weight * x + self.bias
    }
}

let model = LinearModel::new(2.0, 1.0);
let y = model.predict(3.0);  // 7.0
```

**차이점:**

- ✅ Rust는 **의존성 없음** (순수 수학)
- ✅ **컴파일 타임 최적화**
- ✅ **제로 코스트 추상화**
- ✅ **메모리 안전성 보장**

---

## 🔐 halo2 ZK 회로

### **회로 구조**

```rust
// 제약조건: result = weight * x
meta.create_gate("linear regression", |meta| {
    let s = meta.query_selector(selector);
    let x = meta.query_advice(advice[0], Rotation::cur());
    let weight = meta.query_advice(advice[1], Rotation::cur());
    let result = meta.query_advice(advice[2], Rotation::cur());

    vec![s * (weight * x - result)]
});
```

### **Python EZKL vs Rust halo2**

| 항목            | EZKL             | halo2             |
| --------------- | ---------------- | ----------------- |
| **추상화 수준** | 높음 (자동)      | 낮음 (수동)       |
| **학습 곡선**   | 쉬움 ⭐          | 어려움 ⭐⭐⭐⭐⭐ |
| **제어 수준**   | 제한적           | 완전한 제어       |
| **성능**        | 좋음             | 최고 ⚡           |
| **사용 사례**   | 프로토타입, 학습 | 프로덕션, 최적화  |

---

## 💡 배운 점

### **1. Rust의 장점**

- ✅ **메모리 안전성**: 컴파일 타임에 메모리 오류 방지
- ✅ **제로 코스트 추상화**: 고수준 코드 → 저수준 성능
- ✅ **병렬 처리**: 안전한 멀티스레딩
- ✅ **의존성 관리**: Cargo로 간편한 패키지 관리

### **2. 성능 최적화 포인트**

- 🚀 **인라인 함수**: `#[inline]` 속성
- 🚀 **벡터화**: SIMD 연산 활용
- 🚀 **메모리 레이아웃**: 캐시 친화적 구조
- 🚀 **컴파일 옵션**: `--release` 플래그

### **3. Python vs Rust 선택 기준**

**Python을 선택할 때:**

- 빠른 프로토타이핑
- 데이터 분석 & 시각화
- 풍부한 AI 라이브러리 활용
- 팀 협업 (낮은 진입 장벽)

**Rust를 선택할 때:**

- 프로덕션 배포
- 고성능 요구사항
- 메모리 제약 환경
- 시스템 프로그래밍

---

## 🎯 다음 단계

### **Week 3: Candle 프레임워크**

- [ ] Candle로 PyTorch 모델 변환
- [ ] GPU 가속 활용
- [ ] ONNX 모델 지원
- [ ] 성능 프로파일링

### **Week 4: PyO3 Python 바인딩**

- [ ] Rust 함수를 Python에서 호출
- [ ] NumPy 배열 ↔ Rust 텐서
- [ ] 하이브리드 시스템 구축
- [ ] 패키지 배포 (PyPI)

### **Month 8: 고성능 ZK 시스템**

- [ ] 실제 증명 생성 (halo2)
- [ ] 웹 서버 구축 (Axum)
- [ ] WebAssembly 컴파일
- [ ] 프로덕션 최적화

---

## 📚 참고 자료

### **Rust 학습**

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### **halo2**

- [halo2 Book](https://zcash.github.io/halo2/)
- [halo2 GitHub](https://github.com/zcash/halo2)

### **성능 최적화**

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs](https://github.com/bheisler/criterion.rs) (벤치마킹)

---

**완료일**: 2025-10-20  
**실행 시간**: ~1ms (Rust) vs ~6,500ms (Python)  
**성능 향상**: **5,257배** 🚀  
**다음 단계**: Week 3 - Candle 프레임워크
