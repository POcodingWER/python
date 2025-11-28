use zkml_sentiment::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 ZKML 감성 분석 ZK 증명 검증!\n");

    // 1. 증명 로드
    println!("📂 증명 로딩 중...");
    let proof_json = std::fs::read_to_string("proofs/proof.json")?;
    let proof: SentimentProof = serde_json::from_str(&proof_json)?;
    println!("   ✅ 증명 로드 완료!");

    // 2. 증명 정보 출력
    println!("\n📊 증명 정보:");
    let sentiment_label = match proof.predicted_sentiment {
        0 => "😞 Negative",
        1 => "😐 Neutral",
        2 => "😊 Positive",
        _ => "❓ Unknown",
    };
    println!("   - Sentiment: {}", sentiment_label);
    println!("   - Text Hash: {}...", &proof.text_hash[..16]);
    println!("   - Model Hash: {}...", &proof.model_hash[..16]);
    println!("   - Commitment: {}...", &proof.commitment[..16]);
    println!("   - Timestamp: {}", proof.timestamp);

    if let Some(ref halo2_proof) = proof.halo2_proof {
        println!("   - Halo2 Proof: {:?}... (32 bytes)", &halo2_proof[..8]);
    }

    // 3. 검증 실행
    println!();
    match verify_proof(&proof) {
        Ok(_) => {
            println!("\n🎉 검증 성공! 증명이 유효합니다!");
        }
        Err(e) => {
            println!("\n❌ 검증 실패: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

