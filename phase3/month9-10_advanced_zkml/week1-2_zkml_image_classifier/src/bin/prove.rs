// 🔐 ZK 증명 생성

use zkml_classifier::{SimpleClassifier, generate_synthetic_data, generate_proof};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔐 ZKML 이미지 분류기 - ZK 증명 생성              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. 모델 로드
    println!("📂 [1/4] 모델 로드 중...");
    let model = SimpleClassifier::load("models/classifier.json")?;
    println!("   ✅ 모델 로드 완료!\n");

    // 2. 테스트 이미지 생성
    println!("📊 [2/4] 테스트 이미지 생성 중...");
    let (images, _labels) = generate_synthetic_data(1);
    let image = &images[0];
    println!("   ✅ 이미지 생성 완료!\n");

    // 3. 추론
    println!("🔮 [3/4] AI 추론 중...");
    let predicted_class = model.predict(image);
    println!("   ✅ 예측 결과: {}\n", predicted_class);

    // 4. 증명 생성
    println!("🔐 [4/4] ZK 증명 생성 중...");
    let proof = generate_proof(image, predicted_class)?;
    proof.save("proofs/proof.json")?;
    println!("   ✅ 증명 저장 완료!\n");

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 ✅ 증명 생성 완료! ✅                       ║");
    println!("║                                                              ║");
    println!("║  예측 클래스: {}                                            ║", predicted_class);
    println!("║  이미지 해시: {}... ║", &proof.image_hash[..16]);
    println!("║  타임스탬프: {}                                    ║", proof.timestamp);
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}

