// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🤖 Week 4-13: GPT-2 채팅봇 API
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 목표: 실전 프로젝트 - 대화형 AI 챗봇!
//
// 기능:
// 1. GPT-2 기반 텍스트 생성
// 2. 대화 히스토리 관리
// 3. REST API (Actix-web)
// 4. 웹 UI (채팅 인터페이스)
// 5. Temperature 조절
// 6. 한글/영어 지원
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{
    embedding, layer_norm, linear, loss, ops, AdamW, Embedding, LayerNorm, Linear, Module,
    Optimizer, ParamsAdamW, VarBuilder, VarMap,
};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧠 GPT-2 Block
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct GPT2Block {
    query: Linear,
    key: Linear,
    value: Linear,
    proj: Linear,
    ff1: Linear,
    ff2: Linear,
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
        let x_norm = self.ln1.forward(x)?;
        
        let q = self.query.forward(&x_norm)?;
        let k = self.key.forward(&x_norm)?;
        let v = self.value.forward(&x_norm)?;
        
        let scale = (self.hidden_size as f64).sqrt();
        let k_t = k.transpose(1, 2)?;
        let mut scores = q.matmul(&k_t)?.affine(1.0 / scale, 0.0)?;
        
        let mask_expanded = mask.broadcast_as(scores.shape())?;
        scores = scores.broadcast_add(&mask_expanded)?;
        
        let attention_weights = ops::softmax(&scores, 2)?;
        let attention_output = attention_weights.matmul(&v)?;
        let attention_output = self.proj.forward(&attention_output)?;
        
        let x = (x + attention_output)?;
        let x_norm = self.ln2.forward(&x)?;
        let ff_output = self.ff1.forward(&x_norm)?.gelu()?.apply(&self.ff2)?;
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
    ln_f: LayerNorm,
    lm_head: Linear,
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
        let mask = self.create_causal_mask(seq_len, tokens.device())?;
        
        let token_emb = self.token_embedding.forward(tokens)?;
        let pos_emb = self.position_embedding.forward(positions)?;
        let mut x = (token_emb + pos_emb)?;
        
        for block in self.blocks.iter() {
            x = block.forward(&x, &mask)?;
        }
        
        let x = self.ln_f.forward(&x)?;
        let logits = self.lm_head.forward(&x)?;
        
        Ok(logits)
    }
    
    fn create_causal_mask(&self, seq_len: usize, device: &Device) -> Result<Tensor> {
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
    fn new(max_seq_len: usize) -> Self {
        let mut vocab = HashMap::new();
        let mut idx = 0;
        
        // 특수 토큰
        vocab.insert("<PAD>".to_string(), idx); idx += 1;
        vocab.insert("<UNK>".to_string(), idx); idx += 1;
        vocab.insert("<EOS>".to_string(), idx); idx += 1;
        
        // 학습 데이터에서 단어 추출
        if let Ok(content) = std::fs::read_to_string("training_data.txt") {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                // 구두점 제거
                let cleaned: String = line
                    .chars()
                    .filter(|c| {
                        c.is_alphanumeric() || 
                        c.is_whitespace() || 
                        (*c >= '가' && *c <= '힣') ||
                        (*c >= 'ㄱ' && *c <= 'ㅎ') ||
                        (*c >= 'ㅏ' && *c <= 'ㅣ')
                    })
                    .collect();
                
                for word in cleaned.split_whitespace() {
                    if !vocab.contains_key(word) {
                        vocab.insert(word.to_string(), idx);
                        idx += 1;
                    }
                }
            }
        }
        
        let idx_to_word: HashMap<usize, String> = vocab
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        
        Self {
            vocab,
            idx_to_word,
            max_seq_len,
        }
    }
    
    fn tokenize(&self, text: &str) -> Vec<u32> {
        // 구두점과 이모지 제거
        let cleaned: String = text
            .chars()
            .filter(|c| {
                c.is_alphanumeric() || 
                c.is_whitespace() || 
                (*c >= '가' && *c <= '힣') ||  // 한글
                (*c >= 'ㄱ' && *c <= 'ㅎ') ||  // 한글 자음
                (*c >= 'ㅏ' && *c <= 'ㅣ')     // 한글 모음
            })
            .collect();
        
        let words: Vec<&str> = cleaned.split_whitespace().collect();
        let mut tokens = Vec::new();
        
        for word in words.iter() {
            let token = *self.vocab.get(*word).unwrap_or(&1);
            tokens.push(token as u32);
        }
        
        tokens
    }
    
    fn decode(&self, tokens: &[u32]) -> String {
        let mut result = Vec::new();
        for &token in tokens.iter() {
            if token == 0 || token == 2 {
                break;
            }
            if let Some(word) = self.idx_to_word.get(&(token as usize)) {
                result.push(word.clone());
            }
        }
        result.join(" ")
    }
    
    fn vocab_size(&self) -> usize {
        self.vocab.len()
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎓 학습
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// 🌐 외부 API에서 데이터 가져오기
#[derive(Deserialize, Debug)]
struct QuoteResponse {
    content: String,
    author: String,
}

#[derive(Deserialize, Debug)]
struct JokeResponse {
    joke: String,
}

fn fetch_data_from_api() -> Vec<String> {
    let mut data = Vec::new();
    
    println!("  🌐 외부 API에서 데이터 수집 중...");
    
    // 1. 명언 API (quotable.io) - 10개
    println!("     - 명언 API 호출...");
    for _ in 0..10 {
        if let Ok(response) = reqwest::blocking::get("https://api.quotable.io/random") {
            if let Ok(quote) = response.json::<QuoteResponse>() {
                data.push(format!("{}", quote.content));
                data.push(format!("that is a great quote from {}", quote.author));
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    // 2. 농담 API (icanhazdadjoke.com) - 10개
    println!("     - 농담 API 호출...");
    let client = reqwest::blocking::Client::new();
    for _ in 0..10 {
        if let Ok(response) = client
            .get("https://icanhazdadjoke.com/")
            .header("Accept", "application/json")
            .send()
        {
            if let Ok(joke) = response.json::<JokeResponse>() {
                data.push(joke.joke.clone());
                data.push("that is funny".to_string());
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    println!("  ✅ API에서 {}개 문장 수집 완료!", data.len());
    data
}

fn load_training_data() -> Vec<String> {
    let data_path = "training_data.txt";
    let mut all_data = Vec::new();
    
    // 1. 파일에서 데이터 로드
    if Path::new(data_path).exists() {
        println!("  📂 파일에서 학습 데이터 로드: {}", data_path);
        let content = fs::read_to_string(data_path).unwrap_or_default();
        let file_data: Vec<String> = content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect();
        println!("     - 파일 데이터: {}개", file_data.len());
        all_data.extend(file_data);
    }
    
    // 2. API에서 데이터 추가 (선택적)
    let use_api = std::env::var("USE_API").unwrap_or_default() == "1";
    if use_api {
        let api_data = fetch_data_from_api();
        
        // API 데이터를 training_data.txt에 자동 추가
        if !api_data.is_empty() {
            let mut file_content = fs::read_to_string(data_path).unwrap_or_default();
            if !file_content.ends_with('\n') {
                file_content.push('\n');
            }
            file_content.push_str(&format!("\n# ========== API 데이터 (자동 추가) ==========\n"));
            for sentence in &api_data {
                file_content.push_str(&format!("{}\n", sentence));
            }
            let _ = fs::write(data_path, file_content);
            println!("  ✅ API 데이터 {}개를 training_data.txt에 추가했어요!", api_data.len());
        }
        
        all_data.extend(api_data);
    }
    
    if all_data.is_empty() {
        println!("  ⚠️  데이터 없음, 기본 데이터 사용");
        vec![
            "hello how are you".to_string(),
            "hi i am fine thanks".to_string(),
            "안녕하세요".to_string(),
            "안녕하세요 저는 챗봇이에요".to_string(),
        ]
    } else {
        all_data
    }
}

// 💾 학습에 사용된 데이터를 히스토리 파일로 저장
fn save_training_history(conversations: &[String], original_count: usize) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let history_file = format!("training_history_{}.txt", timestamp);
    
    let mut content = String::new();
    content.push_str(&format!("# 학습 히스토리 - {}\n", timestamp));
    content.push_str(&format!("# 전체 데이터: {}개\n", original_count));
    content.push_str(&format!("# 학습에 사용된 데이터: {}개 ({}%)\n\n", 
                              conversations.len(), 
                              (conversations.len() as f64 / original_count as f64 * 100.0) as usize));
    
    for (i, sentence) in conversations.iter().enumerate() {
        content.push_str(&format!("{:04}. {}\n", i + 1, sentence));
    }
    
    if let Ok(_) = fs::write(&history_file, content) {
        println!("  💾 학습 데이터 저장: {}", history_file);
    }
}

fn train_chatbot(
    model: &GPT2,
    varmap: &VarMap,
    tokenizer: &SimpleTokenizer,
    device: &Device,
) -> Result<()> {
    println!("  🎓 챗봇 학습 시작...");
    
    // 파일에서 대화 데이터 로드
    let mut conversations = load_training_data();
    let original_count = conversations.len();
    
    // 🎲 데이터 랜덤 섞기! (매번 다른 순서로 학습)
    let mut rng = rand::thread_rng();
    conversations.shuffle(&mut rng);
    
    // 📊 데이터 샘플링 (선택적: 전체 데이터 중 일부만 사용)
    // 예: 70% 데이터만 사용 (매번 다른 데이터 조합)
    let sample_ratio = 0.7; // 70% 샘플링
    let sample_size = (conversations.len() as f64 * sample_ratio) as usize;
    conversations.truncate(sample_size);
    
    println!("  📊 학습 데이터: {}개 문장 (랜덤 샘플링 {}%)", conversations.len(), (sample_ratio * 100.0) as usize);
    
    // 💾 학습에 사용된 데이터 저장
    save_training_history(&conversations, original_count);
    
    // 🔄 전이 학습 모드: 더 작은 learning rate, 더 적은 epochs
    let model_exists = Path::new("chatbot_model.safetensors").exists();
    let (lr, epochs) = if model_exists {
        (0.0001, 50)  // 전이 학습: 작은 lr, 적은 epochs
    } else {
        (0.001, 200)  // 처음 학습: 큰 lr, 많은 epochs
    };
    
    println!("  ⚙️  학습 설정: LR={}, Epochs={} ({})", 
             lr, epochs, 
             if model_exists { "전이 학습" } else { "처음 학습" });
    
    let params = ParamsAdamW {
        lr,
        ..Default::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), params)?;
    
    for epoch in 1..=epochs {
        let mut total_loss = 0.0;
        
        for text in conversations.iter() {
            let tokens = tokenizer.tokenize(text);
            if tokens.is_empty() {
                continue;
            }
            
            let mut padded_tokens = tokens.clone();
            while padded_tokens.len() < tokenizer.max_seq_len {
                padded_tokens.push(0);
            }
            
            let positions: Vec<u32> = (0..tokenizer.max_seq_len).map(|x| x as u32).collect();
            
            let tokens_t = Tensor::new(&padded_tokens[..], device)?.unsqueeze(0)?;
            let positions_t = Tensor::new(&positions[..], device)?.unsqueeze(0)?;
            
            let logits = model.forward(&tokens_t, &positions_t)?;
            
            if tokens.len() > 1 {
                let logits_flat = logits.i((.., 0..tokens.len() - 1, ..))?
                    .reshape((tokens.len() - 1, tokenizer.vocab_size()))?;
                let targets = Tensor::new(&tokens[1..], device)?;
                
                let loss = loss::cross_entropy(&logits_flat, &targets)?;
                optimizer.backward_step(&loss)?;
                total_loss += loss.to_scalar::<f32>()?;
            }
        }
        
        if epoch % 50 == 0 || epoch == 1 {
            let avg_loss = total_loss / conversations.len() as f32;
            println!("    Epoch {:3}: Loss = {:.4}", epoch, avg_loss);
        }
    }
    
    println!("  ✅ 학습 완료!");
    
    // 모델 저장
    println!("  💾 모델 저장 중...");
    varmap.save("chatbot_model.safetensors")?;
    println!("  ✅ 모델 저장 완료: chatbot_model.safetensors");
    
    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔮 텍스트 생성
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn generate_response(
    model: &GPT2,
    tokenizer: &SimpleTokenizer,
    prompt: &str,
    max_new_tokens: usize,
    temperature: f32,
    device: &Device,
) -> Result<String> {
    let mut tokens = tokenizer.tokenize(prompt);
    let mut rng = rand::thread_rng();
    
    for _ in 0..max_new_tokens {
        let mut padded_tokens = tokens.clone();
        while padded_tokens.len() < tokenizer.max_seq_len {
            padded_tokens.push(0);
        }
        
        let positions: Vec<u32> = (0..tokenizer.max_seq_len).map(|x| x as u32).collect();
        
        let tokens_t = Tensor::new(&padded_tokens[..], device)?.unsqueeze(0)?;
        let positions_t = Tensor::new(&positions[..], device)?.unsqueeze(0)?;
        
        let logits = model.forward(&tokens_t, &positions_t)?;
        let last_logits = logits.i((0, tokens.len() - 1, ..))?;
        
        let probs = if temperature > 0.0 {
            let scaled_logits = (last_logits / temperature as f64)?;
            ops::softmax(&scaled_logits, 0)?
        } else {
            ops::softmax(&last_logits, 0)?
        };
        
        let probs_vec = probs.to_vec1::<f32>()?;
        let next_token = sample_from_probs(&probs_vec, &mut rng);
        
        if next_token == 2 || next_token == 0 {
            break;
        }
        
        tokens.push(next_token as u32);
        
        if tokens.len() >= tokenizer.max_seq_len {
            break;
        }
    }
    
    Ok(tokenizer.decode(&tokens))
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
// 🌐 API 구조체
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct AppState {
    model: GPT2,
    tokenizer: SimpleTokenizer,
    device: Device,
    history: Mutex<Vec<String>>,
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    #[serde(default = "default_temperature")]
    temperature: f32,
    #[serde(default = "default_max_tokens")]
    max_tokens: usize,
}

fn default_temperature() -> f32 { 0.8 }
fn default_max_tokens() -> usize { 15 }

#[derive(Serialize)]
struct ChatResponse {
    response: String,
    history: Vec<String>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    model: String,
    vocab_size: usize,
}

#[derive(Deserialize)]
struct AddDataRequest {
    sentences: Vec<String>,
}

#[derive(Serialize)]
struct AddDataResponse {
    success: bool,
    added_count: usize,
    total_count: usize,
    message: String,
}

#[derive(Serialize)]
struct TrainDataResponse {
    total_count: usize,
    sentences: Vec<String>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🛣️ API 엔드포인트
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🤖 GPT-2 채팅봇</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }
        
        .container {
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            width: 100%;
            max-width: 600px;
            height: 80vh;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            text-align: center;
        }
        
        .header h1 {
            font-size: 24px;
            margin-bottom: 5px;
        }
        
        .header p {
            font-size: 14px;
            opacity: 0.9;
        }
        
        .chat-box {
            flex: 1;
            overflow-y: auto;
            padding: 20px;
            background: #f5f5f5;
        }
        
        .message {
            margin-bottom: 15px;
            display: flex;
            animation: fadeIn 0.3s;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
        
        .message.user {
            justify-content: flex-end;
        }
        
        .message-content {
            max-width: 70%;
            padding: 12px 16px;
            border-radius: 18px;
            word-wrap: break-word;
        }
        
        .message.user .message-content {
            background: #667eea;
            color: white;
        }
        
        .message.bot .message-content {
            background: white;
            color: #333;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
        }
        
        .input-area {
            padding: 20px;
            background: white;
            border-top: 1px solid #e0e0e0;
        }
        
        .input-group {
            display: flex;
            gap: 10px;
        }
        
        input[type="text"] {
            flex: 1;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 25px;
            font-size: 14px;
            outline: none;
            transition: border-color 0.3s;
        }
        
        input[type="text"]:focus {
            border-color: #667eea;
        }
        
        button {
            padding: 12px 24px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 25px;
            cursor: pointer;
            font-size: 14px;
            font-weight: bold;
            transition: transform 0.2s;
        }
        
        button:hover {
            transform: scale(1.05);
        }
        
        button:active {
            transform: scale(0.95);
        }
        
        button:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
        
        .controls {
            display: flex;
            gap: 10px;
            margin-bottom: 10px;
            align-items: center;
        }
        
        .control-group {
            flex: 1;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        
        .control-group label {
            font-size: 12px;
            color: #666;
            white-space: nowrap;
        }
        
        input[type="range"] {
            flex: 1;
        }
        
        .value-display {
            font-size: 12px;
            color: #667eea;
            font-weight: bold;
            min-width: 30px;
        }
        
        .loading {
            display: none;
            text-align: center;
            padding: 10px;
            color: #999;
        }
        
        .loading.active {
            display: block;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🤖 GPT-2 채팅봇</h1>
            <p>Rust Candle로 만든 AI 챗봇</p>
        </div>
        
        <div class="chat-box" id="chatBox">
            <div class="message bot">
                <div class="message-content">
                    안녕하세요! 👋 저는 GPT-2 기반 챗봇이에요. 무엇을 도와드릴까요?
                </div>
            </div>
        </div>
        
        <div class="loading" id="loading">
            <span>🤔 생각 중...</span>
        </div>
        
        <div class="input-area">
            <div class="controls">
                <div class="control-group">
                    <label>창의성:</label>
                    <input type="range" id="temperature" min="0.1" max="1.5" step="0.1" value="0.8">
                    <span class="value-display" id="tempValue">0.8</span>
                </div>
                <div class="control-group">
                    <label>길이:</label>
                    <input type="range" id="maxTokens" min="5" max="30" step="5" value="15">
                    <span class="value-display" id="tokensValue">15</span>
                </div>
            </div>
            <div class="input-group">
                <input type="text" id="messageInput" placeholder="메시지를 입력하세요..." />
                <button onclick="sendMessage()" id="sendBtn">전송</button>
            </div>
        </div>
    </div>
    
    <script>
        const chatBox = document.getElementById('chatBox');
        const messageInput = document.getElementById('messageInput');
        const sendBtn = document.getElementById('sendBtn');
        const loading = document.getElementById('loading');
        const temperature = document.getElementById('temperature');
        const maxTokens = document.getElementById('maxTokens');
        const tempValue = document.getElementById('tempValue');
        const tokensValue = document.getElementById('tokensValue');
        
        temperature.addEventListener('input', (e) => {
            tempValue.textContent = e.target.value;
        });
        
        maxTokens.addEventListener('input', (e) => {
            tokensValue.textContent = e.target.value;
        });
        
        messageInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendMessage();
            }
        });
        
        async function sendMessage() {
            const message = messageInput.value.trim();
            if (!message) return;
            
            // 사용자 메시지 표시
            addMessage(message, 'user');
            messageInput.value = '';
            sendBtn.disabled = true;
            loading.classList.add('active');
            
            try {
                const response = await fetch('/chat', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        message: message,
                        temperature: parseFloat(temperature.value),
                        max_tokens: parseInt(maxTokens.value)
                    }),
                });
                
                const data = await response.json();
                addMessage(data.response, 'bot');
            } catch (error) {
                addMessage('죄송해요, 오류가 발생했어요. 😢', 'bot');
                console.error('Error:', error);
            } finally {
                sendBtn.disabled = false;
                loading.classList.remove('active');
            }
        }
        
        function addMessage(text, sender) {
            const messageDiv = document.createElement('div');
            messageDiv.className = `message ${sender}`;
            
            const contentDiv = document.createElement('div');
            contentDiv.className = 'message-content';
            contentDiv.textContent = text;
            
            messageDiv.appendChild(contentDiv);
            chatBox.appendChild(messageDiv);
            chatBox.scrollTop = chatBox.scrollHeight;
        }
    </script>
</body>
</html>
        "#
    )
}

#[post("/chat")]
async fn chat(
    data: web::Data<Arc<AppState>>,
    req: web::Json<ChatRequest>,
) -> impl Responder {
    let message = req.message.clone();
    let temperature = req.temperature;
    let max_tokens = req.max_tokens;
    
    // 히스토리에 사용자 메시지 추가
    {
        let mut history = data.history.lock().unwrap();
        history.push(format!("User: {}", message));
    }
    
    // 응답 생성
    let response = match generate_response(
        &data.model,
        &data.tokenizer,
        &message,
        max_tokens,
        temperature,
        &data.device,
    ) {
        Ok(resp) => resp,
        Err(_) => "Sorry, I couldn't generate a response.".to_string(),
    };
    
    // 히스토리에 봇 응답 추가
    {
        let mut history = data.history.lock().unwrap();
        history.push(format!("Bot: {}", response));
    }
    
    let history = data.history.lock().unwrap().clone();
    
    HttpResponse::Ok().json(ChatResponse {
        response,
        history,
    })
}

#[get("/health")]
async fn health(data: web::Data<Arc<AppState>>) -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        model: "GPT-2 Chatbot".to_string(),
        vocab_size: data.tokenizer.vocab_size(),
    })
}

