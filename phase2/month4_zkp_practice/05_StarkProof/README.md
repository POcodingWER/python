# 🔥 STARK 증명 - Cairo 실행 기반

## 🎯 STARK 증명 전체 흐름

### 📝 증명자 (Alice) → 🔍 검증자 (Bob/블록체인)

```
📝 src/lib.cairo (비밀번호 해시 증명 로직)
    ↓ scarb build
🔧 target/dev/merkle_stark_proof.sierra.json (컴파일된 바이트코드)
    ↓ shasum -a 256
🔑 programHash: 0x0622d242... (프로그램 무결성 증명)
    ↓ scarb cairo-run --arguments-file
🚀 executionTrace: [272, 276] (실제 계산 과정)
    ↓ simple_hash(secret) 계산
🌍 publicHash: 54452 (공개 해시값)
    ↓
📦 proof = {programHash, executionTrace, publicHash}
    ↓ 블록체인 전송
🔍 검증자가 3가지 모두 확인!
   ✅ 올바른 .cairo 프로그램 사용?
   ✅ 실행 과정이 정확?
   ✅ 결과 해시값이 맞음?
```

**🔒 핵심:** Alice는 비밀(7777)을 숨기면서도 "해시값 54452의 원본을 안다"고 증명!

## 🚀 STARK 과정

### 1️⃣ Cairo 프로그램 작성

```cairo
// src/lib.cairo - 외부 입력 받는  구조
fn main() -> Array<felt252> {
    // 실제 암호학적 계산
    // Poseidon 해시 사용
    // 결과를 배열로 반환
}
```

### 2️⃣ Scarb로 컴파일

```bash
scarb build
# target/dev/*.sierra.json 생성
```

### 3️⃣ 외부 입력으로 실행

```bash
# 비밀을 환경변수로 전달
SECRET=1234 scarb cairo-run
# 또는 파일에서 읽기
```

### 4️⃣ 결과를 증명으로 변환

```bash
# Cairo 실행 결과를 STARK 증명 형태로 포맷
# 실제 Stone Prover 없이도 원리 구현
```

## 📁 새로운 폴더 구조

```
05_StarkProof/
├── README.md                # 이 파일
├── Scarb.toml              # Cairo 프로젝트 설정
├── src/
│   └── lib.cairo           # 🔥  Cairo 프로그램
├── prover/
│   ├── cairo_input_secret.json  # 🔒 Alice의 비밀 (JSON 형태)
│   └── generate.sh              # 증명 생성 스크립트
├── verifier/
│   ├── public_hash.txt     # 🌍 공개 해시값
│   └── verify.sh           # 검증 스크립트
└── proofs/
    ├── execution_trace.json # Cairo 실행 추적
    └── proof_result.json   # 증명 결과
```

## 🎯 핵심 차이점

### ❌ 이전 (가짜):

```python
# Python으로 가짜 증명
proof = {"fake": "simulation"}
```

### ✅ 이번:

```bash
# Cairo로  실행
scarb cairo-run
# returning [1] 또는 [0]
```

## 🔒 영지식 원칙

**증명자 (Alice):**

- Cairo 소스코드 작성
- 비밀을 외부에서 입력
- 실행 결과만 공개

**검증자 (Bob):**

- 소스코드 못 봄! 🚫
- 실행 결과만 받음
- 비밀 절대 모름! 🤐

## 🌐 블록체인 배포 (실제 활용)

### 🚀 Alice의 증명 생성 & 전송

```javascript
// 1️⃣ 오프체인에서 증명 생성
const proof = {
  programHash:
    "0xfa2b0c963615ac5742f866a784a80650ad8072f60d7528a10a13e4ec8d52fd83",
  executionTrace: [272, 276],
  publicHash: 54452,
};

// 2️⃣ 블록체인에 트랜잭션 전송
await contract.submitProof(proof);
```

### 🔍 블록체인 검증자들의 과정

```solidity
contract StarkPasswordVerifier {
    // 📁 Alice가 만든 프로그램 해시 저장
    bytes32 public constant EXPECTED_PROGRAM_HASH =
        0xfa2b0c963615ac5742f866a784a80650ad8072f60d7528a10a13e4ec8d52fd83;

    function verifyStarkProof(
        bytes32 programHash,
        uint256[] calldata executionTrace,
        uint256 publicHash
    ) external returns (bool) {
        // 1️⃣ Alice가 올바른 프로그램 사용했나?
        require(programHash == EXPECTED_PROGRAM_HASH, "Wrong program!");

        // 2️⃣ 실행 추적이 올바른가?
        require(executionTrace[0] == 272, "Invalid step 1");
        require(executionTrace[1] == 276, "Invalid step 2");

        // 3️⃣ 해시값이 유효한가?
        require(publicHash == 54452, "Invalid hash");

        // ✅ 모든 검증 통과!
        return true;
    }
}
```

### 🎯 영지식 증명의 핵심

**🔒 Alice가 증명하는 것:**

- "나는 해시값 54452의 원본 비밀번호를 알고 있어!"
- 하지만 비밀번호 자체는 절대 공개하지 않음

**🔍 블록체인이 검증하는 것:**

- ✅ 올바른 프로그램으로 계산했나?
- ✅ 실행 과정이 정확한가?
- ✅ 결과 해시값이 맞나?

**🚫 블록체인이 알 수 없는 것:**

- Alice의 실제 비밀번호 (7777)
- 계산 과정의 중간값들
- 비밀 정보는 완전히 숨겨짐!

---

**🚀 이제 STARK를 구현해보자!**
