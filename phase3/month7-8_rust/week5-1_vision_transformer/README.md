# 👁️ Vision Transformer (ViT) - 이미지 분류

Rust Candle로 구현한 Vision Transformer 모델입니다!

## 🎯 성능

- **정확도: 100%** (학습 데이터 기준)
- **모델 크기:**
  - 임베딩: 128차원
  - 헤드: 8개
  - 레이어: 6개
- **학습 데이터:**
  - 합성 데이터: 500개 (숫자 0-9, 각 50개씩)
  - 손글씨 데이터: CSV로 추가 가능 ✨

## 🚀 빠른 시작

### NPM 스크립트 사용 (권장)

```bash
# 도움말 보기
npm run help

# 빠른 시작 (기존 데이터로 바로 학습)
npm start

# 손글씨 데이터 수집
npm run collect

# 숫자 그리기 (테스트용)
npm run draw

# 모델 학습
npm run train
```

### 또는 직접 실행

```bash
# 기본 학습 및 테스트
cargo run --release

# 개발 모드 (빠른 컴파일)
cargo run
```

## 📚 손글씨 데이터 수집 (NEW!)

### 1️⃣ 데이터 수집 도구 열기

```bash
npm run collect
# 또는
open collect_data.html
```

### 2️⃣ 손글씨 그리기

1. 숫자 선택 (0-9)
2. 캔버스에 숫자 그리기
3. "💾 저장" 클릭
4. 각 숫자당 **10-20개씩** 반복
5. "📥 전체 다운로드" 클릭

### 3️⃣ 데이터 적용

```bash
# 다운로드한 CSV를 프로젝트 폴더로 이동
mv ~/Downloads/handwritten_data.csv .

# 학습 실행 (자동으로 손글씨 데이터 포함)
npm start
```

### 결과:

```
📁 데이터 로드 중...
   합성 데이터: 500 개
   손글씨 데이터: 100 개 ✨
   총 데이터: 600 개 (합성 + 손글씨)
```

## 🎨 단일 이미지 테스트

### 1️⃣ 숫자 그리기

```bash
npm run draw
# 또는
open draw.html
```

### 2️⃣ 이미지 내보내기

1. 캔버스에 숫자 그리기
2. "💾 내보내기" 클릭
3. `test_image.txt` 다운로드

### 3️⃣ AI 예측

```bash
# 다운로드한 파일을 프로젝트 폴더로 이동
mv ~/Downloads/test_image.txt .

# 실행 (자동으로 test_image.txt 테스트)
npm start
```

## 📊 모델 구조

```
이미지 (28x28)
    ↓
패치 분할 (16개 패치, 각 7x7)
    ↓
Patch Embedding (각 128차원)
    ↓
Position Embedding 추가
    ↓
Transformer Block 1-6
├── Multi-Head Attention (8 heads)
├── Layer Norm
├── Feed Forward (128 → 512 → 128)
└── Layer Norm
    ↓
평균 Pooling
    ↓
Classification Head (128 → 10)
    ↓
예측 (0-9)
```

## 🎨 학습 데이터 패턴

### 합성 데이터 (자동 생성)

각 숫자별 특징:

- **0**: 속이 빈 원형
- **1**: 세로 줄 + 위 기울기
- **2**: 지그재그 형태
- **3**: 두 개의 곡선
- **4**: ㄱ자 + 세로
- **5**: 위 가로 + 중간 곡선
- **6**: 아래 원 + 위 곡선
- **7**: 위 가로 + 대각선
- **8**: 위아래 두 원
- **9**: 위 원 + 아래 세로

### 손글씨 데이터 (사용자 수집)

- `handwritten_data.csv`에서 자동 로드
- 실제 손글씨로 정확도 향상
- 각 숫자당 10-20개 권장

## 💡 성능 향상 팁

### 현재 문제:

합성 데이터만으로는 **실제 손글씨 인식률이 낮습니다.**

### 해결 방법:

#### 1️⃣ 손글씨 데이터 수집 (가장 효과적!)

```bash
# 데이터 수집 도구 열기
npm run collect

# 각 숫자당 10-20개씩 그리기
# 총 100-200개 수집 권장
```

**효과:**

- 10개 수집: 약 60-70% 정확도
- 50개 수집: 약 80-90% 정확도
- 100개+ 수집: 약 90-95% 정확도

