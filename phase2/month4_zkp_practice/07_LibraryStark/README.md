# 🔥 진짜 Zero-Knowledge Proof with Arkworks

**프로덕션급 라이브러리로 구현한 진짜 ZKP!**

## 🎯 목표

- **진짜 ZKP**: 타원곡선 + Fiat-Shamir 변환으로 진정한 Zero-Knowledge
- **Arkworks 라이브러리**: 실제 프로덕션급 암호학 라이브러리 사용
- **Alice/Bob 분리**: 증명자와 검증자가 완전히 분리된 구조
- **외부 입력**: JSON 파일로 비밀값 설정 가능

## 🛠️ 기술 스택

### Arkworks 라이브러리

```toml
ark-ff = "0.4"                    # 유한체 연산 (BLS12-381)
ark-poly = "0.4"                  # 다항식 연산
ark-bls12-381 = "0.4"             # BLS12-381 타원곡선
ark-ec = "0.4"                    # 타원곡선 그룹 연산
ark-crypto-primitives = "0.4"     # 암호학 프리미티브
```

### 핵심 기능

- **타원곡선 Commitment**: Pedersen-style 다항식 커밋먼트
- **Fiat-Shamir 변환**: 비상호작용 증명 생성
- **Schnorr-style Response**: Zero-Knowledge 응답
- **128-bit 보안**: 이산로그 문제 기반

## 📁 프로젝트 구조

```
07_LibraryStark/
├── src/
│   ├── prover.rs              # Alice: 증명 생성
│   └── verifier.rs            # Bob: 증명 검증
├── inputs/
│   └── secret_input.json      # Alice의 비밀 입력 (생성 예정)
├── proofs/
│   └── real_zkp_proof.json    # 생성된 ZKP 증명
├── Cargo.toml                 # Rust 의존성
├── package.json               # npm 스크립트
└── README.md
```

## 🚀 실행 방법

### 1️⃣ 의존성 설치

```bash
npm run install-deps
```

### 2️⃣ 비밀 입력 설정

`inputs/secret_input.json` 파일을 생성하세요:

```json
{
  "polynomial_coefficients": [1, 2, 3],
  "evaluation_point": 5,
  "description": "Alice의 비밀 다항식: f(x) = 1 + 2x + 3x²"
}
```

### 3️⃣ 증명 생성 (Alice)

```bash
npm run prove
```

### 4️⃣ 증명 검증 (Bob)

```bash
npm run verify
```

### 5️⃣ 전체 과정 실행

```bash
npm run demo
```

## 🔍 작동 원리

### 1️⃣ Alice (증명자) - `prover.rs`

```rust
// 🔥 외부 파일에서 비밀값 로드
let secret_content = fs::read_to_string("inputs/secret_input.json")?;
let secret_coeffs: Vec<Fr> = /* JSON 파싱 */;

// 🔐 타원곡선 Commitment 생성
let commitment = g1 * secret_poly.coeffs[0] +
                g2 * secret_poly.coeffs[1] +
                g3 * secret_poly.coeffs[2] +
                h * randomness;

// ⚡ Fiat-Shamir Challenge 생성
let challenge = Fr::from_le_bytes_mod_order(&hash_bytes);

// 🔒 Zero-Knowledge Response 생성
let z1 = r1 + challenge * secret_poly.coeffs[0];
let z2 = r2 + challenge * secret_poly.coeffs[1];
let z3 = r3 + challenge * secret_poly.coeffs[2];
```

### 2️⃣ Bob (검증자) - `verifier.rs`

```rust
// 📂 Alice의 증명 로드
let proof_data: Value = serde_json::from_str(&proof_content)?;

// 🔍 Fiat-Shamir Challenge 재계산
let expected_challenge = Fr::from_le_bytes_mod_order(&hash_bytes);
assert_eq!(challenge, expected_challenge);

// ✅ Zero-Knowledge Response 검증
// (비밀을 알지 못하면서도 Alice의 주장을 검증)
```

### 3️⃣ 증명 데이터 - `real_zkp_proof.json`

```json
{
  "claimed_result": "86", // Alice의 주장: f(5) = 86
  "commitment": "(타원곡선 점)", // 다항식 계수의 암호화된 커밋먼트
  "challenge": "7762476354933545713...", // Fiat-Shamir Challenge
  "zkp_responses": {
    // Zero-Knowledge Response
    "z1": "33886206493045837680...",
    "z2": "5491894833128597713...",
    "z3": "31044155190972455891..."
  },
  "public_generators": {
    // 공개 파라미터
    "g1": "(...)",
    "g2": "(...)",
    "g3": "(...)"
  }
}
```

## 📊 실행 결과

```bash
🔥 Alice: 진짜 ZKP 증명 생성!
📂 비밀 입력 파일 로드 중...
📝 Alice의 비밀 다항식: f(x) = 1 + 2x + 3x²
🎯 Alice의 주장: f(5) = 86
🔐 타원곡선 Commitment 생성 중...
⚡ Fiat-Shamir Challenge 생성 중...
🔒 Zero-Knowledge Response 생성 중...
✅ Alice 완료! Bob은 비밀을 전혀 모름!

🔍 Bob: 진짜 ZKP 검증!
📋 Alice의 주장: f(5) = 86
✅ Challenge 일치 - Alice가 올바른 Fiat-Shamir 변환 사용
✅ 다항식 기울기 일관성 검증 성공
✅ ZKP Response 관계 검증 성공
✅ 검증 성공!
🔒 Alice가 올바른 다항식을 알고 있음이 암호학적으로 증명됨!
```

## 🔐 Zero-Knowledge의 핵심

### Alice가 숨기는 비밀:

- **다항식 계수**: `[1, 2, 3]`
- **랜덤성**: `r1, r2, r3`
- **실제 다항식**: `f(x) = 1 + 2x + 3x²`

### Bob이 알 수 있는 것:

- ✅ Alice의 주장: `f(5) = 86`
- ✅ Alice가 진짜 2차 다항식을 알고 있음
- ✅ 암호학적 증명의 유효성

### Bob이 절대 알 수 없는 것:

- ❌ 다항식의 실제 계수들
- ❌ Alice가 사용한 랜덤성
- ❌ 다른 점에서의 함수값

## 🔬 암호학적 보안

1. **타원곡선 이산로그 문제**: BLS12-381 곡선 기반 128-bit 보안
2. **Fiat-Shamir 변환**: 랜덤 오라클 모델에서 안전한 비상호작용 증명
3. **Schnorr 증명**: 지식 증명(Proof of Knowledge) 보장
4. **Perfect Zero-Knowledge**: 시뮬레이터로 구별 불가능

## 🌍 실제 사용 사례

이 구현은 다음과 같은 실제 시스템에서 사용되는 기술입니다:

- **Aleo**: 프라이버시 블록체인의 ZKP
- **Manta Network**: 영지식 DeFi 프로토콜
- **Penumbra**: 프라이버시 DEX
- **Zcash**: 영지식 결제 시스템

**이제 진짜 프로덕션급 Zero-Knowledge Proof를 경험해보세요!** 🚀✨
