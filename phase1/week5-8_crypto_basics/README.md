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

- [ ] **RSA 암호**

  - 수학적 기초 (소수, 모듈러 연산)
  - 키 생성, 암호화, 복호화 과정
  - Python 구현 실습

- [ ] **타원곡선 암호 (ECC)**
  - 타원곡선의 수학적 성질
  - ECDSA 디지털 서명
  - Bitcoin/Ethereum에서의 활용

### Week 7: 디지털 서명과 PKI

- [ ] **디지털 서명**

  - 서명 생성과 검증 과정
  - 메시지 인증과 부인 방지
  - 실제 구현 실습

- [ ] **PKI (Public Key Infrastructure)**
  - 인증서와 CA(Certificate Authority)
  - 신뢰 체인 구조
  - TLS/SSL 동작 원리

### Week 8: ZKP 입문

- [ ] **Zero-Knowledge의 정의**

  - Completeness (완전성)
  - Soundness (건실성)
  - Zero-Knowledge (영지식성)

- [ ] **알리바바 동굴 예제**

  - Interactive ZKP 이해
  - Prover와 Verifier 역할
  - 실제 시뮬레이션 구현

- [ ] **zk-SNARKs vs zk-STARKs**
  - 각각의 특징과 장단점
  - Trusted Setup의 필요성
  - 블록체인에서의 활용 사례

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
2. **RSA 암호화 시스템** - 메시지 암복호화
3. **디지털 서명 검증기** - 파일 무결성 확인

### 중급 실습

4. **간단한 ZKP 프로토콜** - 나이 증명 시스템
5. **Merkle Tree 구현** - 데이터 무결성 증명
6. **Commitment Scheme** - 비밀 값 약속 시스템 ✅ **완료!**

## 🎯 완료 기준

- [x] 해시 함수 알고리즘의 원리 이해 ✅ (SHA-256, MD5)
- [x] Python으로 기본 해시 시스템 구현 ✅ (hashlib, 블록체인)
- [ ] ZKP의 3가지 조건 완전 이해
- [ ] 알리바바 동굴 시뮬레이션 구현
- [ ] 암호학 개념 정리 노트 작성

## 🔗 유용한 링크

- [Cryptography.io](https://cryptography.io/) - Python 암호화 라이브러리
- [ZK-Learning](https://zk-learning.org/) - ZKP 학습 자료
- [Circom Documentation](https://docs.circom.io/) - ZK 회로 언어
