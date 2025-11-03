# 🎨 DCGAN (Deep Convolutional GAN) - 고해상도 이미지 생성

**Week 5-3: DCGAN을 이용한 32x32 고해상도 숫자 이미지 생성**

> "Convolutional Layer로 더 선명하고 안정적인 이미지 생성!"

---

## 📋 목차
1. [프로젝트 개요](#-프로젝트-개요)
2. [DCGAN이란?](#-dcgan이란)
3. [실행 방법](#-실행-방법)
4. [GAN vs DCGAN 비교](#-gan-vs-dcgan-비교)
5. [구현 세부사항](#-구현-세부사항)
6. [기술 스택](#-기술-스택)

---

## 🎯 프로젝트 개요

**DCGAN (Deep Convolutional GAN)**을 Rust로 구현하여 고해상도 숫자 이미지를 생성합니다.

### 주요 기능
- ✅ **Deep Convolutional Generator**: ConvTranspose2d로 고해상도 생성
- ✅ **Deep Convolutional Discriminator**: Conv2d로 정밀 판별
- ✅ **32x32 고해상도**: 기존 28x28보다 더 선명!
- ✅ **조건부 생성**: 원하는 숫자 지정 가능
- ✅ **안정적인 학습**: Convolutional 구조로 더 안정적
- ✅ **변수화된 설정**: 한 줄만 수정!

### 학습 데이터
- **합성 MNIST 데이터**: 2,000개의 32x32 이미지
- **이미지 크기**: 32x32 픽셀 (고해상도!)
- **정규화**: -1 ~ 1 범위 (tanh 활성화 함수)

---

## 🧠 DCGAN이란?

### 핵심 개념

**DCGAN = Deep Convolutional + GAN**

```
┌─────────────────────────────────────────────────────────┐
│                  DCGAN 구조                              │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  랜덤 노이즈 (100차원)                                    │
│       ↓                                                  │
│  ┌──────────────┐                                       │
│  │   FC Layer   │  100 → 256*4*4                        │
│  └──────┬───────┘                                       │
│         ↓                                                │
│  ┌──────────────┐                                       │
│  │ConvTranspose │  4x4 → 8x8 (256 → 128 channels)       │
│  └──────┬───────┘                                       │
│         ↓                                                │
│  ┌──────────────┐                                       │
│  │ConvTranspose │  8x8 → 16x16 (128 → 64 channels)      │
│  └──────┬───────┘                                       │
│         ↓                                                │
│  ┌──────────────┐                                       │
│  │ConvTranspose │  16x16 → 32x32 (64 → 1 channel)       │
│  └──────┬───────┘                                       │
│         ↓                                                │
│  고해상도 이미지 (32x32)                                  │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### DCGAN의 핵심 특징

1. **Convolutional Layers**
   - Generator: ConvTranspose2d (Upsampling)
   - Discriminator: Conv2d (Downsampling)
   - Fully Connected Layer 최소화

2. **Batch Normalization**
   - 학습 안정화
   - 더 빠른 수렴

3. **LeakyReLU**
   - Discriminator에서 gradient 소실 방지
   - α = 0.2

4. **Tanh & Sigmoid**
   - Generator 출력: Tanh (-1 ~ 1)
   - Discriminator 출력: Sigmoid (0 ~ 1)

---

## 🚀 실행 방법

### 0️⃣ 숫자 변경하기

**`src/main.rs` 파일의 467번째 줄만 수정하세요!**

```rust
let target_digit = 0; // 👈 생성할 숫자 (0~9)
```

### 1️⃣ 기본 실행

```bash
# DCGAN 학습 시작 (150 epochs)
npm start

# 또는
cargo run --release
```

### 2️⃣ 생성된 이미지 확인

```bash
# 이미지 폴더 열기
npm run view

# 또는
open generated_images/
```

### 3️⃣ 기타 명령어

```bash
# 도움말
npm run help

# Release 빌드
npm run build

# 정리
npm run clean
```

---

## 📊 GAN vs DCGAN 비교

| 구분 | 기본 GAN (Week 5-2) | DCGAN (Week 5-3) |
|------|---------------------|------------------|
| **이미지 크기** | 28x28 | **32x32** (고해상도!) |
| **Generator** | Fully Connected | **ConvTranspose2d** |
| **Discriminator** | Fully Connected | **Conv2d** |
| **파라미터 수** | 적음 | 많음 (더 복잡) |
| **학습 안정성** | 보통 | **더 안정적** |
| **이미지 품질** | 보통 | **더 선명** |
| **학습 속도** | 빠름 | 느림 |
| **메모리 사용** | 적음 | 많음 |

### 언제 DCGAN을 사용하나요?

✅ **고해상도 이미지**가 필요할 때  
✅ **이미지 품질**이 중요할 때  
✅ **안정적인 학습**이 필요할 때  
✅ **프로덕션 환경**에서 사용할 때

---

## 🔧 구현 세부사항

### 1. DCGAN Generator

```rust
struct DCGenerator {
    fc: Linear,              // (100+10) → 256*4*4
    deconv1: ConvTranspose2d, // 256 → 128 (4x4 → 8x8)
    deconv2: ConvTranspose2d, // 128 → 64 (8x8 → 16x16)
    deconv3: ConvTranspose2d, // 64 → 1 (16x16 → 32x32)
}

// 순전파
[noise + label] → FC → Reshape(256,4,4)
  → ConvTranspose(8x8) → ConvTranspose(16x16) → ConvTranspose(32x32)
  → Tanh → image (32x32)
```

**특징:**
- 입력: 100차원 노이즈 + 10차원 레이블
- 출력: 32x32 이미지
- Upsampling: ConvTranspose2d (stride=2)
- 활성화 함수: LeakyReLU → Tanh

### 2. DCGAN Discriminator

```rust
struct DCDiscriminator {
    conv1: Conv2d,  // (1+10) → 64 (32x32 → 16x16)
    conv2: Conv2d,  // 64 → 128 (16x16 → 8x8)
    conv3: Conv2d,  // 128 → 256 (8x8 → 4x4)
    fc: Linear,     // 256*4*4 → 1
}

// 순전파
[image + label] → Conv(16x16) → Conv(8x8) → Conv(4x4)
  → Flatten → FC → Sigmoid → prob (0~1)
```

**특징:**
- 입력: 32x32 이미지 + 10차원 레이블
- 출력: 확률 (0 = 가짜, 1 = 진짜)
- Downsampling: Conv2d (stride=2)
- 활성화 함수: LeakyReLU → Sigmoid

### 3. 하이퍼파라미터

| 파라미터 | 값 | 설명 |
|---------|-----|------|
| **Target Digit** | **0~9** | **생성할 숫자** |
| Image Size | 32x32 | 고해상도! |
| Latent Dimension | 100 | 노이즈 벡터 차원 |
| Batch Size | 32 | 배치 크기 |
| G Learning Rate | 0.0002 | Generator 학습률 |
| D Learning Rate | 0.00005 | Discriminator 학습률 |
| Epochs | 150 | 학습 에포크 |
| Training Samples | 2,000 | 학습 데이터 개수 |

---

## 🛠️ 기술 스택

### Rust 라이브러리

| 라이브러리 | 버전 | 용도 |
|-----------|------|------|
| `candle-core` | 0.8.4 | 텐서 연산 |
| `candle-nn` | 0.8.4 | 신경망 레이어 (Conv, ConvTranspose) |
| `rand` | 0.8.5 | 랜덤 노이즈 생성 |
| `image` | 0.25 | 이미지 저장 |

### 주요 기능

1. **Convolutional Layers**
   - Conv2d: Downsampling
   - ConvTranspose2d: Upsampling
   - Stride, Padding 설정

2. **신경망**
   - Linear Layer
   - AdamW Optimizer
   - VarMap (파라미터 관리)

3. **이미지 처리**
   - 32x32 PNG 저장
   - GrayImage 생성

---

## 🎓 학습 포인트

### 1. DCGAN의 핵심 개념

- **Convolutional 구조**: 이미지에 특화된 구조
- **Upsampling**: ConvTranspose2d로 해상도 증가
- **Downsampling**: Conv2d로 특징 추출
- **안정적인 학습**: Fully Connected보다 안정적

### 2. 기본 GAN과의 차이

- **구조**: FC → Convolutional
- **해상도**: 28x28 → 32x32
- **품질**: 더 선명하고 구조화된 이미지
- **안정성**: 더 안정적인 학습

### 3. 성능 지표

- **이미지 품질**: 더 선명하고 디테일함
- **학습 안정성**: Mode Collapse 적음
- **메모리**: 더 많이 사용 (Convolutional Layer)
- **속도**: 기본 GAN보다 느림

---

## 🚧 개선 방향

### ✅ 현재 구현

- [x] Deep Convolutional 구조
- [x] 32x32 고해상도
- [x] 조건부 생성
- [x] 안정적인 학습

### 🔮 추가 개선

- [ ] **Batch Normalization**: 더 안정적인 학습
- [ ] **64x64 해상도**: 더 높은 해상도
- [ ] **컬러 이미지**: RGB 3채널
- [ ] **Progressive GAN**: 점진적 해상도 증가
- [ ] **StyleGAN**: 스타일 제어

---

## 📚 참고 자료

### 논문
- [DCGAN (2015)](https://arxiv.org/abs/1511.06434) - Unsupervised Representation Learning with Deep Convolutional Generative Adversarial Networks

### 핵심 기여
- Convolutional 구조를 GAN에 적용
- 안정적인 학습 가이드라인 제시
- 고해상도 이미지 생성 가능

---

## 🎉 결론

이 프로젝트에서 구현한 내용:

✅ **Deep Convolutional Generator**: ConvTranspose2d로 Upsampling  
✅ **Deep Convolutional Discriminator**: Conv2d로 Downsampling  
✅ **32x32 고해상도**: 기본 GAN보다 더 선명  
✅ **조건부 생성**: 원하는 숫자 지정 가능  
✅ **안정적인 학습**: Convolutional 구조의 장점  

### 핵심 성과

- 🎯 **고해상도 이미지**: 32x32 픽셀
- 🎯 **더 선명한 품질**: Convolutional Layer 효과
- 🎯 **안정적인 학습**: Mode Collapse 감소
- 🎯 **실용적인 구조**: 프로덕션 환경에 적합

### 다음 단계

1. **Batch Normalization 추가**: 더 안정적인 학습
2. **64x64 해상도**: 더 높은 해상도
3. **StyleGAN**: 최신 GAN 기술

---

**Made with 🦀 Rust + 🔥 Candle**

*"Convolutional Layer로 더 선명하고 안정적인 이미지 생성!"* 🎨✨

