# 🧠 AI 모델 총정리 - Month 3 복습 + ZKML 연결

> **Month 3에서 배운 AI 모델들**을 복습하고 **ZKML 적용 가능성**을 분석합니다.

---

## 📚 학습한 모델 목록

| 모델                  | 파일                                   | 복잡도       | ZKML 적용 | 현재 상태        |
| --------------------- | -------------------------------------- | ------------ | --------- | ---------------- |
| **Linear Regression** | 01_pytorch_basics.ipynb                | ⭐           | ✅ 완료   | Week 1 ✅        |
| **MLP (분류)**        | 02_classification_neural_network.ipynb | ⭐⭐         | ✅ 완료   | Week 2 ✅        |
| **Tiny MLP (MNIST)**  | -                                      | ⭐⭐⭐       | ✅ 완료   | Week 3 ✅        |
| **CNN**               | 03_cnn_basics.ipynb                    | ⭐⭐⭐⭐     | ❌ 불가   | Conv2d 미지원    |
| **RNN**               | 04_rnn_basics.ipynb                    | ⭐⭐⭐⭐     | ❌ 불가   | 순환 구조 미지원 |
| **LSTM**              | 05_lstm_advanced.ipynb                 | ⭐⭐⭐⭐⭐   | ❌ 불가   | Gate 구조 미지원 |
| **Transformer**       | 06_transformer_attention.ipynb         | ⭐⭐⭐⭐⭐⭐ | ❌ 불가   | Attention 미지원 |

---

## 1️⃣ Linear Regression (선형 회귀)

### 📖 Month 3 복습

**파일**: `phase2/month3_ai_advanced/01_pytorch_basics.ipynb`

**개념**:

```python
# 가장 단순한 모델: y = wx + b
class LinearRegression(nn.Module):
    def __init__(self):
        super().__init__()
        self.linear = nn.Linear(1, 1)  # 입력 1개, 출력 1개

    def forward(self, x):
        return self.linear(x)
```

**특징**:

- 🎯 파라미터: weight(1) + bias(1) = **2개**
- 📊 연산: `y = w × x + b` (곱셈 1번, 덧셈 1번)
- ⚡ 증명 시간: ~30초

### ✅ ZKML 적용 (Week 1 완료)

**파일**: `week1_ezkl_basics/01_simple_linear_regression.py`

**결과**:

- ✅ 정상 작동
- ✅ 파라미터: 3개 (weight, bias, 추가 파라미터)
- ✅ 증명 시간: ~30초
- ✅ Proof 크기: 17 KB

**핵심 배운 점**:

- PyTorch → ONNX → EZKL 전체 파이프라인
- Settings, Witness, Proof, PK/VK 역할
- ZK 증명 생성 및 검증 과정

---

## 2️⃣ MLP (Multi-Layer Perceptron)

### 📖 Month 3 복습

**파일**: `phase2/month3_ai_advanced/02_classification_neural_network.ipynb`

**개념**:

```python
# 다층 신경망: 여러 층을 쌓아 복잡한 패턴 학습
class IrisClassifier(nn.Module):
    def __init__(self):
        super().__init__()
        self.fc1 = nn.Linear(4, 16)   # 입력층
        self.fc2 = nn.Linear(16, 8)   # 은닉층
        self.fc3 = nn.Linear(8, 3)    # 출력층

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = self.fc3(x)
        return x
```

**특징**:

- 🎯 파라미터: ~200개 (레이어마다 weight + bias)
- 📊 연산: Linear → ReLU → Linear → ReLU → Linear
- ⚡ 학습: Iris 데이터셋 (150 샘플, 4 특징, 3 클래스)
- 🎯 정확도: 97.5%

### ✅ ZKML 적용 (Week 2 완료)

**파일**: `week2_real_models/01_mlp_zkml.py`

**수정사항**:

```python
# ReLU 제거 (EZKL 호환성)
class IrisClassifier(nn.Module):
    def __init__(self):
        super().__init__()
        self.fc1 = nn.Linear(4, 8)    # 입력층
        self.fc2 = nn.Linear(8, 3)    # 출력층

    def forward(self, x):
        x = self.fc1(x)  # ReLU 없음
        x = self.fc2(x)
        return x
```

**결과**:

- ✅ 정상 작동
- ✅ 파라미터: 43개
- ✅ 증명 시간: ~2분
- ✅ Proof 크기: 17.84 KB
- ✅ 정확도: 100% (단순한 데이터셋)

**핵심 배운 점**:

