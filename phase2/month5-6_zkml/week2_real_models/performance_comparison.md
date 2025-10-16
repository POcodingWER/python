# 📊 ZKML 성능 비교 분석

> **Week 1 vs Week 2**: 모델 복잡도에 따른 증명 시간 변화

⚠️ **주의**: 이 문서는 **이론적 예상치 및 학습 자료**입니다.

- Week 2-1 (MLP Iris)는 실제 테스트 완료 ✅
- Week 2-2 이후는 참고용 예상치
- 실제 측정은 사용자 환경에 따라 다를 수 있음

## 🔬 참고 환경

```
CPU: Apple M1/M2 또는 Intel i7+
RAM: 16GB
Python: 3.10
EZKL: 최신 버전
증명 타입: Single Proof
```

---

## 📈 성능 비교표

| 모델                       | 레이어 | 파라미터 수 | ONNX 크기 | 증명 시간 | Proof 크기 | PK 크기 | 실무 적용           |
| -------------------------- | ------ | ----------- | --------- | --------- | ---------- | ------- | ------------------- |
| **Week 1: Linear**         | 1      | 3           | ~2 KB     | ~30초     | ~17 KB     | ~500 MB | ✅ 즉시 가능        |
| **Week 2-1: MLP**          | 3      | ~200        | ~5 KB     | 1-2분     | ~20 KB     | ~700 MB | ✅ 가능             |
| **Week 2-2: CNN (Simple)** | 5      | ~5,000      | ~50 KB    | 5-10분    | ~30 KB     | ~1.5 GB | ⚠️ 경량화 필요      |
| **Full CNN (CIFAR-10)**    | 7+     | ~50,000     | ~500 KB   | 30-60분   | ~50 KB     | ~3 GB   | ❌ 실용적 X         |
| **RNN/LSTM**               | 3-5    | ~10,000     | ~100 KB   | 10-20분   | ~40 KB     | ~2 GB   | ⚠️ Transformer 권장 |

---

## 💡 핵심 인사이트

### 1. 파라미터 수 vs 증명 시간

```
파라미터 10배 증가 → 증명 시간 약 5배 증가

Linear (3개)     →  MLP (200개)   →  CNN (5,000개)
  30초                1-2분              5-10분
   ↓                   ↓                   ↓
  1x                  4x                 20x
```

**결론**: 선형적으로 증가하지 않음! 파라미터가 많아질수록 가속화 효과.

---

### 2. 레이어 타입별 복잡도

```
연산 타입별 ZK 회로 복잡도 (상대적 비교)

Linear (Dense):       ⭐ (가장 간단)
ReLU:                 ⭐⭐ (비선형이지만 단순)
Conv2d:               ⭐⭐⭐⭐ (공간 연산)
MaxPool2d:            ⭐⭐⭐ (비교 연산)
BatchNorm:            ⭐⭐⭐ (통계 연산)
LSTM/GRU:             ⭐⭐⭐⭐⭐ (순환 구조)
Attention:            ⭐⭐⭐⭐⭐⭐ (행렬 곱 다수)
```

**결론**: Conv2d, LSTM, Attention은 ZK 회로로 변환시 매우 복잡!

---

### 3. 입력 크기의 영향

```
입력 차원과 SRS 크기 관계

Linear (1차원):       4개 입력   → logrows = 10
MLP (1차원):          4개 입력   → logrows = 12
CNN (2차원):          784개 입력 → logrows = 17
CNN (RGB):            3072개 입력 → logrows = 20+

logrows 증가 → SRS 크기 2배 증가 (지수적)
logrows 17 → ~16MB
logrows 20 → ~128MB
```

**결론**: 이미지/텍스트 등 고차원 입력은 SRS 크기 급증!

---

### 4. Proof 크기 vs 증명 시간

```
증명 생성 시간은 길지만, 증명 자체는 작다!

증명 시간:  30초 → 2분 → 10분 (증가)
Proof 크기: 17KB → 20KB → 30KB (거의 변화 없음)

블록체인에 올리는 건 Proof뿐이므로:
→ 가스비는 거의 동일!
```

**결론**: 증명 생성은 로컬에서, 검증은 온체인에서!

---

## 🎯 실무 적용 가이드

### ✅ 즉시 사용 가능 (증명 시간 < 1분)

```
모델 타입:
- Linear Regression
- Logistic Regression
- 단순 MLP (레이어 3개 이하)
- Shallow Decision Tree

사용 사례:
- AI API 검증
- 간단한 추천 시스템
- 온체인 게임 AI
```

---

### ⚠️ 최적화 후 가능 (증명 시간 1-10분)

```
모델 타입:
- Deep MLP (레이어 5개+)
- 경량 CNN (MobileNet 스타일)
- 단순 RNN/LSTM

최적화 방법:
1. 프루닝 (Pruning) - 불필요한 뉴런 제거
2. 양자화 (Quantization) - FP32 → INT8
3. Knowledge Distillation - 큰 모델 → 작은 모델
4. 레이어 수 감소

사용 사례:
- 의료 AI (프라이버시 중요)
- 금융 AI (검증 필요)
- Edge AI 검증
```

---

### ❌ 현재 실용적 X (증명 시간 > 30분)

