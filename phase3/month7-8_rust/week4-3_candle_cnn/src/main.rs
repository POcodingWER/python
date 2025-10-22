// Week 4-3: CNN 합성곱 신경망(Convolutional Neural Network) 🧠
// MNIST 손글씨 숫자 인식 모델
//
// 🔥 CNN 구조:
// 1. Conv2D (합성곱 레이어) - 이미지 특징 추출
// 2. MaxPool2D (풀링) - 크기 축소
// 3. Flatten - 1차원으로 변환
// 4. Linear (Dense) - 분류
//
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{
    conv2d, linear, loss, AdamW, Conv2dConfig, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🧠 CNN: MNIST 손글씨 숫자 인식");
    println!("{}", "=".repeat(60));

    // 1. 간단한 데이터 생성 (실제로는 MNIST 데이터셋 사용)
    println!("\n[1] 학습 데이터 생성");
    let (train_images, train_labels) = generate_dummy_mnist(100)?;
    println!("  📊 이미지 100개 생성! (28x28 픽셀)");
    println!("  💡 패턴: 0~9까지 실제 숫자 모양!");

    // 2. CNN 모델 생성
    println!("\n[2] CNN 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = SimpleCNN::new(vb)?;
    println!("  🧠 CNN 모델 생성 완료!");
    println!("  구조: Conv2D → MaxPool → Flatten → Linear");

    // 3. 학습 전 예측
    println!("\n[3] 학습 전 예측 (랜덤 가중치)");
    let test_image = generate_single_image(5, &device)?;
    let pred_before = model.forward(&test_image)?;
    let class_before = argmax(&pred_before)?;
    println!("  예측 클래스: {} (랜덤이라 이상함)", class_before);

    // 4. 학습!
    println!("\n[4] 학습 시작! 🔥");
    train_cnn(&model, &varmap, &train_images, &train_labels, 50)?;

    // 5. 학습 후 예측
    println!("\n[5] 학습 후 예측 ✨");
    let test_cases = vec![
        ("숫자 0 (원형)", 0),
        ("숫자 2 (Z자)", 2),
        ("숫자 5 (S자)", 5),
        ("숫자 7 (ㄱ자)", 7),
        ("숫자 9 (d자)", 9),
    ];

    for (label, target) in test_cases {
        let test_img = generate_single_image(target, &device)?;
        let prediction = model.forward(&test_img)?;
        let predicted_class = argmax(&prediction)?;
        let correct = if predicted_class == target {
            "✅"
        } else {
            "❌"
        };
        println!(
            "  {} (정답: {}) → 예측: {} {}",
            label, target, predicted_class, correct
        );
    }

    // 6. 모델 저장
    println!("\n[6] 모델 저장");
    varmap.save("cnn_model.safetensors")?;
    println!("  💾 모델 저장 완료: cnn_model.safetensors");

    println!("\n{}", "=".repeat(60));
    println!("✅ CNN 학습 완료! 이미지 분류 가능!");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 간단한 CNN 모델 구조
struct SimpleCNN {
    conv1: candle_nn::Conv2d, // 합성곱 레이어
    fc1: candle_nn::Linear,   // 완전 연결 레이어
}

impl SimpleCNN {
    fn new(vb: VarBuilder) -> Result<Self> {
        // Conv2D: 입력 1채널 → 출력 8채널, 커널 3x3
        let conv_config = Conv2dConfig {
            padding: 1,
            stride: 1,
            dilation: 1,
            groups: 1,
        };
        let conv1 = conv2d(1, 8, 3, conv_config, vb.pp("conv1"))?;

        // Linear: 8 * 14 * 14 → 10 (숫자 0~9)
        // MaxPool로 28x28 → 14x14로 축소됨
        let fc1 = linear(8 * 14 * 14, 10, vb.pp("fc1"))?;

        Ok(Self { conv1, fc1 })
    }
}

impl Module for SimpleCNN {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // 1. Conv2D
        let x = self.conv1.forward(x)?;

        // 2. ReLU 활성화
        let x = x.relu()?;

        // 3. MaxPool2D (2x2) - 수동 구현
        let x = maxpool2d(&x, 2)?;

        // 4. Flatten
        let batch_size = x.dims()[0];
        let x = x.reshape(&[batch_size, 8 * 14 * 14])?;

        // 5. Linear (Dense)
        let x = self.fc1.forward(&x)?;

        Ok(x)
    }
}

/// MaxPool2D 구현 (간단 버전)
fn maxpool2d(x: &Tensor, kernel_size: usize) -> Result<Tensor> {
    // [batch, channels, height, width]
    let dims = x.dims();
    let (batch, channels, h, w) = (dims[0], dims[1], dims[2], dims[3]);

    let new_h = h / kernel_size;
    let new_w = w / kernel_size;

    let mut result = Vec::new();

    for b in 0..batch {
        for c in 0..channels {
            for i in 0..new_h {
                for j in 0..new_w {
                    // 2x2 영역에서 최댓값 찾기
                    let y_start = i * kernel_size;
                    let x_start = j * kernel_size;

                    let mut max_val = f32::NEG_INFINITY;

                    for dy in 0..kernel_size {
                        for dx in 0..kernel_size {
                            let slice = x
                                .i((b, c, y_start + dy, x_start + dx))?
                                .to_scalar::<f32>()?;
                            if slice > max_val {
                                max_val = slice;
                            }
                        }
                    }

                    result.push(max_val);
                }
            }
        }
    }

    let device = x.device();
    Tensor::from_vec(result, &[batch, channels, new_h, new_w], device)
}

/// Argmax: 가장 큰 값의 인덱스 반환
fn argmax(tensor: &Tensor) -> Result<usize> {
    let values = tensor.to_vec2::<f32>()?;
    let mut max_idx = 0;
    let mut max_val = f32::NEG_INFINITY;

    for (i, &val) in values[0].iter().enumerate() {
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
    }

    Ok(max_idx)
}

/// 더미 MNIST 데이터 생성 (패턴 있는 데이터!)
fn generate_dummy_mnist(n: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut rng = rand::rng();

    let mut images = Vec::new();
    let mut labels = Vec::new();

    for _ in 0..n {
        // 레이블 먼저 선택 (0~9)
        let label = rng.random_range(0..10);

        // 숫자 모양 그리기! (노이즈 없이 순수 0/1)
        let digit_image = draw_digit(label);

        // 그대로 추가 (0.0 또는 1.0만!)
        for pixel in digit_image {
            images.push(pixel);
        }

        // One-hot encoding
        for i in 0..10 {
            labels.push(if i == label { 1.0f32 } else { 0.0f32 });
        }
    }

    // 🔢 첫 번째 이미지 배열 데이터 출력
    println!("\n  🔢 첫 번째 이미지 배열 (images[0..784]):");
    println!("  {:?}", &images[0..784]);

    // [batch, channels, height, width]
    let x = Tensor::from_vec(images, &[n, 1, 28, 28], &device)?;

    // [batch, 10] - One-hot encoded
    let y = Tensor::from_vec(labels, &[n, 10], &device)?;

    println!("  📊 이미지 500개 생성! (28x28 픽셀)");
    Ok((x, y))
}

/// 숫자 모양 그리기 (28x28) - 순수 0/1
fn draw_digit(digit: usize) -> Vec<f32> {
    let mut img = vec![0.0f32; 28 * 28];

    match digit {
        0 => {
            // 0: 원형
            for y in 8..20 {
                for x in 10..18 {
                    let dy = (y as f32 - 14.0).abs();
                    let dx = (x as f32 - 14.0).abs();
                    if dy + dx > 3.0 && dy + dx < 8.0 {
                        img[y * 28 + x] = 1.0;
                    }
                }
            }
        }
        1 => {
            // 1: 세로줄
            for y in 6..22 {
                img[y * 28 + 14] = 1.0;
                img[y * 28 + 13] = 1.0;
            }
        }
        2 => {
            // 2: ㄱ자 위아래
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 중간
            for x in 10..18 {
                img[20 * 28 + x] = 1.0;
            } // 아래
            for y in 8..14 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽 위
            for y in 14..20 {
                img[y * 28 + 10] = 1.0;
            } // 왼쪽 아래
        }
        3 => {
            // 3: E 모양
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 중간
            for x in 10..18 {
                img[20 * 28 + x] = 1.0;
            } // 아래
            for y in 8..20 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽
        }
        4 => {
            // 4: ㅏ 모양
            for y in 8..14 {
                img[y * 28 + 10] = 1.0;
            } // 왼쪽
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 가로
            for y in 8..20 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽
        }
        5 => {
            // 5: S 모양
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 중간
            for x in 10..18 {
                img[20 * 28 + x] = 1.0;
            } // 아래
            for y in 8..14 {
                img[y * 28 + 10] = 1.0;
            } // 왼쪽 위
            for y in 14..20 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽 아래
        }
        6 => {
            // 6: b 모양
            for y in 8..20 {
                img[y * 28 + 10] = 1.0;
            } // 왼쪽
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 중간
            for x in 10..18 {
                img[20 * 28 + x] = 1.0;
            } // 아래
            for y in 14..20 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽 아래
        }
        7 => {
            // 7: ㄱ자
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for y in 8..20 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽
        }
        8 => {
            // 8: 8자 (위아래 원)
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 중간
            for x in 10..18 {
                img[20 * 28 + x] = 1.0;
            } // 아래
            for y in 8..14 {
                img[y * 28 + 10] = 1.0;
                img[y * 28 + 17] = 1.0;
            } // 위 양옆
            for y in 14..20 {
                img[y * 28 + 10] = 1.0;
                img[y * 28 + 17] = 1.0;
            } // 아래 양옆
        }
        9 => {
            // 9: d 모양
            for y in 8..20 {
                img[y * 28 + 17] = 1.0;
            } // 오른쪽
            for x in 10..18 {
                img[8 * 28 + x] = 1.0;
            } // 위
            for x in 10..18 {
                img[14 * 28 + x] = 1.0;
            } // 중간
            for x in 10..18 {
                img[20 * 28 + x] = 1.0;
            } // 아래
            for y in 8..14 {
                img[y * 28 + 10] = 1.0;
            } // 왼쪽 위
        }
        _ => {}
    }

    img
}

/// 특정 숫자의 이미지 생성 (테스트용)
fn generate_single_image(target: usize, device: &Device) -> Result<Tensor> {
    let digit_image = draw_digit(target);
    Tensor::from_vec(digit_image, &[1, 1, 28, 28], device)
}

/// CNN 학습 함수
fn train_cnn(
    model: &impl Module,
    varmap: &VarMap,
    train_x: &Tensor,
    train_y: &Tensor,
    epochs: usize,
) -> Result<()> {
    let params = ParamsAdamW {
        lr: 0.01, // 학습률 높임!
        ..Default::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), params)?;

    for epoch in 0..epochs {
        // Forward pass
        let predictions = model.forward(train_x)?;

        // Softmax Cross Entropy Loss (간단 버전: MSE 사용)
        // 실제로는 CrossEntropyLoss 사용해야 함
        let loss = loss::mse(&predictions, train_y)?;

        // Backward pass
        optimizer.backward_step(&loss)?;

        // 5 epoch마다 출력
        if epoch % 5 == 0 {
            let loss_val = loss.to_vec0::<f32>()?;
            println!("  Epoch {:3}/{}  Loss: {:.4}", epoch, epochs, loss_val);
        }
    }

    println!("  🎉 학습 완료!");
    Ok(())
}