- 파라미터 증가 → 증명 시간 증가 (선형적이지 않음)
- ReLU 제거해도 단순 문제는 해결 가능
- Linear만으로도 충분한 표현력

---

## 3️⃣ CNN (Convolutional Neural Network)

### 📖 Month 3 복습

**파일**: `phase2/month3_ai_advanced/03_cnn_basics.ipynb`

**개념**:

```python
# 이미지 처리 특화 모델
class CIFAR10_CNN(nn.Module):
    def __init__(self):
        super().__init__()
        # 특징 추출
        self.conv1 = nn.Conv2d(3, 32, 3, padding=1)   # 32×32×32
        self.pool1 = nn.MaxPool2d(2, 2)               # 16×16×32
        self.conv2 = nn.Conv2d(32, 64, 3, padding=1)  # 16×16×64
        self.pool2 = nn.MaxPool2d(2, 2)               # 8×8×64
        self.conv3 = nn.Conv2d(64, 128, 3, padding=1) # 8×8×128
        self.pool3 = nn.MaxPool2d(2, 2)               # 4×4×128

        # 분류
        self.fc1 = nn.Linear(128 * 4 * 4, 512)
        self.fc2 = nn.Linear(512, 10)

    def forward(self, x):
        x = self.pool1(F.relu(self.conv1(x)))
        x = self.pool2(F.relu(self.conv2(x)))
        x = self.pool3(F.relu(self.conv3(x)))
        x = x.view(-1, 128 * 4 * 4)
        x = F.relu(self.fc1(x))
        x = self.fc2(x)
        return x
```

**특징**:

- 🎯 파라미터: ~50,000개
- 📊 연산: Conv2d, ReLU, MaxPool2d 반복
- ⚡ 학습: CIFAR-10 (60,000 이미지, 10 클래스)
- 🎯 정확도: 76.22% (10 epochs)

**핵심 개념**:

- **Convolution**: 이미지의 지역적 패턴 추출
- **Pooling**: 다운샘플링으로 중요 특징만 남김
- **Feature Map**: 층마다 학습된 필터

### ⚠️ ZKML 적용 (대기 중)

**시도했던 것**: `week2_real_models/02_cnn_zkml.py` (삭제됨)

**문제점**:

1. ❌ Conv2d → ONNX Opset 18 호환성 문제
2. ❌ MaxPool2d → EZKL 지원 제한
3. ❌ ReLU → Opset 18 문제
4. ❌ 파라미터 수 → 증명 시간 30분+

**대안 (Week 3 계획)**:

```python
# 완전 단순화 MLP (이미지 flatten)
class SimpleMLP(nn.Module):
    def __init__(self):
        super().__init__()
        self.fc1 = nn.Linear(784, 128)  # 28×28 → 128
        self.fc2 = nn.Linear(128, 10)   # 128 → 10

    def forward(self, x):
        x = x.view(-1, 784)  # Flatten
        x = self.fc1(x)
        x = self.fc2(x)
        return x
```

**예상 결과**:

- 🎯 파라미터: ~100,000개
- ⚠️ 증명 시간: 10-30분 (예상)
- ⚠️ 정확도: 낮음 (ReLU 없이)

---

## 4️⃣ RNN (Recurrent Neural Network)

### 📖 Month 3 복습

**파일**: `phase2/month3_ai_advanced/04_rnn_basics.ipynb`

**개념**:

```python
# 시퀀스 데이터 처리 (텍스트, 시계열)
class SentimentRNN(nn.Module):
    def __init__(self, vocab_size, embedding_dim, hidden_dim):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, embedding_dim)
        self.rnn = nn.RNN(embedding_dim, hidden_dim, batch_first=True)
        self.fc = nn.Linear(hidden_dim, 2)  # 긍정/부정

    def forward(self, x):
        embedded = self.embedding(x)
        output, hidden = self.rnn(embedded)
        logits = self.fc(hidden.squeeze(0))
        return logits
```

**특징**:

- 🎯 파라미터: ~10,000개
- 📊 연산: Embedding → RNN (순환) → Linear
- ⚡ 학습: 한국어 감정 분석
- 🎯 정확도: 100% (간단한 데이터)

**핵심 개념**:

- **Hidden State**: 이전 정보를 기억
- **순환 구조**: t-1 출력 → t 입력
- **시퀀스 처리**: 가변 길이 입력

### ❌ ZKML 적용 (Week 3 실험 완료 - 불가능 확인)

