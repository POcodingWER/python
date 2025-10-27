# Week 4-13: GPT-2 채팅봇 API 🤖💬

> **완료일**: 2025-10-27  
> **목표**: 실전 프로젝트 - 대화형 AI 챗봇 웹 서비스!  
> **성과**: REST API + 웹 UI + 전이 학습 + 자동 데이터 수집 완성! 🎉

---

## 🎯 프로젝트 개요

**GPT-2 기반 대화형 AI 챗봇**을 Rust로 구현하고, 웹 API로 서비스하는 실전 프로젝트!

### ✨ 주요 기능

1. **🤖 GPT-2 텍스트 생성**

   - 4-Layer Transformer
   - Causal Attention (미래 차단)
   - Temperature Sampling

2. **🌐 REST API 서버**

   - `POST /chat` - 대화 생성
   - `GET /health` - 서버 상태
   - `POST /train/add` - 학습 데이터 추가
   - `GET /train/data` - 학습 데이터 조회

3. **💬 웹 UI**

   - 실시간 채팅 인터페이스
   - 대화 히스토리 표시
   - 한글/영어 지원

4. **🧠 전이 학습 (Transfer Learning)**

   - 기존 모델 가중치 로드
   - 점진적 학습 (낮은 LR)
   - 자동 가중치 병합

5. **📊 자동 데이터 수집**

   - API 데이터 페칭 (quotable.io, icanhazdadjoke.com)
   - `training_data.txt` 자동 업데이트
   - 학습 히스토리 저장 (`training_history_*.txt`)

6. **🔤 동적 Tokenizer**
   - 자동 단어 사전 생성
   - 한글/영어 이중 언어
   - 특수 문자 필터링

---

## 🚀 실행 방법

### 1. 서버 시작

```bash
cd week4-13_chatbot_api
cargo run --release
```

서버가 `http://localhost:3000`에서 시작됩니다!

### 2. 웹 UI 접속

브라우저에서 `http://localhost:3000` 열기

### 3. API 사용

```bash
# 채팅
curl -X POST http://localhost:3000/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "안녕하세요"}'

# 서버 상태
curl http://localhost:3000/health

# 학습 데이터 추가
curl -X POST http://localhost:3000/train/add \
  -H "Content-Type: application/json" \
  -d '{"sentences": ["새로운 문장1", "새로운 문장2"]}'
```

---

## 📊 학습 데이터

### `training_data.txt`

- **650줄** 한글/영어 대화 데이터
- 인사, 감사, 사과, 일상 대화 등
- API 데이터 자동 추가 가능

### API 데이터 수집

```bash
# API 데이터 포함 학습
USE_API=1 cargo run --release
```

- **quotable.io**: 명언 데이터
- **icanhazdadjoke.com**: 농담 데이터

### 학습 히스토리

매 학습마다 `training_history_XXXXX.txt` 파일 생성:

- 사용된 학습 데이터 샘플 (70%)
- 타임스탬프
- 재현 가능성

---

## 🧠 모델 구조

### GPT-2 Architecture

```
Input Text
    ↓
Token Embedding (vocab_size → 128)
    ↓
Position Embedding (max_seq_len → 128)
    ↓
GPT2Block 0 (Self-Attention + FFN)
    ↓
GPT2Block 1
    ↓
GPT2Block 2
    ↓
GPT2Block 3
    ↓
LM Head (128 → vocab_size)
    ↓
Output Logits → Sample → Next Token
```

### 하이퍼파라미터

```rust
embed_size: 128
num_layers: 4
max_seq_len: 32
vocab_size: 동적 (training_data.txt 기반)

// 전이 학습
lr: 0.0001 (기존 모델), 0.001 (새 모델)
epochs: 50 (전이), 200 (새 모델)
sample_ratio: 0.7 (70% 샘플링)
```

---

## 🔧 기술 스택

### Rust Dependencies

```toml
[dependencies]
candle-core = "0.8.4"      # Tensor 연산
candle-nn = "0.8.4"        # Neural Network
actix-web = "4.4"          # 웹 서버
actix-cors = "0.7"         # CORS
serde = "1.0"              # JSON 직렬화
tokio = "1.35"             # 비동기 런타임
reqwest = "0.11"           # HTTP 클라이언트
rand = "0.8.5"             # 랜덤 샘플링
```

### 핵심 기능

