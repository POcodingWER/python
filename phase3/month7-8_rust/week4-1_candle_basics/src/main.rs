// Week 4: Candle 기초 - Linear Regression 🕯️
// Hugging Face의 Rust ML 프레임워크!

use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{linear, Module, VarBuilder, VarMap};

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🕯️ Candle: Rust ML Framework (Hugging Face)");
    println!("{}", "=".repeat(60));

    // 1. 기본 텐서 연산
    println!("\n[1] 기본 텐서 연산");
    basic_tensor_ops()?;

    // 2. Linear Regression (수동)
    println!("\n[2] Linear Regression (수동 구현)");
    manual_linear_regression()?;

    // 3. Linear Regression (Candle NN)
    println!("\n[3] Linear Regression (Candle NN 모듈)");
    candle_linear_regression()?;

    println!("\n{}", "=".repeat(60));
    println!("✅ Week 4 Candle 기초 완료!");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 1. 기본 텐서 연산
fn basic_tensor_ops() -> Result<()> {
    let device = Device::Cpu;

    // 텐서 생성
    let a = Tensor::new(&[1.0f32, 2.0, 3.0], &device)?;
    let b = Tensor::new(&[4.0f32, 5.0, 6.0], &device)?;

    println!("  a = {:?}", a.to_vec1::<f32>()?);
    println!("  b = {:?}", b.to_vec1::<f32>()?);

    // 덧셈
    let c = (&a + &b)?;
    println!("  a + b = {:?}", c.to_vec1::<f32>()?);

    // 곱셈
    let d = (&a * &b)?;
    println!("  a * b = {:?}", d.to_vec1::<f32>()?);

    // 내적
    let a_row = a.reshape((1, 3))?;
    let b_col = b.reshape((3, 1))?;
    let e = a_row.matmul(&b_col)?;
    let result = e.to_vec2::<f32>()?[0][0];
    println!("  dot(a, b) = {}", result);

    Ok(())
}

/// 2. Linear Regression (수동 구현)
fn manual_linear_regression() -> Result<()> {
    let device = Device::Cpu;

    // y = 2x + 1 모델
    let weight = Tensor::new(&[[2.0f32]], &device)?;
    let bias = Tensor::new(&[1.0f32], &device)?;

    // 입력 데이터
    let x = Tensor::new(&[[1.0f32], [2.0], [3.0], [4.0], [5.0]], &device)?;

    // 예측: y = Wx + b
    let y_pred = x.matmul(&weight.t()?)?.broadcast_add(&bias)?;

    println!("  모델: y = 2x + 1");
    println!("  입력 x: {:?}", x.to_vec2::<f32>()?);
    println!("  예측 y: {:?}", y_pred.to_vec2::<f32>()?);

    Ok(())
}

/// 3. Linear Regression (Candle NN 모듈)
fn candle_linear_regression() -> Result<()> {
    let device = Device::Cpu;

    // VarMap & VarBuilder 생성
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    // Linear 레이어 생성 (1 입력 → 1 출력)
    let linear_layer = linear(1, 1, vb.pp("linear"))?;

    println!("  Candle NN Linear 모듈 생성 완료!");

    // 입력 데이터
    let x = Tensor::new(&[[1.0f32], [2.0], [3.0], [4.0], [5.0]], &device)?;

    // Forward pass
    let y_pred = linear_layer.forward(&x)?;

    println!("  입력 x: {:?}", x.to_vec2::<f32>()?);
    println!("  예측 y: {:?}", y_pred.to_vec2::<f32>()?);
    println!("  ℹ️ 랜덤 초기화되어 예측값이 매번 다름");

    Ok(())
}
