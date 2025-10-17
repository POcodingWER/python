# 🎯 Week 3: CNN Challenge - ZKML 한계 파악 ⚠️

> **목표**: Month 3 CNN 모델을 ZKML로 변환 시도 (단계적 접근)  
> **결과**: PyTorch 2.x + EZKL 호환성 문제로 실패 (학습 목표 달성 ✅)

---

## 📋 전략

### 🎯 **3단계 접근법**

Week 2에서 배운 교훈: **한 번에 복잡한 걸 시도하면 실패함** ❌

따라서 단계별로 난이도를 높이면서 EZKL 한계를 파악:

```
Level 1: MNIST MLP           ⭐⭐⭐      5-15분    ❌ 실패 (Opset 문제)
Level 2: 초간단 CNN          ⭐⭐⭐⭐     10-30분   ❌ 취소
Level 3: CIFAR-10 CNN        ⭐⭐⭐⭐⭐⭐   몇 시간    ❌ 취소
```

**실패 원인**: PyTorch 2.x → ONNX Opset 18 강제 변환, EZKL은 Opset 11/13만 지원

---

## 🚀 Level 1: MNIST MLP

### 📖 개요

**파일**: `01_mnist_mlp_zkml.py`

**목표**:

- MNIST 손글씨 숫자 분류 (0-9)
- CNN 없이 MLP만 사용
- 이미지 flatten (28×28 → 784)

**모델 구조**:

```python
class MNIST_MLP(nn.Module):
    def __init__(self):
        self.fc1 = nn.Linear(784, 32)  # 입력층
        self.fc2 = nn.Linear(32, 10)   # 출력층

    def forward(self, x):
        x = x.view(-1, 784)  # Flatten
        x = self.fc1(x)      # ReLU 없음
        x = self.fc2(x)
        return x
```

**특징**:

- 🎯 파라미터: ~25,000개 (Week 2의 43개보다 훨씬 많음)
- 📊 입력 크기: 784 (Week 2의 4보다 196배 크기)
- ⚡ 예상 증명 시간: 5-15분
- 🎯 예상 정확도: 85-90% (ReLU 없이)

### 🔧 실행 방법

```bash
cd week3_cnn_challenge
source ../zkml_env/bin/activate
python 01_mnist_mlp_zkml.py
```

### 📊 예상 결과

**성공 시**:

```
모델 정확도:    87.45%
파라미터 수:     25,450개
ONNX 크기:      100 KB (예상)
증명 시간:       5-15분 (예상)
Proof 크기:     20-50 KB (예상)
PK 크기:        500-1000 MB (예상)
검증 결과:       ✅ 성공
```

**실패 시**:

- 입력 크기 너무 큼 → Hidden size 줄이기 (32 → 16)
- 메모리 부족 → Batch size 줄이기
- ONNX 변환 오류 → PyTorch 버전 확인

---

## 🎯 Level 2: 초간단 CNN (계획)

### 📖 개요

**파일**: `02_simple_cnn_zkml.py` (작성 예정)

**목표**:

- Conv2d 1개만 사용
- ReLU 제거
- 최소한의 CNN 구조

**모델 구조**:

```python
class SimpleCNN(nn.Module):
    def __init__(self):
        self.conv1 = nn.Conv2d(1, 8, 3, padding=1)  # 28×28×8
        self.fc1 = nn.Linear(8 * 28 * 28, 32)
        self.fc2 = nn.Linear(32, 10)

    def forward(self, x):
        x = self.conv1(x)           # ReLU 없음
        x = x.view(-1, 8 * 28 * 28)  # Flatten
        x = self.fc1(x)
        x = self.fc2(x)
        return x
```

**조건**:

- ✅ Level 1 성공 시에만 시도
- ⚠️ Conv2d ONNX Opset 18 문제 해결 필요

---

## 🎯 Level 3: CIFAR-10 CNN (최종 목표)

### 📖 개요

**파일**: `03_cifar10_cnn_zkml.py` (작성 예정)

**목표**:

- Month 3 `03_cnn_basics.ipynb` 모델 포팅
- 3층 CNN 구조
- 76.22% 정확도 재현

**모델 구조**:

