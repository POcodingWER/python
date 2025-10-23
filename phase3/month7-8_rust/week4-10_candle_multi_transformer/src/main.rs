// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🏗️ Multi-Layer Transformer: GPT 스타일 구조
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 참고: https://wikidocs.net/31380
// 📚 개념:
//   실제 GPT, BERT 같은 모델은 Transformer 블록을 여러 개 쌓아요!
//   각 층이 점점 더 복잡한 패턴을 학습합니다.
//
// 🏗️ 구조:
//
//   Input
//     ↓
//   ┌─────────────────────────┐
//   │ Transformer Block 1     │ ← 기본 패턴
//   │ - Attention             │
//   │ - Feed Forward          │
//   │ - Layer Norm            │
//   └─────────────────────────┘
//     ↓
//   ┌─────────────────────────┐
//   │ Transformer Block 2     │ ← 더 복잡한 관계
//   │ - Attention             │
//   │ - Feed Forward          │
//   │ - Layer Norm            │
//   └─────────────────────────┘
//     ↓
//   ┌─────────────────────────┐
//   │ Transformer Block 3     │ ← 깊은 이해
//   │ - Attention             │
//   │ - Feed Forward          │
//   │ - Layer Norm            │
//   └─────────────────────────┘
//     ↓
//   Output
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔑 왜 여러 층을 쌓나요?
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 1층: "1 다음은 2" (단순)
// 2층: "짝수 다음은 +2" (패턴)
// 3층: "수열의 규칙 이해" (추상화)
//
// → 층이 깊을수록 더 복잡한 관계를 학습!
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎯 실제 모델:
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// GPT-2:    12개 블록
// GPT-3:    96개 블록
// BERT:     12개 블록 (Base), 24개 블록 (Large)
//
// 우리는 3개 블록으로 시작!
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use candle_core::{DType, Device, IndexOp, Result, Tensor, D};
use candle_nn::{
    embedding, linear, loss, ops, AdamW, Embedding, LayerNorm, Linear, Module, Optimizer,
    ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("{}", "=".repeat(70));
    println!("🏗️ Multi-Layer Transformer: 감성 분석 (Sentiment Analysis)");
    println!("{}", "=".repeat(70));

    // 1. 단어 사전 구축
    println!("\n[1] 단어 사전 구축");
    let vocab = build_vocab();
    println!("  📚 총 {} 개 단어", vocab.len());

    // 2. 학습 데이터 생성
    println!("\n[2] 학습 데이터 생성");
    let training_data = vec![
        // 긍정 문장 (label = 1)
        (vec![3, 4, 5], 1),   // I love AI
        (vec![8, 9, 10], 1),  // Rust is amazing
        (vec![3, 11, 12], 1), // I like programming
        (vec![13, 9, 14], 1), // code is fun
        (vec![8, 9, 15], 1),  // Rust is great
        (vec![3, 4, 13], 1),  // I love code
        // 부정 문장 (label = 0)
        (vec![3, 6, 7], 0),   // I hate bugs
        (vec![16, 9, 17], 0), // this is bad
        (vec![3, 18, 19], 0), // I dislike errors
        (vec![7, 9, 20], 0),  // bugs are annoying
        (vec![16, 9, 21], 0), // this is terrible
        (vec![3, 6, 19], 0),  // I hate errors
    ];

    println!("  📊 {} 개 문장 (긍정 6개, 부정 6개)", training_data.len());
    println!("\n  예시:");
    println!("    긍정: \"I love AI\" → 😊");
    println!("    부정: \"I hate bugs\" → 😞");

    let (train_x, train_y) = prepare_sentiment_data(&training_data)?;

    // 3. Multi-Layer Transformer 모델 생성
    println!("\n[3] Multi-Layer Transformer 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let vocab_size = 25; // 단어 사전 크기
    let embed_size = 32; // 임베딩 차원
    let num_layers = 3; // Transformer 층 수
    let max_seq_len = 3; // 최대 문장 길이

    let model = SentimentTransformer::new(vb, vocab_size, embed_size, num_layers, max_seq_len)?;
    println!("  🧠 {}층 Transformer 생성 완료!", num_layers);
    println!("  📊 구조: Token Embedding → Position Encoding → 3층 Transformer → 분류");

    // 4. 학습
    println!("\n[4] 학습 시작!");
    println!("  💡 긍정/부정 감성 학습 중...");
    train_sentiment(&model, &varmap, &train_x, &train_y, 500)?;

    // 5. 테스트
    println!("\n[5] 감성 분석 테스트");

    test_sentiment(&model, &device, vec![3, 4, 5], "I love AI");
    test_sentiment(&model, &device, vec![3, 6, 7], "I hate bugs");
    test_sentiment(&model, &device, vec![8, 9, 10], "Rust is amazing");
    test_sentiment(&model, &device, vec![16, 9, 17], "this is bad");
    test_sentiment(&model, &device, vec![3, 11, 12], "I like programming");
    test_sentiment(&model, &device, vec![7, 9, 20], "bugs are annoying");

    println!("\n{}", "=".repeat(70));
    println!("🎉 Multi-Layer Transformer 감성 분석 완성!");
    println!("💡 {}층 구조로 문맥을 이해해요!", num_layers);
    println!("{}", "=".repeat(70));

    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📚 단어 사전
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn build_vocab() -> HashMap<String, usize> {
    let mut vocab = HashMap::new();
    vocab.insert("<PAD>".to_string(), 0);
    vocab.insert("<UNK>".to_string(), 1);
    vocab.insert("".to_string(), 2);
    vocab.insert("I".to_string(), 3);
    vocab.insert("love".to_string(), 4);
    vocab.insert("AI".to_string(), 5);
    vocab.insert("hate".to_string(), 6);
    vocab.insert("bugs".to_string(), 7);
    vocab.insert("Rust".to_string(), 8);
    vocab.insert("is".to_string(), 9);
    vocab.insert("amazing".to_string(), 10);
    vocab.insert("like".to_string(), 11);
    vocab.insert("programming".to_string(), 12);
    vocab.insert("code".to_string(), 13);
    vocab.insert("fun".to_string(), 14);
    vocab.insert("great".to_string(), 15);
    vocab.insert("this".to_string(), 16);
    vocab.insert("bad".to_string(), 17);
    vocab.insert("dislike".to_string(), 18);
    vocab.insert("errors".to_string(), 19);
    vocab.insert("annoying".to_string(), 20);
    vocab.insert("terrible".to_string(), 21);
    vocab
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📊 학습 데이터 준비
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn prepare_sentiment_data(data: &Vec<(Vec<usize>, usize)>) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut inputs: Vec<u32> = Vec::new();
    let mut labels: Vec<u32> = Vec::new();

    for (tokens, label) in data {
        inputs.extend(tokens.iter().map(|&t| t as u32));
        labels.push(*label as u32);
    }

    let num_samples = data.len();
    let seq_len = data[0].0.len();

    let x = Tensor::from_vec(inputs, &[num_samples, seq_len], &device)?;
    let y = Tensor::from_vec(labels, &[num_samples], &device)?;

    Ok((x, y))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧱 단일 Transformer 블록
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 이 블록이 여러 개 쌓여서 Multi-Layer가 됩니다!
//
struct TransformerBlock {
    // Self-Attention
    query: Linear,
    key: Linear,
    value: Linear,

    // Feed Forward
    ff1: Linear,
    ff2: Linear,

    // Layer Normalization
    norm1: LayerNorm,
    norm2: LayerNorm,

    hidden_size: usize,
}

impl TransformerBlock {
    fn new(vb: VarBuilder, hidden_size: usize, layer_idx: usize) -> Result<Self> {
        // 각 블록마다 고유한 이름 공간 사용!
        let layer_vb = vb.pp(format!("layer_{}", layer_idx));

        // Attention용 Q, K, V
        let query = linear(hidden_size, hidden_size, layer_vb.pp("query"))?;
        let key = linear(hidden_size, hidden_size, layer_vb.pp("key"))?;
        let value = linear(hidden_size, hidden_size, layer_vb.pp("value"))?;

        // Feed Forward
        let ff1 = linear(hidden_size, hidden_size * 2, layer_vb.pp("ff1"))?;
        let ff2 = linear(hidden_size * 2, hidden_size, layer_vb.pp("ff2"))?;

        // Layer Norm
        let norm1 = LayerNorm::new(
            Tensor::ones(&[hidden_size], DType::F32, layer_vb.device())?,
            Tensor::zeros(&[hidden_size], DType::F32, layer_vb.device())?,
            1e-5,
        );
        let norm2 = LayerNorm::new(
            Tensor::ones(&[hidden_size], DType::F32, layer_vb.device())?,
            Tensor::zeros(&[hidden_size], DType::F32, layer_vb.device())?,
            1e-5,
        );

        Ok(Self {
            query,
            key,
            value,
            ff1,
            ff2,
            norm1,
            norm2,
            hidden_size,
        })
    }

    /// 단일 블록의 forward
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (batch_size, seq_len, _) = x.dims3()?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 1️⃣ Self-Attention
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let q = self.query.forward(x)?;
        let k = self.key.forward(x)?;
        let v = self.value.forward(x)?;

        let k_t = k.transpose(D::Minus2, D::Minus1)?;
        let scores = q.matmul(&k_t)?;

        let d_k = self.hidden_size as f64;
        let scale = 1.0 / d_k.sqrt();
        let scaled_scores = (scores * scale)?;

        let attention_weights = ops::softmax(&scaled_scores, D::Minus1)?;
        let attention_output = attention_weights.matmul(&v)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 2️⃣ Add & Normalize (Residual Connection)
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let add1 = (attention_output + x)?; // ← Residual!
        let norm1_output = self.norm1.forward(&add1)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 3️⃣ Feed Forward
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let ff_output1 = self.ff1.forward(&norm1_output)?;
        let ff_output1_relu = ff_output1.relu()?;
        let ff_output2 = self.ff2.forward(&ff_output1_relu)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 4️⃣ Add & Normalize (Residual Connection)
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let add2 = (ff_output2 + norm1_output)?; // ← Residual!
        let norm2_output = self.norm2.forward(&add2)?;

        Ok(norm2_output)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🏗️ 감성 분석 Transformer
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// Token Embedding + Positional Encoding + Multi-Layer Transformer + 분류
//
struct SentimentTransformer {
    // Token Embedding: 단어 → 벡터
    token_embedding: Embedding,

    // Positional Encoding: 위치 정보
    position_embedding: Embedding,

    // 여러 Transformer 블록!
    blocks: Vec<TransformerBlock>,

    // 분류 레이어 (긍정/부정)
    classifier: Linear,

    embed_size: usize,
}

impl SentimentTransformer {
    fn new(
        vb: VarBuilder,
        vocab_size: usize,
        embed_size: usize,
        num_layers: usize,
        max_seq_len: usize,
    ) -> Result<Self> {
        // Token Embedding
        let token_embedding = embedding(vocab_size, embed_size, vb.pp("token_embed"))?;

        // Positional Encoding
        let position_embedding = embedding(max_seq_len, embed_size, vb.pp("pos_embed"))?;

        // Transformer 블록들
        let mut blocks = Vec::new();
        for i in 0..num_layers {
            let block = TransformerBlock::new(vb.clone(), embed_size, i)?;
            blocks.push(block);
        }

        // 분류 레이어 (2개 클래스: 긍정/부정)
        let classifier = linear(embed_size, 2, vb.pp("classifier"))?;

        Ok(Self {
            token_embedding,
            position_embedding,
            blocks,
            classifier,
            embed_size,
        })
    }

    /// Multi-Layer forward: 각 블록을 순차적으로 통과!
    fn forward(&self, input_tokens: &Tensor) -> Result<Tensor> {
        let (batch_size, seq_len) = input_tokens.dims2()?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 1️⃣ Token Embedding: 단어 인덱스 → 벡터
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let token_embed = self.token_embedding.forward(input_tokens)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 2️⃣ Positional Encoding: 위치 정보 추가
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let positions: Vec<u32> = (0..seq_len as u32).collect();
        let position_ids = Tensor::from_vec(positions, &[seq_len], input_tokens.device())?;
        let position_ids = position_ids
            .unsqueeze(0)?
            .broadcast_as((batch_size, seq_len))?;

        let pos_embed = self.position_embedding.forward(&position_ids)?;

        // Token + Position
        let mut current = (token_embed + pos_embed)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 3️⃣ Multi-Layer Transformer: 문맥 이해
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        //
        // Block 1: 단어 간 기본 관계
        // Block 2: 문법적 구조
        // Block 3: 감성 파악
        //

        for block in self.blocks.iter() {
            current = block.forward(&current)?;
        }

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 4️⃣ 평균 풀링: 문장 전체 표현
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let pooled = current.mean(1)?; // [batch, embed_size]

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 5️⃣ 분류: 긍정/부정
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let logits = self.classifier.forward(&pooled)?; // [batch, 2]

        Ok(logits)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎓 학습
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn train_sentiment(
    model: &SentimentTransformer,
    varmap: &VarMap,
    train_x: &Tensor,
    train_y: &Tensor,
    epochs: usize,
) -> Result<()> {
    let params = ParamsAdamW {
        lr: 0.001,
        ..Default::default()
    };

    let mut optimizer = AdamW::new(varmap.all_vars(), params)?;

    for epoch in 0..epochs {
        let logits = model.forward(train_x)?;
        let loss = loss::cross_entropy(&logits, train_y)?;

        optimizer.backward_step(&loss)?;

        if epoch % 100 == 0 || epoch == epochs - 1 {
            let loss_val = loss.to_scalar::<f32>()?;
            println!("  Epoch {:3}: Loss = {:.6}", epoch, loss_val);
        }
    }

    println!("  ✅ 학습 완료!");
    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧪 테스트
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn test_sentiment(
    model: &SentimentTransformer,
    device: &Device,
    tokens: Vec<usize>,
    text: &str,
) -> Result<()> {
    let input_tokens: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
    let input_tensor = Tensor::from_vec(input_tokens, &[1, tokens.len()], device)?;

    let logits = model.forward(&input_tensor)?;
    let probs = ops::softmax(&logits, 1)?;

    let probs_vec = probs.to_vec2::<f32>()?;
    let neg_prob = probs_vec[0][0];
    let pos_prob = probs_vec[0][1];

    let prediction = if pos_prob > neg_prob {
        "긍정 😊"
    } else {
        "부정 😞"
    };
    let confidence = pos_prob.max(neg_prob) * 100.0;

    println!("\n  📝 \"{}\"", text);
    println!("    예측: {} (확신도: {:.1}%)", prediction, confidence);
    println!(
        "    상세: 부정 {:.1}% | 긍정 {:.1}%",
        neg_prob * 100.0,
        pos_prob * 100.0
    );

    Ok(())
}
