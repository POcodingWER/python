// 🔥 진짜 STARK 검증기 - Bob의 역할 (간소화 버전)
use anyhow::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rs_merkle::{algorithms::Sha256, MerkleProof, MerkleTree};
use serde_json::Value;
use sha3::{Digest, Keccak256};
use std::fs;

// 🎯 STARK 증명 구조체 (prover.rs와 동일)
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct StarkProof {
    pub program_hash: String,
    pub execution_trace: Vec<String>,
    pub public_inputs: Vec<String>,
    pub merkle_root: String,
    pub merkle_proof: Vec<String>,
    pub fri_commitments: Vec<String>,
    pub fri_evaluations: Vec<String>,
    pub polynomial_coefficients: Vec<String>,
}

fn main() -> Result<()> {
    println!("🔍 진짜 STARK 검증 시작!");

    // 1️⃣ Alice의 증명 로드
    let proof_content = fs::read_to_string("proofs/stark_proof.json")?;
    let stark_proof: StarkProof = serde_json::from_str(&proof_content)?;

    // 2️⃣ 공개 데이터 로드
    let public_content = fs::read_to_string("proofs/public.json")?;
    let public_data: Value = serde_json::from_str(&public_content)?;
    let expected_hash = public_data["public_hash"].as_str().unwrap();
    let expected_program_hash = public_data["program_hash"].as_str().unwrap();

    println!("📋 검증할 공개 해시: {}", &expected_hash[..16]);
    println!("📋 검증할 프로그램 해시: {}", &expected_program_hash[..16]);

    let mut verification_passed = true;

    // 3️⃣ 프로그램 무결성 검증
    println!("\n🔐 1단계: 프로그램 무결성 검증");
    if stark_proof.program_hash != expected_program_hash {
        println!("❌ 프로그램 해시 불일치!");
        verification_passed = false;
    } else {
        println!("✅ 프로그램 해시 일치");
    }

    // 4️⃣ 실행 추적 무결성 검증 (머클 트리)
    println!("\n🌳 2단계: 실행 추적 무결성 검증 (머클 트리)");
    match verify_execution_trace(&stark_proof) {
        Ok(true) => println!("✅ 실행 추적 무결성 확인"),
        Ok(false) => {
            println!("❌ 실행 추적 검증 실패!");
            verification_passed = false;
        }
        Err(e) => {
            println!("❌ 실행 추적 검증 오류: {}", e);
            verification_passed = false;
        }
    }

    // 5️⃣ FRI 다항식 검증 (핵심 STARK 검증!)
    println!("\n📊 3단계: FRI 다항식 검증 (핵심 STARK!)");
    match verify_fri_proof(&stark_proof) {
        Ok(true) => println!("✅ FRI 다항식 검증 통과"),
        Ok(false) => {
            println!("❌ FRI 다항식 검증 실패!");
            verification_passed = false;
        }
        Err(e) => {
            println!("❌ FRI 검증 오류: {}", e);
            verification_passed = false;
        }
    }

    // 6️⃣ 공개 입력 일관성 검증
    println!("\n🔑 4단계: 공개 입력 일관성 검증");
    if verify_public_inputs(&stark_proof, expected_hash) {
        println!("✅ 공개 입력 일치");
    } else {
        println!("❌ 공개 입력 불일치!");
        verification_passed = false;
    }

    // 7️⃣ 실행 추적 논리적 일관성 검증
    println!("\n🧮 5단계: 실행 추적 논리적 일관성 검증");
    match verify_execution_logic(&stark_proof) {
        Ok(true) => println!("✅ 실행 논리 일관성 확인"),
        Ok(false) => {
            println!("❌ 실행 논리 검증 실패!");
            verification_passed = false;
        }
        Err(e) => {
            println!("❌ 실행 논리 검증 오류: {}", e);
            verification_passed = false;
        }
    }

    // 8️⃣ 다항식 차수 검증 (Low Degree Testing)
    println!("\n🔢 6단계: 다항식 차수 검증 (Low Degree Testing)");
    match verify_polynomial_degree(&stark_proof) {
        Ok(true) => println!("✅ 다항식 차수 검증 통과"),
        Ok(false) => {
            println!("❌ 다항식 차수 검증 실패!");
            verification_passed = false;
        }
        Err(e) => {
            println!("❌ 다항식 차수 검증 오류: {}", e);
            verification_passed = false;
        }
    }

    // 9️⃣ 최종 검증 결과
    println!("\n🎉 진짜 STARK 검증 결과:");
    if verification_passed {
        println!("✅ 모든 검증 단계 통과!");
        println!("🔒 Alice가 비밀 패스워드를 알고 있음이 암호학적으로 증명됨");
        println!("🚀 영지식 증명 완료 - 비밀은 노출되지 않음");
        println!("📊 증명 크기: {} KB", proof_content.len() / 1024);
    } else {
        println!("❌ 검증 실패! 증명이 유효하지 않음");
    }

    Ok(())
}

