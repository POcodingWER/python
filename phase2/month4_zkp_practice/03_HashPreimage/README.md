# 🔐 Hash Preimage ZK Circuit

## 📋 개요

**Hash Preimage 증명**은 해시값을 공개하면서, 원본 데이터를 알고 있다는 것을 영지식으로 증명하는 시스템입니다.

### 🎯 실용적 응용 사례

1. **🔑 패스워드 검증**: 서버에 해시만 저장, 사용자가 패스워드 알고 있음을 증명
2. **💰 커밋-리빌 스킴**: 경매에서 입찰가를 숨기고 나중에 공개
3. **🎲 랜덤 시드 증명**: 게임에서 공정한 랜덤성 보장
4. **📝 디지털 서명**: 메시지 무결성 검증

---

## 🔬 회로 구조

### 입력/출력

```
Private Input:  preimage[8]  (256비트 원본 데이터)
Public Output:  hash[8]      (256비트 SHA-256 해시)

증명: SHA256(preimage) == hash
```

### 핵심 컴포넌트

- **SHA256**: circomlib의 SHA-256 구현
- **Num2Bits/Bits2Num**: 숫자↔비트 변환
- **HashPreimage**: 메인 증명 템플릿
- **PasswordVerify**: 패스워드 검증 특화 템플릿

---

## 🚀 사용법

### 1. 환경 설정

```bash
cd 03_HashPreimage/
pnpm install
```

### 2. 테스트 해시 생성

```bash
# "Hello World!" 문자열의 SHA-256 해시 확인
pnpm run generate-hash
```

### 3. 전체 ZKP 과정 실행

```bash
# 모든 단계 자동 실행
pnpm run all

# 또는 단계별 실행
pnpm run step1    # 회로 컴파일
pnpm run step2-1  # Powers of Tau
pnpm run step5    # 최종 검증
```

### 4. 사용자 정의 입력 테스트

```bash
# input_hash.json 수정 후
pnpm run step4-1  # Witness 재계산
pnpm run step4-2  # 증명 재생성
pnpm run step5    # 검증
```

---

## 📊 입력 데이터 형식

### input_hash.json 예시

```json
{
  "preimage": [
    "0x48656c6c", // "Hell"
    "0x6f20576f", // "o Wo"
    "0x726c6421", // "rld!"
    "0x00000000", // 패딩
    "0x00000000", // 패딩
    "0x00000000", // 패딩
    "0x00000000", // 패딩
    "0x00000000" // 패딩
  ]
}
```

### 문자열 → 16진수 변환

```bash
# "Hello World!" → 16진수
echo -n "Hello World!" | xxd -p
# 결과: 48656c6c6f20576f726c6421

# 8바이트씩 분할
# 48656c6c, 6f20576f, 726c6421, 00000000...
```

---

## 🔍 Phase 1 Python vs Circom 비교

| 항목          | Phase 1 (Python)  | Phase 2 (Circom) | 개선점           |
| ------------- | ----------------- | ---------------- | ---------------- |
| **언어**      | py_ecc            | Circom + snarkjs | 🚀 실전 도구     |
| **성능**      | 교육용 시뮬레이션 | 프로덕션 수준    | ⚡ 10-100배 빠름 |
| **호환성**    | Python 생태계     | 웹/블록체인      | 🌐 범용성        |
| **회로 크기** | 제한 없음         | 최적화 필요      | 🎯 효율성 강화   |
| **검증 시간** | 수 초             | 밀리초 단위      | ⚡ 실시간 검증   |

---

## 🛠️ 고급 활용

### 1. 패스워드 검증 시스템

```circom
template PasswordVerify() {
    signal private input password[4];    // 128비트 패스워드
    signal output password_hash[8];      // 256비트 해시

    // 패스워드를 256비트로 패딩
    // HashPreimage 컴포넌트 활용
}
```

### 2. 다중 해시 증명

```circom
template MultiHashPreimage(n) {
    signal private input preimages[n][8];
    signal output hashes[n][8];

    component hash_provers[n];
    for (var i = 0; i < n; i++) {
        hash_provers[i] = HashPreimage();
        // ...
    }
}
```

### 3. 조건부 해시 증명

```circom
template ConditionalHash() {
    signal private input preimage[8];
    signal private input condition;
    signal output hash[8];

    // condition이 1일 때만 해시 계산
    component conditional = HashPreimage();
    // ...
}
```

---

## 📈 성능 최적화

### 회로 크기 최적화

- **제약 조건 수**: ~28,000개 (SHA-256 기준)
- **가스비**: ~500,000 gas (Ethereum 배포 시)
- **증명 시간**: 2-5초 (로컬 환경)
- **검증 시간**: 10-50ms

### 최적화 팁

1. **입력 크기 최소화**: 불필요한 패딩 제거
2. **해시 함수 선택**: SHA-256 vs Poseidon vs MiMC
3. **병렬 처리**: 다중 해시 동시 계산
4. **회로 재사용**: 공통 컴포넌트 모듈화

---

## 🔒 보안 고려사항

### 잠재적 취약점

1. **Preimage 추측**: 짧은 입력은 브루트포스 가능
2. **사이드 채널**: 실행 시간으로 정보 유출
3. **해시 충돌**: SHA-256 충돌 시 증명 무효화
4. **입력 검증**: 잘못된 형식의 입력 처리

### 보안 강화 방법

1. **솔트 추가**: 레인보우 테이블 공격 방지
2. **최소 엔트로피**: 입력 복잡도 강제
3. **타임아웃**: 브루트포스 시간 제한
4. **감사**: 정기적인 회로 보안 검토

---

## 🎯 다음 단계

Week 2 완료 후:

- [ ] **Merkle Tree 회로** 구현
- [ ] **성능 벤치마크** 비교
- [ ] **웹 인터페이스** 구축
- [ ] **블록체인 연동** 준비

---

**생성일**: 2024년 9월 30일  
**Week 2 시작**: Hash Preimage 마스터하기! 🚀
