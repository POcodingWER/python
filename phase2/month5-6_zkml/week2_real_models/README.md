# Week 2: 실전 AI 모델 → ZKML 변환 🎯

> **목표**: Month 3-4에서 배운 PyTorch 모델들을 ZKML로 변환하여 실무 역량 확보

## 🎯 학습 목표

Week 1에서 **선형 회귀**로 ZKML 기초를 다졌다면, Week 2는 **실전 AI 모델**을 ZKML로 변환합니다!

### ✅ 완료 목표

- [ ] Multi-layer NN → ZKML 변환
- [ ] CNN (MNIST) → ZKML 변환
- [ ] 모델 크기 vs 증명 시간 비교 분석
- [ ] 실무 적용 가능성 평가

### ⚠️ Week 2 범위

- ✅ MLP (Iris 분류) - 기본 다층 신경망 (**테스트 완료**)
- ❌ CNN/RNN - EZKL 호환성 이슈로 Week 3-4로 이동
  - 현재 EZKL은 단순한 모델(파라미터 ~100개 이하)만 안정적
  - 대형 모델은 향후 EZKL 업데이트 대기

---

## 📁 프로젝트 구조

```
week2_real_models/
├── README.md                          # 이 파일
├── .gitignore                         # 생성 파일 제외
├── 01_mlp_zkml.py                     # Multi-layer NN → ZKML (테스트 완료 ✅)
└── performance_comparison.md          # 성능 벤치마크 결과

주의: CNN/RNN은 EZKL 호환성 이슈로 인해 제외됨
```

---

## 🚀 빠른 시작

### ⚠️ **중요: EZKL 호환성 고려 사항**

```
✅ 실행 가능 (ReLU 제거 버전):
- Linear 레이어만 사용하도록 단순화
- Week 1과 동일한 방식으로 정상 작동
- MLP (Iris): 2분 이내 완료 ✅
- MLP (MNIST): 5-10분 예상

⚠️ 원본 CNN (ReLU/Conv2d 포함):
- PyTorch 2.9.0 → ONNX Opset 18 자동 변환
- EZKL이 Opset 18의 고급 연산 미지원
- 향후 EZKL 업데이트 후 사용 가능

💡 학습 전략:
1. 현재 코드로 ZKML 워크플로우 마스터 (실행 가능 ✅)
2. 모델 복잡도 증가에 따른 증명 시간 체감
3. README의 성능 비교 분석으로 CNN 이해
```

### 1. 환경 확인

```bash
# 가상환경 활성화 (이미 Week 1에서 설정 완료)
cd /Users/soon/Desktop/ttttttt/python/phase2/month5-6_zkml
source zkml_env/bin/activate

# Week 2 폴더로 이동
cd week2_real_models
```

### 2. 예제 실행

```bash
# ✅ Multi-layer NN (Iris 분류)
python 01_mlp_zkml.py

# 실행 결과:
# - 훈련 정확도: 100%
# - 증명 시간: 1-2분
# - 파일 크기: ONNX 3.7KB, Proof 17.8KB
```

---

## 📚 학습 순서

### Step 1: Multi-layer Neural Network (MLP)

**파일**: `01_mlp_zkml.py`

**Month 3 복습**: `phase2/month3_ai_advanced/02_classification_neural_network.ipynb`

**학습 내용**:

- 아이리스 분류 모델 (3층 신경망)
- 입력: 4개 특징 → 출력: 3개 클래스
- Week 1 대비 **레이어 수 증가** → 회로 복잡도 변화 관찰

**핵심 포인트**:

```python
# Month 3 모델
class IrisClassifier(nn.Module):
    def __init__(self):
        self.fc1 = nn.Linear(4, 16)   # 입력층
        self.fc2 = nn.Linear(16, 8)   # 은닉층
        self.fc3 = nn.Linear(8, 3)    # 출력층

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = self.fc3(x)
        return x

# ZKML 변환시 고려사항
# 1. ReLU 연산 → ZK 회로로 변환
# 2. 레이어 수 증가 → Proving Key 크기 증가
# 3. 입력 차원 증가 → SRS 크기 결정
```

---

