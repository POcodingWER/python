// Week 4-7: RNN 추가 학습 (Fine-tuning) 🔄
//
// 🎯 목표: 기존 모델에 새 데이터로 추가 학습!
//
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{linear, loss, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use rand::Rng;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🔄 RNN 추가 학습 (Fine-tuning)");
    println!("{}", "=".repeat(60));

    let device = Device::Cpu;

    // 1. 기존 모델 불러오기
    println!("\n[1] 기존 모델 불러오기");
    let mut varmap = VarMap::new();
    varmap.load("../week4-5_candle_rnn_stock/stock_rnn.safetensors")?;
    println!("  ✅ 기존 모델 로드 완료!");

    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = StockRNN::new(vb)?;

    // 2. 불러온 모델로 예측 테스트
    println!("\n[2] 추가 학습 전 예측");
    let test_prices = vec![
        10000.0, 10200.0, 10500.0, 10300.0, 10600.0, 10800.0, 11000.0,
    ];
    let test_seq = create_sequence(&test_prices)?;
    let pred_before = model.forward(&test_seq)?;
    let pred_val_before = pred_before.i((0, 0))?.to_scalar::<f32>()?;

    let min = test_prices.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = test_prices
        .iter()
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;
    let pred_price_before = pred_val_before * range + min;

    println!("  🔮 예측: {:>7.0}원", pred_price_before);

    // 3. 새로운 데이터 생성
    println!("\n[3] 새로운 학습 데이터 생성");
    let (new_train_x, new_train_y) = generate_stock_data(100, 7)?;
    println!("  📊 100개 새 패턴 생성!");

    // 4. 추가 학습!
    println!("\n[4] 추가 학습 시작! 🔥");
    println!("  💡 기존 지식 + 새 데이터 = 더 똑똑해짐!");
    train_rnn(&model, &varmap, &new_train_x, &new_train_y, 50)?;

    // 5. 추가 학습 후 예측
    println!("\n[5] 추가 학습 후 예측");
    let pred_after = model.forward(&test_seq)?;
    let pred_val_after = pred_after.i((0, 0))?.to_scalar::<f32>()?;
    let pred_price_after = pred_val_after * range + min;

    println!("  🔮 예측: {:>7.0}원", pred_price_after);
    println!("  📊 변화: {:+.0}원", pred_price_after - pred_price_before);

    // 6. 새 버전으로 저장
    println!("\n[6] 모델 저장");
    println!("  💾 옵션 1: 덮어쓰기 (기존 파일 업데이트)");
    println!("  💾 옵션 2: 새 파일 (버전 관리)");

    // 버전 관리 방식
    varmap.save("stock_rnn_v2.safetensors")?;
    println!("  ✅ stock_rnn_v2.safetensors 저장 완료!");

    // 또는 덮어쓰기
    // varmap.save("../week4-5_candle_rnn_stock/stock_rnn.safetensors")?;
    // println!("  ✅ 기존 파일 업데이트 완료!");

    println!("\n{}", "=".repeat(60));
    println!("✅ 추가 학습 완료! 모델이 더 똑똑해졌어요!");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 주식 예측 RNN
struct StockRNN {
    rnn_layer: Linear,
    output_layer: Linear,
    hidden_size: usize,
}

impl StockRNN {
    fn new(vb: VarBuilder) -> Result<Self> {
        let hidden_size = 32;
        let rnn_layer = linear(1 + hidden_size, hidden_size, vb.pp("rnn"))?;
        let output_layer = linear(hidden_size, 1, vb.pp("output"))?;

        Ok(Self {
            rnn_layer,
            output_layer,
            hidden_size,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let device = x.device();
        let batch_size = x.dim(0)?;
        let seq_len = x.dim(1)?;

        let mut hidden = Tensor::zeros(&[batch_size, self.hidden_size], DType::F32, device)?;

        for t in 0..seq_len {
            let x_t = x.i((.., t, ..))?;
            let combined = Tensor::cat(&[&x_t, &hidden], 1)?;
            hidden = self.rnn_layer.forward(&combined)?.tanh()?;
        }

        self.output_layer.forward(&hidden)
    }
}

/// 주식 데이터 생성
fn generate_stock_data(n: usize, seq_len: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut rng = rand::thread_rng();
    let mut sequences = Vec::new();
    let mut targets = Vec::new();

    for _ in 0..n {
        let base_price = rng.gen_range(8000.0..12000.0);
        let pattern = rng.gen_range(0..3);

        let mut prices = vec![base_price];

        for i in 1..=seq_len {
            let prev = prices[i - 1];
            let next: f32 = match pattern {
                0 => prev + rng.gen_range(50.0..300.0),
                1 => prev - rng.gen_range(50.0..300.0),
                _ => prev + rng.gen_range(-100.0..100.0),
            };
            prices.push(next.max(1000.0));
        }

        let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;

        for i in 0..seq_len {
            let normalized = (prices[i] - min) / range;
            sequences.push(normalized);
        }

        let target_normalized = (prices[seq_len] - min) / range;
        targets.push(target_normalized);
    }

    let x = Tensor::from_vec(sequences, &[n, seq_len, 1], &device)?;
    let y = Tensor::from_vec(targets, &[n, 1], &device)?;

    Ok((x, y))
}

/// 시퀀스 생성
fn create_sequence(prices: &[f32]) -> Result<Tensor> {
    let device = Device::Cpu;

    let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;

    let normalized: Vec<f32> = prices.iter().map(|p| (p - min) / range).collect();

    Tensor::from_vec(normalized, &[1, prices.len(), 1], &device)
}

/// RNN 학습
fn train_rnn(
    model: &StockRNN,
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

    println!("  🎉 추가 학습 완료!");
    Ok(())
}
