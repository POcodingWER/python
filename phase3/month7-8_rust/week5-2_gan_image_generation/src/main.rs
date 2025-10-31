use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{ops, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use rand::Rng;
use std::fs;

// ============================================
// Binary Cross Entropy Loss 구현
// ============================================
fn binary_cross_entropy(pred: &Tensor, target: &Tensor) -> Result<Tensor> {
    // BCE = -[y*log(p) + (1-y)*log(1-p)]
    let eps = 1e-7; // 수치 안정성을 위한 작은 값

    let pred_clamped = pred.clamp(eps, 1.0 - eps)?;
    let log_pred = pred_clamped.log()?;
    let one_minus_pred = (1.0 - &pred_clamped)?;
    let log_one_minus_pred = one_minus_pred.log()?;

    let one_minus_target = (1.0 - target)?;

    let loss1 = (target * log_pred)?;
    let loss2 = (one_minus_target * log_one_minus_pred)?;
    let loss = (loss1 + loss2)?;
    let loss = loss.neg()?;

    loss.mean_all()
}

// ============================================
// Generator: 노이즈 + 레이블 → 이미지 생성 (Conditional GAN)
// ============================================
struct Generator {
    fc1: Linear,
    fc2: Linear,
    fc3: Linear,
}

impl Generator {
    fn new(
        vb: VarBuilder,
        latent_dim: usize,
        num_classes: usize, // 레이블 개수 (0~9 = 10개)
        hidden_dim: usize,
        output_dim: usize,
    ) -> Result<Self> {
        // 노이즈 + 레이블을 함께 입력
        let input_dim = latent_dim + num_classes;
        let fc1 = candle_nn::linear(input_dim, hidden_dim * 2, vb.pp("fc1"))?;
        let fc2 = candle_nn::linear(hidden_dim * 2, hidden_dim * 4, vb.pp("fc2"))?;
        let fc3 = candle_nn::linear(hidden_dim * 4, output_dim, vb.pp("fc3"))?;

        Ok(Self { fc1, fc2, fc3 })
    }

    fn forward(&self, noise: &Tensor, label: &Tensor) -> Result<Tensor> {
        // noise (batch, latent_dim) + label (batch, num_classes) → 이미지 (batch, 28*28)
        let x = Tensor::cat(&[noise, label], 1)?; // 노이즈와 레이블 결합

        let x = self.fc1.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        let x = self.fc2.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        let x = self.fc3.forward(&x)?;
        let x = x.tanh()?; // -1 ~ 1 범위로 정규화

        Ok(x)
    }
}

// ============================================
// Discriminator: 이미지 + 레이블 → 진짜/가짜 판별 (Conditional GAN)
// ============================================
struct Discriminator {
    fc1: Linear,
    fc2: Linear,
    fc3: Linear,
}

impl Discriminator {
    fn new(
        vb: VarBuilder,
        input_dim: usize,
        num_classes: usize,
        hidden_dim: usize,
    ) -> Result<Self> {
        // 이미지 + 레이블을 함께 입력
        let total_input = input_dim + num_classes;
        let fc1 = candle_nn::linear(total_input, hidden_dim * 2, vb.pp("fc1"))?;
        let fc2 = candle_nn::linear(hidden_dim * 2, hidden_dim, vb.pp("fc2"))?;
        let fc3 = candle_nn::linear(hidden_dim, 1, vb.pp("fc3"))?;

        Ok(Self { fc1, fc2, fc3 })
    }

    fn forward(&self, image: &Tensor, label: &Tensor) -> Result<Tensor> {
        // 이미지 (batch, 28*28) + label (batch, num_classes) → 확률 (batch, 1)
        let x = Tensor::cat(&[image, label], 1)?; // 이미지와 레이블 결합

        let x = self.fc1.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        let x = self.fc2.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        let x = self.fc3.forward(&x)?;
        let x = candle_nn::ops::sigmoid(&x)?; // 0 ~ 1 확률

        Ok(x)
    }
}

// ============================================
// 합성 MNIST 스타일 데이터 생성
// ============================================
fn generate_mnist_style_data(
    num_samples: usize,
    target_digit: usize,
    device: &Device,
) -> Result<Tensor> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for _ in 0..num_samples {
        let digit = target_digit; // 지정된 숫자만 생성!
        let mut image = vec![0.0f32; 28 * 28];

        match digit {
            0 => {
                // 원 그리기
                for i in 8..20 {
                    for j in 8..20 {
                        let dx = (i as f32 - 14.0).abs();
                        let dy = (j as f32 - 14.0).abs();
                        if dx * dx + dy * dy < 36.0 && dx * dx + dy * dy > 16.0 {
                            image[i * 28 + j] = 1.0;
                        }
                    }
                }
            }
            1 => {
                // 세로선
                for i in 6..22 {
                    image[i * 28 + 14] = 1.0;
                    image[i * 28 + 13] = 1.0;
                }
            }
            2 => {
                // 2 모양
                for j in 8..20 {
                    image[8 * 28 + j] = 1.0;
                    image[14 * 28 + j] = 1.0;
                    image[20 * 28 + j] = 1.0;
                }
                for i in 8..14 {
                    image[i * 28 + 19] = 1.0;
                }
                for i in 14..20 {
                    image[i * 28 + 8] = 1.0;
                }
            }
            3 => {
                // 3 모양
                for j in 8..20 {
                    image[8 * 28 + j] = 1.0;
                    image[14 * 28 + j] = 1.0;
                    image[20 * 28 + j] = 1.0;
                }
                for i in 8..20 {
                    image[i * 28 + 19] = 1.0;
                }
            }
            4 => {
                // 4 모양
                for i in 6..14 {
                    image[i * 28 + 8] = 1.0;
                }
                for j in 8..20 {
                    image[14 * 28 + j] = 1.0;
                }
                for i in 6..22 {
                    image[i * 28 + 19] = 1.0;
                }
            }
            5 => {
                // 5 모양
                for j in 8..20 {
                    image[8 * 28 + j] = 1.0;
                    image[14 * 28 + j] = 1.0;
                    image[20 * 28 + j] = 1.0;
                }
                for i in 8..14 {
                    image[i * 28 + 8] = 1.0;
                }
                for i in 14..20 {
                    image[i * 28 + 19] = 1.0;
                }
            }
            6 => {
                // 6 모양
                for j in 8..20 {
                    image[8 * 28 + j] = 1.0;
                    image[14 * 28 + j] = 1.0;
                    image[20 * 28 + j] = 1.0;
                }
                for i in 8..20 {
                    image[i * 28 + 8] = 1.0;
                }
                for i in 14..20 {
                    image[i * 28 + 19] = 1.0;
                }
            }
            7 => {
                // 7 모양
                for j in 8..20 {
                    image[8 * 28 + j] = 1.0;
                }
                for i in 8..22 {
                    image[i * 28 + 19] = 1.0;
                }
            }
            8 => {
                // 8 모양 (두 개의 원)
                for i in 8..20 {
                    for j in 8..20 {
                        let dx1 = (i as f32 - 11.0).abs();
                        let dy1 = (j as f32 - 14.0).abs();
                        let dx2 = (i as f32 - 17.0).abs();
                        let dy2 = (j as f32 - 14.0).abs();
                        if (dx1 * dx1 + dy1 * dy1 < 16.0 && dx1 * dx1 + dy1 * dy1 > 6.0)
                            || (dx2 * dx2 + dy2 * dy2 < 16.0 && dx2 * dx2 + dy2 * dy2 > 6.0)
                        {
                            image[i * 28 + j] = 1.0;
                        }
                    }
                }
            }
            9 => {
                // 9 모양
                for j in 8..20 {
                    image[8 * 28 + j] = 1.0;
                    image[14 * 28 + j] = 1.0;
                    image[20 * 28 + j] = 1.0;
                }
                for i in 8..14 {
                    image[i * 28 + 8] = 1.0;
                }
                for i in 8..20 {
                    image[i * 28 + 19] = 1.0;
                }
            }
            _ => {}
        }

        // 노이즈 추가
        for pixel in image.iter_mut() {
            if rng.gen::<f32>() < 0.05 {
                *pixel = rng.gen::<f32>();
            }
        }

        data.extend(image);
    }

    // -1 ~ 1 범위로 정규화 (GAN에서 tanh 사용)
    let data: Vec<f32> = data.iter().map(|&x| x * 2.0 - 1.0).collect();

    Tensor::from_vec(data, (num_samples, 28 * 28), device)
}

