# 🚀 AI + ZKP 마스터 플랜 2025

**목표**: 1년 안에 AI와 Zero-Knowledge Proof 전문가 되기!

## 📋 프로젝트 개요

이 저장소는 **AI(인공지능) + ZKP(영지식 증명)** 전문가가 되기 위한 체계적인 학습 과정을 담고 있습니다.
특히 **ZKML(Zero-Knowledge Machine Learning)** 분야의 전문성 개발을 목표로 합니다.

### 🎯 최종 목표

- **Python 기반 AI/ML 개발 역량**
- **암호학 및 ZKP 이론 이해**
- **ZKML 실전 프로젝트 구현**
- **Rust로 고성능 시스템 개발**
- **포트폴리오 완성 및 취업/창업 준비**

---

## 📁 프로젝트 구조

```
📦 AI_ZKP_Master_Plan_2025/
├── 📋 AI_ZKP_MASTER_PLAN.md          # 전체 로드맵
├── 📊 WEEKLY_TRACKER.md              # 주간 학습 추적
├── 📄 requirements.txt               # Python 패키지 의존성
├──
├── 📂 phase1/ (기초 다지기 1-2개월)
│   ├── 📂 week1-2_python_basics/      # ✅ Python 기초 완료
│   │   ├── 01_python_basics.ipynb
│   │   ├── 02_control_statements.ipynb
│   │   ├── 03_functions.ipynb
│   │   ├── 04_data_structures.ipynb
│   │   ├── 05_mini_projects.ipynb
│   │   ├── 06_numpy_basics.ipynb
│   │   ├── 07_pandas_basics.ipynb
│   │   └── exercises.py
│   │
│   ├── 📂 week3-4_ai_basics/          # ✅ AI 기초 완료
│   │   ├── 01_sklearn_intro.ipynb
│   │   ├── 02_algorithm_comparison.ipynb
│   │   ├── 03_regression_algorithms.ipynb
│   │   ├── 04_cross_validation.ipynb
│   │   ├── 05_evaluation_metrics.ipynb
│   │   ├── 06_titanic_project.ipynb
│   │   ├── 07_ai_summary_guide.ipynb
│   │   └── README.md
│   │
│   └── 📂 week5-8_crypto_basics/      # ✅ 암호학 기초 완료
│       ├── 01_hash_functions.ipynb
│       ├── 02_symmetric_crypto.ipynb
│       ├── 03_rsa_basics.ipynb
│       ├── 04_ecc_elliptic_curve.ipynb
│       ├── 05_digital_signatures.ipynb
│       ├── 06_pki_basics.ipynb
│       ├── 07_zkp_basics.ipynb
│       ├── 08_zkp_basics2.ipynb
│       ├── 09_python_production_snarks.ipynb
│       ├── 10_python_production_starks.ipynb
│       ├── 11_python_production_halo.ipynb
│       └── README.md
│
├── 📂 phase2/ (실전 돌입 2-4개월)
│   ├── 📂 month3_ai_advanced/         # PyTorch & 딥러닝
│   ├── 📂 month4_zkp_practice/        # Circom & snarkjs
│   └── 📂 month5-6_zkml/              # EZKL & ZKML 프로젝트
│
├── 📂 phase3/ (전문가 되기 4-6개월)
│   ├── 📂 month7-8_rust/              # Rust & candle
│   ├── 📂 month9-10_advanced_zkml/    # 고급 ZKML
│   └── 📂 month11-12_portfolio/       # 포트폴리오 완성
│
├── 📂 projects/                       # 실전 프로젝트들
├── 📂 resources/                      # 학습 자료
│   ├── 📂 books/                      # 추천 도서
│   ├── 📂 papers/                     # 논문 모음
│   └── 📂 tutorials/                  # 튜토리얼
```

---

## 🎯 현재 진행 상황

### ✅ 완료 (Week 1-2)

- **Python 기초 문법** - JS 경험을 활용한 빠른 습득
- **NumPy 기초** - AI를 위한 배열 연산 및 선형대수
- **Pandas 기초** - 데이터 분석 및 전처리 패턴
- **Jupyter Notebook** - 환경 구축 및 노트북 작성법

### ✅ 완료 (Week 3-4: AI 기초)

