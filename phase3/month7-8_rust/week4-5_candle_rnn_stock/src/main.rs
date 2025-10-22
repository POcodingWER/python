// Week 4-5: RNN 실생활 예제 - 주식 가격 예측 📈
//
// 🎯 목표: 과거 7일 주가 → 내일 주가 예측
//
// 예시:
// 월: 10,000원
// 화: 10,200원 (+200)
// 수: 10,500원 (+300)
// 목: 10,300원 (-200)
// 금: 10,600원 (+300)
// 토: 10,800원 (+200)
// 일: 11,000원 (+200)
// → 월요일은? RNN이 예측!
//
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{linear, loss, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use rand::Rng;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("📈 RNN: 주식 가격 예측");
    println!("{}", "=".repeat(60));

    // 1. 주식 데이터 생성
    println!("\n[1] 주식 데이터 생성");
    let (train_x, train_y) = generate_stock_data(300, 7)?;
    println!("  📊 300개 주식 패턴 생성!");
    println!("  📏 입력: 과거 7일 가격 → 출력: 내일 가격");
    println!("  💡 패턴: 상승, 하락, 횡보 등 다양한 시나리오");

    // 2. RNN 모델 생성
    println!("\n[2] RNN 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = StockRNN::new(vb)?;
    println!("  🧠 주식 예측 RNN 생성 완료!");
    println!("  구조: RNN(1→32) → Linear(32→1)");

    // 3. 학습 전 예측
    println!("\n[3] 학습 전 예측 (랜덤 가중치)");
    let test_prices = vec![
        10000.0, 10200.0, 10500.0, 10300.0, 10600.0, 10800.0, 11000.0,
    ];
    println!("  📅 과거 7일 주가:");
    for (i, price) in test_prices.iter().enumerate() {
        let day = match i {
            0 => "월",
            1 => "화",
            2 => "수",
            3 => "목",
            4 => "금",
            5 => "토",
            6 => "일",
            _ => "?",
        };
        println!("    {}: {:>7.0}원", day, price);
    }

    let test_seq = create_sequence(&test_prices)?;
    let pred_before = model.forward(&test_seq)?;
    let pred_val_before = pred_before.i((0, 0))?.to_scalar::<f32>()?;

    // 역정규화
    let min = test_prices.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = test_prices
        .iter()
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;
    let pred_price_before = pred_val_before * range + min;

    println!("  🔮 예측 (랜덤): {:>7.0}원", pred_price_before);

    // 4. 학습!
    println!("\n[4] 학습 시작! 🔥");
    train_rnn(&model, &varmap, &train_x, &train_y, 150)?;

    // 5. 학습 후 예측
    println!("\n[5] 학습 후 예측 ✨");
    let pred_after = model.forward(&test_seq)?;
    let pred_val_after = pred_after.i((0, 0))?.to_scalar::<f32>()?;

    // 역정규화
    let pred_price_after = pred_val_after * range + min;

    // 실제 패턴 분석 (상승 추세)
    let trend = (test_prices[6] - test_prices[0]) / 6.0; // 평균 일일 상승
    let expected = test_prices[6] + trend;

    println!("  📅 과거 7일 주가:");
    for (i, price) in test_prices.iter().enumerate() {
        let day = match i {
            0 => "월",
            1 => "화",
            2 => "수",
            3 => "목",
            4 => "금",
            5 => "토",
            6 => "일",
            _ => "?",
        };
        let change = if i > 0 {
            format!("({:+.0})", price - test_prices[i - 1])
        } else {
            String::new()
        };
        println!("    {}: {:>7.0}원 {}", day, price, change);
    }
    println!("  📊 평균 일일 변화: {:+.0}원", trend);
    println!("  🔮 AI 예측: {:>7.0}원", pred_price_after);
    println!("  📈 추세 예측: {:>7.0}원", expected);
    println!("  ✅ 오차: {:>7.0}원", (pred_price_after - expected).abs());

    // 6. 다양한 패턴 테스트
    println!("\n[6] 다양한 주식 패턴 테스트");
    test_stock_patterns(&model)?;

    // 7. 모델 저장
    println!("\n[7] 모델 저장");
    varmap.save("stock_rnn.safetensors")?;
    println!("  💾 모델 저장 완료: stock_rnn.safetensors");

    println!("\n{}", "=".repeat(60));
    println!("✅ 주식 예측 RNN 완료!");
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
        let hidden_size = 32; //기억력 크기
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

/// 주식 데이터 생성 (다양한 패턴)
fn generate_stock_data(n: usize, seq_len: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut rng = rand::thread_rng();
    let mut sequences = Vec::new();
    let mut targets = Vec::new();

    for _ in 0..n {
        let base_price = rng.gen_range(8000.0..12000.0);
        let pattern = rng.gen_range(0..3);

        let mut prices = vec![base_price];

        // 다양한 패턴 생성
        for i in 1..=seq_len {
            let prev = prices[i - 1];
            let next: f32 = match pattern {
                0 => {
                    // 상승 추세
                    prev + rng.gen_range(50.0..300.0)
                }
                1 => {
                    // 하락 추세
                    prev - rng.gen_range(50.0..300.0)
                }
                _ => {
                    // 횡보 (변동 작음)
                    prev + rng.gen_range(-100.0..100.0)
                }
            };
            prices.push(next.max(1000.0)); // 최소 1000원
        }

        // 정규화 (0~1 범위로)
        let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;

        // 입력: 처음 7일 가격 → 출력: 내일 가격
        for i in 0..seq_len {
            let normalized = (prices[i] - min) / range;
            sequences.push(normalized);
        }

        // 출력: 내일 가격
        let target_normalized = (prices[seq_len] - min) / range;
        targets.push(target_normalized);
    }

    let x = Tensor::from_vec(sequences, &[n, seq_len, 1], &device)?;
    let y = Tensor::from_vec(targets, &[n, 1], &device)?;

    Ok((x, y))
}

/// 테스트용 시퀀스 생성
fn create_sequence(prices: &[f32]) -> Result<Tensor> {
    let device = Device::Cpu;

    // 정규화
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

        if epoch % 30 == 0 {
            let loss_val = loss.to_vec0::<f32>()?;
            println!("  Epoch {:3}/{}  Loss: {:.6}", epoch, epochs, loss_val);
        }
    }

    println!("  🎉 학습 완료!");
    Ok(())
}

/// 다양한 주식 패턴 테스트
fn test_stock_patterns(model: &StockRNN) -> Result<()> {
    let patterns = vec![
        (
            "급등 패턴",
            vec![
                10000.0, 10300.0, 10800.0, 11500.0, 12300.0, 13200.0, 14200.0,
            ],
        ),
        (
            "급락 패턴",
            vec![14000.0, 13500.0, 12800.0, 12000.0, 11100.0, 10100.0, 9000.0],
        ),
        (
            "횡보 패턴",
            vec![10000.0, 10100.0, 9900.0, 10050.0, 9950.0, 10000.0, 10100.0],
        ),
        (
            "V자 반등",
            vec![12000.0, 11000.0, 10000.0, 9500.0, 10000.0, 11000.0, 12000.0],
        ),
    ];

    for (name, prices) in patterns {
        let x = create_sequence(&prices)?;
        let pred = model.forward(&x)?;
        let pred_val = pred.i((0, 0))?.to_scalar::<f32>()?;

        // 역정규화
        let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;
        let predicted_price = pred_val * range + min;

        // 추세 계산
        let trend = (prices[6] - prices[0]) / 6.0;
        let expected = prices[6] + trend;

        println!("  📊 {} (마지막: {:.0}원)", name, prices[6]);
        println!(
            "     → AI 예측: {:.0}원, 추세 예측: {:.0}원",
            predicted_price, expected
        );
    }

    Ok(())
}