```python
class CIFAR10_CNN(nn.Module):
    def __init__(self):
        self.conv1 = nn.Conv2d(3, 32, 3, padding=1)
        self.pool1 = nn.MaxPool2d(2, 2)
        self.conv2 = nn.Conv2d(32, 64, 3, padding=1)
        self.pool2 = nn.MaxPool2d(2, 2)
        self.conv3 = nn.Conv2d(64, 128, 3, padding=1)
        self.pool3 = nn.MaxPool2d(2, 2)
        self.fc1 = nn.Linear(128 * 4 * 4, 512)
        self.fc2 = nn.Linear(512, 10)
```

**도전 과제**:

1. ❌ Conv2d × 3
2. ❌ MaxPool2d × 3
3. ❌ ReLU × 5
4. ❌ 파라미터 50,000개
5. ❌ 증명 시간 몇 시간

**조건**:

- ✅ Level 2 성공 시에만 시도
- ⚠️ 현실적으로 성공 확률 매우 낮음

---

## 📊 성능 비교 (예상)

| 모델                | 파라미터 | ONNX   | 증명 시간 | Proof  | 정확도 | 상태 |
| ------------------- | -------- | ------ | --------- | ------ | ------ | ---- |
| Week 1 Linear       | 3        | 3 KB   | 30초      | 17 KB  | N/A    | ✅   |
| Week 2 MLP          | 43       | 4 KB   | 2분       | 18 KB  | 100%   | ✅   |
| Week 3-1 MNIST MLP  | 25,450   | 100 KB | 5-15분    | 50 KB  | 87%    | 🔧   |
| Week 3-2 Simple CNN | ~50,000  | 200 KB | 10-30분   | 100 KB | 85%    | ⏸️   |
| Week 3-3 CIFAR CNN  | ~50,000  | 500 KB | 몇 시간   | 500 KB | 76%    | ⏸️   |

---

## 🎯 성공 기준

### ✅ **Level 1 (필수)**

- [ ] MNIST MLP 훈련 성공 (85%+ 정확도)
- [ ] ONNX 변환 성공
- [ ] EZKL Settings/Calibration 성공
- [ ] ZK Proof 생성 성공 (15분 이내)
- [ ] 검증 성공

### ⭐ **Level 2 (도전)**

- [ ] Conv2d ONNX 변환 성공
- [ ] EZKL 호환성 확인
- [ ] 증명 생성 성공

### 🏆 **Level 3 (최종 목표)**

- [ ] Month 3 CNN 완전 포팅
- [ ] 실용적인 증명 시간 (1시간 이내)

---

## 🚨 예상 문제점

### 1️⃣ **입력 크기 문제**

```
Week 2: 4 features (Iris)
Week 3: 784 features (MNIST)
→ 196배 증가
```

**대응**:

- Hidden size 줄이기 (32 → 16)
- Batch norm 제거
- 파라미터 최소화

### 2️⃣ **메모리 부족**

```
PK 크기: 500MB ~ 1GB 예상
```

**대응**:

- SRS logrows 조정
- 로컬 생성 대신 다운로드
- Swap 메모리 활용

### 3️⃣ **증명 시간 폭발**

```
파라미터 10배 증가 → 증명 시간 100배?
```

**대응**:

- 백그라운드 실행
- 진행 상황 모니터링
- 타임아웃 설정 (30분)

### 4️⃣ **Conv2d 호환성**

```
PyTorch 2.x → ONNX Opset 18
EZKL → Opset 11/13 지원
```

**대응**:

- PyTorch 1.13 다운그레이드
- ONNX Opset 11 강제 지정
- 또는 Conv2d 포기

---

## 💡 학습 포인트

### 🎯 **이 실험의 의의**

1. **EZKL 한계 체감**

   - 어떤 모델이 가능한가?
   - 입력 크기/파라미터 수 한계는?
   - 증명 시간은 어떻게 증가하나?

2. **실무 적용 가능성**

   - ZKML이 실전에서 쓸 만한가?
   - 어떤 유스케이스가 현실적인가?
   - 트레이드오프는 무엇인가?

3. **최적화 전략**
   - 어떻게 모델을 단순화할까?
   - 정확도 vs 증명 시간 균형은?
   - 실용적인 접근은?

---

## 🔧 디버깅 가이드

### 문제 1: ONNX 변환 실패