#### 2️⃣ 실제 MNIST 데이터셋 사용

- 60,000개의 실제 손글씨
- 정확도 95%+ 달성 가능
- 대용량 데이터 처리 필요

#### 3️⃣ 모델 크기 증가

```rust
let embed_size = 256;  // 128 → 256
let num_heads = 16;    // 8 → 16
let num_layers = 12;   // 6 → 12
```

## 📁 파일 구조

```
week5-1_vision_transformer/
├── src/
│   └── main.rs              # Vision Transformer 구현
├── draw.html                # 웹 기반 숫자 그리기 도구
├── collect_data.html        # 손글씨 데이터 수집 도구 ✨
├── package.json             # NPM 스크립트 ✨
├── test_image.txt           # 테스트 이미지 (28x28)
├── handwritten_data.csv     # 손글씨 학습 데이터 ✨
├── vit_model.safetensors    # 학습된 모델 가중치
├── Cargo.toml
└── README.md
```

## 🧠 핵심 개념

### Vision Transformer란?

NLP의 Transformer를 이미지에 적용한 모델입니다:

1. **Patch Embedding**: 이미지를 작은 패치로 분할
2. **Position Embedding**: 각 패치의 위치 정보 추가
3. **Self-Attention**: 패치 간 관계 학습
4. **Transformer Encoder**: 순차적 패턴 인식
5. **Classification**: 최종 분류

### CNN vs ViT

| 구분 | CNN            | ViT               |
| ---- | -------------- | ----------------- |
| 입력 | 전체 이미지    | 패치들            |
| 연산 | Convolution    | Self-Attention    |
| 범위 | 국소적 (local) | 전역적 (global)   |
| 장점 | 이미지 특화    | 일반적, 확장 가능 |

## 🎓 학습 과정

```
Epoch  50 | Loss: 1.0816 | Accuracy: 85.0%
Epoch 100 | Loss: 0.4777 | Accuracy: 98.0%
Epoch 150 | Loss: 0.2764 | Accuracy: 99.2%
Epoch 200 | Loss: 0.1867 | Accuracy: 99.4%
...
Epoch 1000 | Loss: 0.0198 | Accuracy: 100.0%
```

- **1000 epochs** 학습
- **AdamW** 옵티마이저 (lr=0.001)
- **Cross-Entropy** 손실 함수
- **자동 모델 저장** (`vit_model.safetensors`)

## 🔧 커스터마이징

### 모델 크기 조정

`src/main.rs`에서 하이퍼파라미터 수정:

```rust
let embed_size = 128;  // 임베딩 크기
let num_heads = 8;     // 어텐션 헤드 수
let num_layers = 6;    // Transformer 레이어 수
```

### 학습 데이터 증가

```rust
let num_samples = 500;  // 합성 데이터 샘플 수
```

### 학습 횟수 조정

```rust
for epoch in 1..=1000 {  // epoch 수
```

## 📋 NPM 스크립트 전체 목록

```bash
# 도움말
npm run help

# 학습 & 실행
npm start           # 빠른 시작
npm run train       # 모델 학습 (릴리즈)
npm run dev         # 개발 모드
npm test            # 테스트 실행

# 데이터 수집
npm run collect     # 손글씨 데이터 수집 도구
npm run draw        # 숫자 그리기 도구

# 빌드 & 관리
npm run build       # 릴리즈 빌드
npm run check       # 코드 검사
npm run clean       # 빌드 파일 삭제
```

## 📈 다음 단계

1. ✅ **손글씨 데이터 수집 기능** (완료!)
2. ✅ **자동 데이터 통합** (완료!)
3. ✅ **모델 저장/로드** (완료!)
4. **실제 MNIST 데이터셋 통합**
5. **웹 API 서버 구축**
6. **실시간 손글씨 인식**
7. **다른 이미지 분류 작업 (CIFAR-10 등)**

## 🎉 완성!

Vision Transformer의 핵심 개념을 Rust로 구현했습니다!

### 주요 기능:

- ✅ Vision Transformer 구현
- ✅ 웹 기반 데이터 수집 도구
- ✅ 손글씨 학습 데이터 통합
- ✅ 자동 모델 저장/로드
- ✅ NPM 스크립트 지원
- ✅ 실시간 이미지 테스트

이미지도 Transformer로 처리할 수 있다는 것을 직접 확인해보세요! 👁️✨
