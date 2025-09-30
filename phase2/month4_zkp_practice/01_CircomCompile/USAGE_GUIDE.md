# 🎯 ZKP 단계별 실행 가이드

## 🚀 두 가지 실행 방법

### 📦 방법 1: pnpm 사용 (추천)

```bash
# 📋 도움말 보기
pnpm run help

# 🔥 단계별 실행
pnpm run step1      # 1단계: 회로 컴파일
pnpm run step2-1    # 2-1단계: Powers of Tau 시작
pnpm run step2-2    # 2-2단계: 엔트로피 기여
pnpm run step2-3    # 2-3단계: Phase 2 준비
pnpm run step3-1    # 3-1단계: ZKey 생성
pnpm run step3-2    # 3-2단계: ZKey 기여
pnpm run step3-3    # 3-3단계: 검증키 추출
pnpm run step4      # 4단계: Witness 계산
pnpm run step5-1    # 5-1단계: 증명 생성
pnpm run step5-2    # 5-2단계: 증명 검증

# 🎉 한 번에 모든 단계 실행
pnpm run all

# 🧹 정리
pnpm run clean

# 📊 상태 확인
pnpm run status
```

### 🖥️ 방법 2: 터미널 스크립트 사용

```bash
# 📋 도움말 보기
./run_step.sh help

# 🔥 단계별 실행
./run_step.sh 1      # 1단계: 회로 컴파일
./run_step.sh 2-1    # 2-1단계: Powers of Tau 시작
./run_step.sh 2-2    # 2-2단계: 엔트로피 기여
./run_step.sh 2-3    # 2-3단계: Phase 2 준비
./run_step.sh 3-1    # 3-1단계: ZKey 생성
./run_step.sh 3-2    # 3-2단계: ZKey 기여
./run_step.sh 3-3    # 3-3단계: 검증키 추출
./run_step.sh 4      # 4단계: Witness 계산
./run_step.sh 5-1    # 5-1단계: 증명 생성
./run_step.sh 5-2    # 5-2단계: 증명 검증

# 🎉 한 번에 모든 단계 실행
./run_step.sh all

# 🧹 정리
./run_step.sh clean

# 📊 상태 확인
./run_step.sh status
```

## 🎮 초보자 추천 실행 순서

### 🥇 첫 번째 실행 (학습용)

```bash
# 1️⃣ 현재 상태 확인
pnpm run status

# 2️⃣ 1단계부터 차근차근
pnpm run step1      # 컴파일 결과 확인
pnpm run step2-1    # Powers of Tau 시작
pnpm run step2-2    # 엔트로피 기여
pnpm run step2-3    # Phase 2 준비

# 3️⃣ 중간 상태 확인
pnpm run status

# 4️⃣ 나머지 단계 진행
pnpm run step3-1    # ZKey 생성
pnpm run step3-2    # ZKey 기여
pnpm run step3-3    # 검증키 추출
pnpm run step4      # Witness 계산
pnpm run step5-1    # 증명 생성
pnpm run step5-2    # 검증

# 5️⃣ 최종 상태 확인
pnpm run status
```

### 🥈 두 번째 실행 (빠른 테스트)

```bash
# 🧹 정리 후 한 번에 실행
pnpm run clean
pnpm run all
```

## 🔧 문제 해결

### ❌ 에러가 발생했을 때

```bash
# 1️⃣ 현재 상태 확인
pnpm run status

# 2️⃣ 정리 후 다시 시작
pnpm run clean
pnpm run step1

# 3️⃣ 특정 단계만 다시 실행
pnpm run step2-1    # 예: 2-1단계만 다시
```

### 📁 필요한 파일들

실행 전 확인해야 할 파일들:

- ✅ `01_multiplier.circom` - 회로 파일
- ✅ `input.json` - 입력 데이터
- ✅ `package.json` - 스크립트 설정
- ✅ `run_step.sh` - 터미널 스크립트

## 🎯 각 단계별 생성 파일

| 단계 | 생성되는 파일                                    | 설명                  |
| ---- | ------------------------------------------------ | --------------------- |
| 1    | `*.r1cs`, `*.wasm`, `*.sym`, `01_multiplier_js/` | 컴파일된 회로         |
| 2-1  | `pot12_0000.ptau`                                | 초기 Powers of Tau    |
| 2-2  | `pot12_0001.ptau`                                | 엔트로피 추가된 파일  |
| 2-3  | `pot12_final.ptau`                               | 최종 Powers of Tau    |
| 3-1  | `multiplier_0000.zkey`                           | 초기 ZKey             |
| 3-2  | `multiplier_final.zkey`                          | 최종 ZKey             |
| 3-3  | `verification_key.json`                          | 검증키                |
| 4    | `witness.wtns`                                   | Witness 파일          |
| 5-1  | `proof.json`, `public.json`                      | 증명과 공개 입력      |
| 5-2  | -                                                | 검증 결과 (콘솔 출력) |

## 🎉 성공 메시지

모든 단계가 성공하면 마지막에 이런 메시지가 나타납니다:

```
[INFO]  snarkJS: OK!
🎉 검증 완료! ZKP 성공!
```

이제 **단계별로 천천히 실행해보세요!** 🚀

