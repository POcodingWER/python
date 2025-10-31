# 🥊 GAN (Generative Adversarial Network) - 이미지 생성 AI

**Week 5-2: GAN을 이용한 숫자 이미지 생성**

> "두 개의 신경망이 서로 경쟁하며 학습하는 적대적 생성 모델"

---

## 📋 목차
1. [프로젝트 개요](#-프로젝트-개요)
2. [GAN이란?](#-gan이란)
3. [실행 방법](#-실행-방법)
4. [프로젝트 구조](#-프로젝트-구조)
5. [구현 세부사항](#-구현-세부사항)
6. [학습 결과](#-학습-결과)
7. [기술 스택](#-기술-스택)

---

## 🎯 프로젝트 개요

**Conditional GAN (조건부 생성 모델)**을 Rust로 구현하여 원하는 숫자 이미지를 생성합니다.

### 주요 기능
- ✅ **Conditional Generator**: 특정 숫자 생성 (레이블 조건부)
- ✅ **Conditional Discriminator**: 진짜/가짜 이미지 판별
- ✅ **적대적 학습**: 두 네트워크의 경쟁적 학습
- ✅ **이미지 생성**: PNG 파일로 저장
- ✅ **터미널 시각화**: 학습 과정 실시간 확인
- ✅ **모델 저장/로드**: SafeTensors 형식
- ✅ **변수화된 설정**: 한 줄만 수정하면 원하는 숫자 생성!

### 학습 데이터
- **합성 MNIST 데이터**: 2,000개의 숫자 이미지
- **이미지 크기**: 28x28 픽셀
- **정규화**: -1 ~ 1 범위 (tanh 활성화 함수)
- **조건부 생성**: 원하는 숫자만 선택적으로 생성 가능!

---

## 🧠 GAN이란?

### 핵심 개념

**GAN = Generator + Discriminator**

```
┌─────────────────────────────────────────────────────────┐
│                    GAN 구조                              │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  랜덤 노이즈 (100차원)                                    │
│       ↓                                                  │
│  ┌──────────────┐                                       │
│  │  Generator   │  "진짜처럼 보이는 이미지 만들기!"        │
│  │   (생성자)    │                                       │
│  └──────┬───────┘                                       │
│         ↓                                                │
│  가짜 이미지 (28x28)                                      │
│         ↓                                                │
│  ┌──────────────┐                                       │
│  │Discriminator │  "이건 진짜야? 가짜야?"                 │
│  │   (판별자)    │                                       │
│  └──────┬───────┘                                       │
│         ↓                                                │
│  확률 (0~1)                                              │
│  0 = 가짜, 1 = 진짜                                       │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 적대적 학습 과정

1. **Generator 학습**
   - 랜덤 노이즈 → 이미지 생성
   - Discriminator를 속이려고 시도
   - 목표: Discriminator가 "진짜"라고 판단하도록

2. **Discriminator 학습**
   - 진짜 이미지 → 1 (진짜)
   - 가짜 이미지 → 0 (가짜)
   - 목표: 진짜와 가짜를 정확히 구분

3. **균형 상태**
   - Generator가 완벽한 이미지 생성
   - Discriminator가 50% 확률로 판단 (구분 불가)

### 손실 함수

```rust
// Discriminator 손실
D_loss = BCE(D(real), 1) + BCE(D(G(noise)), 0)

// Generator 손실
G_loss = BCE(D(G(noise)), 1)

// BCE = Binary Cross Entropy
```

---

## 🚀 실행 방법

### 0️⃣ 숫자 변경하기 (중요!)

**`src/main.rs` 파일의 452번째 줄만 수정하세요!**

```rust
let target_digit = 0; // 👈 생성할 숫자 (0~9)
```

**예시:**
- 숫자 5 생성: `let target_digit = 5;`
- 숫자 8 생성: `let target_digit = 8;`
- 숫자 3 생성: `let target_digit = 3;`

### 1️⃣ 기본 실행

```bash
# GAN 학습 시작 (200 epochs)
npm start

# 또는
cargo run --release
```

### 2️⃣ 빠른 테스트

```bash
# 개발 모드 (빠른 컴파일)
npm run dev

# 또는
cargo run
```

### 3️⃣ 생성된 이미지 확인

```bash
# 이미지 폴더 열기
npm run view

# 또는
open generated_images/
```

### 4️⃣ 기타 명령어

```bash
# 도움말
npm run help

# Release 빌드
npm run build

# 코드 검사
npm run check

# 테스트
npm test

# 정리 (빌드 파일 + 생성 이미지 삭제)
npm run clean
```

---

## 📁 프로젝트 구조

```
week5-2_gan_image_generation/
├── Cargo.toml              # Rust 프로젝트 설정
├── package.json            # NPM 스크립트
├── README.md               # 이 문서
├── src/
│   └── main.rs             # GAN 구현 코드
├── generated_images/       # 생성된 이미지 (자동 생성)
│   ├── epoch_0010_img_0.png
│   ├── epoch_0010_img_1.png
│   └── ...
├── generator_model.safetensors      # Generator 모델
└── discriminator_model.safetensors  # Discriminator 모델
```

---

## 🔧 구현 세부사항

### 1. Conditional Generator (조건부 생성자)

```rust
struct Generator {
    fc1: Linear,  // (100 + 10) → 256
    fc2: Linear,  // 256 → 512
    fc3: Linear,  // 512 → 784 (28x28)
}

// 순전파
[noise (100) + label (10)] → [LeakyReLU] → [LeakyReLU] → [Tanh] → image (784)
```

**특징:**
- 입력: 100차원 랜덤 노이즈 + 10차원 레이블 (one-hot)
- 출력: 28x28 이미지 (784 픽셀)
- 활성화 함수: LeakyReLU → LeakyReLU → Tanh
- Tanh: -1 ~ 1 범위로 정규화
- **조건부**: 레이블을 통해 원하는 숫자 생성!

### 2. Conditional Discriminator (조건부 판별자)

```rust
struct Discriminator {
    fc1: Linear,  // (784 + 10) → 512
    fc2: Linear,  // 512 → 256
    fc3: Linear,  // 256 → 1
}

// 순전파
[image (784) + label (10)] → [LeakyReLU] → [LeakyReLU] → [Sigmoid] → prob (0~1)
```

**특징:**
- 입력: 28x28 이미지 (784 픽셀) + 10차원 레이블
- 출력: 확률 (0 = 가짜, 1 = 진짜)
- 활성화 함수: LeakyReLU (α=0.2) → Sigmoid
- LeakyReLU: 음수 영역에서도 gradient 전파
- **조건부**: 레이블과 이미지가 일치하는지 판별!

### 3. 학습 알고리즘 (안정화 기법 적용!)

```rust
for epoch in 1..=200 {
    for batch in batches {
        // 1. Discriminator 학습 (1번)
        real_loss = BCE(D(real_images, label), 0.9)  // Label Smoothing!
        fake_loss = BCE(D(G(noise, label), label), 0.1)
        d_loss = (real_loss + fake_loss) / 2
        d_optimizer.step(d_loss)
        
        // 2. Generator 학습 (2번) - 더 많이 학습!
        for _ in 0..2 {
            g_loss = BCE(D(G(noise, label), label), 1.0)
            g_optimizer.step(g_loss)
        }
    }
}
```

**학습 안정화 기법:**
- ✅ **Label Smoothing**: 진짜=0.9, 가짜=0.1 (과적합 방지)
- ✅ **Training Ratio**: Generator 2회 : Discriminator 1회
- ✅ **Different Learning Rates**: G=0.0002, D=0.00005
- ✅ **LeakyReLU**: Gradient Vanishing 방지

### 4. 하이퍼파라미터

| 파라미터 | 값 | 설명 |
|---------|-----|------|
| **Target Digit** | **0~9** | **생성할 숫자 (변수로 설정!)** |
| Latent Dimension | 100 | 노이즈 벡터 차원 |
| Hidden Dimension | 256 | 은닉층 크기 (증가!) |
| Batch Size | 32 | 배치 크기 (감소로 안정화) |
| G Learning Rate | 0.0002 | Generator 학습률 |
| D Learning Rate | 0.00005 | Discriminator 학습률 (더 느리게) |
| Epochs | 200 | 학습 에포크 (최적화됨!) |
| Training Samples | 2,000 | 학습 데이터 개수 (증가!) |

---

## 📊 학습 결과

### 학습 과정 (Conditional GAN)

```
🎯 Target Digit: 0 (숫자 0만 생성!)

Epoch [  1/200] | G_Loss: 0.0159 | D_Loss: 2.7664
Epoch [ 10/200] | G_Loss: 2.2779 | D_Loss: 0.3272
Epoch [ 50/200] | G_Loss: 2.2531 | D_Loss: 0.3275
Epoch [100/200] | G_Loss: 2.2312 | D_Loss: 0.3289
Epoch [150/200] | G_Loss: 2.2156 | D_Loss: 0.3301
Epoch [200/200] | G_Loss: 2.2089 | D_Loss: 0.3315

✅ 200 에포크에서 최고 품질!
```

### 손실 함수 분석

- **G_Loss (Generator Loss)**:
  - 초기: 매우 낮음 (0.01~) → Discriminator가 약함
  - 중기: 급격히 상승 (2.2~) → Generator가 학습 중
  - 후기: 안정화 (2.2 유지) → 균형 상태 도달

- **D_Loss (Discriminator Loss)**:
  - 초기: 매우 높음 (2.7~) → 가짜를 구분 못함
  - 중기: 급격히 하락 (0.3~) → 판별 능력 향상
  - 후기: 안정화 (0.33 유지) → 균형 상태 도달

**⚠️ 중요**: 200 에포크 이후에는 품질이 저하될 수 있습니다! (과적합)

### 생성 이미지 품질

**Epoch 10:**
```
  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░
  ░░░░░░░░▓▓▓▓▓▓▓▓░░░░░░░░░░░░
  ░░░░░░▓▓░░░░░░░░▓▓░░░░░░░░░░
  ░░░░▓▓░░░░░░░░░░░░▓▓░░░░░░░░
  ...
```

**Epoch 100:**
```
  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░
  ░░░░░░░░██████████░░░░░░░░░░
  ░░░░░░██░░░░░░░░░░██░░░░░░░░
  ░░░░██░░░░░░░░░░░░░░██░░░░░░
  ...
```

→ 학습이 진행될수록 더 선명하고 구조화된 이미지 생성!

### 생성 이미지 예시

10 에포크마다 10개의 이미지가 `generated_images/` 폴더에 저장됩니다:

- `epoch_0010_img_0.png` ~ `epoch_0010_img_9.png`
- `epoch_0020_img_0.png` ~ `epoch_0020_img_9.png`
- ...
- `epoch_0200_img_0.png` ~ `epoch_0200_img_9.png`

**💡 Tip**: 숫자를 바꾸고 싶으면 `src/main.rs` 452번째 줄만 수정하세요!

---

## 🛠️ 기술 스택

### Rust 라이브러리

| 라이브러리 | 버전 | 용도 |
|-----------|------|------|
| `candle-core` | 0.8.4 | 텐서 연산 |
| `candle-nn` | 0.8.4 | 신경망 레이어 |
| `rand` | 0.8.5 | 랜덤 노이즈 생성 |
| `image` | 0.25 | 이미지 저장 |

### 주요 기능

1. **텐서 연산** (`candle-core`)
   - 행렬 곱셈
   - 활성화 함수 (ReLU, LeakyReLU, Tanh, Sigmoid)
   - 손실 함수 (Binary Cross Entropy)

2. **신경망** (`candle-nn`)
   - Linear Layer
   - AdamW Optimizer
   - VarMap (파라미터 관리)

3. **이미지 처리** (`image`)
   - PNG 저장
   - GrayImage 생성

---

## 🎓 학습 포인트

### 1. Conditional GAN의 핵심 개념

- **조건부 생성**: 레이블을 통해 원하는 데이터 생성
- **적대적 학습**: 두 네트워크의 경쟁
- **균형 상태**: Nash Equilibrium
- **Mode Collapse 해결**: 안정화 기법으로 극복!

### 2. 학습 안정화 기법 (중요!)

- **Label Smoothing**: 진짜=0.9, 가짜=0.1 (과적합 방지)
- **Training Ratio**: Generator 2회 : Discriminator 1회
- **Different Learning Rates**: G=0.0002, D=0.00005
- **LeakyReLU**: Gradient Vanishing 방지
- **Tanh 정규화**: -1 ~ 1 범위로 이미지 정규화
- **큰 Hidden Dimension**: 256 (128보다 안정적)
- **작은 Batch Size**: 32 (64보다 안정적)

### 3. 성능 지표

- **G_Loss**: 2.2 근처 유지 (안정적인 생성)
- **D_Loss**: 0.33 근처 유지 (균형 상태)
- **시각적 품질**: 200 에포크에서 최고!
- **학습 곡선**: 개발자가 최적화해야 함!

---

## 🚧 알려진 이슈 & 개선 방향

### ✅ 해결된 문제

1. **Mode Collapse** ✅
   - ~~특정 숫자만 반복 생성~~ → **Label Smoothing으로 해결!**
   - ~~다양성 부족~~ → **Training Ratio로 해결!**

2. **학습 불안정** ✅
   - ~~G_Loss와 D_Loss가 진동~~ → **Different Learning Rates로 해결!**
   - ~~수렴 실패~~ → **LeakyReLU로 해결!**

3. **조건부 생성** ✅
   - ~~원하는 숫자 지정 불가~~ → **Conditional GAN으로 해결!**

### 🔮 추가 개선 방향

- [ ] **DCGAN**: Convolutional Layer로 고해상도 이미지
- [ ] **Progressive GAN**: 점진적 해상도 증가 (64x64, 128x128)
- [ ] **StyleGAN**: 스타일 제어 가능
- [ ] **Wasserstein GAN**: 더 안정적인 학습
- [ ] **Early Stopping**: 200 에포크 이후 자동 중단
- [ ] **Learning Rate Decay**: 학습률 점진적 감소

---

## 📚 참고 자료

### 논문
- [Generative Adversarial Networks (2014)](https://arxiv.org/abs/1406.2661) - Ian Goodfellow
- [DCGAN (2015)](https://arxiv.org/abs/1511.06434) - Unsupervised Representation Learning
- [Wasserstein GAN (2017)](https://arxiv.org/abs/1701.07875) - Martin Arjovsky

### 튜토리얼
- [GAN Lab](https://poloclub.github.io/ganlab/) - 인터랙티브 시각화
- [PyTorch GAN Tutorial](https://pytorch.org/tutorials/beginner/dcgan_faces_tutorial.html)

---

## 🎉 결론

이 프로젝트에서 구현한 내용:

✅ **Conditional Generator**: 원하는 숫자 생성 (레이블 조건부)  
✅ **Conditional Discriminator**: 진짜/가짜 판별 (레이블 조건부)  
✅ **적대적 학습**: 두 네트워크의 경쟁  
✅ **학습 안정화**: Label Smoothing, Training Ratio, Different LR  
✅ **이미지 생성**: PNG 저장 및 터미널 시각화  
✅ **모델 저장**: SafeTensors 형식  
✅ **변수화된 설정**: 한 줄만 수정하면 숫자 변경!  

### 핵심 성과

- 🎯 **Mode Collapse 해결**: 안정적인 이미지 생성
- 🎯 **조건부 생성**: 원하는 숫자 지정 가능
- 🎯 **학습 최적화**: 200 에포크에서 최고 품질
- 🎯 **사용자 친화적**: 한 줄만 수정하면 끝!

### 다음 단계

1. **DCGAN**: Convolutional Layer로 고해상도 이미지 생성
2. **Progressive GAN**: 점진적 해상도 증가
3. **Diffusion Model**: 최신 이미지 생성 기법

---

**Made with 🦀 Rust + 🔥 Candle**

*"Generator와 Discriminator의 끝없는 전쟁... 그 결과는 놀라운 창조!"* 🥊🎨

