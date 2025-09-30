# 🎯 ZKP 완전 초보자 가이드

## 📋 목차

1. [파일 구조 이해하기](#파일-구조-이해하기)
2. [단계별 실행 과정](#단계별-실행-과정)
3. [각 명령어가 하는 일](#각-명령어가-하는-일)
4. [실제 실행해보기](#실제-실행해보기)

---

## 📁 파일 구조 이해하기

### 🔧 내가 작성하는 파일들

```
01_multiplier.circom    ← 내가 만든 ZK 회로 (a * b = c 증명)
input.json             ← 내 비밀 값들 (a=3, b=11)
```

### 🤖 컴파일러가 만드는 파일들

```
01_multiplier.r1cs     ← 회로를 수학 공식으로 변환한 파일
01_multiplier.sym      ← 디버깅용 심볼 파일
01_multiplier_js/      ← JavaScript로 변환된 회로
  ├── 01_multiplier.wasm        ← 실제 계산하는 WebAssembly 파일
  ├── witness_calculator.js     ← 계산 도구
  └── generate_witness.js       ← 증인 생성 스크립트
```

### 🔐 ZKP 시스템이 만드는 파일들

```
witness.wtns           ← 내 비밀로 만든 "증인" 파일
proof.json            ← 최종 증명서
public.json           ← 공개되는 결과 (33)
verification_key.json  ← 검증용 열쇠
```

---

## 🚀 단계별 실행 과정

### 1단계: 회로 컴파일

```bash
# 회로 컴파일 (기본)
# 의미: 내가 작성한 ZK 회로(.circom)를 컴퓨터가 실행할 수 있는 형태로 변환
# 비유: 한국어로 쓴 수학 문제를 컴퓨터 언어로 번역
circom 01_multiplier.circom --r1cs --wasm --sym

# 특정 폴더에 출력하고 싶을 때
# 의미: 컴파일 결과물을 지정한 폴더에 정리해서 저장
# 비유: 번역한 문서들을 특정 서랍에 정리해서 보관
mkdir -p 폴더명 && circom 01_multiplier.circom --r1cs --wasm --sym -o 폴더명
```

📁 생성된 파일들:

```
01_multiplier.r1cs        (264B) ← 수학적 제약 조건 (바이너리)
01_multiplier.sym         (39B)  ← 디버깅용 심볼 정보
01_multiplier_js/         ← JavaScript 실행 환경
  ├── 01_multiplier.wasm  (34KB) ← 실제 계산 엔진 (WebAssembly)
  ├── generate_witness.js (697B) ← 증인 생성 도구
  └── witness_calculator.js (10KB) ← WASM 로더
```

🎯 핵심 요약:
회로 컴파일 = 사람이 읽을 수 있는 ZK 회로를 기계가 실행할 수 있게 변환
목적: Circom 언어로 작성한 곱셈 증명 로직을 실행 가능한 형태로 만들기
방법: .circom 파일 → .r1cs (수학 공식) + .wasm (실행 파일) + 도구들
보안: 회로 로직 자체는 공개되어도 됨 (비밀은 나중에 입력할 데이터)
결과: ZK 증명을 생성할 수 있는 모든 도구 준비 완료

**무슨 일?** 내가 쓴 회로를 컴퓨터가 이해할 수 있게 변환

### 2단계: 신뢰할 수 있는 설정 (Trusted Setup)

```bash
# Powers of Tau 시작 1️⃣ bn128 타원곡선 "도구" 불러오기 ✅
# 의미: 빈 의식에서 시작해서 첫 번째 해시 생성
# 비유: 빈 투표함 준비 완료
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v

# 기여하기 (랜덤성 추가) 2️⃣ 엔트로피 넣어서 복잡하게 만들기 ✅
# 의미: 내 비밀 엔트로피를 넣어서 랜덤성 추가
# 비유: 투표함에 내 자물쇠 추가
# 비밀키 "사용자만 아는 비밀키"
echo "사용자만 아는 비밀키" | snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v

# Phase 2 준비 3️⃣ 완성된 암호파일 (타원곡선 기반) ✅
# 의미: 범용 → 회로 전용으로 변환하는 복잡한 수학 연산
# 비유: 일반 투표함 → 특정 선거용 투표함으로 개조
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v
```

📊 각 부분의 의미:

```
pot12_0000.ptau
├── pot = Powers of Tau
├── 12 = 보안 매개변수 (2^12 = 4,096)
├── 0000 = 기여 단계 번호
└── .ptau = Powers of Tau 파일 확장자
```

📁 생성된 파일들:

```
pot12_0000.ptau  (1.5MB) ← 초기 빈 의식
pot12_0001.ptau  (1.5MB) ← 내 기여가 추가된 버전
pot12_final.ptau (4.7MB) ← Phase 2 준비 완료된 최종 버전
```

🎯 핵심 요약:
Trusted Setup = 여러 사람이 순서대로 비밀을 섞어서 만드는 안전한 공통 기준
목적: 아무도 혼자서는 조작할 수 없는 ZKP 기반 구축
방법: 각자 비밀 엔트로피 기여 → 즉시 삭제 → 결과만 공유
보안: 참여자 중 1명이라도 정직하면 전체 시스템 안전
결과: 모두가 신뢰할 수 있는 공통 참조 문자열

**무슨 일?** 안전한 ZKP를 위한 공통 매개변수 생성

### 3단계: 회로별 키 생성

```bash
# 회로용 키 생성
# 의미: 범용 Powers of Tau를 특정 회로(곱셈)에 맞게 변환
# 비유: 일반 자물쇠를 우리 집 문에 맞는 전용 자물쇠로 제작
snarkjs groth16 setup 01_multiplier.r1cs pot12_final.ptau multiplier_0000.zkey

# 키에 기여하기 (회로별 추가 보안)
# 의미: 회로 전용 키에도 내 랜덤성을 추가해서 더욱 안전하게 만듦
# 비유: 전용 자물쇠에 내만의 추가 보안 장치 설치
# 비밀키 "사용자만 아는 비밀키" (2단계 Trusted Setup이랑 달라도 상관은 없음)
echo "사용자만 아는 비밀키" | snarkjs zkey contribute multiplier_0000.zkey multiplier_final.zkey --name="Key contribution" -v

# 검증 키 추출
# 의미: 증명을 검증할 때 사용할 공개 키를 별도 파일로 추출
# 비유: 자물쇠는 비밀로 하고, 열쇠 확인용 틀만 공개
snarkjs zkey export verificationkey multiplier_final.zkey verification_key.json
```

📁 생성된 파일들:

```
multiplier_0000.zkey     (2.6KB) ← 초기 회로 전용 키
multiplier_final.zkey    (2.6KB) ← 내 기여가 추가된 최종 키
verification_key.json    (1.2KB) ← 검증 전용 공개 키
```

🎯 핵심 요약:
회로별 키 생성 = 범용 도구를 특정 작업(곱셈 증명)에 맞게 전문화
목적: 우리 회로만을 위한 전용 증명/검증 키 쌍 생성
방법: Powers of Tau + 회로 정보 → 회로 전용 키 + 추가 랜덤성 기여
보안: 회로별로도 추가 엔트로피를 넣어서 이중 보안
결과: 곱셈 증명 전용 키와 검증 키 완성

**무슨 일?** 이 회로 전용 암호화 키들 생성

### 4단계: 증명 생성

```bash
# 증인 계산 (내 비밀 사용)
# 의미: 내 비밀 입력(a=3, b=11)으로 회로의 모든 중간 계산값 생성
# 비유: 시험 답안지 작성 (모든 계산 과정과 답을 적음)
snarkjs wtns calculate 01_multiplier_js/01_multiplier.wasm input.json witness.wtns

# 증명 생성
# 의미: 답안지(witness)와 증명 키를 사용해서 ZK 증명서 생성
# 비유: 답안지를 암호화해서 "정답을 안다"는 증명서 만들기
snarkjs groth16 prove multiplier_final.zkey witness.wtns proof.json public.json
```

📁 생성된 파일들:

```
witness.wtns    (204B) ← 내 비밀로 계산한 모든 중간값 (비밀!)
proof.json      (1.1KB) ← ZK 증명서 (공개 가능)
public.json     (9B)   ← 공개되는 결과값 ["33"]
```

🎯 핵심 요약:
증명 생성 = 비밀을 숨긴 채로 "정답을 안다"는 증명서 작성
목적: 3과 11을 공개하지 않고도 "33을 만드는 두 수를 안다" 증명
방법: 비밀 입력 → 증인 계산 → 암호학적 증명 생성
보안: 증명서만 봐서는 원래 비밀(3, 11)을 절대 알 수 없음
결과: 작고 빠르게 검증 가능한 ZK 증명 완성

**무슨 일?** 내 비밀(3, 11)로 "33을 만들 수 있다"는 증명서 작성

### 5단계: 검증

```bash
# 증명 검증
# 의미: 검증 키, 공개 결과, 증명서를 사용해서 증명이 유효한지 확인
# 비유: 암호화된 답안지가 정말 정답인지 채점하기 (답은 모르지만 맞는지 확인)
snarkjs groth16 verify verification_key.json public.json proof.json
```

📊 검증 과정:

```
입력 파일들:
verification_key.json  ← 검증용 공개 키 (3단계에서 생성)
public.json           ← 공개 결과 ["33"]
proof.json            ← ZK 증명서 (4단계에서 생성)

검증 결과:
✅ OK! (증명 유효) 또는 ❌ Invalid (증명 무효)
```

🎯 핵심 요약:
검증 = 비밀을 모르는 상태에서 증명이 올바른지 수학적으로 확인
목적: 증명자가 정말로 정답을 아는지 확인 (하지만 정답은 여전히 비밀)
방법: 검증 키 + 공개 결과 + 증명서 → 수학적 검증 알고리즘
보안: 검증 과정에서도 원래 비밀(3, 11)은 절대 노출되지 않음
결과: "이 사람은 정말로 33을 만드는 두 수를 안다" 확신

**무슨 일?** 증명서가 진짜인지 확인

---

## 🔍 각 명령어가 하는 일

### `input.json`이 왜 저렇게 생겼나?

```json
{
  "a": "3",
  "b": "11"
}
```

- **이유**: Circom 회로의 `signal input a`, `signal input b`와 정확히 매칭
- **규칙**: 회로에서 정의한 입력 신호 이름과 동일해야 함
- **타입**: 문자열로 써야 함 (숫자 아님!)

### `01_multiplier_js/` 폴더는 뭐야?

- **생성**: `circom` 명령어가 `--wasm` 옵션으로 자동 생성
- **역할**: JavaScript에서 회로를 실행할 수 있게 해주는 도구들
- **핵심**: `01_multiplier.wasm` - 실제 계산을 수행하는 파일

### `witness.wtns`는 뭐야?

- **정의**: 내 비밀 입력으로 회로의 모든 중간 계산값을 담은 파일
- **내용**: a=3, b=11, c=33, 그리고 모든 중간 계산 결과
- **중요**: 이 파일 자체는 비밀이 들어있음! 하지만 증명에서는 숨겨짐

---

## 🎮 실제 실행해보기

### 단계별 실행

```bash
# 1. 회로 컴파일
circom 01_multiplier.circom --r1cs --wasm --sym

# 2. 신뢰 설정 (시간 좀 걸림)
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
echo "사용자만 아는 비밀키" | snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v

# 3. 키 생성
snarkjs groth16 setup 01_multiplier.r1cs pot12_final.ptau multiplier_0000.zkey
echo "사용자만 아는 비밀키" | snarkjs zkey contribute multiplier_0000.zkey multiplier_final.zkey --name="Key contribution" -v
snarkjs zkey export verificationkey multiplier_final.zkey verification_key.json

# 4. 증명 생성
snarkjs wtns calculate 01_multiplier_js/01_multiplier.wasm input.json witness.wtns
snarkjs groth16 prove multiplier_final.zkey witness.wtns proof.json public.json

# 5. 검증
snarkjs groth16 verify verification_key.json public.json proof.json
```

### 결과 확인

```bash
# 공개 결과 보기
cat public.json
# 출력: ["33"]

# 증명서 크기 확인
ls -la proof.json
# 매우 작은 파일이지만 강력한 증명!
```

---

## 🎯 핵심 포인트

1. **`input.json`**: 내 비밀, 회로 입력과 정확히 매칭되어야 함
2. **`01_multiplier_js/`**: 컴파일러가 자동 생성, 건드리지 말 것
3. **실행 순서**: 컴파일 → 신뢰설정 → 키생성 → 증명생성 → 검증
4. **최종 목표**: `public.json`에 33, 하지만 3과 11은 비밀 유지

## 전체과정

- 1단계: 문제 컴파일 ✅
- 2단계: 안전한 암호화 기반 구축 (매우 중요!) ✅
- 3단계: 문제별 전용 암호화 ✅
- 4단계: 비밀 입력으로 증명 생성 ✅
- 5단계: 검증자가 "이새끼는 아는구만!" 확인 ✅
