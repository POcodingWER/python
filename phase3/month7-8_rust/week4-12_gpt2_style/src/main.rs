// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🤖 Week 4-12: 진짜 GPT-2 스타일 모델
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 목표: GPT-2의 핵심 특징 구현!
//
// GPT-2 핵심 특징:
// 1. Causal (Masked) Attention - 미래를 못 봄
// 2. Auto-regressive Generation - 한 단어씩 생성
// 3. Layer Normalization
// 4. Residual Connections
// 5. Large Scale (우리는 작게 시작)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{
    embedding, layer_norm, linear, loss, ops, AdamW, Embedding, LayerNorm, Linear, Module,
    Optimizer, ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;
use std::collections::HashMap;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧠 GPT-2 Transformer Block (Causal Attention!)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct GPT2Block {
    // Attention
    query: Linear,
    key: Linear,
    value: Linear,
    proj: Linear, // Attention output projection

    // Feed Forward
    ff1: Linear,
    ff2: Linear,

    // Layer Normalization
    ln1: LayerNorm,
    ln2: LayerNorm,

    hidden_size: usize,
}

impl GPT2Block {
    fn new(vb: VarBuilder, hidden_size: usize) -> Result<Self> {
        let query = linear(hidden_size, hidden_size, vb.pp("query"))?;
        let key = linear(hidden_size, hidden_size, vb.pp("key"))?;
        let value = linear(hidden_size, hidden_size, vb.pp("value"))?;
        let proj = linear(hidden_size, hidden_size, vb.pp("proj"))?;

        let ff1 = linear(hidden_size, hidden_size * 4, vb.pp("ff1"))?;
        let ff2 = linear(hidden_size * 4, hidden_size, vb.pp("ff2"))?;

        let ln1 = layer_norm(hidden_size, 1e-5, vb.pp("ln1"))?;
        let ln2 = layer_norm(hidden_size, 1e-5, vb.pp("ln2"))?;

        Ok(Self {
            query,
            key,
            value,
            proj,
            ff1,
            ff2,
            ln1,
            ln2,
            hidden_size,
        })
    }

