# Month 3: AI Advanced - PyTorch & 딥러닝 🔥

## 📋 학습 목표

Phase 1에서 scikit-learn으로 기초를 다졌으니, 이제 **딥러닝의 세계**로!

✅ **PyTorch 마스터** - 현업 표준 프레임워크  
✅ **CNN 구현** - 이미지 분류의 정석  
✅ **RNN/LSTM** - 시계열 & 자연어 처리  
✅ **Transformer 기초** - AI 혁명의 핵심

### 🎯 **Phase 1 → Phase 2 업그레이드:**

| Phase 1 (scikit-learn) | Phase 2 (PyTorch) | 발전 |
| ---------------------- | ----------------- | ---- |
| 간단한 분류/회귀       | 복잡한 딥러닝     | 🚀   |
| 작은 데이터셋          | 대용량 데이터     | 📈   |
| 전통적 ML              | 최신 AI 기술      | 🔥   |
| CPU 연산               | GPU 가속          | ⚡   |

---

## 📚 주차별 계획

### Week 1: PyTorch 기초 & 환경 설정

- [ ] **PyTorch 환경 구축**

  - [ ] CUDA/MPS 설정 (GPU 가속)
  - [ ] PyTorch vs TensorFlow 비교
  - [ ] Tensor 연산 마스터
  - [ ] autograd (자동 미분) 이해

- [ ] **첫 신경망 구현**
  - [ ] Linear Layer로 간단한 분류
  - [ ] Loss function & Optimizer
  - [ ] Training loop 구현
  - [ ] scikit-learn과 성능 비교

### Week 2: CNN (Convolutional Neural Networks)

- [ ] **CNN 이론 이해**

  - [ ] Convolution 연산 원리
  - [ ] Pooling, Stride, Padding
  - [ ] Feature Map 시각화
  - [ ] CNN vs 일반 NN 비교

- [ ] **CIFAR-10 이미지 분류 프로젝트**
  - [ ] 데이터 로더 & 전처리
  - [ ] CNN 아키텍처 설계
  - [ ] 훈련 & 검증 파이프라인
  - [ ] 예측 결과 시각화

### Week 3: RNN/LSTM & 자연어 처리

- [ ] **RNN 계열 이해**

  - [ ] Vanilla RNN의 한계
  - [ ] LSTM/GRU 구조 이해
  - [ ] 시계열 데이터 처리
  - [ ] Sequence-to-Sequence

- [ ] **텍스트 생성 프로젝트**
  - [ ] 텍스트 전처리 & 토크나이징
  - [ ] 언어 모델 구현
  - [ ] 셰익스피어 스타일 텍스트 생성
  - [ ] 감정 분석 모델

### Week 4: Transformer & Attention

- [ ] **Transformer 혁명 이해**

  - [ ] Self-Attention 메커니즘
  - [ ] Multi-Head Attention
  - [ ] Positional Encoding
  - [ ] Encoder-Decoder 구조

- [ ] **미니 GPT 구현**
  - [ ] 간소화된 Transformer 구현
  - [ ] 작은 텍스트 데이터셋 훈련
  - [ ] 텍스트 생성 & 완성
  - [ ] Attention 가중치 시각화

---

## 💻 핵심 프로젝트

### 🏆 **메인 프로젝트 (선택 1개)**

1. **🖼️ 이미지 분류 마스터**

   - CIFAR-10/CIFAR-100 정복
   - CNN 아키텍처 실험
   - 전이학습 (Transfer Learning)
   - 웹앱으로 배포

2. **📝 자연어 처리 챌린지**

   - 영화 리뷰 감정 분석
   - 챗봇 구현
   - 텍스트 요약 시스템
   - 한국어 처리 도전

3. **🤖 미니 ChatGPT**
   - Transformer from scratch
   - 작은 언어 모델 훈련
   - 웹 인터페이스 구축
   - 대화 시스템 구현

### 🎯 **서브 프로젝트**

- **성능 최적화**: GPU 가속, 배치 처리
- **모델 해석**: Grad-CAM, Attention 시각화
- **MLOps 기초**: 모델 저장/로드, 버전 관리

---

## 📖 추천 학습 자료

### 🆓 무료 자료

- **PyTorch 공식 튜토리얼**: [pytorch.org/tutorials](https://pytorch.org/tutorials/)
- **Fast.ai 강의**: 실전 중심의 딥러닝
- **CS231n Stanford**: CNN 강의 (한국어 자막)
- **Hugging Face Course**: Transformer & NLP

### 💰 유료 (추천)

- **"밑바닥부터 시작하는 딥러닝 2"** - 실전 구현
- **"딥러닝을 위한 수학"** - 수학적 기초
- **Coursera Deep Learning Specialization** - Andrew Ng

---

## 🛠️ 기술 스택

### 🔧 **핵심 라이브러리**

```python
# 딥러닝 프레임워크
torch>=2.0.0          # PyTorch 메인
torchvision           # 이미지 처리
torchaudio            # 오디오 처리
transformers          # Hugging Face

# 데이터 & 시각화
datasets              # 표준 데이터셋
matplotlib            # 기본 시각화
seaborn               # 고급 시각화
wandb                 # 실험 추적 (선택)

# 유틸리티
tqdm                  # 진행 표시
numpy                 # 수치 연산
pandas                # 데이터 조작
```

### ⚡ **하드웨어 요구사항**

- **CPU**: 충분 (기본 실습)
- **GPU**: 권장 (대용량 모델)
- **RAM**: 8GB+ (16GB 권장)
- **Storage**: 10GB+ (데이터셋 & 모델)

---

## 🎯 완료 기준

- [ ] PyTorch로 3개 이상 딥러닝 모델 구현
- [ ] CNN, RNN, Transformer 각각 1개씩 프로젝트 완성
- [ ] GPU 가속 활용한 대용량 모델 훈련
- [ ] 모델 성능 Phase 1 대비 현저한 향상
- [ ] 웹앱 or API로 모델 배포

---

## 🎓 평가 방식

### 📊 **정량적 평가**

- **모델 정확도**: Phase 1 대비 10% 이상 향상
- **코드 품질**: 모듈화, 문서화, 재사용성
- **실행 속도**: GPU 활용한 효율적 훈련

### 📝 **정성적 평가**

- **이론 이해**: 딥러닝 핵심 개념 설명 가능
- **문제 해결**: 새로운 문제에 적절한 모델 선택
- **창의성**: 독창적인 프로젝트 아이디어

---

## 🚀 빠른 시작

### 1. 환경 설정

```bash
cd phase2/month3_ai_advanced/

# PyTorch 설치 (CUDA 버전 확인 후)
pip install torch torchvision torchaudio

# 추가 라이브러리
pip install transformers datasets matplotlib seaborn tqdm

# GPU 확인
python -c "import torch; print(f'CUDA: {torch.cuda.is_available()}')"
```

### 2. 첫 번째 노트북 실행

```bash
jupyter notebook 01_pytorch_basics.ipynb
```

---

## 💡 학습 팁

### ✅ **Do**

- **매일 1-2시간** 꾸준한 실습
- **코드 따라치기** → **변형해보기** → **새로 만들기**
- **에러 로그 분석** 습관화
- **GPU 메모리 관리** 의식적 연습

### ❌ **Don't**

- 이론만 공부하고 실습 안하기
- 복잡한 모델부터 시작하기
- 에러 나면 바로 포기하기
- 결과만 보고 과정 무시하기

---

**시작일**: TBD  
**목표 완료일**: 4주 후  
**현재 상태**: 환경 설정 대기 🚀
