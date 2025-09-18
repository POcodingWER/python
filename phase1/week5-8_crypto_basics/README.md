# Week 5-8: 암호학 기초 🔐

## 📋 학습 목표

- 현대 암호학의 핵심 개념 이해
- ZKP(Zero-Knowledge Proof) 기초 다지기
- 실습을 통한 암호화 알고리즘 체험

## 📚 주차별 계획

### Week 5: 해시 함수와 대칭 암호

- [x] **해시 함수** ✅

  - [x] SHA-256, MD5 원리와 특성 (MD5 취약, SHA-256 안전)
  - [x] 일방향성, 충돌 저항성 이해 (단방향, 검증은 원본+nonce로만)
  - [x] Python hashlib 실습 (SHA-256 vs MD5 비교 테스트)
  - [x] 블록체인에서의 활용 (해시 체인, 위조 방지 구현)

- [x] **대칭 암호** ✅
  - [x] AES 기본 원리 (XOR 시뮬레이션으로 개념 이해)
  - [x] ECB, CBC 모드 차이점 (패턴 보임 vs 패턴 숨김)
  - [x] 대칭 vs 공개키 암호 비교 (키 공유 문제 vs 해결책)

### Week 6: 공개키 암호

- [x] **RSA 암호** ✅

  - [x] 수학적 기초 (소수, 모듈러 연산, 소인수분해의 어려움)
  - [x] 키 생성, 암호화, 복호화 과정 (p, q → n, φ(n) → e, d)
  - [x] Python 구현 실습 (소수 생성, RSA 암복호화, 디지털 서명)

- [x] **타원곡선 암호 (ECC)** ✅
  - [x] 타원곡선의 수학적 성질 (y² = x³ + ax + b, 점 덧셈)
  - [x] ECDSA 디지털 서명 (개인키 서명, 공개키 검증)
  - [x] Bitcoin/Ethereum에서의 활용 (거래 서명 vs SHA-256 마이닝)
  - [x] ECC vs RSA 비교 (256비트 vs 2048비트, 효율성)
  - [x] 이산로그 문제 (k×G 쉬움, k 역산 어려움)
  - [x] 점 덧셈 시각화 및 무차별 대입 공격 시뮬레이션

### Week 7: 디지털 서명과 PKI

- [x] **디지털 서명** ✅

  - [x] RSA vs ECDSA 서명 방식 비교 (교육용 vs 실무용)
  - [x] 서명 생성과 검증 과정 (개인키로 서명, 공개키로 검증)
  - [x] 메시지 인증과 부인 방지 원리 (3가지 보장)
  - [x] k(nonce)의 중요성과 보안 위험 (소니 PS3 해킹 사례)
  - [x] 비트코인 ECDSA 거래 서명 실습
  - [x] 블록체인에서의 디지털 서명 활용 (UTXO, 트랜잭션 검증)

- [x] **PKI (Public Key Infrastructure)** ✅
  - [x] 인증서와 CA(Certificate Authority) (디지털 신분증 개념)
  - [x] 신뢰 체인 구조 (Root → Intermediate → End Entity)
  - [x] TLS/SSL 동작 원리 (PKI + 대칭암호 하이브리드)
  - [x] 브라우저 인증서 검증 과정 실습
  - [x] 실제 CA 생태계 이해 (상용/무료/정부)

### Week 8: ZKP 입문

- [x] **Zero-Knowledge의 정의** ✅

  - [x] Completeness (완전성) - 진실은 항상 증명 가능
  - [x] Soundness (건실성) - 거짓은 속일 수 없음
  - [x] Zero-Knowledge (영지식성) - 비밀은 완전 보호

- [x] **알리바바 동굴 예제** ✅

  - [x] Interactive ZKP 이해 (상호작용형 증명)
  - [x] Prover와 Verifier 역할 체험
  - [x] 실제 시뮬레이션 구현 및 확률 분석

- [ ] **zk-SNARKs vs zk-STARKs vs Halo**
  - zk-SNARKs: 작고 빠름 vs Trusted Setup 위험
  - zk-STARKs: 투명하고 양자내성 vs 큰 증명 크기
  - Halo: Setup-free SNARKs의 혁신
  - 실제 블록체인 프로젝트 비교 (Zcash, StarkNet, Mina)

## 📖 추천 학습 자료

### 무료 자료

- **"암호학 첫걸음"** 유튜브 강의
- **Coursera**: Introduction to Applied Cryptography
- **ZK-Learning.org**: ZKP 기초 강의

### 유료 (추천)

- **"Real-World Cryptography"** - David Wong
- **"Serious Cryptography"** - Jean-Philippe Aumasson

## 💻 실습 프로젝트

### 기초 실습

1. **해시 체인 구현** - 간단한 블록체인 ✅ **완료!**
2. **RSA 암호화 시스템** - 메시지 암복호화 ✅ **완료!**
3. **ECC 점 덧셈 시각화** - 타원곡선 무차별 대입 공격 ✅ **완료!**
4. **디지털 서명 시스템** - RSA/ECDSA 서명 및 검증 ✅ **완료!**

### 중급 실습

5. **알리바바 동굴 ZKP** - Interactive 증명 시스템 ✅ **완료!**
6. **zk-SNARKs vs STARKs vs Halo** - 고급 ZKP 비교 분석
7. **Merkle Tree 구현** - 데이터 무결성 증명
8. **Commitment Scheme** - 비밀 값 약속 시스템 ✅ **완료!**

## 🎯 완료 기준

- [x] 해시 함수 알고리즘의 원리 이해 ✅ (SHA-256, MD5)
- [x] Python으로 기본 해시 시스템 구현 ✅ (hashlib, 블록체인)
- [x] 대칭 암호와 공개키 암호 차이점 이해 ✅ (AES vs RSA vs ECC)
- [x] RSA와 ECC 수학적 원리 완전 이해 ✅ (소인수분해 vs 이산로그)
- [x] 디지털 서명 완전 이해 ✅ (RSA/ECDSA, k 보안, 블록체인 적용)
- [x] PKI 신뢰 체인 완전 이해 ✅ (Root CA, 인증서, TLS Handshake)
- [x] ZKP의 3가지 조건 완전 이해 ✅ (Completeness, Soundness, Zero-Knowledge)
- [x] 알리바바 동굴 시뮬레이션 구현 ✅ (Interactive ZKP 체험)
- [ ] 암호학 개념 정리 노트 작성

## 🔗 유용한 링크

- [Cryptography.io](https://cryptography.io/) - Python 암호화 라이브러리
- [ZK-Learning](https://zk-learning.org/) - ZKP 학습 자료
- [Circom Documentation](https://docs.circom.io/) - ZK 회로 언어