### Step 2: CNN (CIFAR-10 이미지 분류)

**파일**: `02_cnn_zkml.py`

**Month 3 복습**: `phase2/month3_ai_advanced/03_cnn_basics.ipynb`

**학습 내용**:

- CIFAR-10 분류 모델 (3층 CNN)
- 입력: 32×32×3 이미지 → 출력: 10개 클래스
- Conv2d, MaxPool2d → ZK 회로 변환 도전

**핵심 포인트**:

```python
# Month 3 모델
class CIFAR10_CNN(nn.Module):
    def __init__(self):
        self.features = nn.Sequential(
            nn.Conv2d(3, 32, kernel_size=3, padding=1),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(kernel_size=2, stride=2),

            nn.Conv2d(32, 64, kernel_size=3, padding=1),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(kernel_size=2, stride=2),

            nn.Conv2d(64, 128, kernel_size=3, padding=1),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(kernel_size=2, stride=2),
        )
        self.classifier = nn.Sequential(
            nn.Dropout(0.5),
            nn.Linear(128 * 4 * 4, 512),
            nn.ReLU(inplace=True),
            nn.Dropout(0.5),
            nn.Linear(512, 10)
        )

# ZKML 변환시 고려사항
# 1. Dropout → ONNX 변환시 제거됨 (추론 모드)
# 2. Conv2d 연산 → 대량 증명 시간 (수분 예상)
# 3. 입력 크기 (3072개 픽셀) → SRS 크기 대폭 증가
# 4. 실무에서는 축소 버전 사용 권장
```

**⚠️ 주의사항**:

- CNN은 **증명 시간이 매우 길 수 있습니다** (5-30분 예상)
- 실습용으로 **간소화된 버전**을 제공합니다
- 실무에서는 **모델 경량화 필수**

---

### ⚠️ RNN은 Week 3-4로 이동

**이유**:

- ONNX 변환 복잡성 (Embedding, 순환 구조)
- PyTorch 2.x → ONNX Opset 호환성 이슈
- 실무에서는 Transformer 선호

**Week 2 집중**:

- MLP와 CNN으로 ZKML 기본기 확립
- 모델 복잡도 증가에 따른 증명 시간 이해
- 실무 적용 가능성 판단 능력 배양

---

## 🔬 성능 비교 분석

### 예상 결과 (Week 1 vs Week 2)

| 모델               | 파라미터 수 | ONNX 크기 | 증명 시간 | Proof 크기 | 실무 적용           |
| ------------------ | ----------- | --------- | --------- | ---------- | ------------------- |
| **Week 1: Linear** | 3개         | 2KB       | 30초      | 17KB       | ✅ 즉시 가능        |
| **MLP (Iris)**     | ~200개      | 5KB       | 1-2분     | 20KB       | ✅ 가능             |
| **CNN (Simple)**   | ~5,000개    | 50KB      | 5-10분    | 30KB       | ⚠️ 경량화 필요      |
| **CNN (Full)**     | ~50,000개   | 500KB     | 30-60분   | 50KB       | ❌ 실용적 X         |
| **RNN**            | ~10,000개   | 100KB     | 10-20분   | 40KB       | ⚠️ Transformer 권장 |

### 실무 인사이트

```
💡 "ZKML은 모델 크기와 증명 시간의 트레이드오프"

1️⃣ **즉시 사용 가능**: Linear, 단순 MLP
   - API 검증, 블록체인 AI

2️⃣ **최적화 후 가능**: 경량 CNN, LSTM
   - 프루닝, 양자화 후 사용

3️⃣ **현실적으로 어려움**: 대형 CNN, Transformer
   - 연구 단계, 하드웨어 발전 필요
```

---

## 💡 핵심 개념

### ONNX 변환 호환성

```python
# ✅ ONNX 변환 잘 되는 연산
- Linear, Conv2d, MaxPool2d
- ReLU, Sigmoid, Tanh
- BatchNorm (추론 모드)

# ⚠️ 주의 필요한 연산
- Embedding (정수 입력 처리)
- RNN/LSTM (순환 구조)
- Dropout (추론시 제거됨)

# ❌ 지원 안 되는 연산
- 동적 shape 연산
- Custom CUDA kernels
- 일부 고급 PyTorch 연산
```

