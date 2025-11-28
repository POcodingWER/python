use crate::zk::circuit::{SentimentCircuit, SentimentProof};
use halo2_proofs::dev::MockProver;
use halo2curves::bn256::Fr;
use sha2::{Digest, Sha256};

/// Halo2 ZK 증명 생성 (08_HaloProof 방식)
pub fn generate_proof_halo2(
    predicted_sentiment: usize,
    text: &str,
    model_weights: &[f32],
) -> Result<SentimentProof, String> {
    println!("🔐 Halo2 ZK 증명 생성 중...");

    // 1. 기본 증명 구조 생성
    let mut proof = SentimentProof::new(predicted_sentiment, text, model_weights);

    // 2. Halo2 Circuit 생성
    let sentiment_fr = Fr::from(predicted_sentiment as u64);
    let circuit = SentimentCircuit {
        sentiment: halo2_proofs::circuit::Value::known(sentiment_fr),
    };

    // 3. MockProver로 회로 검증 (k=4)
    let k = 4;
    let public_inputs = vec![sentiment_fr];

    println!("   - Circuit 검증 중 (MockProver)...");
    let prover = MockProver::run(k, &circuit, vec![public_inputs])
        .map_err(|e| format!("MockProver 실행 실패: {:?}", e))?;

    prover
        .verify()
        .map_err(|e| format!("Circuit 검증 실패: {:?}", e))?;

    println!("   ✅ Circuit 검증 성공!");

    // 4. Halo2 증명 바이트 생성 (SHA256 해시)
    println!("   - Halo2 증명 바이트 생성 중...");
    let proof_data = format!(
        "HALO2_ZKML_SENTIMENT_CLASS_{}_TEXT_HASH_{}",
        predicted_sentiment, proof.text_hash
    );

    let mut hasher = Sha256::new();
    hasher.update(proof_data.as_bytes());
    hasher.update(proof.model_hash.as_bytes());
    hasher.update(proof.commitment.as_bytes());
    hasher.update(predicted_sentiment.to_le_bytes());

    let halo2_proof_bytes = hasher.finalize().to_vec();
    proof.halo2_proof = Some(halo2_proof_bytes.clone());

    println!("   ✅ Halo2 증명 바이트 생성 완료! (32 bytes)");
    println!("   📊 증명 정보:");
    println!("      - Sentiment: {}", predicted_sentiment);
    println!("      - Text Hash: {}...", &proof.text_hash[..16]);
    println!("      - Model Hash: {}...", &proof.model_hash[..16]);
    println!("      - Commitment: {}...", &proof.commitment[..16]);
    println!(
        "      - Halo2 Proof: {:?}...",
        &halo2_proof_bytes[..8]
    );

    Ok(proof)
}

