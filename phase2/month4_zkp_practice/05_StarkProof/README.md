# 05_StarkProof - STARK 증명 시스템 🌟

## 📋 프로젝트 개요

**STARK (Scalable Transparent ARgument of Knowledge)**를 사용하여 머클트리 포함 증명을 구현합니다.

### 🎯 **학습 목표**

- ✅ **Cairo 언어** 마스터
- ✅ **STARK 프로토콜** 이해
- ✅ **Trusted Setup 없는** ZK 체험
- ✅ **양자 저항성** 확인
- ✅ **SNARK와 비교** 분석

---

## 🔍 STARK vs SNARK 비교

| 특징              | SNARK (04_MerkleTree) | **STARK (이 프로젝트)** |
| ----------------- | --------------------- | ----------------------- |
| **Trusted Setup** | 필요 ❌               | **불필요 ✅**           |
| **증명 크기**     | 매우 작음 (200바이트) | **큼 (수십KB)**         |
| **검증 시간**     | 매우 빠름             | **빠름**                |
| **양자 저항**     | 없음                  | **있음 ✅**             |
| **투명성**        | 낮음                  | **높음 ✅**             |
| **확장성**        | 제한적                | **우수 ✅**             |

---

## 🛠️ 구현 계획

### Week 1: Cairo 환경 & 기초

- [ ] **Cairo 개발 환경 구축**

  - [ ] Cairo 컴파일러 설치
  - [ ] StarkNet 개발 도구 설정
  - [ ] VS Code 확장 설치
  - [ ] 첫 Hello World 프로그램

- [ ] **Cairo 언어 학습**
  - [ ] 기본 문법 (felt, struct, function)
  - [ ] 메모리 모델 이해
  - [ ] 어서션과 제약조건
  - [ ] 내장 함수 활용

### Week 2: 머클트리 STARK 구현

- [ ] **머클트리 로직 Cairo 포팅**

  - [ ] Poseidon 해시 함수 구현
  - [ ] 머클 경로 검증 로직
  - [ ] pathElements/pathIndices 처리
  - [ ] 04_MerkleTree와 동일한 로직

- [ ] **STARK 증명 생성**
  - [ ] Cairo 프로그램 컴파일
  - [ ] 실행 트레이스 생성
  - [ ] STARK 증명 생성
  - [ ] 증명 검증

### Week 3: 테스트 & 최적화

- [ ] **테스트 케이스 작성**

  - [ ] 유효한 머클 증명 테스트
  - [ ] 무효한 머클 증명 테스트
  - [ ] 다양한 트리 깊이 테스트
  - [ ] 엣지 케이스 처리

- [ ] **성능 최적화**
  - [ ] 프로그램 크기 최적화
  - [ ] 실행 단계 수 최소화
  - [ ] 메모리 사용량 최적화
  - [ ] 증명 생성 시간 측정

---

## 📁 파일 구조

```
05_StarkProof/
├── README.md                 # 이 파일
├── cairo_programs/
│   ├── merkle_proof.cairo   # 머클트리 증명 프로그램
│   └── poseidon.cairo       # Poseidon 해시 구현
├── inputs/
│   ├── input_valid.json     # 유효한 증명 입력
│   └── input_invalid.json   # 무효한 증명 입력
├── proofs/
│   ├── proof_valid.json     # 생성된 유효 증명
│   └── proof_invalid.json   # 생성된 무효 증명
├── scripts/
│   ├── compile.sh           # Cairo 컴파일 스크립트
│   ├── prove.sh             # STARK 증명 생성
│   └── verify.sh            # STARK 증명 검증
└── package.json             # 자동화 스크립트
```

---

## 🚀 사용법

### 1. 환경 설정

```bash
# Cairo 설치
curl -L https://github.com/starkware-libs/cairo/releases/download/v2.4.0/cairo-lang-2.4.0.tar.gz | tar xz
export PATH="$PATH:$HOME/.cairo/bin"

# 의존성 설치
pnpm install
```

### 2. 컴파일 & 실행

```bash
# Cairo 프로그램 컴파일
pnpm run compile

# STARK 증명 생성
pnpm run prove-valid
pnpm run prove-invalid

# 증명 검증
pnpm run verify-valid
pnpm run verify-invalid
```

### 3. 벤치마크

```bash
# 성능 측정
pnpm run benchmark

# SNARK와 비교
pnpm run compare-with-snark
```

---

## 🔬 예상 결과

### 📊 **성능 예측**

- **증명 생성 시간**: 5-10초 (SNARK: 2-5초)
- **증명 크기**: 50-100KB (SNARK: 200바이트)
- **검증 시간**: 10-50ms (SNARK: 5-10ms)
- **메모리 사용량**: 높음 (SNARK: 낮음)

### ✅ **STARK의 장점 체험**

- **투명성**: Trusted setup 불필요
- **양자 저항**: 해시 기반 보안
- **확장성**: 큰 계산에 유리
- **투명성**: 모든 과정 검증 가능

---

## 🎯 학습 포인트

### 🧠 **핵심 개념**

1. **FRI (Fast Reed-Solomon Interactive Oracle Proofs)**
2. **Polynomial Commitment Schemes**
3. **Arithmetization of Computation**
4. **Low-Degree Testing**

### 🔍 **SNARK와의 차이점**

- **수학적 기반**: 타원곡선 vs 해시함수
- **보안 가정**: 이산로그 vs 충돌저항성
- **투명성**: 불투명 vs 완전투명
- **확장성**: 제한적 vs 우수

---

## 📚 참고 자료

- [StarkWare 공식 문서](https://docs.starkware.co/)
- [Cairo 언어 가이드](https://www.cairo-lang.org/docs/)
- [STARK 논문](https://eprint.iacr.org/2018/046.pdf)
- [04_MerkleTree SNARK 구현](../04_MerkleTree/)

---

**시작일**: 2024년 10월 13일  
**예상 완료일**: 2024년 10월 20일  
**난이도**: ⭐⭐⭐⭐ (고급)  
**상태**: 🚧 진행 중