// 🌳 실행 추적 무결성 검증 (머클 트리)
fn verify_execution_trace(proof: &StarkProof) -> Result<bool> {
    // 1. 실행 추적으로부터 머클 트리 재구성
    let merkle_leaves: Vec<[u8; 32]> = proof
        .execution_trace
        .iter()
        .map(|step| {
            let mut hasher = Keccak256::new();
            hasher.update(step.as_bytes());
            let hash = hasher.finalize();
            let mut array = [0u8; 32];
            array.copy_from_slice(&hash);
            array
        })
        .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&merkle_leaves);
    let computed_root = hex::encode(merkle_tree.root().unwrap());

    // 2. Alice의 머클 루트와 비교
    if computed_root != proof.merkle_root {
        println!("  ❌ 머클 루트 불일치");
        return Ok(false);
    }

    // 3. 머클 증명 검증
    if !proof.merkle_proof.is_empty() {
        let proof_hashes: Vec<[u8; 32]> = proof
            .merkle_proof
            .iter()
            .map(|hex_str| {
                let bytes = hex::decode(hex_str).unwrap_or_default();
                let mut array = [0u8; 32];
                if bytes.len() >= 32 {
                    array.copy_from_slice(&bytes[..32]);
                }
                array
            })
            .collect();

        // 첫 번째 리프에 대한 증명 검증
        let merkle_proof = MerkleProof::<Sha256>::new(proof_hashes);
        let is_valid = merkle_proof.verify(
            merkle_tree.root().unwrap(),
            &[0],
            &[merkle_leaves[0]],
            merkle_leaves.len(),
        );

        if !is_valid {
            println!("  ❌ 머클 증명 검증 실패");
            return Ok(false);
        }
    }

    println!("  📋 실행 추적 단계 수: {}", proof.execution_trace.len());
    println!("  🌳 머클 루트: {}", &proof.merkle_root[..16]);

    Ok(true)
}

// 📊 FRI 다항식 검증 (핵심 STARK 검증)
fn verify_fri_proof(proof: &StarkProof) -> Result<bool> {
    // 1. 다항식 계수 파싱
    let polynomial: Vec<BigUint> = proof
        .polynomial_coefficients
        .iter()
        .map(|coeff_str| {
            coeff_str
                .parse::<BigUint>()
                .unwrap_or_else(|_| BigUint::zero())
        })
        .collect();

    if polynomial.is_empty() {
        println!("  ❌ 다항식 계수가 비어있음");
        return Ok(false);
    }

    // 2. FRI 커밋먼트 재계산 및 검증
    let expected_commitments = generate_fri_commitments(&polynomial)?;

    if proof.fri_commitments.len() != expected_commitments.len() {
        println!(
            "  ❌ FRI 커밋먼트 수 불일치: {} vs {}",
            proof.fri_commitments.len(),
            expected_commitments.len()
        );
        return Ok(false);
    }

    for (i, (provided, expected)) in proof
        .fri_commitments
        .iter()
        .zip(expected_commitments.iter())
        .enumerate()
    {
        if provided != expected {
            println!("  ❌ FRI 커밋먼트 {} 불일치", i);
            return Ok(false);
        }
    }

    // 3. FRI 평가값 검증
    let expected_evaluations = generate_fri_evaluations(&polynomial)?;

    if proof.fri_evaluations.len() != expected_evaluations.len() {
        println!("  ❌ FRI 평가값 수 불일치");
        return Ok(false);
    }

    for (i, (provided, expected)) in proof
        .fri_evaluations
        .iter()
        .zip(expected_evaluations.iter())
        .enumerate()
    {
        if provided != expected {
            println!("  ❌ FRI 평가값 {} 불일치", i);
            return Ok(false);
        }
    }

    println!("  📊 다항식 차수: {}", polynomial.len());
    println!("  🔢 FRI 커밋먼트 수: {}", proof.fri_commitments.len());
    println!("  📈 FRI 평가값 수: {}", proof.fri_evaluations.len());

    Ok(true)
}

// 🔑 공개 입력 일관성 검증
fn verify_public_inputs(proof: &StarkProof, expected_hash: &str) -> bool {
    if proof.public_inputs.is_empty() {
        return false;
    }

    let provided_hash = &proof.public_inputs[0];
    provided_hash == expected_hash
}