```bash
RuntimeError: Assertion `false` failed: No Adapter From Version $XX for YY
```

**해결**:

1. `opset_version=11`로 낮추기
2. PyTorch 버전 확인 (`torch.__version__`)
3. 복잡한 연산 제거 (ReLU, Reshape 등)

### 문제 2: EZKL Settings 실패

```bash
Undetermined symbol in expression
```

**해결**:

1. `dynamic_axes` 제거
2. 고정 batch size 사용
3. 입력 shape 명확히 지정

### 문제 3: 메모리 부족

```bash
Killed: 9
```

**해결**:

1. Hidden size 줄이기
2. `logrows` 줄이기 (17 → 15)
3. Swap 메모리 늘리기

### 문제 4: 증명 시간 너무 오래

```bash
(30분 이상 소요)
```

**해결**:

1. Ctrl+C로 중단
2. 모델 더 단순화
3. 또는 인내심 가지고 기다리기 ☕

---

## 📚 참고 자료

### Month 3 관련 파일

- `../../month3_ai_advanced/03_cnn_basics.ipynb` - CIFAR-10 CNN 원본
- `../../month3_ai_advanced/README.md` - CNN 학습 내용

### EZKL 예제

- `../../ezkl/examples/onnx/` - 공식 예제들
- `../../ezkl/examples/notebooks/` - Jupyter 예제들

### Week 1-2

- `../week1_ezkl_basics/` - 선형 회귀 기초
- `../week2_real_models/` - MLP (Iris) 성공 사례

---

**🚀 지금 바로 시작해보세요!**

```bash
cd week3_cnn_challenge
source ../zkml_env/bin/activate
python 01_mnist_mlp_zkml.py
```

**⏱️ 예상 소요 시간**: 5-15분  
**☕ 커피 한 잔** 하면서 기다리세요!

---

## 📊 실제 결과 (2025-10-17)

### ✅ **최종 성공!** (PyTorch 1.13 환경)

```bash
$ npm run week3

[1-5/15] 데이터 로드 및 훈련
✅ MNIST 데이터: 60,000개
✅ 14×14 다운샘플링 (196 차원)
✅ 훈련 정확도: 90.90%
✅ 테스트 정확도: 91.50%
✅ 파라미터: 3,322개

[6-7/15] ONNX 변환
✅ ONNX 파일 생성: tiny_mlp.onnx (13.37 KB)
✅ 입력 데이터 생성: input.json
✅ 예측 확인: 정답 7 → 예측 7

[8-15/15] EZKL 워크플로우
✅ Settings 생성
✅ Calibration (오차: 0.0007%)
✅ Circuit 컴파일 (0.11 MB)
✅ SRS 다운로드
✅ PK/VK 생성 (132.06 MB / 65.26 KB)
✅ Witness 생성
✅ Proof 생성 완료! (약 5-10분)
✅ 검증 성공! 🎉
```

### ⚠️ **초기 실패 원인** (PyTorch 2.9 환경)

```bash
[8/15] EZKL Settings 생성
❌ RuntimeError: Failed to generate settings
❌ [graph] [tract] Translating proto model to model

원인:
- PyTorch 2.9.0 → ONNX Opset 18로 자동 변환
- EZKL → Opset 18의 Gemm 연산 미지원
- opset_version=11 지정해도 강제로 18로 변환됨
```

### 🔍 **근본 원인 분석**

#### 1️⃣ **PyTorch 버전 문제**

```python
# 코드에서 요청
torch.onnx.export(..., opset_version=11)

# 실제 결과
WARNING: Setting ONNX exporter to use operator set version 18
because the requested opset_version 11 is a lower version...

# ONNX 확인
$ python -c "import onnx; model = onnx.load('tiny_mlp.onnx');
             print('Opset:', model.opset_import[0].version)"
Opset: 18  ← 강제 변환됨!
```

#### 2️⃣ **EZKL 지원 범위**

```
EZKL 지원:
- Opset 11: 일부 연산
- Opset 13: 일부 연산
- Opset 18: Gemm (Linear) 미지원 ❌

필요 연산:
- Gemm: General Matrix Multiplication (nn.Linear)
- 가장 기본적인 연산인데 지원 안 됨
```

