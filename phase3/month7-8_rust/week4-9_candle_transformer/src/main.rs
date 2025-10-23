// Week 4-9: Transformer - Attention is All You Need! 🚀
//
// 🎯 목표: Attention 메커니즘으로 시퀀스 처리!
// 참고: https://wikidocs.net/31379
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📚 Transformer란?
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// Transformer = Attention 블록을 여러 개 쌓은 구조!
//
// 핵심 아이디어:
//   RNN/LSTM: 순차 처리 (느림)
//   Transformer: 병렬 처리 + Attention (빠름!)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🏗️ Transformer 구조
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 1개 블록:
//   ┌─────────────────────────┐
//   │  Multi-Head Attention   │ ← 여러 관점으로 관계 파악
//   └─────────────────────────┘
//            ↓
//   ┌─────────────────────────┐
//   │  Add & Normalize        │ ← 안정화
//   └─────────────────────────┘
//            ↓
//   ┌─────────────────────────┐
//   │  Feed Forward           │ ← 일반 신경망
//   └─────────────────────────┘
//            ↓
//   ┌─────────────────────────┐
//   │  Add & Normalize        │ ← 안정화
//   └─────────────────────────┘
//
// 이걸 여러 번 반복!
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔑 핵심 개념: Attention
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// Attention(Q, K, V) = softmax(Q·K^T / √d_k) · V
//
// Q (Query):  "지금 이해하려는 것"
// K (Key):    "각 요소의 특징"
// V (Value):  "실제 정보"
//
// 예시:
//   문장: "The cat sat"
//   "sat"을 이해할 때:
//     Q = "sat의 의미는?"
//     K = [The, cat, sat]의 특징
//     V = [The, cat, sat]의 정보
//
//   결과:
//     The: 0.1 (10% 집중)
//     cat: 0.8 (80% 집중!) ← 주어니까!
//     sat: 0.1 (10% 집중)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎯 실전 예제: 시퀀스 예측
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 입력: [1, 2, 3, 4, 5]
// 출력: [2, 3, 4, 5, 6] (다음 숫자 예측)
//
// Attention이 패턴을 학습:
//   1 → 2 (관계: +1)
//   2 → 3 (관계: +1)
//   3 → 4 (관계: +1)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{
    linear, loss, ops, AdamW, LayerNorm, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🤖 Transformer: Attention is All You Need!");
    println!("{}", "=".repeat(60));

    // 1. 시퀀스 데이터 생성
    println!("\n[1] 시퀀스 데이터 생성");
    let (train_x, train_y) = generate_sequence_data(200, 5)?;
    println!("  📊 200개 시퀀스 생성!");
    println!("  📏 입력: [1,2,3,4,5] → 출력: [2,3,4,5,6]");
    println!("  💡 Attention으로 패턴 학습!");

    // 2. Transformer 모델 생성
    println!("\n[2] Transformer 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = SimpleTransformer::new(vb)?;
    println!("  🧠 Transformer 모델 생성 완료!");
    println!("  구조: Attention → Feed Forward → Output");

    // 3. 학습
    println!("\n[3] 학습 시작!");
    train_transformer(&model, &varmap, &train_x, &train_y, 500)?;

    // 4. 예측 테스트
    println!("\n[4] 예측 테스트");

    // 정규화된 테스트 데이터
    let test_input = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
    let test_output = vec![2.0f32, 3.0, 4.0, 5.0, 6.0];

    let min_val = test_input[0];
    let max_val = test_output[4];
    let range = max_val - min_val;

    let normalized_input: Vec<f32> = test_input.iter().map(|x| (x - min_val) / range).collect();

    let test_seq = Tensor::new(&normalized_input[..], &device)?.reshape((1, 5, 1))?;

    println!("\n  📊 테스트 1: [1, 2, 3, 4, 5]");

    let prediction = model.forward(&test_seq)?;

    print!("  🔮 예측: [");
    for i in 0..5 {
        let norm_val = prediction.i((0, i, 0))?.to_scalar::<f32>()?;
        let actual_val = norm_val * range + min_val;
        print!("{:.1}", actual_val);
        if i < 4 {
            print!(", ");
        }
    }
    println!("]");
    println!("  ✅ 정답: [2, 3, 4, 5, 6]");

    // 테스트 케이스 2: [100, 200, 300, 400, 500] → [200, 300, 400, 500, 600]
    println!("\n  📊 테스트 2: [100, 200, 300, 400, 500]");
    let test_input2 = vec![100.0f32, 200.0, 300.0, 400.0, 500.0];
    let test_output2 = vec![200.0f32, 300.0, 400.0, 500.0, 600.0];

    let min_val2 = test_input2[0];
    let max_val2 = test_output2[4];
    let range2 = max_val2 - min_val2;

    let normalized_input2: Vec<f32> = test_input2
        .iter()
        .map(|x| (x - min_val2) / range2)
        .collect();

    let test_seq2 = Tensor::new(&normalized_input2[..], &device)?.reshape((1, 5, 1))?;
    let prediction2 = model.forward(&test_seq2)?;

    print!("  🔮 예측: [");
    for i in 0..5 {
        let norm_val = prediction2.i((0, i, 0))?.to_scalar::<f32>()?;
        let actual_val = norm_val * range2 + min_val2;
        print!("{:.1}", actual_val);
        if i < 4 {
            print!(", ");
        }
    }
    println!("]");
    println!("  ✅ 정답: [200, 300, 400, 500, 600]");

    // 5. 모델 저장
    println!("\n[5] 모델 저장");
    varmap.save("transformer_model.safetensors")?;
    println!("  ✅ transformer_model.safetensors 저장 완료!");

    println!("\n{}", "=".repeat(60));
    println!("✅ Transformer 완료! Attention으로 패턴 학습! 🎉");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 간단한 Transformer 모델
struct SimpleTransformer {
    // Attention 레이어
    query: Linear,
    key: Linear,
    value: Linear,

    // Feed Forward 레이어
    ff1: Linear,
    ff2: Linear,

    // Layer Normalization
    norm1: LayerNorm,
    norm2: LayerNorm,

    // 출력 레이어
    output: Linear,

    hidden_size: usize,
}

impl SimpleTransformer {
    fn new(vb: VarBuilder) -> Result<Self> {
        let hidden_size = 32;

        // Attention용 Q, K, V
        let query = linear(1, hidden_size, vb.pp("query"))?;
        let key = linear(1, hidden_size, vb.pp("key"))?;
        let value = linear(1, hidden_size, vb.pp("value"))?;

        // Feed Forward
        let ff1 = linear(hidden_size, hidden_size * 2, vb.pp("ff1"))?;
        let ff2 = linear(hidden_size * 2, hidden_size, vb.pp("ff2"))?;

        // Layer Norm
        let norm1 = LayerNorm::new(
            Tensor::ones(&[hidden_size], DType::F32, vb.device())?,
            Tensor::zeros(&[hidden_size], DType::F32, vb.device())?,
            1e-5,
        );
        let norm2 = LayerNorm::new(
            Tensor::ones(&[hidden_size], DType::F32, vb.device())?,
            Tensor::zeros(&[hidden_size], DType::F32, vb.device())?,
            1e-5,
        );

        // 출력
        let output = linear(hidden_size, 1, vb.pp("output"))?;

        Ok(Self {
            query,
            key,
            value,
            ff1,
            ff2,
            norm1,
            norm2,
            output,
            hidden_size,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (batch_size, seq_len, _) = x.dims3()?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 1️⃣ Self-Attention: 시퀀스 내 관계 파악
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        // Q, K, V 생성
        let q = self.query.forward(x)?; // [batch, seq, hidden]
        let k = self.key.forward(x)?; // [batch, seq, hidden]
        let v = self.value.forward(x)?; // [batch, seq, hidden]

        // Attention 계산: Q·K^T / √d_k
        let k_t = k.transpose(1, 2)?; // [batch, hidden, seq]
        let scores = q.matmul(&k_t)?; // [batch, seq, seq]

        let d_k = (self.hidden_size as f64).sqrt();
        let scaled_scores = (scores / d_k)?;

        // Softmax로 확률화
        let attention_weights = ops::softmax(&scaled_scores, 2)?;

        // V와 곱해서 최종 출력
        let attention_output = attention_weights.matmul(&v)?; // [batch, seq, hidden]

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 2️⃣ Add & Normalize: 잔차 연결 + 정규화
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        // 입력을 hidden_size로 변환
        let x_proj = self.query.forward(x)?;
        let x1 = (x_proj + attention_output)?;
        let x1_norm = self.norm1.forward(&x1)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 3️⃣ Feed Forward: 일반 신경망
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let ff_output = self.ff1.forward(&x1_norm)?;
        let ff_output = ff_output.relu()?;
        let ff_output = self.ff2.forward(&ff_output)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 4️⃣ Add & Normalize: 다시 잔차 연결 + 정규화
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        let x2 = (x1_norm + ff_output)?;
        let x2_norm = self.norm2.forward(&x2)?;

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // 5️⃣ 최종 출력
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        self.output.forward(&x2_norm)
    }
}

/// 시퀀스 데이터 생성 (정규화 버전)
fn generate_sequence_data(n: usize, seq_len: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut rng = rand::thread_rng();

    let mut sequences_x = Vec::new();
    let mut sequences_y = Vec::new();

    for _ in 0..n {
        let start = rng.gen_range(0.0..10.0);

        // 입력 시퀀스 (등차수열, step=1)
        let mut input_seq = Vec::new();
        for i in 0..seq_len {
            input_seq.push(start + i as f32);
        }

        // 출력 시퀀스 (다음 값들, +1씩)
        let mut output_seq = Vec::new();
        for i in 1..=seq_len {
            output_seq.push(start + i as f32);
        }

        // 정규화 (0~1 범위로)
        let min_val = input_seq[0];
        let max_val = output_seq[seq_len - 1];
        let range = max_val - min_val;

        for val in input_seq {
            sequences_x.push((val - min_val) / range);
        }

        for val in output_seq {
            sequences_y.push((val - min_val) / range);
        }
    }

    let x = Tensor::from_vec(sequences_x, &[n, seq_len, 1], &device)?;
    let y = Tensor::from_vec(sequences_y, &[n, seq_len, 1], &device)?;

    Ok((x, y))
}

/// Transformer 학습
fn train_transformer(
    model: &SimpleTransformer,
    varmap: &VarMap,
    train_x: &Tensor,
    train_y: &Tensor,
    epochs: usize,
) -> Result<()> {
    let params = ParamsAdamW {
        lr: 0.01,
        ..Default::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), params)?;

    for epoch in 0..epochs {
        let predictions = model.forward(train_x)?;
        let loss = loss::mse(&predictions, train_y)?;
        optimizer.backward_step(&loss)?;

        if epoch % 10 == 0 {
            let loss_val = loss.to_vec0::<f32>()?;
            println!("  Epoch {:3}/{}  Loss: {:.6}", epoch, epochs, loss_val);
        }
    }

    println!("  🎉 학습 완료!");
    Ok(())
}