- **scikit-learn 기초** - 분류/회귀 알고리즘 실습 ✅
- **머신러닝 프로젝트** - 타이타닉 생존 예측 완료 ✅
- **교차 검증 & 평가 지표** - 모델 성능 평가 ✅
- **AI 알고리즘 비교** - 회귀/분류 알고리즘 완전 이해 ✅

### ✅ 완료 (Week 5-8: 암호학 & ZKP)

- **해시 함수 & 대칭 암호** - SHA-256, AES 구현 ✅
- **공개키 암호** - RSA, ECC(타원곡선) 마스터 ✅
- **디지털 서명 & PKI** - ECDSA, 인증서 체인 ✅
- **ZKP 삼대장** - SNARKs, STARKs, Halo 완전 정복 ✅
- **실시간 성능 비교** - 동적 벤치마크 구현 ✅

### 🎯 다음 목표 (Phase 2)

- **PyTorch & 딥러닝** - 고급 AI 모델 구현
- **Circom & snarkjs** - ZK 회로 프로그래밍
- **ZKML 프로젝트** - AI + ZKP 결합

---

## 🚀 빠른 시작

### 1. 환경 설정

```bash
# 저장소 클론
git clone https://github.com/POcodingWER/python.git
cd python

# 가상환경 생성 (권장)
python -m venv venv
source venv/bin/activate  # macOS/Linux
# venv\Scripts\activate   # Windows

# 의존성 설치
pip install -r requirements.txt

# Jupyter 노트북 실행
jupyter notebook
```

### 2. 학습 순서

1. **Phase 1 기초** → `phase1/week1-2_python_basics/` 노트북들 실행
2. **진행 추적** → `WEEKLY_TRACKER.md`에서 체크리스트 확인
3. **전체 계획** → `AI_ZKP_MASTER_PLAN.md`에서 로드맵 확인

---

## 📊 학습 진행률

```
전체 진행률: ████████░░░░░░░░░░░░ 25% (3/12개월)

Phase 1 기초: ████████████████░░ 75% (6/8주)
├── Week 1-2 Python: ████████████ 100% ✅
├── Week 3-4 AI기초: ████████████ 100% ✅
└── Week 5-8 암호학: ████████████ 100% ✅

Phase 2 실전: ░░░░░░░░░░░░ 0%
Phase 3 전문가: ░░░░░░░░░░░░ 0%
```

---

## 📚 핵심 학습 자료

### 무료 리소스

- **AI**: [CS229 Stanford](https://cs229.stanford.edu/), [Fast.ai](https://www.fast.ai/)
- **ZKP**: [ZK-Learning.org](https://zk-learning.org/), [Circom Docs](https://docs.circom.io/)
- **Rust**: [The Rust Book](https://doc.rust-lang.org/book/) (한국어판 있음)

### 추천 도서

- "혼자 공부하는 머신러닝" (AI 기초)
- "Real-World Cryptography" (암호학)
- "밑바닥부터 시작하는 딥러닝" (딥러닝)

---

## 🎯 주요 마일스톤

| 시기          | 목표              | 성과물                      |
| ------------- | ----------------- | --------------------------- |
| **3개월 후**  | 기본 AI + ZKP     | 간단한 ML모델 + ZK증명 생성 |
| **6개월 후**  | ZKML 프로젝트     | 개인정보보호 AI 시스템      |
| **9개월 후**  | Rust 고성능 구현  | 블록체인 연동 ZKML          |
| **12개월 후** | 전문가 포트폴리오 | 취업/창업 준비 완료         |

---

## 💡 학습 철학

1. **실습 위주**: 이론보다 직접 만들어보기
2. **꾸준함**: 매일 1-2시간씩 지속적 학습
3. **기록 습관**: 배운 것은 반드시 정리하고 공유
4. **커뮤니티**: 혼자 하지 말고 함께 성장
5. **완주**: 포기하지 않고 끝까지 해내기

---

## 🤝 기여 및 피드백

이 프로젝트는 개인 학습용이지만, 같은 목표를 가진 분들과의 교류를 환영합니다!

- **Issues**: 질문이나 제안사항
- **Pull Requests**: 개선사항이나 추가 자료
- **Discussions**: 학습 경험 공유

---

**시작일**: 2025년 1월 12일  
**목표 완료일**: 2025년 12월 31일  
**현재 상태**: Phase 1 기초 완료 (Week 1-8), Phase 2 실전 진행 예정 🚀