#[post("/train/add")]
async fn add_training_data(req: web::Json<AddDataRequest>) -> impl Responder {
    let data_path = "training_data.txt";
    
    // 기존 데이터 읽기
    let mut existing_data = match fs::read_to_string(data_path) {
        Ok(content) => content,
        Err(_) => String::from("# GPT-2 채팅봇 학습 데이터\n"),
    };
    
    // 새 데이터 추가
    let added_count = req.sentences.len();
    for sentence in &req.sentences {
        if !sentence.trim().is_empty() {
            existing_data.push_str(&format!("{}\n", sentence.trim()));
        }
    }
    
    // 파일에 저장
    match fs::write(data_path, &existing_data) {
        Ok(_) => {
            let total_count = existing_data.lines()
                .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                .count();
            
            HttpResponse::Ok().json(AddDataResponse {
                success: true,
                added_count,
                total_count,
                message: format!("{}개 문장이 추가되었습니다. 재학습이 필요합니다.", added_count),
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(AddDataResponse {
                success: false,
                added_count: 0,
                total_count: 0,
                message: format!("파일 저장 실패: {}", e),
            })
        }
    }
}

#[get("/train/data")]
async fn get_training_data() -> impl Responder {
    let data_path = "training_data.txt";
    
    match fs::read_to_string(data_path) {
        Ok(content) => {
            let sentences: Vec<String> = content
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                .map(|line| line.to_string())
                .collect();
            
            HttpResponse::Ok().json(TrainDataResponse {
                total_count: sentences.len(),
                sentences,
            })
        }
        Err(_) => {
            HttpResponse::Ok().json(TrainDataResponse {
                total_count: 0,
                sentences: vec![],
            })
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🚀 메인
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🤖 GPT-2 채팅봇 API");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let device = Device::Cpu;
    let tokenizer = SimpleTokenizer::new(30);
    
    println!("  📚 Vocab 크기: {}", tokenizer.vocab_size());
    
    // 모델 생성
    let mut varmap = VarMap::new();
    let model_path = "chatbot_model.safetensors";
    
    // 🔄 기존 가중치 로드 시도 (전이 학습)
    if Path::new(model_path).exists() {
        println!("  📥 기존 모델 로드 중: {}", model_path);
        match varmap.load(model_path) {
            Ok(_) => println!("  ✅ 기존 가중치 로드 완료! (전이 학습 모드)"),
            Err(e) => println!("  ⚠️  로드 실패, 새로 학습: {}", e),
        }
    } else {
        println!("  ⚠️  저장된 모델 없음 (처음부터 학습)");
    }
    
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    
    let model = GPT2::new(
        vb.clone(),
        tokenizer.vocab_size(),
        128,
        4,
        30,
    ).expect("Failed to create model");
    
    println!("  🧠 모델 구조:");
    println!("     - Embed Size: 128");
    println!("     - Layers: 4");
    println!("     - Max Seq Len: 30");
    
    // 🎓 추가 학습 (기존 가중치 기반)
    println!();
    train_chatbot(&model, &varmap, &tokenizer, &device).expect("Training failed");
    
    // AppState 생성
    let app_state = Arc::new(AppState {
        model,
        tokenizer,
        device,
        history: Mutex::new(Vec::new()),
    });
    
    println!("\n  🌐 서버 시작!");
    println!("     - URL: http://localhost:3000");
    println!("     - API: POST /chat");
    println!("     - Health: GET /health");
    println!("     - 학습 데이터 추가: POST /train/add");
    println!("     - 학습 데이터 조회: GET /train/data");
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    HttpServer::new(move || {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(index)
            .service(chat)
            .service(health)
            .service(add_training_data)
            .service(get_training_data)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