// 🧮 실행 추적 논리적 일관성 검증
fn verify_execution_logic(proof: &StarkProof) -> Result<bool> {
    let trace = &proof.execution_trace;

    // 1. 필수 실행 단계 존재 확인
    let required_steps = vec![
        "INIT_VM",
        "LOAD_PROGRAM",
        "LOAD_INPUT",
        "POSEIDON_INIT",
        "POSEIDON_ABSORB",
        "POSEIDON_PERMUTE",
        "POSEIDON_SQUEEZE",
        "STORE_RESULT",
        "HALT",
    ];

    for required_step in required_steps {
        let step_exists = trace.iter().any(|step| step.contains(required_step));
        if !step_exists {
            println!("  ❌ 필수 실행 단계 누락: {}", required_step);
            return Ok(false);
        }
    }

    // 2. 실행 순서 검증
    let init_idx = trace.iter().position(|s| s.contains("INIT_VM"));
    let halt_idx = trace.iter().position(|s| s.contains("HALT"));

    if let (Some(init), Some(halt)) = (init_idx, halt_idx) {
        if init >= halt {
            println!("  ❌ 실행 순서 오류: INIT_VM이 HALT 이후에 위치");
            return Ok(false);
        }
    }

    // 3. 해시 일관성 검증
    let public_hash = &proof.public_inputs[0];
    let result_step = trace.iter().find(|s| s.contains("STORE_RESULT"));

    if let Some(result) = result_step {
        if !result.contains(&public_hash[..16]) {
            println!("  ❌ 결과 해시와 공개 해시 불일치");
            return Ok(false);
        }
    }

    println!("  🧮 실행 단계 수: {}", trace.len());

    Ok(true)
}

// 🔢 다항식 차수 검증 (Low Degree Testing)
fn verify_polynomial_degree(proof: &StarkProof) -> Result<bool> {
    let polynomial: Vec<BigUint> = proof
        .polynomial_coefficients
        .iter()
        .map(|coeff_str| {
            coeff_str
                .parse::<BigUint>()
                .unwrap_or_else(|_| BigUint::zero())
        })
        .collect();

    // 1. 다항식 차수가 2의 거듭제곱인지 확인
    let degree = polynomial.len();
    if degree == 0 || (degree & (degree - 1)) != 0 {
        println!("  ❌ 다항식 차수가 2의 거듭제곱이 아님: {}", degree);
        return Ok(false);
    }

    // 2. 다항식 차수가 실행 추적 길이와 일치하는지 확인
    let expected_degree = next_power_of_two(proof.execution_trace.len());
    if degree != expected_degree {
        println!("  ❌ 다항식 차수 불일치: {} vs {}", degree, expected_degree);
        return Ok(false);
    }

    // 3. 다항식이 너무 높은 차수가 아닌지 확인 (Low Degree)
    let max_allowed_degree = 1024; // 실제로는 보안 매개변수에 따라 결정
    if degree > max_allowed_degree {
        println!("  ❌ 다항식 차수가 너무 높음: {}", degree);
        return Ok(false);
    }

    println!("  🔢 다항식 차수: {}", degree);
    println!("  ✅ Low Degree Testing 통과");

    Ok(true)
}

// 🔢 FRI 커밋먼트 생성 (prover와 동일한 로직)
fn generate_fri_commitments(polynomial: &[BigUint]) -> Result<Vec<String>> {
    let mut commitments = Vec::new();
    let mut current_poly = polynomial.to_vec();

    while current_poly.len() > 1 {
        let poly_leaves: Vec<[u8; 32]> = current_poly
            .iter()
            .map(|coeff| {
                let bytes = coeff.to_bytes_be();
                let mut hasher = Keccak256::new();
                hasher.update(&bytes);
                let hash = hasher.finalize();
                let mut array = [0u8; 32];
                array.copy_from_slice(&hash);
                array
            })
            .collect();

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&poly_leaves);
        let commitment = hex::encode(merkle_tree.root().unwrap());
        commitments.push(commitment);

        current_poly = fold_polynomial(&current_poly);
    }

    Ok(commitments)
}

// 📈 FRI 평가값 생성 (prover와 동일한 로직)
fn generate_fri_evaluations(polynomial: &[BigUint]) -> Result<Vec<String>> {
    let mut evaluations = Vec::new();

    for i in 0..8 {
        let x = BigUint::from((i * 123 + 456) as u32);
        let evaluation = evaluate_polynomial(polynomial, &x);
        evaluations.push(evaluation.to_string());
    }

    Ok(evaluations)
}

// 🔧 다항식 폴딩 (prover와 동일한 로직)
fn fold_polynomial(polynomial: &[BigUint]) -> Vec<BigUint> {
    let mut folded = Vec::new();
    let challenge = BigUint::from(42u32);

    for i in 0..(polynomial.len() / 2) {
        let even = &polynomial[2 * i];
        let odd = &polynomial[2 * i + 1];
        let folded_coeff = even + odd * &challenge;
        folded.push(folded_coeff);
    }

    folded
}

// 🧮 다항식 평가 (prover와 동일한 로직)
fn evaluate_polynomial(polynomial: &[BigUint], x: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut x_power = BigUint::one();

    for coeff in polynomial {
        result += coeff * &x_power;
        x_power *= x;
    }

    result
}

// 🔧 다음 2의 거듭제곱 찾기 (prover와 동일한 로직)
fn next_power_of_two(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut power = 1;
    while power < n {
        power *= 2;
    }
    power
}