### 모델 경량화 기법

```python
# 1. 파라미터 프루닝 (가지치기)
# 중요하지 않은 가중치 제거

# 2. 양자화 (Quantization)
# FP32 → INT8 변환

# 3. Knowledge Distillation
# 큰 모델 → 작은 모델 압축

# 4. 아키텍처 단순화
# 레이어 수 감소, 필터 수 감소
```

---

## 🛠️ 실행 명령어

### NPM Scripts (권장)

```bash
# Week 2 전체 실행
npm run week2

# 개별 예제 실행
npm run week2:mlp
npm run week2:cnn
npm run week2:rnn
```

### 수동 실행

```bash
cd /Users/soon/Desktop/ttttttt/python/phase2/month5-6_zkml
source zkml_env/bin/activate
cd week2_real_models

python 01_mlp_zkml.py
python 02_cnn_zkml.py
python 03_rnn_zkml.py
```

---

## 📊 학습 체크리스트

- [ ] MLP → ONNX → ZKML 전체 파이프라인 이해
- [ ] CNN 변환시 증명 시간 증가 체감
- [ ] RNN 변환의 어려움 (순환 구조) 이해
- [ ] 모델 크기 vs 증명 시간 관계 분석
- [ ] 실무 적용시 경량화 필요성 인지
- [ ] Week 1 대비 복잡도 증가 체험

---

## 🎯 다음 단계 (Week 3-4)

### Week 3: ZKML 최적화

- 모델 프루닝 & 양자화
- 증명 시간 단축 기법
- 배치 증명 (여러 입력 동시 처리)

### Week 4: 실전 프로젝트

- 웹 API + ZKML 통합
- 블록체인 연동
- 실제 서비스 시나리오 구현

---

## 💡 자주 묻는 질문 (FAQ)

### Q1: CNN 증명 시간이 너무 오래 걸려요 (30분+)

**A**: 정상입니다! CNN은 연산량이 많아 증명 시간이 깁니다.

- **해결책**: 간소화된 버전 사용 (필터 수 감소)
- **실무**: 모델 경량화 + 하드웨어 가속 필수

### Q2: RNN ONNX 변환이 안돼요

**A**: RNN은 ONNX 변환이 까다롭습니다.

- **대안 1**: 단순 MLP로 시작 (시퀀스 무시)
- **대안 2**: LSTM 대신 Transformer 사용
- **실무**: Hugging Face 모델 활용

### Q3: 모델을 단순화하면 정확도가 떨어지지 않나요?

**A**: 트레이드오프가 존재합니다.

- **ZKML 목표**: 정확도 < 검증 가능성
- **실무**: 80% 정확도 + 증명 가능 > 99% 정확도 + 증명 불가
- **해결책**: Knowledge Distillation으로 성능 유지

### Q4: Week 1 대비 어떤 점이 다른가요?

| 측면            | Week 1        | Week 2          |
| --------------- | ------------- | --------------- |
| **모델**        | Linear (단순) | CNN, RNN (복잡) |
| **증명 시간**   | 30초          | 5-30분          |
| **실무 난이도** | 즉시 사용     | 최적화 필요     |
| **학습 목표**   | 기초 개념     | 실전 적용       |

---

## 🔗 참고 자료

### 내부 링크

- [Week 1: EZKL 기초](../week1_ezkl_basics/README.md)
- [Month 3: AI Advanced](../../month3_ai_advanced/README.md)

### 외부 링크

- [EZKL Documentation](https://docs.ezkl.xyz/)
- [ONNX Model Zoo](https://github.com/onnx/models)
- [PyTorch ONNX Export](https://pytorch.org/docs/stable/onnx.html)

---

**🎉 Week 2를 통해 실전 AI 모델의 ZKML 변환 경험을 쌓아보세요!** 🚀🔐

**난이도**: ⭐⭐⭐⭐ (Week 1 대비 상승)  
**예상 시간**: 8-12시간  
**선행 조건**: Week 1 완료 + Month 3 AI 지식
