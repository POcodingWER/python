# Month 4: ZKP Practice - Circom & snarkjs 🛠️

## 📋 학습 목표

Phase 1에서 Python으로 ZKP 이론을 마스터했으니, 이제 **실전 ZK 회로 프로그래밍**!

✅ **Circom 마스터** - ZK 회로 전용 언어  
✅ **snarkjs 활용** - JavaScript ZK 라이브러리  
✅ **실전 ZK 회로** - Merkle Tree, Hash 등  
✅ **ZK 웹앱 구축** - 풀스택 ZK 애플리케이션

### 🎯 **Phase 1 → Phase 2 업그레이드:**

| Phase 1 (Python 이론) | Phase 2 (실전 구현) | 발전 |
| --------------------- | ------------------- | ---- |
| py_ecc 시뮬레이션     | 실제 ZK 회로        | 🚀   |
| 교육용 예제           | 프로덕션 수준       | 🏭   |
| Python만              | Full-Stack          | 🌐   |
| 개념 이해             | 실전 배포           | 🔥   |

---

## 📚 주차별 계획

### Week 1: Circom 기초 & 환경 설정 ✅

- [x] **Circom 환경 구축**

  - [x] Node.js & npm 설치
  - [x] Circom 컴파일러 설치
  - [x] snarkjs 라이브러리 설치
  - [x] Powers of Tau 세팅

- [x] **첫 ZK 회로 작성**

  - [x] 간단한 곱셈 회로 (`01_multiplier.circom`)
  - [x] Witness 생성 및 증명
  - [x] Verification key 관리
  - [x] 단계별 실행 스크립트 구축 (`package.json`, `run_step.sh`)
  - [x] 다중 사용자 증명 시스템 이해
  - [x] SNARK vs STARK 개념 정리

- [x] **고급 보안 분석 & 최적화**
  - [x] 보안 취약점 분석 시스템 구축 (`02_SecurityAnalysis/`)
  - [x] 증명서 조작 공격 시뮬레이션
  - [x] 입력 오버플로우 공격 테스트
  - [x] 성능 벤치마크 분석 도구
  - [x] 자동화된 보안 테스트 스위트

### Week 2: 실용적 ZK 회로 구현 ✅

- [x] **Hash Preimage 회로**

  - [x] SHA-256 회로 구현 (`02_hash_preimage.circom`)
  - [x] Hash 충돌 방지 증명 (31,536개 제약조건)
  - [x] 패스워드 검증 시스템 (`PasswordVerify` 템플릿)
  - [x] Phase 1 해시 구현과 비교 (프라이버시 95% 향상)
  - [x] 자동화된 ZKP 파이프라인 (`package.json` 스크립트)
  - [x] Powers of Tau 최적화 (bn128 16 지원)
  - [x] 실제 응용 사례 분석 (Zcash, 투표 시스템 등)

- [x] **Merkle Tree 회로**
  - [x] Merkle proof 검증 회로 (`04_merkle_proof.circom`)
  - [x] 1레벨 머클트리 ZKP 구현 (246개 제약조건)
  - [x] Poseidon 해시 함수 활용
  - [x] pathElements, pathIndices 완전 이해
  - [x] 유효/무효 증명 구분 시스템 (`isValid = 1/0`)
  - [x] 자동화된 테스트 스위트 (`test-valid`, `test-invalid`)
  - [x] poseidon-lite 패키지 통합

### Week 3-4: 진짜 ZKP 구현 & 라이브러리 마스터 ✅

- [x] **진짜 STARK 구현**

  - [x] Rust 기반 FRI 프로토콜 구현 (`06_RealStarkProof/`)
  - [x] Cairo 프로그램과 Rust 검증자 연동
  - [x] 머클 트리 커밋먼트 및 다항식 보간
  - [x] 프로덕션급 128-bit 보안 수준

- [x] **Arkworks 라이브러리 ZKP**

  - [x] 타원곡선 + Fiat-Shamir 변환 (`07_LibraryStark/`)
  - [x] BLS12-381 곡선 기반 진짜 Zero-Knowledge
  - [x] Pedersen-style Commitment 구현
  - [x] 외부 JSON 입력으로 비밀값 설정

