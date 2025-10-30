// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 👁️ Week 5-1: Vision Transformer (ViT)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 목표: 이미지 분류를 Transformer로!
//
// Vision Transformer 구조:
// 1. 이미지를 패치로 분할 (16x16 픽셀씩)
// 2. 각 패치를 벡터로 변환 (Patch Embedding)
// 3. Position Embedding 추가
// 4. Transformer Encoder 통과
// 5. Classification Head로 분류
//
// 예제: MNIST 스타일 숫자 이미지 분류
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::{
    embedding, layer_norm, linear, loss, ops, AdamW, Embedding, LayerNorm, Linear, Module,
    Optimizer, ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;
use std::fs;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧩 Patch Embedding Layer
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 이미지를 패치로 나누고 벡터로 변환
// 예: 28x28 이미지 → 4x4 패치 (각 7x7) → 16개 패치 → 각 128차원 벡터
//

struct PatchEmbedding {
    projection: Linear, // 패치 → 벡터 변환
    patch_size: usize,
}

impl PatchEmbedding {
    fn new(vb: VarBuilder, _img_size: usize, patch_size: usize, embed_size: usize) -> Result<Self> {
        let num_pixels = patch_size * patch_size;
        let projection = linear(num_pixels, embed_size, vb.pp("projection"))?;

        Ok(Self {
            projection,
            patch_size,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // x: [batch, height, width]
        // → [batch, num_patches, patch_size * patch_size]

        let (_batch_size, height, width) = x.dims3()?;
        let num_patches_h = height / self.patch_size;
        let num_patches_w = width / self.patch_size;

        let mut patches = Vec::new();

        for i in 0..num_patches_h {
            for j in 0..num_patches_w {
                // 패치 추출
                let y_start = i * self.patch_size;
                let x_start = j * self.patch_size;

                let patch =
                    x.narrow(1, y_start, self.patch_size)?
                        .narrow(2, x_start, self.patch_size)?;

                // [batch, patch_size, patch_size] → [batch, patch_size * patch_size]
                let patch_flat = patch.flatten(1, 2)?;
                patches.push(patch_flat);
            }
        }

        // [batch, num_patches, patch_size * patch_size]
        let patches_tensor = Tensor::stack(&patches, 1)?;

        // [batch, num_patches, embed_size]
        self.projection.forward(&patches_tensor)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎯 Multi-Head Self-Attention
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct MultiHeadAttention {
    query: Linear,
    key: Linear,
    value: Linear,
    proj: Linear,
    num_heads: usize,
    head_dim: usize,
}

impl MultiHeadAttention {
    fn new(vb: VarBuilder, embed_size: usize, num_heads: usize) -> Result<Self> {
        let head_dim = embed_size / num_heads;

        Ok(Self {
            query: linear(embed_size, embed_size, vb.pp("query"))?,
            key: linear(embed_size, embed_size, vb.pp("key"))?,
            value: linear(embed_size, embed_size, vb.pp("value"))?,
            proj: linear(embed_size, embed_size, vb.pp("proj"))?,
            num_heads,
            head_dim,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (batch_size, seq_len, embed_size) = x.dims3()?;

        // Q, K, V 생성
        let q = self.query.forward(x)?;
        let k = self.key.forward(x)?;
        let v = self.value.forward(x)?;

        // [batch, seq_len, num_heads, head_dim]
        let q = q
            .reshape((batch_size, seq_len, self.num_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()?; // [batch, num_heads, seq_len, head_dim]
        let k = k
            .reshape((batch_size, seq_len, self.num_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()?;
        let v = v
            .reshape((batch_size, seq_len, self.num_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()?;

        // Attention scores
        let scale = (self.head_dim as f64).sqrt();
        let k_t = k.transpose(D::Minus2, D::Minus1)?.contiguous()?;
        let scores = (q.matmul(&k_t)? / scale)?;
        let attn = ops::softmax(&scores, D::Minus1)?;

        // Weighted sum
        let out = attn.matmul(&v)?; // [batch, num_heads, seq_len, head_dim]
        let out = out
            .transpose(1, 2)?
            .contiguous()?
            .reshape((batch_size, seq_len, embed_size))?;

        self.proj.forward(&out)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🏗️ Transformer Encoder Block
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct TransformerBlock {
    attention: MultiHeadAttention,
    ln1: LayerNorm,
    ff1: Linear,
    ff2: Linear,
    ln2: LayerNorm,
}

impl TransformerBlock {
    fn new(vb: VarBuilder, embed_size: usize, num_heads: usize, ff_hidden: usize) -> Result<Self> {
        Ok(Self {
            attention: MultiHeadAttention::new(vb.pp("attn"), embed_size, num_heads)?,
            ln1: layer_norm(embed_size, 1e-5, vb.pp("ln1"))?,
            ff1: linear(embed_size, ff_hidden, vb.pp("ff1"))?,
            ff2: linear(ff_hidden, embed_size, vb.pp("ff2"))?,
            ln2: layer_norm(embed_size, 1e-5, vb.pp("ln2"))?,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // Self-Attention + Residual
        let attn_out = self.attention.forward(x)?;
        let x = (x + attn_out)?;
        let x = self.ln1.forward(&x)?;

        // Feed-Forward + Residual
        let ff_out = self.ff1.forward(&x)?.gelu()?;
        let ff_out = self.ff2.forward(&ff_out)?;
        let x = (&x + ff_out)?;
        self.ln2.forward(&x)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎨 Vision Transformer
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

struct VisionTransformer {
    patch_embed: PatchEmbedding,
    pos_embed: Embedding,
    blocks: Vec<TransformerBlock>,
    ln: LayerNorm,
    head: Linear,
    num_patches: usize,
}

impl VisionTransformer {
    fn new(
        vb: VarBuilder,
        img_size: usize,
        patch_size: usize,
        embed_size: usize,
        num_heads: usize,
        num_layers: usize,
        num_classes: usize,
    ) -> Result<Self> {
        let num_patches = (img_size / patch_size) * (img_size / patch_size);
        let ff_hidden = embed_size * 4;

        let patch_embed =
            PatchEmbedding::new(vb.pp("patch_embed"), img_size, patch_size, embed_size)?;
        let pos_embed = embedding(num_patches, embed_size, vb.pp("pos_embed"))?;

        let mut blocks = Vec::new();
        for i in 0..num_layers {
            blocks.push(TransformerBlock::new(
                vb.pp(&format!("block{}", i)),
                embed_size,
                num_heads,
                ff_hidden,
            )?);
        }

        let ln = layer_norm(embed_size, 1e-5, vb.pp("ln"))?;
        let head = linear(embed_size, num_classes, vb.pp("head"))?;

        Ok(Self {
            patch_embed,
            pos_embed,
            blocks,
            ln,
            head,
            num_patches,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // Patch Embedding
        let x = self.patch_embed.forward(x)?; // [batch, num_patches, embed_size]

        // Position Embedding
        let positions = Tensor::arange(0u32, self.num_patches as u32, x.device())?;
        let pos_emb = self.pos_embed.forward(&positions)?;
        let x = x.broadcast_add(&pos_emb)?;

        // Transformer Blocks
        let mut x = x;
        for block in &self.blocks {
            x = block.forward(&x)?;
        }

        // Classification Head (CLS token 대신 평균)
        let x = self.ln.forward(&x)?;
        let x = x.mean(1)?; // [batch, embed_size]
        self.head.forward(&x)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📁 이미지 로드 함수
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn load_image_from_file(path: &str, device: &Device) -> Result<Tensor> {
    let content = fs::read_to_string(path)
        .map_err(|e| candle_core::Error::Msg(format!("Failed to read file: {}", e)))?;

    let mut pixels = Vec::new();

    for line in content.lines() {
        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue; // 주석과 빈 줄 건너뛰기
        }

        for num_str in line.split_whitespace() {
            if let Ok(val) = num_str.parse::<f32>() {
                pixels.push(val);
            }
        }
    }

    if pixels.len() != 28 * 28 {
        return Err(candle_core::Error::Msg(format!(
            "Expected 784 pixels (28x28), got {}",
            pixels.len()
        )));
    }

    // [1, 28, 28] 형태로 변환 (배치 크기 1)
    Tensor::from_vec(pixels, (1, 28, 28), device)
}

fn visualize_image(img: &[f32], width: usize) {
    println!("\n📸 이미지 미리보기:");
    for i in 0..width {
        print!("   ");
        for j in 0..width {
            let val = img[i * width + j];
            if val > 0.5 {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📁 손글씨 CSV 로드
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn load_handwritten_data(device: &Device) -> Result<(Vec<Vec<f32>>, Vec<u32>)> {
    let csv_path = "handwritten_data.csv";

    if !std::path::Path::new(csv_path).exists() {
        return Ok((Vec::new(), Vec::new()));
    }

    let content = fs::read_to_string(csv_path)
        .map_err(|e| candle_core::Error::Msg(format!("Failed to read CSV: {}", e)))?;

    let mut images = Vec::new();
    let mut labels = Vec::new();

    for line in content.lines() {
        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 785 {
            // 1 label + 784 pixels
            continue;
        }

        // 레이블 파싱
        if let Ok(label) = parts[0].parse::<u32>() {
            // 픽셀 데이터 파싱
            let mut pixels = Vec::new();
            for i in 1..785 {
                if let Ok(pixel) = parts[i].parse::<f32>() {
                    pixels.push(pixel);
                } else {
                    break;
                }
            }

            if pixels.len() == 784 {
                images.push(pixels);
                labels.push(label);
            }
        }
    }

    Ok((images, labels))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📊 데이터 생성
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn generate_mnist_style_data(device: &Device) -> Result<(Tensor, Tensor)> {
    let mut rng = rand::thread_rng();
    let num_samples = 500; // 50 → 500개로 증가!
    let img_size = 28;

    let mut images = Vec::new();
    let mut labels = Vec::new();

    for _ in 0..num_samples {
        let label = rng.gen_range(0..10);
        let mut img = vec![0.0f32; img_size * img_size];

        // 간단한 패턴 생성 (숫자별 특징)
        for i in 0..img_size {
            for j in 0..img_size {
                let idx = i * img_size + j;

                // 숫자별 더 정확한 패턴
                let center_x = 14.0;
                let center_y = 14.0;
                let di = (i as f32 - center_y).abs();
                let dj = (j as f32 - center_x).abs();
                let dist = (di * di + dj * dj).sqrt();

                match label {
                    0 => {
                        // 0: 원형 (속이 빈)
                        if dist > 6.0 && dist < 10.0 {
                            img[idx] = 1.0;
                        }
                    }
                    1 => {
                        // 1: 세로 줄 (약간 기울어진)
                        if (j >= 12 && j <= 14) || (i < 5 && j >= 11 && j <= 13) {
                            img[idx] = 1.0;
                        }
                        if i > 23 && j >= 10 && j <= 16 {
                            img[idx] = 1.0;
                        }
                    }
                    2 => {
                        // 2: 지그재그 형태
                        if (i < 7 && dist < 8.0 && i >= j - 14)
                            || (i >= 7 && i < 14 && j < 18 && j > 10)
                            || (i >= 14 && i < 21 && j > 10)
                            || (i >= 21 && j >= 8)
                        {
                            img[idx] = 1.0;
                        }
                    }
                    3 => {
                        // 3: 두 개의 곡선
                        if (i < 12 && dist < 9.0 && j > 14) || (i >= 12 && dist < 9.0 && j > 14) {
                            img[idx] = 1.0;
                        }
                    }
                    4 => {
                        // 4: ㄱ자 + 세로
                        if (j == 14 && i >= 10)
                            || (i == 14 && j <= 14)
                            || (j >= 10 && i >= 0 && i <= 2)
                        {
                            img[idx] = 1.0;
                        }
                    }
                    5 => {
                        // 5: 위 가로 + 중간 곡선
                        if (i < 5 && j <= 16)
                            || (i >= 5 && i < 12 && j <= 12)
                            || (i >= 12 && dist < 8.0 && j > 14)
                        {
                            img[idx] = 1.0;
                        }
                    }
                    6 => {
                        // 6: 아래 원 + 위 곡선
                        if (i > 10 && dist < 9.0) || (i <= 14 && j < 15 && j > 10) {
                            img[idx] = 1.0;
                        }
                    }
                    7 => {
                        // 7: 위 가로 + 대각선
                        if (i < 5 && j > 8) || (i >= 5 && j == 20 - i) {
                            img[idx] = 1.0;
                        }
                    }
                    8 => {
                        // 8: 위아래 두 원
                        if ((i < 14 && dist < 7.0 && dist > 4.0)
                            || (i >= 14 && dist < 7.0 && dist > 4.0))
                        {
                            img[idx] = 1.0;
                        }
                    }
                    9 => {
                        // 9: 위 원 + 아래 세로
                        if (i < 14 && dist < 8.0 && dist > 5.0) || (i >= 14 && j > 16 && j < 20) {
                            img[idx] = 1.0;
                        }
                    }
                    _ => {}
                }

                // 약간의 노이즈 추가 (실제 데이터처럼)
                if rng.gen::<f32>() < 0.05 {
                    img[idx] = rng.gen::<f32>() * 0.3;
                }
            }
        }

        images.push(img);
        labels.push(label as u32);
    }

    let images_flat: Vec<f32> = images.into_iter().flatten().collect();
    let images_tensor = Tensor::from_vec(images_flat, (num_samples, img_size, img_size), device)?;

    let labels_tensor = Tensor::from_vec(labels, num_samples, device)?;

    Ok((images_tensor, labels_tensor))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎓 학습
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn train(
    model: &VisionTransformer,
    varmap: &VarMap,
    images: &Tensor,
    labels: &Tensor,
    _device: &Device,
) -> Result<()> {
    let mut opt = AdamW::new(
        varmap.all_vars(),
        ParamsAdamW {
            lr: 0.001,
            ..Default::default()
        },
    )?;

    println!("\n🎓 Vision Transformer 학습 시작...\n");

    for epoch in 1..=1000 {
        let logits = model.forward(images)?;
        let loss = loss::cross_entropy(&logits, labels)?;

        opt.backward_step(&loss)?;

        if epoch % 50 == 0 {
            // 10 → 50으로 변경 (출력 줄이기)
            let loss_val = loss.to_vec0::<f32>()?;

            // 정확도 계산
            let preds = logits.argmax(D::Minus1)?.to_vec1::<u32>()?;
            let labels_vec = labels.to_vec1::<u32>()?;
            let correct = preds
                .iter()
                .zip(labels_vec.iter())
                .filter(|(p, l)| p == l)
                .count();
            let accuracy = correct as f32 / labels_vec.len() as f32 * 100.0;

            println!(
                "Epoch {:3} | Loss: {:.4} | Accuracy: {:.1}%",
                epoch, loss_val, accuracy
            );
        }
    }

    println!("\n✅ 학습 완료!\n");

    // 모델 저장
    println!("💾 모델 저장 중...");
    varmap.save("vit_model.safetensors")?;
    println!("   ✅ vit_model.safetensors 저장 완료!\n");

    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🚀 메인
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn main() -> Result<()> {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("👁️  Vision Transformer (ViT) - 이미지 분류");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let device = Device::cuda_if_available(0)?;

    // 하이퍼파라미터 (성능 향상!)
    let img_size = 28;
    let patch_size = 7;
    let embed_size = 128; // 64 → 128로 증가
    let num_heads = 8; // 4 → 8로 증가
    let num_layers = 6; // 3 → 6로 증가
    let num_classes = 10;

    println!("📊 모델 설정:");
    println!("   이미지 크기: {}x{}", img_size, img_size);
    println!("   패치 크기: {}x{}", patch_size, patch_size);
    println!(
        "   패치 개수: {}",
        (img_size / patch_size) * (img_size / patch_size)
    );
    println!("   임베딩 크기: {}", embed_size);
    println!("   헤드 개수: {}", num_heads);
    println!("   레이어 개수: {}", num_layers);
    println!("   클래스 개수: {}\n", num_classes);

    // 모델 생성
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = VisionTransformer::new(
        vb,
        img_size,
        patch_size,
        embed_size,
        num_heads,
        num_layers,
        num_classes,
    )?;

    // 데이터 생성
    println!("📁 데이터 로드 중...");

    // 1. 합성 데이터 생성
    let (synthetic_images, synthetic_labels) = generate_mnist_style_data(&device)?;
    println!("   합성 데이터: {} 개", synthetic_images.dim(0)?);

    // 2. 손글씨 데이터 로드
    let (handwritten_imgs, handwritten_lbls) = load_handwritten_data(&device)?;
    let handwritten_count = handwritten_imgs.len();

    let (images, labels) = if handwritten_count > 0 {
        println!("   손글씨 데이터: {} 개 ✨", handwritten_count);

        // 손글씨 데이터를 Tensor로 변환
        let handwritten_flat: Vec<f32> = handwritten_imgs.into_iter().flatten().collect();
        let handwritten_images =
            Tensor::from_vec(handwritten_flat, (handwritten_count, 28, 28), &device)?;
        let handwritten_labels = Tensor::from_vec(handwritten_lbls, handwritten_count, &device)?;

        // 합성 데이터와 손글씨 데이터 합치기
        let combined_images = Tensor::cat(&[&synthetic_images, &handwritten_images], 0)?;
        let combined_labels = Tensor::cat(&[&synthetic_labels, &handwritten_labels], 0)?;

        println!(
            "   총 데이터: {} 개 (합성 + 손글씨)",
            combined_images.dim(0)?
        );
        (combined_images, combined_labels)
    } else {
        println!("   손글씨 데이터 없음 (collect_data.html로 수집하세요!)");
        println!("   총 데이터: {} 개", synthetic_images.dim(0)?);
        (synthetic_images, synthetic_labels)
    };

    println!("   레이블: 0-9 (10 클래스)\n");

    // 학습
    train(&model, &varmap, &images, &labels, &device)?;

    // 테스트
    println!("🧪 테스트 중...\n");
    let logits = model.forward(&images)?;
    let preds = logits.argmax(D::Minus1)?.to_vec1::<u32>()?;
    let labels_vec = labels.to_vec1::<u32>()?;

    println!("📊 예측 결과 (처음 10개):");
    println!("   실제 | 예측");
    println!("   -----+-----");
    for i in 0..10.min(preds.len()) {
        let correct = if preds[i] == labels_vec[i] {
            "✅"
        } else {
            "❌"
        };
        println!("   {}     | {}      {}", labels_vec[i], preds[i], correct);
    }

    let correct = preds
        .iter()
        .zip(labels_vec.iter())
        .filter(|(p, l)| p == l)
        .count();
    let accuracy = correct as f32 / labels_vec.len() as f32 * 100.0;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎉 최종 정확도: {:.1}%", accuracy);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("🧠 Vision Transformer 핵심 개념:");
    println!("   1️⃣  이미지 → 패치 분할 (7x7)");
    println!("   2️⃣  패치 → 벡터 임베딩");
    println!("   3️⃣  Position Embedding 추가");
    println!("   4️⃣  Multi-Head Self-Attention");
    println!("   5️⃣  Transformer Encoder");
    println!("   6️⃣  Classification Head");
    println!("\n👁️  이미지도 Transformer로 처리 가능!\n");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 🖼️ 실제 이미지 테스트
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🖼️  실제 이미지 테스트");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if std::path::Path::new("test_image.txt").exists() {
        println!("📁 test_image.txt 로드 중...");

        match load_image_from_file("test_image.txt", &device) {
            Ok(test_img) => {
                // 이미지 시각화
                let img_vec = test_img.flatten_all()?.to_vec1::<f32>()?;
                visualize_image(&img_vec, 28);

                // 예측
                println!("🔮 예측 중...");
                let logits = model.forward(&test_img)?;
                let pred = logits.argmax(D::Minus1)?.to_vec1::<u32>()?;

                println!("   예측 결과: 숫자 {} (확률 분포 아래 참고)", pred[0]);

                // 확률 분포
                let probs = ops::softmax(&logits, D::Minus1)?.to_vec2::<f32>()?;
                println!("\n   각 숫자별 확률:");
                for i in 0..10 {
                    let prob = probs[0][i] * 100.0;
                    let bar_len = (prob / 5.0) as usize;
                    let bar = "█".repeat(bar_len);
                    println!("   {} | {:5.1}% {}", i, prob, bar);
                }

                println!("\n✅ 실제 이미지 테스트 완료!\n");
            }
            Err(e) => {
                println!("   ⚠️  이미지 로드 실패: {}\n", e);
            }
        }
    } else {
        println!("💡 test_image.txt 파일이 없습니다.");
        println!("   28x28 크기의 숫자 이미지를 만들어보세요!\n");
        println!("   예시 형식:");
        println!("   0 0 0 1 1 1 0 0 ...");
        println!("   0 0 1 1 1 1 1 0 ...");
        println!("   ...\n");
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Ok(())
}