1. **Actix-web**: 고성능 웹 서버
2. **Candle**: Rust AI 프레임워크
3. **Safetensors**: 모델 저장/로드
4. **Reqwest**: API 데이터 페칭
5. **Serde**: JSON 처리

---

## 📁 프로젝트 구조

```
week4-13_chatbot_api/
├── src/
│   └── main.rs                      # 메인 코드 (1,137줄)
├── training_data.txt                # 학습 데이터 (650줄)
├── training_history_*.txt           # 학습 히스토리
├── chatbot_model.safetensors        # 모델 가중치 (3.9MB)
├── Cargo.toml                       # 의존성
└── README.md                        # 이 파일
```

---

## 💡 핵심 구현

### 1. 동적 Tokenizer

```rust
fn build_vocab_from_file(path: &str) -> HashMap<String, usize> {
    let mut vocab = HashMap::new();
    // 특수 토큰
    vocab.insert("<PAD>".to_string(), 0);
    vocab.insert("<UNK>".to_string(), 1);
    vocab.insert("<EOS>".to_string(), 2);

    // training_data.txt에서 자동 추출
    for line in fs::read_to_string(path).unwrap().lines() {
        for word in line.split_whitespace() {
            let cleaned = word.chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace() || (*c >= '가' && *c <= '힣'))
                .collect::<String>();
            if !cleaned.is_empty() && !vocab.contains_key(&cleaned) {
                vocab.insert(cleaned, vocab.len());
            }
        }
    }
    vocab
}
```

### 2. 전이 학습

```rust
// 기존 모델 로드
let mut varmap = VarMap::new();
if Path::new("chatbot_model.safetensors").exists() {
    varmap.load("chatbot_model.safetensors")?;
    println!("✅ 기존 가중치 로드 완료! (전이 학습 모드)");
}

// 학습 후 저장
train_chatbot(&model, &varmap, &tokenizer, &device)?;
varmap.save("chatbot_model.safetensors")?;
```

### 3. API 데이터 수집

```rust
fn fetch_data_from_api() -> Vec<String> {
    let mut data = Vec::new();

    // quotable.io - 명언
    if let Ok(response) = reqwest::blocking::get("https://api.quotable.io/quotes/random?limit=10") {
        // ... JSON 파싱
    }

    // icanhazdadjoke.com - 농담
    for _ in 0..5 {
        if let Ok(response) = reqwest::blocking::get("https://icanhazdadjoke.com/") {
            // ... JSON 파싱
        }
    }

    data
}
```

### 4. 학습 히스토리 저장

```rust
fn save_training_history(conversations: &[String], original_count: usize) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let filename = format!("training_history_{}.txt", timestamp);
    let content = format!(
        "# 학습 히스토리\n# 원본: {}개, 샘플: {}개\n\n{}",
        original_count,
        conversations.len(),
        conversations.join("\n")
    );

    fs::write(&filename, content).ok();
}
```

---

## 🎯 학습 과정

### 1. 초기 학습 (처음 실행)

```
📚 학습 데이터 로드: 650개 문장
🔀 데이터 셔플링 완료
📊 샘플링: 455개 (70%)
💾 학습 히스토리 저장: training_history_1761548231.txt

⚠️  저장된 모델 없음 (처음부터 학습)
🎓 학습 시작 (처음 학습)
   Learning Rate: 0.001
   Epochs: 200

Epoch 1/200 | Loss: 5.234
Epoch 2/200 | Loss: 4.891
...
Epoch 200/200 | Loss: 0.123

✅ 모델 저장: chatbot_model.safetensors
```

### 2. 전이 학습 (두 번째 실행)

```
📥 기존 모델 로드 중: chatbot_model.safetensors
✅ 기존 가중치 로드 완료! (전이 학습 모드)

🎓 학습 시작 (전이 학습)
   Learning Rate: 0.0001 (낮은 LR)
   Epochs: 50 (적은 Epochs)

Epoch 1/50 | Loss: 0.098
Epoch 2/50 | Loss: 0.087
...
Epoch 50/50 | Loss: 0.065

✅ 모델 업데이트 완료!
```

---

## 📈 성능 지표

### 모델 크기

- **가중치 파일**: 3.9MB (`chatbot_model.safetensors`)
- **학습 데이터**: 16KB (`training_data.txt`)
- **바이너리**: ~15MB (release 빌드)

### 학습 시간