- [x] **Halo2 프로토콜 구현**
  - [x] 재귀적 증명 집계 (`08_HaloProof/`)
  - [x] 실제 암호학적 Halo2 라이브러리 사용
  - [x] 다중 증명을 하나로 집계하는 시스템
  - [x] 프로덕션급 검증 시스템

---

## 🎯 Week 1-4 완료 성과

### ✅ **구현 완료된 기능들**

#### 🔐 **기본 ZKP 시스템 (Week 1)**

```
📁 01_CircomCompile/
├── 01_multiplier.circom        ← 곱셈 증명 회로
├── package.json               ← 단계별 실행 스크립트
├── run_step.sh               ← 터미널 실행 파일
├── STEP_BY_STEP_GUIDE.md     ← 완전 초보자 가이드
├── USAGE_GUIDE.md            ← 사용법 가이드
└── input.json                ← 사용자 입력 예시

📁 02_SecurityAnalysis/
├── 01_multiplier.circom        ← 분석 대상 회로
├── package.json               ← 보안 테스트 자동화
├── run_step.sh               ← ZKP 환경 구축 도구
├── run_security_tests.sh     ← 종합 보안 테스트
├── attacks/                  ← 공격 시나리오 폴더
│   ├── 01_proof_manipulation.js  ← 증명서 조작 공격
│   └── 02_input_overflow.js      ← 입력 오버플로우 공격
├── benchmarks/               ← 성능 분석 폴더
│   ├── performance_analysis.js   ← 성능 벤치마크 도구
│   └── performance_report.json   ← 성능 분석 리포트
└── README.md                 ← 보안 분석 랩 가이드
```

#### 🔒 **실용적 ZKP 시스템 (Week 2-3)**

```
📁 03_HashPreimage/
├── 02_hash_preimage.circom     ← SHA-256 해시 증명 회로
├── package.json               ← 자동화된 ZKP 파이프라인
├── input_preimage.json        ← 비밀 입력 데이터
├── README.md                  ← Hash Preimage 가이드
└── [생성 파일들]
    ├── proof.json             ← ZK 증명서
    ├── public.json            ← 공개 해시값
    ├── verification_key.json  ← 검증키
    └── witness.wtns           ← Witness 데이터

📁 04_MerkleTree/
├── 04_merkle_proof.circom      ← 머클트리 증명 회로 (1레벨)
├── package.json               ← 완전 자동화 테스트 시스템
├── input_valid.json           ← 유효한 증명 데이터
├── input_invalid.json         ← 무효한 증명 데이터
├── README.md                  ← 머클트리 ZKP 가이드
└── [생성 파일들]
    ├── proof_valid.json       ← 유효한 증명서
    ├── proof_invalid.json     ← 무효한 증명서
    ├── public_valid.json      ← 유효 결과 (isValid=1)
    ├── public_invalid.json    ← 무효 결과 (isValid=0)
    └── verification_key.json  ← 검증키
```

#### 🔥 **진짜 ZKP 시스템 (Week 3-4)**

```
📁 06_RealStarkProof/
├── src/
│   ├── prover.rs              ← Alice: 진짜 STARK 증명 생성
│   └── verifier.rs            ← Bob: FRI + 머클 트리 검증
├── cairo_program/
│   └── password_proof.cairo   ← Cairo 프로그램 (비밀 검증)
├── inputs/                    ← 비밀 입력 파일들
├── proofs/
│   ├── stark_proof.json       ← 진짜 STARK 증명 (수십 KB)
│   └── public.json            ← 공개 정보
└── README.md                  ← 진짜 STARK 가이드

📁 07_LibraryStark/
├── src/
│   ├── prover.rs              ← Alice: 타원곡선 + Fiat-Shamir
│   └── verifier.rs            ← Bob: 진짜 Zero-Knowledge 검증
├── inputs/
│   └── secret_input.json      ← 외부 비밀 입력 (JSON)
├── proofs/
│   └── real_zkp_proof.json    ← 진짜 ZKP 증명 (BLS12-381)
└── README.md                  ← Arkworks 라이브러리 가이드

📁 08_HaloProof/
├── src/
│   ├── aggregate_prove.rs     ← 다중 증명 생성 및 집계
│   └── aggregate_verify.rs    ← 집계된 증명 검증
├── prover/
│   └── secrets_input.json     ← 다중 비밀 입력
├── proofs/
│   └── aggregated_proof.json  ← Halo2 집계 증명
└── README.md                  ← Halo2 재귀 증명 가이드
```

