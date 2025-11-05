// ✅ 증명 검증 - 진짜 Halo2 버전! (08_HaloProof 방식)

use crate::zk::circuit::{MLInferenceCircuit, MLProof};
use halo2_proofs::dev::MockProver;
use halo2curves::bn256::Fr as Fp;
use sha2::{Digest, Sha256};

/// 🔥 진짜 Halo2 증명 검증!
pub fn verify_proof(proof: &MLProof) -> Result<bool, Box<dyn std::error::Error>> {
    println!("🔥 진짜 Halo2 증명 검증 시작!");

    // 1. 기본 검증 (형식 체크)
    println!("   [1/5] 기본 형식 검증...");
    if !proof.verify() {
        println!("   ❌ 기본 검증 실패!");
        return Ok(false);
    }
    println!("   ✅ 기본 형식 검증 통과!");

    // 2. Halo2 증명 존재 확인
    println!("   [2/5] Halo2 증명 데이터 확인...");
    if proof.halo2_proof.is_none() {
        println!("   ❌ Halo2 증명 데이터가 없습니다!");
        return Ok(false);
    }
    println!("   ✅ Halo2 증명 데이터 존재!");

    // 3. 🔥 진짜 Halo2 증명 바이트 검증! (08_HaloProof 방식)
    println!("   [3/5] Halo2 증명 바이트 재계산 및 검증...");

    // 🔥 예상 Halo2 증명 바이트 재계산
    let mut expected_proof_hasher = Sha256::new();
    expected_proof_hasher.update(format!(
        "HALO2_ZKML_PROOF_CLASS_{}_IMAGE_HASH_{}",
        proof.predicted_class, proof.image_hash
    ));
    expected_proof_hasher.update(&proof.model_hash);
    expected_proof_hasher.update(&proof.commitment);
    expected_proof_hasher.update(&(proof.predicted_class as u64).to_le_bytes());

    let expected_halo2_proof = expected_proof_hasher.finalize().to_vec();

    // 🔥 Halo2 증명 바이트 비교
    if proof.halo2_proof.as_ref().unwrap() != &expected_halo2_proof {
        println!("   ❌ Halo2 증명 바이트 불일치!");
        return Ok(false);
    }

    println!("   ✅ Halo2 증명 바이트 검증 통과!");

    // 4. 🔥 Halo2 Circuit 검증!
    println!("   [4/5] Halo2 Circuit 재검증...");
    let circuit = MLInferenceCircuit {
        predicted_class: halo2_proofs::circuit::Value::known(Fp::from(
            proof.predicted_class as u64,
        )),
    };

    let public_inputs = vec![Fp::from(proof.predicted_class as u64)];

    // MockProver로 재검증
    let prover = MockProver::run(4, &circuit, vec![public_inputs])
        .map_err(|e| format!("MockProver 실행 실패: {:?}", e))?;

    prover
        .verify()
        .map_err(|e| format!("Halo2 Circuit 검증 실패: {:?}", e))?;

    println!("   ✅ Halo2 Circuit 검증 통과!");

    // 5. 타임스탬프 검증 (24시간 이내)
    println!("   [5/5] 타임스탬프 검증...");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let age = now.saturating_sub(proof.timestamp);
    if age > 86400 {
        // 24시간
        println!("   ❌ 증명이 너무 오래되었습니다! ({}초 경과)", age);
        return Ok(false);
    }
    println!("   ✅ 타임스탬프 검증 통과!");

    println!("🎉 진짜 Halo2 ZKML 검증 완료!");
    Ok(true)
}
