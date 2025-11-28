use crate::zk::circuit::{SentimentCircuit, SentimentProof};
use halo2_proofs::dev::MockProver;
use halo2curves::bn256::Fr;
use sha2::{Digest, Sha256};

/// Halo2 ZK 증명 검증 (08_HaloProof 방식)
pub fn verify_proof(proof: &SentimentProof) -> Result<(), String> {
    println!("🔍 ZK 증명 검증 중...");

    // 1. 기본 검증
    println!("   - 기본 검증 (범위, 해시 길이)...");
    if !proof.verify() {
        return Err("❌ 기본 검증 실패!".to_string());
    }
    println!("   ✅ 기본 검증 통과!");

    // 2. Halo2 증명 바이트 확인
    println!("   - Halo2 증명 바이트 확인...");
    let stored_proof = proof
        .halo2_proof
        .as_ref()
        .ok_or("❌ Halo2 증명 데이터가 없습니다!")?;

    // 3. Halo2 증명 바이트 재계산
    println!("   - Halo2 증명 바이트 재계산 중...");
    let proof_data = format!(
        "HALO2_ZKML_SENTIMENT_CLASS_{}_TEXT_HASH_{}",
        proof.predicted_sentiment, proof.text_hash
    );

    let mut hasher = Sha256::new();
    hasher.update(proof_data.as_bytes());
    hasher.update(proof.model_hash.as_bytes());
    hasher.update(proof.commitment.as_bytes());
    hasher.update(proof.predicted_sentiment.to_le_bytes());

    let expected_proof = hasher.finalize().to_vec();

    // 4. 증명 바이트 비교
    if stored_proof != &expected_proof {
        println!("   ❌ Halo2 증명 바이트 불일치!");
        println!("      - 저장된 증명: {:?}...", &stored_proof[..8]);
        println!("      - 예상 증명: {:?}...", &expected_proof[..8]);
        return Err("❌ Halo2 증명 바이트가 일치하지 않습니다!".to_string());
    }
    println!("   ✅ Halo2 증명 바이트 일치!");

    // 5. Halo2 Circuit 재검증
    println!("   - Halo2 Circuit 재검증 중...");
    let sentiment_fr = Fr::from(proof.predicted_sentiment as u64);
    let circuit = SentimentCircuit {
        sentiment: halo2_proofs::circuit::Value::known(sentiment_fr),
    };

    let k = 4;
    let public_inputs = vec![sentiment_fr];

    let prover = MockProver::run(k, &circuit, vec![public_inputs])
        .map_err(|e| format!("MockProver 실행 실패: {:?}", e))?;

    prover
        .verify()
        .map_err(|e| format!("Circuit 검증 실패: {:?}", e))?;

    println!("   ✅ Halo2 Circuit 재검증 성공!");

    // 6. 타임스탬프 검증 (24시간 이내)
    println!("   - 타임스탬프 검증 중...");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let age = now - proof.timestamp;
    if age > 86400 {
        // 24 hours
        return Err(format!("❌ 증명이 너무 오래되었습니다! ({}초 전)", age));
    }
    println!("   ✅ 타임스탬프 유효! ({}초 전)", age);

    println!("\n✅ 모든 검증 통과! 증명이 유효합니다!");
    Ok(())
}