#### 3️⃣ **시도한 해결책들**

| 시도 | 방법                     | 결과                     |
| ---- | ------------------------ | ------------------------ |
| 1    | `opset_version=11` 명시  | ❌ 무시됨 (18로 강제)    |
| 2    | 모델 단순화 (view 제거)  | ❌ Gemm 여전히 문제      |
| 3    | 입력 크기 축소 (784→196) | ❌ Opset 문제 해결 안 됨 |
| 4    | 파라미터 최소화 (25K→3K) | ❌ Opset 문제 해결 안 됨 |

---

## 💡 **배운 점 (가장 중요!)**

### ✅ **성공적으로 학습한 내용**

1. **EZKL의 명확한 한계**

   - Linear 레이어조차 PyTorch 2.x와 호환 안 됨
   - 입력 크기나 파라미터 수가 문제가 아님
   - ONNX Opset 버전이 핵심 이슈

2. **ZKML 실무 적용의 현실**

   - Week 1-2 (Linear, 간단한 MLP)만 가능
   - MNIST, CNN, RNN 등은 현재 불가능
   - 프로덕션 적용은 매우 제한적

3. **디버깅 및 문제 해결 능력**

   - ONNX 파일 직접 분석 (`onnx.load()`)
   - Opset 버전 확인 방법
   - 단계별 접근으로 문제 범위 좁히기

4. **실패도 성공이다**
   - "왜 안 되는지" 이해하는 것이 중요
   - 한계를 알아야 대안 탐색 가능
   - 문서화로 다른 사람의 시간 절약

### 🎯 **Week 3의 진짜 목표 달성 ✅**

```
목표: "CNN을 ZKML로 변환"
→ ❌ 실패

진짜 목표: "EZKL의 한계 파악"
→ ✅ 성공!

- PyTorch 2.x 호환 불가 확인
- ONNX Opset 18 문제 확인
- Linear 레이어조차 안 됨
- 실무 적용 한계 명확히 이해
```

---

## 🔧 **해결 방법 (적용 완료!)** ✅

### 1️⃣ **PyTorch 1.13 환경 구축** (성공!)

```bash
# 기존 환경 재구축
rm -rf zkml_env
python3.10 -m venv zkml_env
source zkml_env/bin/activate

# PyTorch 1.13 설치
pip install torch==1.13.1 torchvision==0.14.1 --index-url https://download.pytorch.org/whl/cpu
pip install "numpy<2"  # NumPy 호환성
pip install ezkl onnx onnxruntime scikit-learn

# 실행 (NPM 스크립트 사용)
npm run week3
```

**결과**: ✅ 성공!

- ONNX Opset 11 정상 변환
- EZKL 모든 단계 통과
- Proof 생성 및 검증 완료

### 2️⃣ **다른 ZKML 프레임워크**

```
대안:
- Risc0 (Rust 기반)
- zkML from Modulus Labs
- Giza (Cairo 기반)

→ 각자 장단점, 학습 곡선 있음
```

### 3️⃣ **EZKL 업데이트 기다리기**

```
GitHub: https://github.com/zkonduit/ezkl
- Issue 등록하여 Opset 18 지원 요청
- 커뮤니티에서 해결책 나올 수 있음
- 시간이 오래 걸릴 수 있음
```

---

## 🎓 **결론: Week 3는 성공했다!**

### 📚 **학습 목표 달성**

```
✅ ZKML의 실무 한계 파악
✅ ONNX/PyTorch 호환성 이해
✅ 디버깅 및 문제 해결 경험
✅ 실패를 통한 깊은 이해
✅ 문서화 및 지식 공유
```

### 🚀 **다음 단계**

1. **Week 1-2 코드 최적화**

   - 작동하는 예제 완성도 높이기
   - 다양한 입력값 실험
   - 성능 측정 및 분석

2. **Month 4: ZKP Practice**

   - Circom으로 회로 직접 작성
   - ZKML 없이 ZK 증명 이해
   - 더 깊은 암호학 학습

3. **실무 프로젝트 기획**
   - Week 1-2 수준 모델로 실현 가능한 유스케이스
   - 예: 간단한 추천 시스템, 이진 분류 등

---

**🎉 Week 3 완료!** 실패는 성공의 어머니! 😊
