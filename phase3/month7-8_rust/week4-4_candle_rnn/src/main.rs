// Week 4-4: RNN 순환 신경망(Recurrent Neural Network) 🔄
// 참고: https://wikidocs.net/64066
// 시계열 데이터 예측 - 사인파 패턴 학습
//
// 🔥 RNN 특징:
// 1. 순차 데이터 처리 (시간, 문장 등)
// 2. 이전 상태를 기억 (hidden state)
// 3. 같은 가중치를 반복 사용
//
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{linear, loss, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use std::f32::consts::PI;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🔄 RNN: 시계열 데이터 예측 (사인파)");
    println!("{}", "=".repeat(60));

    // 1. 시계열 데이터 생성 (사인파)
    println!("\n[1] 시계열 데이터 생성");
    let (train_x, train_y) = generate_sine_data(500, 10)?;
    println!("  📊 500개 시퀀스 생성!");
    println!("  📏 입력: 과거 10개 값 → 출력: 다음 1개 값");

    // 2. RNN 모델 생성
    println!("\n[2] RNN 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = SimpleRNN::new(vb)?;
    println!("  🧠 RNN 모델 생성 완료!");
    println!("  구조: RNN(1→32) → Linear(32→1)");

    // 3. 학습 전 예측
    println!("\n[3] 학습 전 예측 (랜덤 가중치)");
    let test_seq = generate_test_sequence()?;
    let pred_before = model.forward(&test_seq)?;
    let pred_val_before = pred_before.i((0, 0))?.to_scalar::<f32>()?;
    println!("  입력: sin(0°~90°) 10개 값");
    println!("  예측: {:.4} (랜덤)", pred_val_before);

    // 4. 학습!
    println!("\n[4] 학습 시작! 🔥");
    train_rnn(&model, &varmap, &train_x, &train_y, 200)?;

    // 5. 학습 후 예측
    println!("\n[5] 학습 후 예측 ✨");
    let pred_after = model.forward(&test_seq)?;
    let pred_val_after = pred_after.i((0, 0))?.to_scalar::<f32>()?;
    let expected = (100.0_f32 * PI / 180.0).sin();
    println!("  입력: sin(0°~90°) 10개 값");
    println!("  예측: {:.4}", pred_val_after);
    println!("  정답: {:.4} (sin(100°))", expected);
    println!("  오차: {:.4}", (pred_val_after - expected).abs());

    // 6. 여러 패턴 테스트
    println!("\n[6] 다양한 패턴 테스트");
    test_patterns(&model)?;

    // 7. 모델 저장
    println!("\n[7] 모델 저장");
    varmap.save("rnn_model.safetensors")?;
    println!("  💾 모델 저장 완료: rnn_model.safetensors");

    println!("\n{}", "=".repeat(60));
    println!("✅ RNN 학습 완료! 시계열 예측 가능!");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 간단한 RNN 모델
struct SimpleRNN {
    rnn_layer: Linear,    // RNN 레이어 (input + hidden → hidden)
    output_layer: Linear, // 출력 레이어 (hidden → output)
    hidden_size: usize,
}

impl SimpleRNN {
    fn new(vb: VarBuilder) -> Result<Self> {
        let hidden_size = 32;

        // RNN: (input_size + hidden_size) → hidden_size
        let rnn_layer = linear(1 + hidden_size, hidden_size, vb.pp("rnn"))?;

        // Output: hidden_size → 1
        let output_layer = linear(hidden_size, 1, vb.pp("output"))?;

        Ok(Self {
            rnn_layer,
            output_layer,
            hidden_size,
        })
    }

    /// Forward pass: 시퀀스 처리
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // x shape: [batch, seq_len, 1]
        let device = x.device();
        let batch_size = x.dim(0)?;
        let seq_len = x.dim(1)?;

        // 초기 hidden state (모두 0)
        let mut hidden = Tensor::zeros(&[batch_size, self.hidden_size], DType::F32, device)?;

        // 시퀀스를 순차적으로 처리
        for t in 0..seq_len {
            // 현재 시점의 입력
            let x_t = x.i((.., t, ..))?; // [batch, 1]

            // [input, hidden] 결합
            let combined = Tensor::cat(&[&x_t, &hidden], 1)?; // [batch, 1+hidden_size]

            // RNN 계산: hidden_new = tanh(W * [x, h])
            hidden = self.rnn_layer.forward(&combined)?.tanh()?;
        }

        // 마지막 hidden state로 출력 계산
        self.output_layer.forward(&hidden)
    }
}

/// 사인파 시계열 데이터 생성
fn generate_sine_data(n: usize, seq_len: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut sequences = Vec::new();
    let mut targets = Vec::new();

    for i in 0..n {
        // 랜덤 시작점
        let start = (i as f32) * 2.0 * PI / (n as f32);

        // seq_len개의 연속된 사인 값
        for j in 0..seq_len {
            let angle = start + (j as f32) * PI / 180.0; // 1도씩 증가
            sequences.push((angle).sin());
        }

        // 다음 값이 타겟
        let next_angle = start + (seq_len as f32) * PI / 180.0;
        targets.push(next_angle.sin());
    }

    // [n, seq_len, 1] 형태로 변환
    let x = Tensor::from_vec(sequences, &[n, seq_len, 1], &device)?;

    // [n, 1] 형태
    let y = Tensor::from_vec(targets, &[n, 1], &device)?;

    Ok((x, y))
}

/// 테스트용 시퀀스 생성 (0°~90°)
fn generate_test_sequence() -> Result<Tensor> {
    let device = Device::Cpu;
    let mut seq = Vec::new();

    // 0°~90° (10개 값)
    for i in 0..10 {
        let angle = (i as f32) * 10.0 * PI / 180.0;
        seq.push(angle.sin());
    }

    // [1, 10, 1] 형태
    Tensor::from_vec(seq, &[1, 10, 1], &device)
}

/// RNN 학습 함수
fn train_rnn(
    model: &SimpleRNN,
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
        // Forward pass
        let predictions = model.forward(train_x)?;

        // MSE Loss
        let loss = loss::mse(&predictions, train_y)?;

        // Backward pass
        optimizer.backward_step(&loss)?;

        // 20 epoch마다 출력
        if epoch % 20 == 0 {
            let loss_val = loss.to_vec0::<f32>()?;
            println!("  Epoch {:3}/{}  Loss: {:.6}", epoch, epochs, loss_val);
        }
    }

    println!("  🎉 학습 완료!");
    Ok(())
}

/// 다양한 패턴 테스트
fn test_patterns(model: &SimpleRNN) -> Result<()> {
    let device = Device::Cpu;

    let test_cases = vec![
        ("상승 패턴 (0°~90°)", 0.0, 10.0),
        ("하강 패턴 (90°~180°)", 90.0, 10.0),
        ("정점 부근 (80°~170°)", 80.0, 10.0),
    ];

    for (label, start_deg, step_deg) in test_cases {
        let mut seq = Vec::new();

        for i in 0..10 {
            let angle = (start_deg + (i as f32) * step_deg) * PI / 180.0;
            seq.push(angle.sin());
        }

        let x = Tensor::from_vec(seq.clone(), &[1, 10, 1], &device)?;
        let pred = model.forward(&x)?;
        let pred_val = pred.i((0, 0))?.to_scalar::<f32>()?;

        let next_angle = (start_deg + 10.0 * step_deg) * PI / 180.0;
        let expected = next_angle.sin();

        println!("  {} → 예측: {:.4}, 정답: {:.4}", label, pred_val, expected);
    }

    Ok(())
}