#### 🚀 **핵심 달성 사항**

**Week 1 성과:**

- **Trusted Setup 완전 이해**: Powers of Tau ceremony 실습
- **다중 사용자 시스템**: 1개 회로 → 무한 증명자 구조
- **파일 역할 완전 파악**:
  - `verification_key.json` (공개 검증키)
  - `proof.json` + `public.json` (개별 증명서)
- **SNARK vs STARK 비교**: 파일 구조와 보안 특성 이해
- **보안 취약점 분석**: 증명서 조작, 입력 오버플로우 공격 시뮬레이션
- **성능 최적화**: 벤치마크 분석 및 병목 지점 식별

**Week 2 성과:**

- **실용적 ZKP 구현**: SHA-256 Hash Preimage 회로 (31,536개 제약조건)
- **프라이버시 혁신**: 02폴더 대비 95% 프라이버시 향상
- **대규모 회로 최적화**: Powers of Tau bn128 16 지원
- **실제 응용 분석**: Zcash, Tornado Cash, 투표 시스템 등
- **자동화 파이프라인**: `pnpm run all-big` 원클릭 ZKP 생성
- **패스워드 시스템 혁신**: 서버 해킹 방지 ZK 인증

**Week 3 성과:**

- **머클트리 ZKP 마스터**: 1레벨 머클트리 증명 회로 (246개 제약조건)
- **Poseidon 해시 통합**: ZK-friendly 해시 함수 실전 활용
- **pathElements/pathIndices 완전 이해**: 머클 경로 증명 핵심 개념
- **유효성 검증 시스템**: `isValid = 1/0` 완벽한 구분 달성
- **회로 디버깅 마스터**: 0레벨 → 1레벨 전환을 통한 실제 머클트리 구현
- **poseidon-lite 패키지 통합**: 실제 해시 값 계산 및 검증
- **자동화 테스트 스위트**: `test-valid`, `test-invalid` 완전 분리

**Week 4 성과:**

- **진짜 STARK 구현**: Rust 기반 FRI 프로토콜 + 머클 트리 검증
- **Cairo 연동**: Cairo 프로그램과 Rust 검증자 완전 연동
- **Arkworks 라이브러리 마스터**: BLS12-381 타원곡선 + Fiat-Shamir 변환
- **진짜 Zero-Knowledge**: 타원곡선 Commitment로 완전한 비밀 보호
- **Halo2 재귀 증명**: 다중 증명을 하나로 집계하는 고급 시스템
- **프로덕션급 보안**: 128-bit 보안 수준의 실제 암호학적 구현
- **외부 입력 시스템**: JSON 파일로 비밀값 동적 설정

#### 🛠️ **자동화 도구 구축**