// ============================================
// 이미지 저장 함수 (Conditional GAN)
// ============================================
fn save_generated_images(
    generator: &Generator,
    epoch: usize,
    latent_dim: usize,
    target_digit: usize,
    device: &Device,
) -> Result<()> {
    // 10개의 이미지 생성
    let noise = Tensor::randn(0f32, 1f32, (10, latent_dim), device)?;

    // 레이블을 one-hot 인코딩
    let mut label_data = vec![0.0f32; 10 * 10]; // 10개 샘플 x 10개 클래스
    for i in 0..10 {
        label_data[i * 10 + target_digit] = 1.0;
    }
    let labels = Tensor::from_vec(label_data, (10, 10), device)?;

    let generated = generator.forward(&noise, &labels)?;

    // -1 ~ 1 → 0 ~ 255 변환
    let generated = ((generated + 1.0)? * 127.5)?;
    let generated = generated.clamp(0.0, 255.0)?;

    let generated_data = generated.to_vec2::<f32>()?;

    // 디렉토리 생성
    fs::create_dir_all("generated_images")?;

    // 각 이미지 저장
    for (idx, img_data) in generated_data.iter().enumerate() {
        let img_u8: Vec<u8> = img_data.iter().map(|&x| x as u8).collect();

        let img = image::GrayImage::from_raw(28, 28, img_u8)
            .ok_or_else(|| candle_core::Error::Msg("Failed to create image".to_string()))?;

        let filename = format!("generated_images/epoch_{:04}_img_{}.png", epoch, idx);
        img.save(&filename)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to save image: {}", e)))?;
    }

    println!("✅ Epoch {} 이미지 저장 완료!", epoch);
    Ok(())
}

