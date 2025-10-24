// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🤖 Mini GPT: 텍스트 생성 AI
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 참고: GPT = Generative Pre-trained Transformer
//
// 📚 핵심 개념:
//   1. Auto-regressive: 한 단어씩 순차적으로 생성
//   2. Temperature: 창의성 조절 (낮음=안전, 높음=창의적)
//   3. Top-k Sampling: 상위 k개 단어 중에서 선택
//
// 🎯 목표:
//   입력: "I love"
//   출력: "I love AI and programming"
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use candle_core::{DType, Device, IndexOp, Result, Tensor, D};
use candle_nn::{
    embedding, linear, loss, ops, AdamW, Embedding, LayerNorm, Linear, Module, Optimizer,
    ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("{}", "=".repeat(70));
    println!("🤖 Mini GPT: 텍스트 생성 AI");
    println!("{}", "=".repeat(70));

    // 1. 텍스트 파일 읽기
    println!("\n[1] 텍스트 파일 읽기");
    let text = fs::read_to_string("training_data_small.txt")
        .expect("❌ training_data_small.txt 파일을 찾을 수 없습니다!");

    println!("  📦 파일 크기: {:.2} KB", text.len() as f64 / 1024.0);

    // 2. 단어 사전 구축 (텍스트에서 자동 생성)
    println!("\n[2] 단어 사전 자동 구축");
    let (vocab, idx_to_word) = build_vocab_from_text(&text);
    println!("  📚 총 {} 개 단어", vocab.len());

    // 3. 학습 데이터 생성 (텍스트를 토큰으로 변환)
    println!("\n[3] 학습 데이터 생성");
    let training_data = tokenize_text(&text, &vocab);
    println!("  📊 {} 개 시퀀스", training_data.len());
    println!("  📝 원본 텍스트: {} 줄", text.lines().count());

    // 첫 3개 문장 미리보기
    println!("\n  예시:");
    for (i, line) in text.lines().take(3).enumerate() {
        println!("    {}. \"{}\"", i + 1, line);
    }

    let (train_x, train_y) = prepare_gpt_data(&training_data)?;

    // 4. Mini GPT 모델 생성
    println!("\n[4] Mini GPT 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let vocab_size = vocab.len();
    let embed_size = 64; // 적당한 크기
    let num_layers = 2; // 2층으로 줄임
    let max_seq_len = 10; // 적당한 길이

    let model = MiniGPT::new(vb, vocab_size, embed_size, num_layers, max_seq_len)?;
    println!("  🧠 {}층 GPT 생성 완료!", num_layers);
    println!("  📊 구조: Token + Position → Transformer → 다음 단어 예측");

    // 5. 학습
    println!("\n[5] 학습 시작!");
    println!("  💡 다음 단어 예측 학습 중...");
    println!("  ⏰ 500줄 셰익스피어 작품으로 빠른 학습!");
    train_gpt(&model, &varmap, &train_x, &train_y, 200)?; // 빠르게!

    // 6. 텍스트 생성 테스트
    println!("\n[6] 텍스트 생성 테스트");

    // 동적으로 시작 단어 찾기 (셰익스피어 스타일)
    let test_phrases = vec![
        vec!["To", "be"],
        vec!["The", "world"],
        vec!["I", "am"],
        vec!["Love", "is"],
        vec!["What", "is"],
    ];

    for phrase in test_phrases {
        if let Some(tokens) = words_to_tokens(&phrase, &vocab) {
            println!("\n  🎯 시작: \"{}\"", phrase.join(" "));
            generate_text(&model, &device, tokens, &idx_to_word, 8, 0.8)?;
        }
    }

    // Temperature 비교
    println!("\n  🌡️  Temperature 비교:");
    println!("    (낮음 = 안전, 높음 = 창의적)");

    if let Some(start_tokens) = words_to_tokens(&vec!["To", "be"], &vocab) {
        println!("\n    Temperature = 0.3 (안전):");
        generate_text(&model, &device, start_tokens.clone(), &idx_to_word, 10, 0.3)?;

        println!("\n    Temperature = 1.0 (보통):");
        generate_text(&model, &device, start_tokens.clone(), &idx_to_word, 10, 1.0)?;

        println!("\n    Temperature = 1.5 (창의적):");
        generate_text(&model, &device, start_tokens, &idx_to_word, 10, 1.5)?;
    }

    println!("\n{}", "=".repeat(70));
    println!("🎉 Mini GPT 완성!");
    println!("💡 Auto-regressive 텍스트 생성 성공!");
    println!("{}", "=".repeat(70));

    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📚 단어 사전 (텍스트에서 자동 생성)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn build_vocab_from_text(text: &str) -> (HashMap<String, usize>, HashMap<usize, String>) {
    let mut vocab = HashMap::new();
    let mut idx = 0;

    // 특수 토큰
    vocab.insert("<PAD>".to_string(), idx);
    idx += 1;
    vocab.insert("<UNK>".to_string(), idx);
    idx += 1;
    vocab.insert("<EOS>".to_string(), idx);
    idx += 1;

    // 텍스트에서 모든 단어 추출
    for line in text.lines() {
        for word in line.split_whitespace() {
            if !vocab.contains_key(word) {
                vocab.insert(word.to_string(), idx);
                idx += 1;
            }
        }
    }

    let idx_to_word: HashMap<usize, String> = vocab.iter().map(|(k, v)| (*v, k.clone())).collect();

    (vocab, idx_to_word)
}

// 텍스트를 토큰 시퀀스로 변환
fn tokenize_text(text: &str, vocab: &HashMap<String, usize>) -> Vec<Vec<usize>> {
    let mut sequences = Vec::new();

    for line in text.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() < 2 {
            continue; // 너무 짧은 문장은 스킵
        }

        let tokens: Vec<usize> = words
            .iter()
            .map(|w| *vocab.get(*w).unwrap_or(&1)) // <UNK> = 1
            .collect();

        sequences.push(tokens);
    }

    sequences
}

// 단어 리스트를 토큰으로 변환
fn words_to_tokens(words: &Vec<&str>, vocab: &HashMap<String, usize>) -> Option<Vec<usize>> {
    let tokens: Vec<usize> = words
        .iter()
        .map(|w| *vocab.get(&w.to_string()).unwrap_or(&1))
        .collect();

    if tokens.iter().all(|&t| t == 1) {
        None // 모든 단어가 <UNK>이면 None
    } else {
        Some(tokens)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📊 학습 데이터 준비
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn prepare_gpt_data(data: &Vec<Vec<usize>>) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut inputs: Vec<u32> = Vec::new();
    let mut targets: Vec<u32> = Vec::new();

    let context_len = 5; // 5개 단어로 다음 단어 예측

    // GPT는 "다음 단어 예측"을 학습!
    // 입력: [I, love, AI, and, programming] → 출력: every
    for seq in data {
        if seq.len() < context_len + 1 {
            continue; // 너무 짧은 시퀀스는 스킵
        }

        for i in 0..=seq.len() - context_len - 1 {
            let context = &seq[i..i + context_len];
            let target = seq[i + context_len];

            inputs.extend(context.iter().map(|&x| x as u32));
            targets.push(target as u32);
        }
    }

    let num_samples = targets.len();

    let x = Tensor::from_vec(inputs, &[num_samples, context_len], &device)?;
    let y = Tensor::from_vec(targets, &[num_samples], &device)?;

    Ok((x, y))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧱 Transformer 블록 (Multi-Layer Transformer에서 재사용)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct TransformerBlock {
    query: Linear,
    key: Linear,
    value: Linear,
    ff1: Linear,
    ff2: Linear,
    norm1: LayerNorm,
    norm2: LayerNorm,
    hidden_size: usize,
}

impl TransformerBlock {
    fn new(vb: VarBuilder, hidden_size: usize, layer_idx: usize) -> Result<Self> {
        let layer_vb = vb.pp(format!("layer_{}", layer_idx));

        let query = linear(hidden_size, hidden_size, layer_vb.pp("query"))?;
        let key = linear(hidden_size, hidden_size, layer_vb.pp("key"))?;
        let value = linear(hidden_size, hidden_size, layer_vb.pp("value"))?;

        let ff1 = linear(hidden_size, hidden_size * 2, layer_vb.pp("ff1"))?;
        let ff2 = linear(hidden_size * 2, hidden_size, layer_vb.pp("ff2"))?;

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

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // Self-Attention
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

        // Add & Norm
        let add1 = (attention_output + x)?;
        let norm1_output = self.norm1.forward(&add1)?;

        // Feed Forward
        let ff_output1 = self.ff1.forward(&norm1_output)?;
        let ff_output1_relu = ff_output1.relu()?;
        let ff_output2 = self.ff2.forward(&ff_output1_relu)?;

        // Add & Norm
        let add2 = (ff_output2 + norm1_output)?;
        let norm2_output = self.norm2.forward(&add2)?;

        Ok(norm2_output)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🤖 Mini GPT 모델
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct MiniGPT {
    token_embedding: Embedding,
    position_embedding: Embedding,
    blocks: Vec<TransformerBlock>,
    output: Linear, // 다음 단어 예측!
    embed_size: usize,
}

impl MiniGPT {
    fn new(
        vb: VarBuilder,
        vocab_size: usize,
        embed_size: usize,
        num_layers: usize,
        max_seq_len: usize,
    ) -> Result<Self> {
        let token_embedding = embedding(vocab_size, embed_size, vb.pp("token_embed"))?;
        let position_embedding = embedding(max_seq_len, embed_size, vb.pp("pos_embed"))?;

        let mut blocks = Vec::new();
        for i in 0..num_layers {
            let block = TransformerBlock::new(vb.clone(), embed_size, i)?;
            blocks.push(block);
        }

        // 다음 단어 예측: embed_size → vocab_size
        let output = linear(embed_size, vocab_size, vb.pp("output"))?;

        Ok(Self {
            token_embedding,
            position_embedding,
            blocks,
            output,
            embed_size,
        })
    }

    fn forward(&self, input_tokens: &Tensor) -> Result<Tensor> {
        let (batch_size, seq_len) = input_tokens.dims2()?;

        // Token Embedding
        let token_embed = self.token_embedding.forward(input_tokens)?;

        // Positional Encoding
        let positions: Vec<u32> = (0..seq_len as u32).collect();
        let position_ids = Tensor::from_vec(positions, &[seq_len], input_tokens.device())?;
        let position_ids = position_ids
            .unsqueeze(0)?
            .broadcast_as((batch_size, seq_len))?;

        let pos_embed = self.position_embedding.forward(&position_ids)?;

        // Token + Position
        let mut current = (token_embed + pos_embed)?;

        // Multi-Layer Transformer
        for block in self.blocks.iter() {
            current = block.forward(&current)?;
        }

        // 마지막 토큰의 표현만 사용 (다음 단어 예측)
        let last_token = current.i((.., seq_len - 1, ..))?; // [batch, embed_size]

        // 다음 단어 예측
        let logits = self.output.forward(&last_token)?; // [batch, vocab_size]

        Ok(logits)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎓 학습
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn train_gpt(
    model: &MiniGPT,
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

        if epoch % 50 == 0 || epoch == epochs - 1 {
            let loss_val = loss.to_scalar::<f32>()?;
            println!("  Epoch {:3}: Loss = {:.6}", epoch, loss_val);
        }
    }

    println!("  ✅ 학습 완료!");
    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎨 텍스트 생성 (Auto-regressive + Temperature)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn generate_text(
    model: &MiniGPT,
    device: &Device,
    mut tokens: Vec<usize>,
    idx_to_word: &HashMap<usize, String>,
    max_new_tokens: usize,
    temperature: f32,
) -> Result<()> {
    let mut generated = tokens.clone();
    let context_len = 5; // prepare_gpt_data와 동일

    for _ in 0..max_new_tokens {
        // 마지막 context_len개 토큰만 사용
        let start_idx = if tokens.len() > context_len {
            tokens.len() - context_len
        } else {
            0
        };
        let context = &tokens[start_idx..];

        // 패딩 추가 (context_len보다 짧으면)
        let mut padded_context = vec![0u32; context_len]; // <PAD> = 0
        let copy_len = context.len().min(context_len);
        for i in 0..copy_len {
            padded_context[context_len - copy_len + i] = context[i] as u32;
        }

        let input_tensor = Tensor::from_vec(padded_context, &[1, context_len], device)?;

        // 다음 단어 예측
        let logits = model.forward(&input_tensor)?;

        // Temperature 적용
        let logits = (logits / temperature as f64)?;

        // Softmax로 확률 변환
        let probs = ops::softmax(&logits, 1)?;

        // 샘플링
        let next_token = sample_from_probs(&probs)?;

        // <PAD>, <UNK>, <EOS> 제외
        if next_token <= 2 {
            break;
        }

        tokens.push(next_token);
        generated.push(next_token);
    }

    // 결과 출력
    print!("    생성: \"");
    for (i, &token_idx) in generated.iter().enumerate() {
        if let Some(word) = idx_to_word.get(&token_idx) {
            if word != "<PAD>" && word != "<UNK>" && word != "<EOS>" {
                print!("{}", word);
                if i < generated.len() - 1 {
                    print!(" ");
                }
            }
        }
    }
    println!("\"");

    Ok(())
}

fn sample_from_probs(probs: &Tensor) -> Result<usize> {
    let probs_vec = probs.to_vec2::<f32>()?;
    let probs_slice = &probs_vec[0];

    let mut rng = rand::thread_rng();
    let random_value: f32 = rng.gen();

    let mut cumsum = 0.0;
    for (i, &prob) in probs_slice.iter().enumerate() {
        cumsum += prob;
        if random_value < cumsum {
            return Ok(i);
        }
    }

    Ok(probs_slice.len() - 1)
}