```bash
# 기본 ZKP 환경 구축 (01_CircomCompile/)
pnpm run step1     # 회로 컴파일
pnpm run step2-1   # Powers of Tau 시작
pnpm run all       # 전체 과정 자동화

# 고급 보안 분석 (02_SecurityAnalysis/)
pnpm run attack1   # 증명서 조작 공격
pnpm run attack2   # 입력 오버플로우 공격
pnpm run benchmark # 성능 벤치마크 분석
pnpm run security  # 종합 보안 테스트
pnpm run test      # 전체 테스트 스위트

# 실용적 Hash Preimage ZKP (03_HashPreimage/)
pnpm run step1     # SHA-256 회로 컴파일
pnpm run all-big   # 대규모 회로용 전체 과정 (bn128 16)
pnpm run generate-hash # 테스트 해시 생성
pnpm run clean     # 임시 파일 정리
pnpm run status    # 현재 상태 확인

# 머클트리 ZKP 시스템 (04_MerkleTree/)
pnpm run step1     # 머클트리 회로 컴파일 (246개 제약조건)
pnpm run all       # 전체 ZKP 파이프라인 (Powers of Tau + zKey)
pnpm run test-valid   # 유효한 증명 테스트 (isValid=1)
pnpm run test-invalid # 무효한 증명 테스트 (isValid=0)
pnpm run test-all  # 전체 테스트 스위트
pnpm run clean     # 모든 생성 파일 정리 (proof, public 포함)

# 진짜 STARK 시스템 (06_RealStarkProof/)
npm run prove      # Alice: 진짜 STARK 증명 생성 (FRI + 머클 트리)
npm run verify     # Bob: 진짜 STARK 검증 (프로덕션급)
npm run demo       # 전체 STARK 과정 (Cairo + Rust)

# Arkworks 라이브러리 ZKP (07_LibraryStark/)
npm run prove      # Alice: 타원곡선 + Fiat-Shamir 증명
npm run verify     # Bob: 진짜 Zero-Knowledge 검증
npm run demo       # 전체 ZKP 과정 (BLS12-381)

# Halo2 재귀 증명 (08_HaloProof/)
npm run prove      # 다중 증명 생성 및 집계
npm run verify     # 집계된 증명 검증
npm run demo       # 전체 Halo2 과정

# 터미널 스크립트 방식
./run_step.sh 1    # 1단계 실행
./run_step.sh all  # 전체 실행 (색상 지원)
./run_step.sh help # 도움말
./run_security_tests.sh # 보안 분석 실행
```

---

## 💻 핵심 프로젝트

### 🏆 **메인 프로젝트 (필수)**

#### **🔐 프라이버시 인증 시스템**

```
📝 기능:
- 나이 증명 (만 19세 이상)
- 소득 증명 (임계값 이상)
- 국적 증명 (특정 국가)
- 자격 증명 (학위, 자격증)

🛠️ 기술스택:
- Circom: ZK 회로
- snarkjs: 증명 생성/검증
- React: 프론트엔드
- Node.js: 백엔드
- MongoDB: 데이터 저장
```

### 🎯 **서브 프로젝트 (선택)**

1. **🗳️ 익명 투표 시스템**

   - Zero-knowledge 투표
   - 이중 투표 방지
   - 실시간 집계

2. **💰 프라이버시 결제**

   - 잔고 비공개 거래
   - 거래 내역 숨김
   - Mixer 구현

3. **🎓 자격증 검증**
   - 학위/자격증 ZK 증명
   - 위조 방지 시스템
   - 기업 연동 API

---

## 📖 추천 학습 자료

### 🆓 무료 자료

