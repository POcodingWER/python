use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{linear, ops, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

// ============================================
// Binary Cross Entropy Loss
// ============================================
fn binary_cross_entropy(pred: &Tensor, target: &Tensor) -> Result<Tensor> {
    let pred = pred.clamp(1e-7f32, 1.0 - 1e-7f32)?;
    let log_pred = pred.log()?;
    let one_minus_pred = (pred.neg()? + 1.0)?;
    let log_one_minus_pred = one_minus_pred.log()?;

    let loss1 = target.mul(&log_pred)?;
    let loss2 = (target.neg()? + 1.0)?.mul(&log_one_minus_pred)?;
    let loss = (loss1 + loss2)?.neg()?;

    loss.mean_all()
}

// ============================================
// DCGAN-style Generator (Fully Connected, 안정적!)
// ============================================
struct DCGANGenerator {
    fc1: Linear,
    fc2: Linear,
    fc3: Linear,
    fc4: Linear,
}

impl DCGANGenerator {
    fn new(vb: VarBuilder, latent_dim: usize, num_classes: usize) -> Result<Self> {
        let input_dim = latent_dim + num_classes;

        // Week 5-2 성공 구조 복원!
        let fc1 = linear(input_dim, 128, vb.pp("fc1"))?;
        let fc2 = linear(128, 256, vb.pp("fc2"))?;
        let fc3 = linear(256, 512, vb.pp("fc3"))?;
        let fc4 = linear(512, 28 * 28, vb.pp("fc4"))?;

        Ok(Self { fc1, fc2, fc3, fc4 })
    }

    fn forward(&self, noise: &Tensor, label: &Tensor) -> Result<Tensor> {
        // Concatenate noise and label
        let x = Tensor::cat(&[noise, label], 1)?;

        // Layer 1
        let x = self.fc1.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        // Layer 2
        let x = self.fc2.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        // Layer 3
        let x = self.fc3.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        // Output layer
        let x = self.fc4.forward(&x)?;
        let x = x.tanh()?;

        // Reshape to (batch, 1, 28, 28)
        let batch_size = x.dim(0)?;
        x.reshape((batch_size, 1, 28, 28))
    }
}

// ============================================
// DCGAN-style Discriminator (Fully Connected, 안정적!)
// ============================================
struct DCGANDiscriminator {
    fc1: Linear,
    fc2: Linear,
    fc3: Linear,
}

impl DCGANDiscriminator {
    fn new(vb: VarBuilder, num_classes: usize) -> Result<Self> {
        let input_dim = 28 * 28 + num_classes;

        // Week 5-2 성공 구조 복원!
        let fc1 = linear(input_dim, 512, vb.pp("fc1"))?;
        let fc2 = linear(512, 256, vb.pp("fc2"))?;
        let fc3 = linear(256, 1, vb.pp("fc3"))?;

        Ok(Self { fc1, fc2, fc3 })
    }

    fn forward(&self, image: &Tensor, label: &Tensor) -> Result<Tensor> {
        // Flatten image
        let x = image.flatten_from(1)?;

        // Concatenate image and label
        let x = Tensor::cat(&[&x, label], 1)?;

        // Layer 1
        let x = self.fc1.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        // Layer 2
        let x = self.fc2.forward(&x)?;
        let x = ops::leaky_relu(&x, 0.2)?;

        // Output layer (fc3만 사용!)
        let x = self.fc3.forward(&x)?;
        let x = ops::sigmoid(&x)?;

        Ok(x)
    }
}

// ============================================
// MNIST 데이터 다운로드 및 로딩
// ============================================
fn download_mnist() -> Result<()> {
    let data_dir = "mnist_data";
    fs::create_dir_all(data_dir)?;

    let base_url = "https://ossci-datasets.s3.amazonaws.com/mnist/";
    let files = ["train-images-idx3-ubyte.gz", "train-labels-idx1-ubyte.gz"];

    println!("📥 MNIST 데이터 확인 중...");

    for file in &files {
        let path = format!("{}/{}", data_dir, file);

        // 파일이 이미 있으면 스킵!
        if Path::new(&path).exists() {
            println!("  ✅ {} 이미 존재 (다운로드 스킵)", file);
            continue;
        }

        println!("  ⬇️  {} 다운로드 중...", file);
        let url = format!("{}{}", base_url, file);

        let response = reqwest::blocking::get(&url)
            .map_err(|e| candle_core::Error::Msg(format!("다운로드 실패: {}", e)))?;

        if !response.status().is_success() {
            return Err(candle_core::Error::Msg(format!(
                "다운로드 실패: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .map_err(|e| candle_core::Error::Msg(format!("데이터 읽기 실패: {}", e)))?;

        let mut file_handle = File::create(&path)
            .map_err(|e| candle_core::Error::Msg(format!("파일 생성 실패: {}", e)))?;
        file_handle
            .write_all(&bytes)
            .map_err(|e| candle_core::Error::Msg(format!("파일 쓰기 실패: {}", e)))?;

        println!("  ✅ {} 다운로드 완료! ({}KB)", file, bytes.len() / 1024);
    }

    println!("✅ MNIST 데이터 준비 완료!");
    Ok(())
}

fn load_mnist_images(path: &str) -> Result<Vec<u8>> {
    let file =
        File::open(path).map_err(|e| candle_core::Error::Msg(format!("파일 열기 실패: {}", e)))?;
    let mut gz = GzDecoder::new(file);
    let mut data = Vec::new();
    gz.read_to_end(&mut data)
        .map_err(|e| candle_core::Error::Msg(format!("압축 해제 실패: {}", e)))?;

    // Skip header (16 bytes)
    Ok(data[16..].to_vec())
}

fn load_mnist_labels(path: &str) -> Result<Vec<u8>> {
    let file =
        File::open(path).map_err(|e| candle_core::Error::Msg(format!("파일 열기 실패: {}", e)))?;
    let mut gz = GzDecoder::new(file);
    let mut data = Vec::new();
    gz.read_to_end(&mut data)
        .map_err(|e| candle_core::Error::Msg(format!("압축 해제 실패: {}", e)))?;

    // Skip header (8 bytes)
    Ok(data[8..].to_vec())
}

fn prepare_mnist_data(target_digit: usize, device: &Device) -> Result<Tensor> {
    println!("📊 MNIST 데이터 로딩 중...");

    let images = load_mnist_images("mnist_data/train-images-idx3-ubyte.gz")?;
    let labels = load_mnist_labels("mnist_data/train-labels-idx1-ubyte.gz")?;

    // 특정 숫자만 필터링
    let mut filtered_images = Vec::new();
    for (i, &label) in labels.iter().enumerate() {
        if label as usize == target_digit {
            let start = i * 28 * 28;
            let end = start + 28 * 28;
            let image: Vec<f32> = images[start..end]
                .iter()
                .map(|&x| (x as f32 / 255.0) * 2.0 - 1.0) // -1 ~ 1로 정규화
                .collect();
            filtered_images.extend(image);
        }
    }

    let num_images = filtered_images.len() / (28 * 28);
    println!(
        "✅ 숫자 {} 이미지 {}개 로딩 완료!",
        target_digit, num_images
    );

    Tensor::from_vec(filtered_images, (num_images, 1, 28, 28), device)
}

// ============================================
// 이미지 저장
// ============================================
fn save_generated_images(
    generator: &DCGANGenerator,
    epoch: usize,
    latent_dim: usize,
    target_digit: usize,
    device: &Device,
) -> Result<()> {
    fs::create_dir_all("generated_images")?;

    let noise = Tensor::randn(0f32, 1f32, (10, latent_dim), device)?;

    let mut label_data = vec![0.0f32; 10 * 10];
    for i in 0..10 {
        label_data[i * 10 + target_digit] = 1.0;
    }
    let labels = Tensor::from_vec(label_data, (10, 10), device)?;

    let generated = generator.forward(&noise, &labels)?;

    // -1 ~ 1 → 0 ~ 255
    let generated = ((generated + 1.0)? * 127.5)?;

    for idx in 0..10 {
        let img_tensor = generated.get(idx)?;
        let img_data = img_tensor.flatten_all()?.to_vec1::<f32>()?;

        let mut img_buffer = image::GrayImage::new(28, 28);

        for i in 0..28 {
            for j in 0..28 {
                let pixel_idx = i * 28 + j;
                let pixel_value = img_data[pixel_idx].clamp(0.0, 255.0) as u8;
                img_buffer.put_pixel(j as u32, i as u32, image::Luma([pixel_value]));
            }
        }

        let filename = format!("generated_images/epoch_{:04}_img_{}.png", epoch, idx);
        img_buffer
            .save(&filename)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    }

    println!("✅ Epoch {} 이미지 저장 완료!", epoch);
    Ok(())
}

// ============================================
// 터미널 시각화
// ============================================
fn visualize_generated_image(
    generator: &DCGANGenerator,
    latent_dim: usize,
    target_digit: usize,
    device: &Device,
) -> Result<()> {
    let noise = Tensor::randn(0f32, 1f32, (1, latent_dim), device)?;

    let mut label_data = vec![0.0f32; 10];
    label_data[target_digit] = 1.0;
    let label = Tensor::from_vec(label_data, (1, 10), device)?;

    let generated = generator.forward(&noise, &label)?;
    let img_data = generated.get(0)?.flatten_all()?.to_vec1::<f32>()?;

    println!("\n🎨 생성된 이미지 (28x28):");
    for i in 0..28 {
        print!("  ");
        for j in 0..28 {
            let pixel_idx = i * 28 + j;
            let pixel = (img_data[pixel_idx] + 1.0) / 2.0;
            if pixel > 0.7 {
                print!("██");
            } else if pixel > 0.4 {
                print!("▓▓");
            } else if pixel > 0.2 {
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
// DCGAN 학습
// ============================================
fn train_dcgan(
    generator: &DCGANGenerator,
    discriminator: &DCGANDiscriminator,
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

        let real_batch = real_images.narrow(0, start, batch_size)?;

        let mut label_data = vec![0.0f32; batch_size * 10];
        for i in 0..batch_size {
            label_data[i * 10 + target_digit] = 1.0;
        }
        let batch_labels = Tensor::from_vec(label_data, (batch_size, 10), device)?;

        // Discriminator 학습
        let noise = Tensor::randn(0f32, 1f32, (batch_size, latent_dim), device)?;
        let fake_images = generator.forward(&noise, &batch_labels)?;

        let real_pred = discriminator.forward(&real_batch, &batch_labels)?;
        let fake_pred = discriminator.forward(&fake_images, &batch_labels)?;

        // DCGAN: Label Smoothing (0.9 for real, 0.0 for fake)
        let real_labels = Tensor::from_vec(vec![0.9f32; batch_size], (batch_size, 1), device)?;
        let fake_labels = Tensor::from_vec(vec![0.0f32; batch_size], (batch_size, 1), device)?;

        let d_loss_real = binary_cross_entropy(&real_pred, &real_labels)?;
        let d_loss_fake = binary_cross_entropy(&fake_pred, &fake_labels)?;
        let d_loss = ((d_loss_real + d_loss_fake)? / 2.0)?;

        d_optimizer.backward_step(&d_loss)?;
        d_loss_sum += d_loss.to_vec0::<f32>()? as f64;

        // Generator 학습 (DCGAN: 1번만!)
        let noise = Tensor::randn(0f32, 1f32, (batch_size, latent_dim), device)?;
        let fake_images = generator.forward(&noise, &batch_labels)?;
        let fake_pred = discriminator.forward(&fake_images, &batch_labels)?;

        let real_labels = Tensor::from_vec(vec![1.0f32; batch_size], (batch_size, 1), device)?;
        let g_loss = binary_cross_entropy(&fake_pred, &real_labels)?;

        g_optimizer.backward_step(&g_loss)?;
        g_loss_sum += g_loss.to_vec0::<f32>()? as f64;
    }

    Ok((
        g_loss_sum / num_batches as f64,
        d_loss_sum / num_batches as f64,
    ))
}

// ============================================
// Main
// ============================================
fn main() -> Result<()> {
    println!("🔥 진짜 DCGAN (Deep Convolutional GAN) 시작!");
    println!("{}", "=".repeat(60));

    // MNIST 데이터 다운로드
    download_mnist()?;

    // 설정 (DCGAN 논문 기반)
    let target_digit = 9;
    let latent_dim = 100;
    let num_classes = 10;
    let batch_size = 64; // DCGAN 논문: 64
    let epochs = 100; // 빠른 결과 확인
    let g_learning_rate = 0.0002; // DCGAN 논문: 0.0002
    let d_learning_rate = 0.0002; // DCGAN 논문: 동일한 학습률!

    let device = Device::Cpu;

    println!("\n📊 DCGAN 설정:");
    println!("  - Target Digit: {}", target_digit);
    println!("  - Architecture: Fully Connected (안정적!)");
    println!("  - Data: 실제 MNIST 데이터");
    println!("  - Batch Size: {}", batch_size);
    println!("  - Epochs: {}", epochs);
    println!();

    // 실제 MNIST 데이터 로딩
    let real_images = prepare_mnist_data(target_digit, &device)?;

    // Generator 초기화
    let g_varmap = VarMap::new();
    let g_vb = VarBuilder::from_varmap(&g_varmap, DType::F32, &device);
    let generator = DCGANGenerator::new(g_vb, latent_dim, num_classes)?;
    println!("✅ DCGAN Generator 초기화 완료! (Convolutional)");

    // Discriminator 초기화
    let d_varmap = VarMap::new();
    let d_vb = VarBuilder::from_varmap(&d_varmap, DType::F32, &device);
    let discriminator = DCGANDiscriminator::new(d_vb, num_classes)?;
    println!("✅ DCGAN Discriminator 초기화 완료! (Convolutional)");
    println!();

    // Optimizer
    let g_params = ParamsAdamW {
        lr: g_learning_rate,
        ..Default::default()
    };
    let mut g_optimizer = AdamW::new(g_varmap.all_vars(), g_params)?;

    let d_params = ParamsAdamW {
        lr: d_learning_rate,
        ..Default::default()
    };
    let mut d_optimizer = AdamW::new(d_varmap.all_vars(), d_params)?;

    println!("🏋️ DCGAN 학습 시작!");
    println!("{}", "=".repeat(60));

    for epoch in 1..=epochs {
        let (g_loss, d_loss) = train_dcgan(
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

        if epoch % 5 == 0 {
            visualize_generated_image(&generator, latent_dim, target_digit, &device)?;
            save_generated_images(&generator, epoch, latent_dim, target_digit, &device)?;
        }
    }

    println!();
    println!("{}", "=".repeat(60));
    println!("🎉 DCGAN 학습 완료!");
    println!("📁 생성된 이미지: generated_images/ 폴더 확인");

    println!();
    println!("🎨 최종 생성 이미지:");
    save_generated_images(&generator, epochs, latent_dim, target_digit, &device)?;

    println!();
    println!("💾 모델 저장 중...");
    g_varmap.save("dcgan_generator.safetensors")?;
    d_varmap.save("dcgan_discriminator.safetensors")?;
    println!("✅ 모델 저장 완료!");

    Ok(())
}