```
모델 타입:
- 대형 CNN (ResNet, VGG)
- Transformer (BERT, GPT)
- 고해상도 이미지 처리

대안:
1. 모델 축소 (Tiny 버전 사용)
2. 중요 부분만 ZKML 적용
3. Rollup + ZKML 하이브리드
4. 하드웨어 가속 (GPU 클러스터)

연구 단계:
- zkML 전용 하드웨어 개발 중
- 알고리즘 최적화 연구 진행
- 2-3년 후 실용화 예상
```

---

## 🔥 최적화 전략

### 1. 모델 프루닝 (Pruning)

```python
# PyTorch 예시
import torch.nn.utils.prune as prune

# 50% 가중치 제거
prune.l1_unstructured(model.fc1, name='weight', amount=0.5)

결과:
파라미터 50% 감소 → 증명 시간 30-40% 감소
```

---

### 2. 양자화 (Quantization)

```python
# PyTorch 동적 양자화
quantized_model = torch.quantization.quantize_dynamic(
    model, {nn.Linear}, dtype=torch.qint8
)

결과:
모델 크기 75% 감소 → 증명 시간 20-30% 감소
```

---

### 3. Knowledge Distillation

```
Teacher (큰 모델, 95% 정확도)
        ↓ (지식 전달)
Student (작은 모델, 90% 정확도)

결과:
파라미터 10배 감소 → 증명 시간 5배 감소
정확도 5% 하락 → 실무에서 허용 가능
```

---

### 4. 레이어 수 감소

```
원본 CNN:
Conv(32) → Conv(64) → Conv(128) → FC(512) → FC(10)
↓
간소화 CNN:
Conv(16) → Conv(32) → FC(10)

결과:
파라미터 80% 감소 → 증명 시간 70% 감소
```

---

## 📊 실제 측정 결과

### ✅ Week 1: Linear Regression (테스트 완료)

```bash
$ python week1_ezkl_basics/01_simple_linear_regression.py

파라미터:        3개
ONNX 크기:       ~2 KB
증명 시간:       ~30초
Proof 크기:      ~17 KB
PK 크기:         ~528 MB
VK 크기:         ~257 KB

결과: ✅ 정상 작동
```

---

### ✅ Week 2-1: MLP (Iris) (테스트 완료)

```bash
$ python week2_real_models/01_mlp_zkml.py

파라미터:        43개 (Linear만 사용)
ONNX 크기:       3.73 KB
증명 시간:       1-2분
Proof 크기:      17.84 KB
PK 크기:         528.25 MB
VK 크기:         257.26 KB

결과: ✅ 정상 작동 (ReLU 제거 버전)
```

---

### ⚠️ Week 2-2: MLP (MNIST) (예상치)

```bash
$ python week2_real_models/02_cnn_zkml.py

파라미터:        ~25,000개
ONNX 크기:       ~100 KB (예상)
증명 시간:       5-15분 (예상)
Proof 크기:      ~30 KB (예상)
PK 크기:         ~1-2 GB (예상)

결과: 테스트 필요 (Linear만 사용)
```

**참고**: Week 2는 EZKL 호환성을 위해 ReLU/Conv2d를 제거하고 Linear만 사용합니다.

---

## 💡 실무 의사결정 트리

```
모델 선택 플로우차트:

1. "실시간 검증 필요?" (블록체인, API)
   YES → Linear/MLP 사용 (< 1분)
   NO  → 다음 단계

2. "프라이버시 중요?" (의료, 금융)
   YES → 최적화 + CNN/RNN 사용 (< 10분)
   NO  → 일반 AI 사용 (ZKML 불필요)

3. "대용량 데이터?" (고해상도 이미지, 긴 텍스트)
   YES → Rollup + ZKML 하이브리드
   NO  → 직접 ZKML 적용

4. "증명 시간 > 10분?"
   YES → 모델 경량화 필수
   NO  → 바로 적용 가능
```

---

## 🚀 로드맵

### 현재 (2025)

- ✅ Linear/MLP: 실용화
- ⚠️ CNN/RNN: 최적화 필요
- ❌ Transformer: 연구 단계

### 1년 후 (2026)

- ✅ CNN/RNN: 실용화 예상
- ⚠️ 소형 Transformer: 가능
- ❌ GPT-4급: 아직 어려움

### 3년 후 (2028)

- ✅ 대부분 모델 실용화
- ✅ 하드웨어 가속 보편화
- ✅ ZKML-as-a-Service 등장

---

## 📚 참고 자료

### 논문

- "zkML: A Survey" (2024)
- "Efficient Zero-Knowledge Proofs for Neural Networks" (2023)
- "Orion: Zero-Knowledge Proof Systems for Neural Networks" (2024)

### 프로젝트

- EZKL: https://docs.ezkl.xyz/
- Modulus Labs: https://modulus.xyz/
- ZKML Community: https://zkml.io/

---

**🎯 결론**: ZKML은 모델 선택과 최적화가 핵심! 실무에서는 "정확도 vs 증명 시간" 트레이드오프를 고려한 의사결정이 필수입니다.

**📊 Week 2 학습 완료 후**: 여러분은 이제 실전 ZKML 프로젝트를 시작할 준비가 되었습니다! 🚀
