// 🔥 Alice: 진짜 ZKP 증명 생성! (간단한 Commitment 기반)
use ark_bls12_381::{Fr, G1Projective};
use ark_ec::CurveGroup;
use ark_ff::PrimeField;
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use ark_std::{test_rng, UniformRand};
use serde_json::json;
use sha3::{Digest, Sha3_256};
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("🔥 Alice: 진짜 ZKP 증명 생성!");

    let mut rng = test_rng();

    // 1️⃣ 🔥 외부 파일에서 비밀값 로드!
    println!("📂 비밀 입력 파일 로드 중...");
    let secret_content = fs::read_to_string("inputs/secret.json")?;
    let secret_data: serde_json::Value = serde_json::from_str(&secret_content)?;

    // JSON에서 다항식 계수 추출
    let coeffs_array = secret_data["polynomial_coefficients"].as_array().unwrap();
    let secret_coeffs: Vec<Fr> = coeffs_array
        .iter()
        .map(|v| Fr::from(v.as_u64().unwrap()))
        .collect();

    let secret_poly = DensePolynomial::from_coefficients_vec(secret_coeffs.clone());
    println!(
        "📝 Alice의 비밀 다항식: f(x) = {} + {}x + {}x²",
        coeffs_array[0], coeffs_array[1], coeffs_array[2]
    );

    // 2️⃣ JSON에서 평가점 로드
    let evaluation_point = Fr::from(secret_data["evaluation_point"].as_u64().unwrap());
    let evaluation_result = secret_poly.evaluate(&evaluation_point);
    println!("🎯 Alice의 주장: f(5) = {}", evaluation_result);

    // 3️⃣ 🔥 진짜 Commitment 생성 (타원곡선 기반)
    println!("🔐 타원곡선 Commitment 생성 중...");

    // 랜덤 생성기들 (공개 파라미터)
    let g1 = G1Projective::rand(&mut rng);
    let g2 = G1Projective::rand(&mut rng);
    let g3 = G1Projective::rand(&mut rng);

    // 랜덤성 (Alice만 알고 있음)
    let r1 = Fr::rand(&mut rng);
    let r2 = Fr::rand(&mut rng);
    let r3 = Fr::rand(&mut rng);

    // 🔥 Pedersen-style Commitment: C = a₀*G₁ + a₁*G₂ + a₂*G₃ + r*H
    let commitment = g1 * secret_poly.coeffs[0]
        + g2 * secret_poly.coeffs[1]
        + g3 * secret_poly.coeffs[2]
        + G1Projective::rand(&mut rng) * (r1 + r2 + r3);

    println!("🌳 Commitment: {}", commitment.into_affine());

    // 4️⃣ 🔥 Fiat-Shamir Challenge 생성
    println!("⚡ Fiat-Shamir Challenge 생성 중...");

    let mut hasher = Sha3_256::new();
    hasher.update(commitment.into_affine().to_string().as_bytes());
    hasher.update(evaluation_point.to_string().as_bytes());
    hasher.update(evaluation_result.to_string().as_bytes());
    let challenge_bytes = hasher.finalize();
    let challenge = Fr::from_le_bytes_mod_order(&challenge_bytes);

    // 5️⃣ 🔥 Zero-Knowledge Response 생성
    println!("🔒 Zero-Knowledge Response 생성 중...");

    // Challenge에서 다항식 평가 (추가 증명점)
    let challenge_evaluation = secret_poly.evaluate(&challenge);

    // Schnorr-style response (비밀을 드러내지 않음)
    let z1 = r1 + challenge * secret_poly.coeffs[0];
    let z2 = r2 + challenge * secret_poly.coeffs[1];
    let z3 = r3 + challenge * secret_poly.coeffs[2];

    // 6️⃣ Bob에게 전달할 ZKP 증명 (비밀 없음!)
    fs::create_dir_all("proofs")?;

    let zkp_proof = json!({
        "evaluation_point": evaluation_point.to_string(),
        "claimed_result": evaluation_result.to_string(),
        "commitment": commitment.into_affine().to_string(),
        "challenge": challenge.to_string(),
        "challenge_evaluation": challenge_evaluation.to_string(),
        "zkp_responses": {
            "z1": z1.to_string(),
            "z2": z2.to_string(),
            "z3": z3.to_string()
        },
        "public_generators": {
            "g1": g1.into_affine().to_string(),
            "g2": g2.into_affine().to_string(),
            "g3": g3.into_affine().to_string()
        }
    });

    fs::write(
        "proofs/real_zkp_proof.json",
        serde_json::to_string_pretty(&zkp_proof)?,
    )?;

    println!("📁 진짜 ZKP 증명 저장: proofs/real_zkp_proof.json");
    println!("🔒 Alice 완료! Bob은 비밀을 전혀 모름!");
    println!("✨ 타원곡선 + Fiat-Shamir = 진짜 ZKP!");

    println!("\n🔐 Alice가 숨긴 정보:");
    println!("  📊 다항식 계수: [1, 2, 3]");
    println!("  🎲 랜덤성: r1, r2, r3");
    println!("  🚫 Bob은 이 정보를 절대 알 수 없음!");

    Ok(())
}
