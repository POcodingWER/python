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

### Week 1: PyTorch 기초 & 환경 설정 ✅

- [x] **PyTorch 환경 구축** ✅

  - [x] CUDA/MPS 설정 (GPU 가속) ✅ - MPS 가속 활성화
  - [x] PyTorch 2.8.0 설치 완료 ✅
  - [x] Tensor 연산 마스터 ✅
  - [x] autograd (자동 미분) 이해 ✅

- [x] **첫 신경망 구현** ✅
  - [x] Linear Regression 구현 ✅ (`01_pytorch_basics.ipynb`)
  - [x] 분류 신경망 구현 ✅ (`02_classification_neural_network.ipynb`)
  - [x] 아이리스 분류 PyTorch 버전 ✅ - **97.5% 정확도 달성!**
  - [x] Multi-layer Neural Network ✅ - **3층 vs 5층 vs 10층 실험 완료**

**🎉 Week 1 완료 성과:**

- **골디락스 원리 체득**: 적절한 모델 복잡도의 중요성 실험으로 증명
- **과적합 현상 관찰**: 층수별 성능 비교를 통한 딥러닝 핵심 이해
- **한글 폰트 문제 해결**: matplotlib 시각화 완벽 구현
- **실무 수준 PyTorch 구현**: autograd, 훈련 루프, 모델 평가 전 과정 마스터

### Week 2: CNN (Convolutional Neural Networks) ✅

- [x] **CNN 이론 이해** ✅

  - [x] Convolution 연산 원리 ✅ - 수동 구현 및 시각화 완료
  - [x] Pooling, Stride, Padding ✅ - MaxPool2d, 패딩 개념 마스터
  - [x] Feature Map 시각화 ✅ - 랜덤 패턴 실험으로 이해 강화
  - [x] CNN vs 일반 NN 비교 ✅ - 파라미터 수 및 성능 비교 분석

- [x] **CIFAR-10 이미지 분류 프로젝트** ✅
  - [x] 데이터 로더 & 전처리 ✅ - 조건부 다운로드, 한글 클래스명 자동 매핑
  - [x] CNN 아키텍처 설계 ✅ - 3층 CNN (Conv2d-ReLU-MaxPool) 구조
  - [x] 훈련 & 검증 파이프라인 ✅ - **76.22% 정확도 달성**
  - [x] 예측 결과 시각화 ✅ - 한글 폰트 문제 해결 완료

**🎉 Week 2 완료 성과:**

- **CIFAR-10 이미지 분류 성공**: 76.22% 정확도 (10 epochs)
- **데이터 증강 실험**: RandomFlip, Rotation, ColorJitter 효과 검증
- **CNN 3총사 마스터**: Conv2d, ReLU, MaxPool2d 핵심 이해
- **실전 PyTorch 활용**: `model.train()`, `model.eval()`, `torch.no_grad()` 완벽 구현
- **성능 최적화 개념**: BatchNorm, Scheduler, Early Stopping 학습

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

- [x] **Week 1 PyTorch 기초** ✅ - Linear Regression + 분류 신경망 완성
- [x] **Week 2 CNN 기초** ✅ - CIFAR-10 이미지 분류 완성
- [ ] PyTorch로 3개 이상 딥러닝 모델 구현 (2/3 완료: 분류 신경망 + CNN)
- [ ] CNN, RNN, Transformer 각각 1개씩 프로젝트 완성 (1/3 완료: CNN)
- [x] GPU 가속 활용한 모델 훈련 ✅ - MPS(Apple M1/M2) 가속 활용
- [x] 모델 성능 Phase 1 대비 현저한 향상 ✅ - 97.5% vs 95% (scikit-learn)
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

**시작일**: 2025년 9월 25일 ✅  
**목표 완료일**: 4주 후  
**현재 상태**: Week 2 완료 ✅ → Week 3 준비 중 🚀
**완성 파일**:

- `01_pytorch_basics.ipynb` ✅ - PyTorch 기초, autograd, Linear Regression
- `02_classification_neural_network.ipynb` ✅ - 3층 신경망, 과적합 실험, 골디락스 원리
- `03_cnn_basics.ipynb` ✅ - CNN 이론, CIFAR-10 분류, 데이터 증강 실험

**다음 할 일**: Week 3 RNN/LSTM & 자연어 처리 시작!