    fn forward(&self, x: &Tensor, mask: &Tensor) -> Result<Tensor> {
        // Pre-LayerNorm (GPT-2 스타일)
        let x_norm = self.ln1.forward(x)?;

        // Causal Self-Attention
        let q = self.query.forward(&x_norm)?;
        let k = self.key.forward(&x_norm)?;
        let v = self.value.forward(&x_norm)?;

        let scale = (self.hidden_size as f64).sqrt();
        let k_t = k.transpose(1, 2)?;
        let mut scores = q.matmul(&k_t)?.affine(1.0 / scale, 0.0)?;

        // Causal Mask 적용 (미래를 못 보게!)
        // scores: (batch, seq_len, seq_len), mask: (seq_len, seq_len)
        let mask_expanded = mask.broadcast_as(scores.shape())?;
        scores = scores.broadcast_add(&mask_expanded)?;

        let attention_weights = ops::softmax(&scores, 2)?;
        let attention_output = attention_weights.matmul(&v)?;
        let attention_output = self.proj.forward(&attention_output)?;

        // Residual Connection
        let x = (x + attention_output)?;

        // Pre-LayerNorm
        let x_norm = self.ln2.forward(&x)?;

        // Feed Forward with GELU (GPT-2 uses GELU, not ReLU!)
        let ff_output = self.ff1.forward(&x_norm)?.gelu()?.apply(&self.ff2)?;

        // Residual Connection
        let output = (x + ff_output)?;

        Ok(output)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎯 GPT-2 모델
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct GPT2 {
    token_embedding: Embedding,
    position_embedding: Embedding,
    blocks: Vec<GPT2Block>,
    ln_f: LayerNorm, // Final layer norm
    lm_head: Linear, // Language model head
    max_seq_len: usize,
}

impl GPT2 {
    fn new(
        vb: VarBuilder,
        vocab_size: usize,
        embed_size: usize,
        num_layers: usize,
        max_seq_len: usize,
    ) -> Result<Self> {
        let token_embedding = embedding(vocab_size, embed_size, vb.pp("token_emb"))?;
        let position_embedding = embedding(max_seq_len, embed_size, vb.pp("pos_emb"))?;

        let mut blocks = Vec::new();
        for i in 0..num_layers {
            let block_vb = vb.pp(format!("layer_{}", i));
            blocks.push(GPT2Block::new(block_vb, embed_size)?);
        }

        let ln_f = layer_norm(embed_size, 1e-5, vb.pp("ln_f"))?;
        let lm_head = linear(embed_size, vocab_size, vb.pp("lm_head"))?;

        Ok(Self {
            token_embedding,
            position_embedding,
            blocks,
            ln_f,
            lm_head,
            max_seq_len,
        })
    }

    fn forward(&self, tokens: &Tensor, positions: &Tensor) -> Result<Tensor> {
        let seq_len = tokens.dim(1)?;

        // Causal Mask 생성 (미래를 못 보게!)
        let mask = self.create_causal_mask(seq_len, tokens.device())?;

        // Token + Position Embedding
        let token_emb = self.token_embedding.forward(tokens)?;
        let pos_emb = self.position_embedding.forward(positions)?;
        let mut x = (token_emb + pos_emb)?;

        // GPT-2 Blocks
        for block in self.blocks.iter() {
            x = block.forward(&x, &mask)?;
        }

        // Final Layer Norm
        let x = self.ln_f.forward(&x)?;

        // Language Model Head
        let logits = self.lm_head.forward(&x)?;

        Ok(logits)
    }

    fn create_causal_mask(&self, seq_len: usize, device: &Device) -> Result<Tensor> {
        // 상삼각 행렬 생성 (i < j인 경우 -inf)
        let mut mask_data = vec![0.0f32; seq_len * seq_len];
        for i in 0..seq_len {
            for j in 0..seq_len {
                if j > i {
                    mask_data[i * seq_len + j] = f32::NEG_INFINITY;
                }
            }
        }

        let mask = Tensor::from_vec(mask_data, (seq_len, seq_len), device)?;

        Ok(mask)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔤 토크나이저
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct SimpleTokenizer {
    vocab: HashMap<String, usize>,
    idx_to_word: HashMap<usize, String>,
    max_seq_len: usize,
}

impl SimpleTokenizer {
    fn new(text: &str, max_seq_len: usize) -> Self {
        let mut vocab = HashMap::new();
        vocab.insert("<PAD>".to_string(), 0);
        vocab.insert("<UNK>".to_string(), 1);
        vocab.insert("<EOS>".to_string(), 2);

        let words: Vec<&str> = text.split_whitespace().collect();
        let unique_words: std::collections::HashSet<_> = words.iter().cloned().collect();

        for (idx, word) in unique_words.iter().enumerate() {
            vocab.insert(word.to_string(), idx + 3);
        }

        let idx_to_word: HashMap<usize, String> =
            vocab.iter().map(|(k, v)| (*v, k.clone())).collect();

        Self {
            vocab,
            idx_to_word,
            max_seq_len,
        }
    }

    fn tokenize(&self, text: &str) -> Vec<u32> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut tokens = Vec::new();

        for word in words.iter().take(self.max_seq_len - 1) {
            let token = *self.vocab.get(*word).unwrap_or(&1);
            tokens.push(token as u32);
        }

        tokens.push(2); // <EOS>

        // Padding
        while tokens.len() < self.max_seq_len {
            tokens.push(0);
        }

        tokens
    }

    fn vocab_size(&self) -> usize {
        self.vocab.len()
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎓 학습
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn train_gpt2(
    model: &GPT2,
    varmap: &VarMap,
    tokenizer: &SimpleTokenizer,
    train_texts: &[&str],
    device: &Device,
) -> Result<()> {
    println!("\n[1] 🎓 GPT-2 학습 시작!");

    let params = ParamsAdamW {
        lr: 0.0003,
        ..Default::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), params)?;

    let epochs = 500;

    for epoch in 1..=epochs {
        let mut total_loss = 0.0;

        for text in train_texts.iter() {
            let tokens = tokenizer.tokenize(text);
            let positions: Vec<u32> = (0..tokenizer.max_seq_len).map(|x| x as u32).collect();

            let tokens_t = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
            let positions_t = Tensor::new(&positions[..], device)?.unsqueeze(0)?;

            // Forward
            let logits = model.forward(&tokens_t, &positions_t)?;

            // Auto-regressive Loss (다음 단어 예측)
            let logits_flat = logits
                .i((.., 0..tokens.len() - 1, ..))?
                .reshape((tokens.len() - 1, tokenizer.vocab_size()))?;
            let targets = Tensor::new(&tokens[1..], device)?;

            let loss = loss::cross_entropy(&logits_flat, &targets)?;

            // Backward
            optimizer.backward_step(&loss)?;

            total_loss += loss.to_scalar::<f32>()?;
        }

        if epoch % 100 == 0 || epoch == 1 {
            let avg_loss = total_loss / train_texts.len() as f32;
            println!("  Epoch {:3}: Loss = {:.4}", epoch, avg_loss);
        }
    }

    println!("  ✅ 학습 완료!");
    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔮 텍스트 생성 (Auto-regressive!)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn generate_text(
    model: &GPT2,
    tokenizer: &SimpleTokenizer,
    prompt: &str,
    max_new_tokens: usize,
    temperature: f32,
    device: &Device,
) -> Result<String> {
    // Prompt를 토큰화 (padding 제거)
    let words: Vec<&str> = prompt.split_whitespace().collect();
    let mut tokens = Vec::new();

    for word in words.iter() {
        let token = *tokenizer.vocab.get(*word).unwrap_or(&1);
        tokens.push(token as u32);
    }

    let mut rng = rand::thread_rng();

    for _ in 0..max_new_tokens {
        // 현재 토큰들을 padding
        let mut padded_tokens = tokens.clone();
        while padded_tokens.len() < tokenizer.max_seq_len {
            padded_tokens.push(0);
        }

        let positions: Vec<u32> = (0..tokenizer.max_seq_len).map(|x| x as u32).collect();

        let tokens_t = Tensor::new(&padded_tokens[..], device)?.unsqueeze(0)?;
        let positions_t = Tensor::new(&positions[..], device)?.unsqueeze(0)?;

        let logits = model.forward(&tokens_t, &positions_t)?;

        // 마지막 유효 토큰의 logits
        let last_logits = logits.i((0, tokens.len() - 1, ..))?;

        // Temperature sampling
        let probs = if temperature > 0.0 {
            let scaled_logits = (last_logits / temperature as f64)?;
            ops::softmax(&scaled_logits, 0)?
        } else {
            ops::softmax(&last_logits, 0)?
        };

        let probs_vec = probs.to_vec1::<f32>()?;

        // Sample from distribution
        let next_token = sample_from_probs(&probs_vec, &mut rng);

        // <EOS> 또는 <PAD>면 종료
        if next_token == 2 || next_token == 0 {
            break;
        }

        tokens.push(next_token as u32);

        if tokens.len() >= tokenizer.max_seq_len {
            break;
        }
    }

    // Tokens를 텍스트로 변환
    let mut result = Vec::new();
    for &token in tokens.iter() {
        if token == 0 || token == 2 {
            break;
        }
        if let Some(word) = tokenizer.idx_to_word.get(&(token as usize)) {
            result.push(word.clone());
        }
    }

    Ok(result.join(" "))
}

fn sample_from_probs(probs: &[f32], rng: &mut rand::rngs::ThreadRng) -> usize {
    let total: f32 = probs.iter().sum();
    let mut rand_val = rng.gen::<f32>() * total;

    for (idx, &prob) in probs.iter().enumerate() {
        rand_val -= prob;
        if rand_val <= 0.0 {
            return idx;
        }
    }

    probs.len() - 1
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🚀 메인
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn main() -> Result<()> {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🤖 진짜 GPT-2 스타일 모델");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let device = Device::Cpu;

    // 학습 데이터
    let training_text = "
        the quick brown fox jumps over the lazy dog
        a journey of a thousand miles begins with a single step
        to be or not to be that is the question
        all that glitters is not gold
        actions speak louder than words
        where there is a will there is a way
        the early bird catches the worm
        practice makes perfect
        time flies like an arrow
        knowledge is power
    ";

    let train_texts = vec![
        "the quick brown fox jumps",
        "a journey of a thousand miles",
        "to be or not to be",
        "all that glitters is not gold",
        "actions speak louder than words",
        "where there is a will",
        "the early bird catches the worm",
        "practice makes perfect",
        "time flies like an arrow",
        "knowledge is power",
    ];

    // 토크나이저
    let tokenizer = SimpleTokenizer::new(training_text, 20);
    println!("  📚 Vocab 크기: {}", tokenizer.vocab_size());

    // 모델 생성
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = GPT2::new(
        vb.clone(),
        tokenizer.vocab_size(),
        128, // embed_size (GPT-2 small: 768)
        4,   // num_layers (GPT-2 small: 12)
        20,  // max_seq_len
    )?;

    println!("  🧠 모델 구조:");
    println!("     - Embed Size: 128");
    println!("     - Layers: 4");
    println!("     - Max Seq Len: 20");
    println!("     - Causal Attention: ✅");
    println!("     - Layer Normalization: ✅");
    println!("     - GELU Activation: ✅");

    // 학습
    train_gpt2(&model, &varmap, &tokenizer, &train_texts, &device)?;

    // 텍스트 생성
    println!("\n[2] 🎨 텍스트 생성!");

    let prompts = vec!["the quick", "knowledge is", "time flies"];

    for prompt in prompts {
        println!("\n  입력: \"{}\"", prompt);

        // Temperature 0.8 (창의적)
        let generated = generate_text(&model, &tokenizer, prompt, 10, 0.8, &device)?;
        println!("  생성 (T=0.8): \"{}\"", generated);

        // Temperature 0.3 (보수적)
        let generated = generate_text(&model, &tokenizer, prompt, 10, 0.3, &device)?;
        println!("  생성 (T=0.3): \"{}\"", generated);
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ 완료!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Ok(())
}
