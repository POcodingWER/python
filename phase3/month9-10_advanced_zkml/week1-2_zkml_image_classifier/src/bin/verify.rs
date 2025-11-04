// ✅ ZK 증명 검증

use zkml_classifier::{verify_proof, MLProof};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          ✅ ZKML 이미지 분류기 - ZK 증명 검증              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. 증명 로드
    println!("📂 [1/2] 증명 로드 중...");
    let proof = MLProof::load("proofs/proof.json")?;
    println!("   ✅ 증명 로드 완료!\n");

    // 2. 증명 검증
    println!("✅ [2/2] 증명 검증 중...");
    let is_valid = verify_proof(&proof)?;
    println!("   ✅ 검증 완료!\n");

    if is_valid {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                 ✅ 검증 성공! ✅                             ║");
        println!("║                                                              ║");
        println!(
            "║  예측 클래스: {}                                            ║",
            proof.predicted_class
        );
        println!("║  이미지 해시: {}... ║", &proof.image_hash[..16]);
        println!(
            "║  타임스탬프: {}                                    ║",
            proof.timestamp
        );
        println!("║                                                              ║");
        println!("║  ✅ 이 결과는 올바른 ML 모델로 계산되었습니다!              ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
    } else {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                 ❌ 검증 실패! ❌                            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
    }

    Ok(())
}