// ============================================
// 이미지 시각화 (터미널) - Conditional GAN
// ============================================
fn visualize_generated_image(
    generator: &Generator,
    latent_dim: usize,
    target_digit: usize,
    device: &Device,
) -> Result<()> {
    let noise = Tensor::randn(0f32, 1f32, (1, latent_dim), device)?;

    // 레이블을 one-hot 인코딩
    let mut label_data = vec![0.0f32; 10];
    label_data[target_digit] = 1.0;
    let label = Tensor::from_vec(label_data, (1, 10), device)?;

    let generated = generator.forward(&noise, &label)?;

    let generated_data = generated.to_vec2::<f32>()?;
    let img_data = &generated_data[0];

    println!("\n🎨 생성된 이미지:");
    for i in 0..28 {
        for j in 0..28 {
            let pixel = img_data[i * 28 + j];
            // -1 ~ 1 범위
            if pixel > 0.3 {
                print!("██");
            } else if pixel > 0.0 {
                print!("▓▓");
            } else if pixel > -0.3 {
                print!("░░");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();

    Ok(())
}

// ============================================
// GAN 학습
// ============================================
fn train_gan(
    generator: &Generator,
    discriminator: &Discriminator,
    real_images: &Tensor,
    g_optimizer: &mut AdamW,
    d_optimizer: &mut AdamW,
    latent_dim: usize,
    batch_size: usize,
    target_digit: usize,
    device: &Device,
) -> Result<(f64, f64)> {
    let num_samples = real_images.dim(0)?;
    let num_batches = num_samples / batch_size;

    let mut g_loss_sum = 0.0;
    let mut d_loss_sum = 0.0;

    for batch_idx in 0..num_batches {
        let start = batch_idx * batch_size;

        // 실제 이미지 배치
        let real_batch = real_images.narrow(0, start, batch_size)?;

        // 레이블을 one-hot 인코딩 (배치 크기만큼)
        let mut label_data = vec![0.0f32; batch_size * 10];
        for i in 0..batch_size {
            label_data[i * 10 + target_digit] = 1.0;
        }
        let batch_labels = Tensor::from_vec(label_data, (batch_size, 10), device)?;

        // ==========================================
        // 1. Discriminator 학습
        // ==========================================

        // Label Smoothing: 진짜 = 0.9, 가짜 = 0.1
        let real_labels = (Tensor::ones((batch_size, 1), DType::F32, device)? * 0.9)?;
        let real_pred = discriminator.forward(&real_batch, &batch_labels)?;
        let d_real_loss = binary_cross_entropy(&real_pred, &real_labels)?;

        // 가짜 이미지 생성 → 0.1 (가짜)
        let noise = Tensor::randn(0f32, 1f32, (batch_size, latent_dim), device)?;
        let fake_images = generator.forward(&noise, &batch_labels)?;
        let fake_labels = (Tensor::zeros((batch_size, 1), DType::F32, device)? + 0.1)?;
        let fake_pred = discriminator.forward(&fake_images, &batch_labels)?;
        let d_fake_loss = binary_cross_entropy(&fake_pred, &fake_labels)?;

        // Discriminator 총 손실
        let d_loss = ((d_real_loss + d_fake_loss)? / 2.0)?;

        d_optimizer.backward_step(&d_loss)?;
        d_loss_sum += d_loss.to_vec0::<f32>()? as f64;

        // ==========================================
        // 2. Generator 학습 (2번 반복 - Discriminator보다 많이)
        // ==========================================

        for _ in 0..2 {
            // Generator는 Discriminator를 속이려고 함
            // 가짜 이미지를 진짜(1.0)로 판별하도록 유도
            let noise = Tensor::randn(0f32, 1f32, (batch_size, latent_dim), device)?;
            let fake_images = generator.forward(&noise, &batch_labels)?;
            let fake_pred = discriminator.forward(&fake_images, &batch_labels)?;

            // Generator 손실: Discriminator가 가짜를 진짜로 판별하도록
            let real_target = Tensor::ones((batch_size, 1), DType::F32, device)?;
            let g_loss = binary_cross_entropy(&fake_pred, &real_target)?;

            g_optimizer.backward_step(&g_loss)?;
            g_loss_sum += g_loss.to_vec0::<f32>()? as f64;
        }
    }

    Ok((
        g_loss_sum / (num_batches * 2) as f64, // Generator는 2번 학습
        d_loss_sum / num_batches as f64,
    ))
}

// ============================================
// 메인 함수
// ============================================
fn main() -> Result<()> {
    println!("🥊 GAN (Generative Adversarial Network) 시작!");
    println!("{}", "=".repeat(60));

    // ============================================
    // 🎯 설정: 여기만 수정하면 됩니다!
    // ============================================
    let target_digit = 4; // 👈 생성할 숫자 (0~9)

    // 하이퍼파라미터
    let latent_dim = 100; // 노이즈 차원
    let num_classes = 10; // 클래스 개수 (0~9)
    let hidden_dim = 256; // 은닉층 크기 증가
    let image_dim = 28 * 28; // 이미지 크기
    let batch_size = 32; // 배치 크기 감소 (더 안정적)
    let epochs = 200; // 에포크 200으로 최적화!
    let num_samples = 2000; // 학습 데이터 증가
    let g_learning_rate = 0.0002; // Generator 학습률
    let d_learning_rate = 0.00005; // Discriminator 학습률 (더 느리게)

    let device = Device::Cpu;

    println!("📊 설정:");
    println!(
        "  - Target Digit: {} (숫자 {}만 생성!)",
        target_digit, target_digit
    );
    println!("  - Latent Dimension: {}", latent_dim);
    println!("  - Hidden Dimension: {}", hidden_dim);
    println!("  - Batch Size: {}", batch_size);
    println!("  - Epochs: {}", epochs);
    println!("  - Generator Learning Rate: {}", g_learning_rate);
    println!("  - Discriminator Learning Rate: {}", d_learning_rate);
    println!("  - Training Samples: {}", num_samples);
    println!();

    // 데이터 생성
    println!("🎲 합성 MNIST 데이터 생성 중 (숫자 {}만)...", target_digit);
    let real_images = generate_mnist_style_data(num_samples, target_digit, &device)?;
    println!(
        "✅ {} 개의 숫자 {} 이미지 생성 완료!",
        num_samples, target_digit
    );
    println!();

    // Generator 초기화 (Conditional GAN)
    let g_varmap = VarMap::new();
    let g_vb = VarBuilder::from_varmap(&g_varmap, DType::F32, &device);
    let generator = Generator::new(
        g_vb.pp("generator"),
        latent_dim,
        num_classes,
        hidden_dim,
        image_dim,
    )?;
    println!("✅ Generator 초기화 완료! (Conditional GAN)");

    // Discriminator 초기화 (Conditional GAN)
    let d_varmap = VarMap::new();
    let d_vb = VarBuilder::from_varmap(&d_varmap, DType::F32, &device);
    let discriminator =
        Discriminator::new(d_vb.pp("discriminator"), image_dim, num_classes, hidden_dim)?;
    println!("✅ Discriminator 초기화 완료! (Conditional GAN)");
    println!();

    // Optimizer 초기화 (서로 다른 학습률)
    let g_params = ParamsAdamW {
        lr: g_learning_rate,
        ..Default::default()
    };
    let mut g_optimizer = AdamW::new(g_varmap.all_vars(), g_params)?;

    let d_params = ParamsAdamW {
        lr: d_learning_rate, // Discriminator는 더 느리게
        ..Default::default()
    };
    let mut d_optimizer = AdamW::new(d_varmap.all_vars(), d_params)?;

    println!("🏋️ GAN 학습 시작!");
    println!("{}", "=".repeat(60));

    for epoch in 1..=epochs {
        let (g_loss, d_loss) = train_gan(
            &generator,
            &discriminator,
            &real_images,
            &mut g_optimizer,
            &mut d_optimizer,
            latent_dim,
            batch_size,
            target_digit,
            &device,
        )?;

        println!(
            "Epoch [{:3}/{}] | G_Loss: {:.4} | D_Loss: {:.4}",
            epoch, epochs, g_loss, d_loss
        );

        // 10 에포크마다 이미지 생성 및 저장
        if epoch % 10 == 0 {
            visualize_generated_image(&generator, latent_dim, target_digit, &device)?;
            save_generated_images(&generator, epoch, latent_dim, target_digit, &device)?;
        }
    }

    println!();
    println!("{}", "=".repeat(60));
    println!("🎉 GAN 학습 완료!");
    println!("📁 생성된 이미지: generated_images/ 폴더 확인");

    // 최종 이미지 생성
    println!();
    println!("🎨 최종 생성 이미지 (10개):");
    save_generated_images(&generator, epochs, latent_dim, target_digit, &device)?;

    // 모델 저장
    println!();
    println!("💾 모델 저장 중...");
    g_varmap.save("generator_model.safetensors")?;
    d_varmap.save("discriminator_model.safetensors")?;
    println!("✅ 모델 저장 완료!");
    println!("  - generator_model.safetensors");
    println!("  - discriminator_model.safetensors");

    Ok(())
}
