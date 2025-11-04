// 🤖 모델 학습

use zkml_classifier::{download_mnist, prepare_mnist_data, SimpleClassifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🤖 ZKML 이미지 분류기 - 모델 학습                 ║");
    println!("║                 (실제 MNIST 데이터)                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. MNIST 다운로드
    println!("📥 [1/4] MNIST 데이터 다운로드 중...");
    download_mnist()?;
    println!();

    // 2. 데이터 로딩
    println!("📊 [2/4] MNIST 데이터 로딩 중...");
    let (images, labels) = prepare_mnist_data()?;
    println!("   ✅ {}개 샘플 로딩 완료!\n", images.len());

    // 3. 모델 학습
    println!("🤖 [3/4] 모델 학습 중...");
    println!("   구조: 784 → 256 (ReLU) → 128 (ReLU) → 10\n");
    let mut model = SimpleClassifier::new();
    model.train(&images, &labels, 50, 0.0005); // 50 epochs, lr=0.0005 (안정적!)
    println!("   ✅ 학습 완료!\n");

    // 4. 모델 저장
    println!("💾 [4/4] 모델 저장 중...");
    model.save("models/classifier.json")?;
    println!("   ✅ models/classifier.json 저장 완료!\n");

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    ✅ 학습 완료! ✅                         ║");
    println!("║                                                              ║");
    println!("║  데이터: 실제 MNIST (60,000개)                              ║");
    println!("║  모델: 784 → 256 → 128 → 10 (히든 레이어 2개)              ║");
    println!("║  Epochs: 50                                                  ║");
    println!("║  Learning Rate: 0.0005 (Gradient Clipping: 0.5)             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}