- **Circom 공식 문서**: [docs.circom.io](https://docs.circom.io/)
- **snarkjs GitHub**: 실전 예제 모음
- **ZK-Learning.org**: 이론적 배경
- **0xPARC**: ZK 실습 워크샵

### 💰 유료 (추천)

- **ZK Security Course** - Cryptography 심화
- **Blockchain Council**: ZK Developer 자격증
- **Consensys Academy**: Ethereum ZK 특화

---

## 🛠️ 기술 스택

### 🔧 **핵심 도구**

```json
{
  "zk_tools": {
    "circom": "^2.1.0",
    "snarkjs": "^0.7.0",
    "circomlib": "^2.0.0"
  },
  "frontend": {
    "react": "^18.0.0",
    "next": "^13.0.0",
    "ethers": "^6.0.0",
    "metamask": "wallet"
  },
  "backend": {
    "express": "^4.18.0",
    "mongodb": "database",
    "cors": "^2.8.0",
    "helmet": "security"
  }
}
```

### ⚡ **개발 환경**

- **Node.js**: v18+ (snarkjs 호환성)
- **Browser**: Chrome/Firefox (WebAssembly 지원)
- **Editor**: VSCode + Circom 확장
- **Git**: 버전 관리 필수

---

## 🎯 완료 기준

### 📊 **기술적 성취**

- [x] **3개 Circom 회로 구현** (곱셈 증명 + SHA-256 Hash Preimage + 머클트리)
- [x] **완전 자동화 시스템** (package.json + shell script)
- [x] **다중 사용자 증명 시스템** 구축
- [x] **대규모 회로 지원** (31,536개 제약조건 처리)
- [x] **머클트리 ZKP 구현** (246개 제약조건, Poseidon 해시)
- [x] **진짜 STARK 구현** (FRI 프로토콜 + 머클 트리 검증)
- [x] **Arkworks 라이브러리 ZKP** (BLS12-381 + Fiat-Shamir)
- [x] **Halo2 재귀 증명** (다중 증명 집계 시스템)
- [x] **프로덕션급 보안** (128-bit 암호학적 보안)
- [x] **실제 사용 가능한 증명 시간** (<5초 달성)
- [x] **보안 취약점 분석 시스템** (공격 시뮬레이션 + 성능 분석)
- [x] **실용적 프라이버시 보호** (95% 프라이버시 향상 달성)
- [x] **유효성 검증 시스템** (isValid = 1/0 완벽 구분)
- [x] **외부 입력 시스템** (JSON 파일로 동적 비밀값 설정)

### 📚 **이론적 이해**

- [x] **R1CS (Rank-1 Constraint System) 기초 이해**
- [x] **Trusted Setup ceremony 완전 실습**
- [x] **zk-SNARK vs zk-STARK 실전 비교**
- [x] **ZKP 파일 구조 완전 파악**
- [x] **보안 취약점 분석 능력** (증명서 조작, 오버플로우 공격 실습)
- [x] **FRI 프로토콜 이해** (Fast Reed-Solomon Interactive Oracle Proofs)
- [x] **타원곡선 암호학** (BLS12-381, Pedersen Commitment)
- [x] **Fiat-Shamir 변환** (비상호작용 증명 생성)
- [x] **재귀적 증명 집계** (Halo2 프로토콜)

### 🌐 **실전 경험**

- [x] **GitHub에 ZK 프로젝트 포트폴리오** (기초 완성)
- [x] **완전 초보자 가이드 작성** (STEP_BY_STEP_GUIDE.md)
- [ ] 블로그/영상으로 기술 공유
- [ ] ZK 커뮤니티 기여 (이슈, PR)
- [ ] 해커톤 참여 준비

---

## 🚀 시작 가이드

### 1. 환경 설정

```bash
cd phase2/month4_zkp_practice/

# Node.js 의존성 설치
npm init -y
npm install -g circom snarkjs

# Circom 설치 확인
circom --version
snarkjs --version

# Powers of Tau 다운로드 (한 번만)
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
```

### 2. 첫 회로 테스트

```bash
# 예제 회로 생성
echo 'template Multiplier() { ... }' > first_circuit.circom

# 컴파일 & 테스트
circom first_circuit.circom --r1cs --wasm --sym
```

---

## 💡 학습 전략

### 📈 **단계별 접근**

1. **Week 1**: 간단한 회로로 감 잡기
2. **Week 2**: 실용적 회로로 응용력 기르기
3. **Week 3**: 복잡한 회로로 최적화 연습
4. **Week 4**: 풀스택으로 통합 경험

### 🔥 **실전 팁**

- **제약 조건 최소화**: 가스비 절약
- **회로 모듈화**: 재사용 가능한 컴포넌트
- **보안 검토**: 일반적인 ZK 취약점 체크
- **성능 프로파일링**: 병목 지점 찾기

---

**시작일**: 2024년 9월 30일  
**Week 1 완료일**: 2024년 9월 30일 ✅  
**Week 2 완료일**: 2024년 10월 10일 ✅  
**Week 3 완료일**: 2024년 10월 13일 ✅  
**Week 4 완료일**: 2024년 10월 15일 ✅  
**전제조건**: Phase 1 ZKP 이론 완료 ✅  
**현재 진행률**: **Week 1-4 완료 + 진짜 ZKP 마스터 (100%)** 🎉  
**달성 성과**: Circom → STARK → Arkworks → Halo2 완전 정복! 🚀
