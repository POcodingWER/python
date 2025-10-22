// Week 4 심화: 공부시간 → 시험점수 예측 모델 🎓
// 실제 데이터로 학습하는 신경망!
//
// 🧠 신경망 NN 모듈 사용 위치:
// 1. linear()      - Linear 레이어 (신경망 구조)
// 2. VarMap        - 가중치 저장소
// 3. VarBuilder    - 가중치 빌더
// 4. Module        - 신경망 인터페이스 (forward 메서드)
// 5. AdamW         - Optimizer (학습 최적화)
// 6. loss::mse()   - Loss 함수 (오차 계산)

use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{
    linear,      // ← NN 모듈! (Linear 레이어)
    loss,        // ← NN 모듈! (Loss 함수)
    AdamW,       // ← NN 모듈! (Optimizer)
    Module,      // ← NN 모듈! (신경망 인터페이스)
    Optimizer,   // ← NN 모듈! (학습 최적화)
    ParamsAdamW, // ← NN 모듈! (Adam 설정)
    VarBuilder,  // ← NN 모듈! (가중치 관리)
    VarMap,      // ← NN 모듈! (가중치 저장소)
};
use rand::Rng;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🎓 공부시간 → 시험점수 예측 모델");
    println!("{}", "=".repeat(60));

    // 1. 데이터 생성
    println!("\n[1] 학습 데이터 생성");
    let (train_x, train_y) = generate_study_data(100)?;

    println!("  📊 학생 100명 데이터 생성!");
    let sample_x = train_x.to_vec2::<f32>()?[0][0];
    let sample_y = train_y.to_vec2::<f32>()?[0][0];
    println!("  예시: 공부 {:.1}시간 → 점수 {:.1}점", sample_x, sample_y);

    // 2. 모델 생성
    println!("\n[2] AI 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new(); // ← NN 모듈! 가중치 저장소
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device); // ← NN 모듈! 가중치 빌더

    let model = linear(1, 1, vb.pp("model"))?; // ← NN 모듈! Linear 레이어
    println!("  🧠 Linear Regression 모델 생성 완료!");

    // 3. 학습 전 예측 (랜덤 가중치)
    println!("\n[3] 학습 전 예측 (랜덤 초기화)");
    let test_hours = Tensor::new(&[[5.0f32]], &device)?;
    let pred_before = model.forward(&test_hours)?; // ← NN 모듈! Module trait의 forward
    println!("  입력: 5시간 공부");
    println!(
        "  예측: {:.1}점 (랜덤이라 이상함)",
        pred_before.to_vec2::<f32>()?[0][0]
    );

    // 4. 학습!
    println!("\n[4] 학습 시작! 🔥");
    train_model(&model, &varmap, &train_x, &train_y, 200)?;

    // 5. 학습 후 예측
    println!("\n[5] 학습 후 예측 ✨");
    let test_cases = vec![
        ("1시간", 1.0),
        ("3시간", 3.0),
        ("5시간", 5.0),
        ("7시간", 7.0),
        ("10시간", 10.0),
    ];

    for (label, hours) in test_cases {
        let input = Tensor::new(&[[hours as f32]], &device)?;
        let prediction = model.forward(&input)?;
        let score = prediction.to_vec2::<f32>()?[0][0].clamp(0.0, 100.0); // 0~100점 제한!
        println!("  공부 {} → 예상 점수: {:.1}점", label, score);
    }

    // 6. 모델 저장
    println!("\n[6] 모델 저장");
    varmap.save("study_model.safetensors")?;
    println!("  💾 모델 저장 완료: study_model.safetensors");

    println!("\n{}", "=".repeat(60));
    println!("✅ 학습 완료! 이제 공부시간으로 점수 예측 가능!");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 학습 데이터 생성: 공부시간 → 시험점수
/// 실제 패턴: 점수 = 공부시간 × 8 + 20 (+ 약간의 노이즈)
fn generate_study_data(n: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut rng = rand::rng(); // ✅ 새로운 방식!

    let mut study_hours = Vec::new();
    let mut scores = Vec::new();

    for _ in 0..n {
        // 공부시간: 0~10시간
        let hours: f32 = rng.random_range(0.0..10.0); // ✅ 새로운 방식!

        // 실제 패턴: 점수 = 8 × 시간 + 20
        // + 노이즈 (-5 ~ +5)
        let noise: f32 = rng.random_range(-5.0..5.0); // ✅ 새로운 방식!
        let score = (hours * 8.0 + 20.0 + noise).clamp(0.0, 100.0);

        study_hours.push(hours);
        scores.push(score);
    }

    let x = Tensor::from_vec(study_hours, &[n, 1], &device)?;
    let y = Tensor::from_vec(scores, &[n, 1], &device)?;

    Ok((x, y))
}

/// 모델 학습 함수
fn train_model(
    model: &impl Module, // ← NN 모듈! Module trait
    varmap: &VarMap,     // ← NN 모듈! 가중치 저장소
    train_x: &Tensor,
    train_y: &Tensor,
    epochs: usize,
) -> Result<()> {
    // Optimizer 설정 (Adam)
    let params = ParamsAdamW {
        // ← NN 모듈! Adam 파라미터
        lr: 0.1, // 학습률 증가
        ..Default::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), params)?; // ← NN 모듈! AdamW Optimizer

    // 학습 루프
    for epoch in 0..epochs {
        // Forward pass
        let predictions = model.forward(train_x)?; // ← NN 모듈! Module의 forward

        // Loss 계산 (MSE: Mean Squared Error)
        let loss = loss::mse(&predictions, train_y)?; // ← NN 모듈! Loss 함수

        // Backward pass (자동 미분)
        optimizer.backward_step(&loss)?; // ← NN 모듈! Optimizer의 학습

        // 10 epoch마다 출력
        if epoch % 10 == 0 {
            let loss_val = loss.to_vec0::<f32>()?;
            println!("  Epoch {:3}/{}  Loss: {:.2}", epoch, epochs, loss_val);
        }
    }

    println!("  🎉 학습 완료!");
    Ok(())
}