- **초기 학습**: ~2분 (200 epochs, 455 샘플)
- **전이 학습**: ~30초 (50 epochs, 455 샘플)
- **추론 속도**: ~100ms/응답

### 메모리 사용

- **학습 중**: ~200MB
- **추론 중**: ~50MB
- **서버 유휴**: ~30MB

---

## 🌟 주요 특징

### 1. 실시간 대화

```
👤 User: 안녕하세요
🤖 Bot: 안녕하세요! 무엇을 도와드릴까요?

👤 User: 오늘 날씨 어때?
🤖 Bot: 오늘 날씨는 맑고 화창합니다!
```

### 2. 대화 히스토리

```json
{
  "response": "안녕하세요! 무엇을 도와드릴까요?",
  "history": ["User: 안녕하세요", "Bot: 안녕하세요! 무엇을 도와드릴까요?"]
}
```

### 3. 동적 학습

```bash
# 새 데이터 추가
curl -X POST http://localhost:3000/train/add \
  -d '{"sentences": ["새로운 대화 패턴"]}'

# 서버 재시작 → 자동 전이 학습!
```

---

## 🚧 알려진 제약사항

### 1. 모델 크기

- **파라미터**: ~100K (GPT-2의 1/1000)
- **품질**: 짧은 응답에 최적화
- **해결**: 더 큰 모델로 확장 가능

### 2. 학습 데이터

- **현재**: 650문장
- **권장**: 10,000+ 문장
- **해결**: API 데이터 수집 활용

### 3. 응답 품질

- **현재**: 간단한 대화 가능
- **한계**: 복잡한 추론 부족
- **해결**: 더 많은 데이터 + 더 큰 모델

---

## 🎓 학습 내용

### 1. Rust 웹 개발

- ✅ Actix-web 프레임워크
- ✅ REST API 설계
- ✅ CORS 설정
- ✅ JSON 직렬화/역직렬화

### 2. AI 모델 서빙

- ✅ 모델 로드/저장
- ✅ 실시간 추론
- ✅ 전이 학습
- ✅ 메모리 관리

### 3. 데이터 파이프라인

- ✅ 파일 I/O
- ✅ API 데이터 페칭
- ✅ 자동 데이터 수집
- ✅ 학습 히스토리 관리

### 4. 프로덕션 고려사항

- ✅ 에러 처리
- ✅ 로깅
- ✅ 상태 관리 (Arc<Mutex>)
- ✅ 비동기 처리

---

## 🔮 향후 개선 방향

### 1. 모델 개선

- [ ] 더 큰 모델 (512 embed, 12 layers)
- [ ] Multi-Head Attention
- [ ] Beam Search
- [ ] Top-k/Top-p Sampling

### 2. 데이터 확장

- [ ] AI Hub 한국어 데이터셋
- [ ] 웹 크롤링
- [ ] 사용자 피드백 수집
- [ ] 데이터 증강

### 3. 기능 추가

- [ ] 대화 컨텍스트 유지
- [ ] 감정 분석
- [ ] 다국어 지원
- [ ] 음성 인터페이스

### 4. 인프라

- [ ] Docker 컨테이너화
- [ ] GPU 가속
- [ ] 로드 밸런싱
- [ ] 모니터링 대시보드

---

## 📚 참고 자료

- [Actix-web Documentation](https://actix.rs/)
- [Candle Framework](https://github.com/huggingface/candle)
- [GPT-2 Paper](https://d4mucfpksywv.cloudfront.net/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)

---

## 🏆 완료 체크리스트

- [x] GPT-2 모델 구현 ✅
- [x] REST API 서버 ✅
- [x] 웹 UI 구현 ✅
- [x] 한글/영어 지원 ✅
- [x] 동적 Tokenizer ✅
- [x] 전이 학습 ✅
- [x] API 데이터 수집 ✅
- [x] 학습 히스토리 저장 ✅
- [x] 모델 저장/로드 ✅
- [x] 실시간 채팅 테스트 ✅

---

**총평**: Week 4-13 GPT-2 채팅봇 API **대성공!** 🎉

Rust로 GPT-2 기반 대화형 AI 챗봇을 완전히 구현하고, REST API로 서비스하는 실전 프로젝트를 완성했습니다! 전이 학습, 자동 데이터 수집, 웹 UI까지 프로덕션 수준의 기능을 모두 갖췄습니다! 🚀

**다음 목표**: Month 8 - 고성능 ZK 시스템 또는 더 큰 LLM 프로젝트!
