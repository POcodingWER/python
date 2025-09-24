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

### Week 1: Circom 기초 & 환경 설정

- [ ] **Circom 환경 구축**

  - [ ] Node.js & npm 설치
  - [ ] Circom 컴파일러 설치
  - [ ] snarkjs 라이브러리 설치
  - [ ] Powers of Tau 세팅

- [ ] **첫 ZK 회로 작성**
  - [ ] 간단한 곱셈 회로
  - [ ] Witness 생성 및 증명
  - [ ] Verification key 관리
  - [ ] Python과 비교 분석

### Week 2: 실용적 ZK 회로 구현

- [ ] **Hash Preimage 회로**

  - [ ] SHA-256 회로 구현
  - [ ] Hash 충돌 방지 증명
  - [ ] 패스워드 검증 시스템
  - [ ] Phase 1 해시 구현과 비교

- [ ] **Merkle Tree 회로**
  - [ ] Merkle proof 검증 회로
  - [ ] 데이터 무결성 증명
  - [ ] 대용량 데이터 처리
  - [ ] 블록체인 연동 준비

### Week 3: 고급 ZK 회로 & 최적화

- [ ] **나이 검증 시스템**

  - [ ] Phase 1 Python 버전을 Circom으로 포팅
  - [ ] 범위 증명 (Range Proof)
  - [ ] 개인정보보호 인증
  - [ ] 성능 최적화

- [ ] **투표 시스템**
  - [ ] 익명 투표 회로
  - [ ] 이중 투표 방지
  - [ ] 결과 집계 증명
  - [ ] 투명성과 프라이버시 양립

### Week 4: 풀스택 ZK 웹앱 구축

- [ ] **프론트엔드 (React/Next.js)**

  - [ ] ZK 증명 UI 구성
  - [ ] Wallet 연동 (MetaMask)
  - [ ] 증명 생성 인터페이스
  - [ ] 결과 시각화

- [ ] **백엔드 (Node.js/Express)**
  - [ ] 증명 검증 API
  - [ ] 데이터베이스 연동
  - [ ] 성능 모니터링
  - [ ] 보안 강화

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

- [ ] 5개 이상의 Circom 회로 구현
- [ ] 1개 이상의 풀스택 ZK 웹앱 완성
- [ ] 가스비 최적화된 스마트 컨트랙트
- [ ] 실제 사용 가능한 증명 시간 (<30초)

### 📚 **이론적 이해**

- [ ] R1CS (Rank-1 Constraint System) 완전 이해
- [ ] Trusted Setup ceremony 참여 경험
- [ ] zk-SNARK vs zk-STARK 실전 비교
- [ ] 보안 취약점 분석 능력

### 🌐 **실전 경험**

- [ ] GitHub에 ZK 프로젝트 포트폴리오
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

**시작일**: TBD  
**목표 완료일**: 4주 후  
**전제조건**: Phase 1 ZKP 이론 완료 ✅  
**다음 단계**: Month 5-6 ZKML 🚀
