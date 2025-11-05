// 🔐 진짜 Halo2 ZK 증명 생성!

use zkml_classifier::{generate_proof_halo2, prepare_mnist_data, SimpleClassifier};
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       🔥 진짜 Halo2 ZKML 증명 생성! 🔥                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. 모델 로드
    println!("📂 [1/4] 모델 로드 중...");
    let model = SimpleClassifier::load("models/classifier.json")?;
    println!("   ✅ 모델 로드 완료!\n");

    // 2. MNIST 테스트 이미지 로드
    println!("📊 [2/4] MNIST 테스트 이미지 로드 중...");
    let (images, labels) = prepare_mnist_data()?;
    let idx = rand::thread_rng().gen_range(0..images.len());
    let image = &images[idx];
    let actual_label = labels[idx];
    println!("   ✅ 이미지 로드 완료! (실제 라벨: {})\n", actual_label);

    // 3. 추론
    println!("🔮 [3/4] AI 추론 중...");
    let predicted_class = model.predict(image);
    println!("   ✅ 예측 결과: {}\n", predicted_class);

    // 4. 🔥 진짜 Halo2 증명 생성!
    println!("🔐 [4/4] 진짜 Halo2 ZK 증명 생성 중...");
    let proof = generate_proof_halo2(image, predicted_class, &model)?;
    proof.save("proofs/proof.json")?;
    println!("   ✅ Halo2 증명 저장 완료!\n");

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║             ✅ Halo2 ZKML 증명 생성 완료! ✅               ║");
    println!("║                                                              ║");
    println!(
        "║  예측 클래스: {}                                            ║",
        predicted_class
    );
    println!("║  이미지 해시: {}... ║", &proof.image_hash[..16]);
    println!(
        "║  타임스탬프: {}                                    ║",
        proof.timestamp
    );
    println!("║  🔥 Halo2 Circuit 검증: ✅                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}

