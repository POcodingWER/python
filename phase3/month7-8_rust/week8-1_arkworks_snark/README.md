# 🔐 Week 8-1: arkworks SNARK

**Rust로 배우는 Zero-Knowledge 암호학**

---

## 📋 목차

1. [프로젝트 개요](#-프로젝트-개요)
2. [구현 내용](#-구현-내용)
3. [실행 방법](#-실행-방법)
4. [성능 결과](#-성능-결과)
5. [핵심 개념](#-핵심-개념)
6. [프로젝트 구조](#-프로젝트-구조)

---

## 🎯 프로젝트 개요

**arkworks**는 Rust로 작성된 고성능 Zero-Knowledge 암호학 라이브러리입니다.

### 목표
- ✅ 타원곡선 암호학 기초 학습
- ✅ Groth16 SNARK 전체 파이프라인 구현
- ✅ R1CS 회로 설계 및 증명
- ✅ 실전 영지식 증명 시스템 구축

### 특징
- 🔐 **진짜 영지식 증명**: 비밀 값을 숨기면서 계산 결과 증명
- ⚡ **초고속 검증**: ~2ms (밀리초 단위!)
- 🎓 **실용적 예제**: 비밀번호 해시, 제곱 증명 등
- 🦀 **Rust 네이티브**: 타입 안전성 + 고성능

---

## 🚀 구현 내용

### 1️⃣ 타원곡선 연산 (BN254, BLS12-381)

```rust
// 필드 연산
a + b  // 덧셈
a * b  // 곱셈
a⁻¹    // 역원

// 곡선 포인트 연산
P + Q      // 포인트 덧셈
2P         // 포인트 배가
s * P      // 스칼라 곱셈
```

**성능**:
- BN254 필드 곱셈: **0.15 μs**
- BN254 스칼라 곱셈: **101 μs**
- BLS12-381 곡선 덧셈: **216 μs**

### 2️⃣ Groth16 SNARK

#### 예시 1: 비밀번호 해시 증명

```rust
// 회로: secret * 7 + 13 = hash
// 목표: secret 값을 숨기면서 hash가 맞음을 증명

🔒 비밀 값: 42 (비공개!)
✅ 공개 해시: 307

Setup:  6.95ms
Prove:  2.14ms
Verify: 2.62ms ⚡
```

**핵심**: 검증자는 `secret=42`를 모르고도 `hash=307`이 올바른 계산 결과임을 확인!

#### 예시 2: 제곱 증명

```rust
// 회로: x² = y
// 목표: x 값을 숨기면서 x² = 25 임을 증명

🔒 비밀 값 x: 5 (비공개!)
✅ 공개 값 y: 25

Setup:  5.33ms
Prove:  1.89ms
Verify: 2.58ms ⚡
```

**핵심**: 검증자는 `x=5`를 모르고도 `x² = 25`임을 확인!

### 3️⃣ R1CS 회로

구현된 회로들:

| 회로 | 제약조건 | 용도 |
|------|----------|------|
| `MultiplyCircuit` | `x * y = z` | 곱셈 증명 |
| `PasswordCircuit` | `secret * 7 + 13 = hash` | 비밀번호 해시 |
| `AgeCircuit` | `age >= 19` | 나이 검증 |
| `SquareCircuit` | `x² = y` | 제곱 증명 |

---

## 💻 실행 방법

### 설치
```bash
cd week8-1_arkworks_snark
cargo build --release
```

### 실행

#### npm 스크립트 (추천!)
```bash
# 타원곡선 데모
npm start
# 또는
npm run curves

# Groth16 SNARK 데모
npm run groth16

# 빌드
npm run build

# 코드 검사
npm run check
```

#### Cargo 직접 사용
```bash
# 타원곡선 데모
cargo run --release

# Groth16 SNARK 데모
cargo run --example groth16_demo --release

# 테스트
cargo test --release
```

---

## 📊 성능 결과

### Groth16 SNARK 성능

| 단계 | 비밀번호 해시 | 제곱 증명 |
|------|---------------|-----------|
| **Setup** | 6.95ms | 5.33ms |
| **Prove** | 2.14ms | 1.89ms |
| **Verify** | 2.62ms ⚡ | 2.58ms ⚡ |

### 타원곡선 성능

| 연산 | BN254 | BLS12-381 |
|------|-------|-----------|
| **필드 곱셈** | 0.15 μs | 0.09 μs |
| **곡선 덧셈** | 34 μs | 216 μs |
| **스칼라 곱셈** | 101 μs | - |

**💡 결론**: 
- 검증 속도가 **2.6ms**로 초고속! ⚡
- BN254가 BLS12-381보다 약 6배 빠름
- Rust + arkworks = 고성능 ZK 시스템

---

## 🔐 핵심 개념

### 영지식 증명 (Zero-Knowledge Proof)

```
증명자: "나는 비밀 값을 알고 있고, 이 계산 결과가 맞습니다"
검증자: "증명을 확인했습니다. 맞네요!"

💡 검증자는 비밀 값을 모릅니다!
```

### Groth16 SNARK

1. **Setup**: 증명 키와 검증 키 생성
2. **Prove**: 비밀 값으로 증명 생성
3. **Verify**: 공개 값으로 증명 검증

### R1CS (Rank-1 Constraint System)

제약조건 시스템으로 회로를 표현:

```rust
// x * y = z 제약조건
cs.enforce_constraint(
    lc!() + x,  // 왼쪽
    lc!() + y,  // 오른쪽
    lc!() + z,  // 결과
)?;
```

---

## 📁 프로젝트 구조

```
week8-1_arkworks_snark/
├── src/
│   ├── lib.rs              # 라이브러리 (유틸리티 함수)
│   ├── main.rs             # 타원곡선 데모
│   └── circuit.rs          # R1CS 회로 구현
├── examples/
│   └── groth16_demo.rs     # Groth16 SNARK 데모
├── Cargo.toml              # 의존성 설정
├── package.json            # npm 스크립트
└── README.md               # 이 파일
```

### 핵심 파일

#### `src/circuit.rs`
```rust
// R1CS 회로 정의
pub struct PasswordCircuit<F: PrimeField> {
    pub secret: Option<F>,  // 🔒 Witness (비공개)
    pub hash: Option<F>,    // ✅ Public Input (공개)
}
```

#### `examples/groth16_demo.rs`
```rust
// Groth16 파이프라인
let pk = Groth16::generate_random_parameters_with_reduction(circuit, &mut rng)?;
let proof = Groth16::create_random_proof_with_reduction(circuit, &pk, &mut rng)?;
let is_valid = Groth16::verify_proof(&pvk, &proof, &public_inputs)?;
```

---

## 🎓 배운 내용

1. ✅ **타원곡선 암호학 기초**
   - BN254, BLS12-381 곡선
   - 필드 연산, 곡선 포인트 연산
   - 성능 벤치마크

2. ✅ **Groth16 SNARK 전체 파이프라인**
   - Setup → Prove → Verify
   - R1CS 회로 설계
   - 영지식 속성 구현

3. ✅ **실전 영지식 증명 시스템**
   - 비밀 값 보호
   - 초고속 검증 (~2ms)
   - 실용적 예제 (비밀번호, 나이, 제곱)

---

## 🚀 다음 단계

### Week 8-2: 실전 활용
- [ ] 복잡한 회로 구현 (Merkle Tree)
- [ ] Python py_ecc vs arkworks 성능 비교
- [ ] 블록체인 통합

### 확장 아이디어
- **익명 투표**: 투표 내용을 숨기면서 유효성 증명
- **프라이버시 거래**: 잔액을 숨기면서 송금 증명
- **신원 인증**: 개인정보 공개 없이 자격 증명

---

## 📚 참고 자료

- [arkworks GitHub](https://github.com/arkworks-rs)
- [Groth16 논문](https://eprint.iacr.org/2016/260.pdf)
- [Zero-Knowledge Proofs 개론](https://z.cash/technology/zksnarks/)

---

**Made with 🦀 Rust + 🔐 arkworks**