**파일**: `week3_cnn_challenge/02_tiny_rnn_zkml.py` (실험)

**실험 결과**:

| 시도                | ONNX 연산                          | Settings | Calibration | 결과 |
| ------------------- | ---------------------------------- | -------- | ----------- | ---- |
| `nn.RNN`            | `RNN`, `Gemm`, `Gather`, `Squeeze` | ✅       | ❌          | 실패 |
| Unrolled RNN        | `Gemm`, `Gather`, `Add`            | ✅       | ❌          | 실패 |
| Flat RNN (MLP 변형) | `Gemm`                             | ✅       | ✅          | 성공 |

**문제점**:

1. ❌ `RNN` 연산 자체 미지원 (EZKL)
2. ❌ `Gather` (인덱싱) 미지원
3. ❌ `Squeeze/Unsqueeze` 미지원
4. ❌ 순환 구조 (Loop/Scan) 미지원
5. ❌ Embedding → 정수 인덱스 처리 복잡
6. ❌ 가변 길이 → EZKL은 고정 길이만 지원

**성공한 것**:

```python
# "Flat RNN" - 사실상 MLP
class FlatRNN(nn.Module):
    def __init__(self):
        self.fc1 = nn.Linear(3, 4)  # [a,b,c] → hidden
        self.fc2 = nn.Linear(4, 1)  # hidden → output

    def forward(self, x):
        # x: (batch, 3) - 이미 flatten된 시퀀스
        # 시간적 순서를 고려하지 않음 (단순 MLP)
        return self.fc2(self.fc1(x))
```

- ✅ ZKML 변환 성공
- ✅ 증명 시간: 1.1초
- ⚠️ 하지만 진짜 RNN이 아님 (시퀀스 순서 무시)

**결론**:

- **진짜 RNN은 현재 ZKML 불가능**
- Time-Delay MLP나 1D CNN 같은 대안 필요
- 또는 다른 ZKML 프레임워크 탐색 필요

**실무 대안**:

- ✅ Time-Delay MLP (시간 윈도우 flatten)
- ✅ 1D CNN (가능성 있음)
- ⚠️ Transformer (Attention 미지원으로 어려움)

---

## 5️⃣ LSTM (Long Short-Term Memory)

### 📖 Month 3 복습

**파일**: `phase2/month3_ai_advanced/05_lstm_advanced.ipynb`

**개념**:

```python
# RNN의 장기 기억 문제 해결
class TextGenerator(nn.Module):
    def __init__(self, vocab_size, embedding_dim, hidden_dim):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, embedding_dim)
        self.lstm = nn.LSTM(embedding_dim, hidden_dim, batch_first=True)
        self.fc = nn.Linear(hidden_dim, vocab_size)

    def forward(self, x):
        embedded = self.embedding(x)
        output, (hidden, cell) = self.lstm(embedded)
        logits = self.fc(output[:, -1, :])
        return logits
```

**특징**:

- 🎯 파라미터: ~10,000개
- 📊 연산: Embedding → LSTM (4개 Gate) → Linear
- ⚡ 학습: 텍스트 생성
- 🎯 성능: RNN보다 긴 시퀀스 처리 가능

**핵심 개념**:

- **Cell State**: 장기 기억
- **Gate**: Forget, Input, Output Gate
- **Gradient Flow**: Vanishing Gradient 해결

### ❌ ZKML 적용 (매우 어려움)

**문제점**:

- RNN의 모든 문제 + Gate 구조 (×4 복잡도)
- ZK 회로로 변환시 복잡도 폭발

---

## 6️⃣ Transformer (Attention)

### 📖 Month 3 복습

**파일**: `phase2/month3_ai_advanced/06_transformer_attention.ipynb`

**개념**:

```python
# Self-Attention 메커니즘
class TransformerBlock(nn.Module):
    def __init__(self, d_model, num_heads):
        super().__init__()
        self.attention = nn.MultiheadAttention(d_model, num_heads)
        self.norm1 = nn.LayerNorm(d_model)
        self.ff = nn.Sequential(
            nn.Linear(d_model, d_model * 4),
            nn.ReLU(),
            nn.Linear(d_model * 4, d_model)
        )
        self.norm2 = nn.LayerNorm(d_model)

    def forward(self, x):
        # Self-Attention
        attn_out, _ = self.attention(x, x, x)
        x = self.norm1(x + attn_out)

        # Feed-Forward
        ff_out = self.ff(x)
        x = self.norm2(x + ff_out)
        return x
```

**특징**:

- 🎯 파라미터: 수백만 ~ 수십억개
- 📊 연산: Attention (행렬 곱 다수), LayerNorm, FFN
- ⚡ 학습: NLP, Vision 등 다양한 분야
- 🎯 성능: SOTA (최고 성능)

**핵심 개념**:

- **Self-Attention**: 모든 위치 간 관계 학습
- **Positional Encoding**: 위치 정보 주입
- **Multi-Head**: 다양한 관점 학습

### ❌ ZKML 적용 (현재 불가능)

**문제점**:

1. ❌ Attention → 행렬 곱 연산 매우 많음
2. ❌ LayerNorm → 통계 연산 (mean, std)
3. ❌ 파라미터 수 → 억 단위 (GPT, BERT)
4. ❌ 증명 시간 → 몇 시간 ~ 며칠

**실무**:

- 매우 단순화된 버전만 가능
- Attention 없이 FFN만 사용
- 또는 Distillation으로 극단적 압축

---

## 🎯 ZKML 적용 가능성 요약 (2025년 기준)

### ✅ **현재 가능** (Week 1-3)

```
Linear Regression  ⭐       ~30초    ✅ 완료
MLP (Iris)         ⭐⭐     ~2분     ✅ 완료
Tiny MLP (MNIST)   ⭐⭐⭐   5-10분   ✅ 완료
```

### ❌ **현재 불가능** (실험 완료)

```
Full CNN           ⭐⭐⭐⭐⭐     N/A      ❌ Conv2d 미지원
RNN/LSTM           ⭐⭐⭐⭐⭐⭐    N/A      ❌ 순환 구조 미지원
Transformer        ⭐⭐⭐⭐⭐⭐⭐   N/A      ❌ Attention 미지원
```

### 💡 **핵심 결론**

- ✅ **가능**: Linear, 간단한 MLP (파라미터 ~3,000개까지)
- ❌ **불가능**: RNN, CNN, Transformer (ONNX Opset 18 + EZKL 미지원)
- ⚠️ **현실**: 2025년 ZKML은 연구/POC 단계, 상용화 어려움

---

## 💡 학습 전략

### 📚 **1단계: 기초 다지기** (현재 완료 ✅)

- Week 1: Linear Regression
- Week 2: MLP (Iris)

### 🔧 **2단계: 실험 및 최적화** (Week 3-4)

1. 파라미터 조정 실험
2. 다양한 입력값 테스트
3. 단순 MLP로 MNIST 도전
4. PyTorch 버전 낮춰서 ReLU 테스트

### 🚀 **3단계: 실전 프로젝트** (Month 6)

1. 간단한 추천 시스템
2. 프라이버시 보호 의료 AI
3. 검증 가능한 금융 AI

---

## 📊 Month 3 vs ZKML 비교

| 측면          | Month 3 (일반 AI) | ZKML                     |
| ------------- | ----------------- | ------------------------ |
| **목표**      | 정확도 최대화     | 검증 가능성 + 프라이버시 |
| **모델**      | 복잡할수록 좋음   | 단순할수록 좋음          |
| **연산**      | ReLU, Conv 자유   | Linear 위주              |
| **파라미터**  | 수백만 ~ 수억     | ~100개 권장              |
| **추론 시간** | 밀리초            | 증명: 분 단위            |
| **활용**      | 성능 중시         | 신뢰 중시                |

---

## 🎯 다음 실습 제안

### 옵션 1: **Month 3 모델 재실행** (복습)

```bash
cd /Users/soon/Desktop/ttttttt/python/phase2/month3_ai_advanced
jupyter notebook

# 추천 순서:
# 1. 01_pytorch_basics.ipynb (Linear)
# 2. 02_classification_neural_network.ipynb (MLP)
# 3. 03_cnn_basics.ipynb (CNN)
```

### 옵션 2: **ZKML 실험** (심화)

```bash
cd /Users/soon/Desktop/ttttttt/python/phase2/month5-6_zkml/week2_real_models

# 01_mlp_zkml.py 수정:
# - 레이어 크기 조정 (8 → 16)
# - 파라미터 증가 효과 관찰
# - 다른 데이터셋 시도
```

### 옵션 3: **새 모델 도전** (Week 3)

```bash
# XOR 문제 (비선형)
# 2차 함수 회귀
# MNIST 단순 MLP
```

---

**🎉 Month 3 학습 내용을 완벽히 이해하고 ZKML과 연결했습니다!**

**다음 단계**: 어떤 실습을 해보고 싶으세요? 😊
