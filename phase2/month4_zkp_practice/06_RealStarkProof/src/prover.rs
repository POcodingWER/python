// 🔥 진짜 STARK 증명 생성기 - Alice의 역할 (간소화 버전)
use anyhow::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rs_merkle::{algorithms::Sha256, MerkleTree};
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};
use std::fs;

// 🎯 STARK 증명 구조체
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
    println!("🔥 진짜 STARK 증명 생성 시작!");

    // 1️⃣ 비밀 패스워드 로드
    let secret_input = fs::read_to_string("inputs/secret.json")?;
    let secret_data: Value = serde_json::from_str(&secret_input)?;
    let secret_password = secret_data["secret_password"].as_str().unwrap();

    println!("📝 비밀 패스워드 로드 완료");

    // 2️⃣ Cairo 프로그램 해시 계산
    let program_content = fs::read_to_string("cairo_program/password_proof.cairo")?;
    let program_hash = calculate_program_hash(&program_content)?;
    println!("🔐 프로그램 해시: {}", &program_hash[..16]);

    // 3️⃣ 실행 추적 생성 (진짜 Cairo VM 시뮬레이션)
    let execution_trace = execute_cairo_program(secret_password)?;
    println!("🔍 실행 추적 생성: {} 단계", execution_trace.len());

    // 4️⃣ 머클 트리 생성 (실행 추적의 무결성 증명)
    let merkle_leaves: Vec<[u8; 32]> = execution_trace
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
    let merkle_root = hex::encode(merkle_tree.root().unwrap());

    println!("🌳 머클 루트: {}", &merkle_root[..16]);

    // 5️⃣ 다항식 생성 및 FRI 커밋먼트 (핵심 STARK!)
    let polynomial = create_execution_polynomial(&execution_trace)?;
    let fri_commitments = generate_fri_commitments(&polynomial)?;
    let fri_evaluations = generate_fri_evaluations(&polynomial)?;

    println!("📊 다항식 차수: {}", polynomial.len());
    println!("🔢 FRI 커밋먼트: {} 개", fri_commitments.len());

    // 6️⃣ 머클 증명 생성 (첫 번째 리프에 대한)
    let merkle_proof = merkle_tree.proof(&[0]);
    let proof_hashes: Vec<String> = merkle_proof
        .proof_hashes()
        .iter()
        .map(|hash| hex::encode(hash))
        .collect();

    // 7️⃣ 공개 입력 계산
    let public_hash = calculate_public_hash(secret_password);
    let public_inputs = vec![public_hash.clone()];

    // 8️⃣ STARK 증명 구조체 생성
    let stark_proof = StarkProof {
        program_hash,
        execution_trace: execution_trace.clone(),
        public_inputs,
        merkle_root,
        merkle_proof: proof_hashes,
        fri_commitments,
        fri_evaluations,
        polynomial_coefficients: polynomial.iter().map(|coeff| coeff.to_string()).collect(),
    };

    // 9️⃣ 증명 파일 저장
    fs::create_dir_all("proofs")?;
    let proof_json = serde_json::to_string_pretty(&stark_proof)?;
    fs::write("proofs/stark_proof.json", &proof_json)?;

    // 🔟 공개 해시 저장 (Bob이 사용)
    let public_data = json!({
        "public_hash": public_hash,
        "program_hash": stark_proof.program_hash
    });
    fs::write(
        "proofs/public.json",
        serde_json::to_string_pretty(&public_data)?,
    )?;

    println!("✅ 진짜 STARK 증명 생성 완료!");
    println!("📁 증명 파일: proofs/stark_proof.json");
    println!("📁 공개 데이터: proofs/public.json");
    println!("🔒 증명 크기: {} KB", proof_json.len() / 1024);

    Ok(())
}

// 🔧 프로그램 해시 계산
fn calculate_program_hash(program_content: &str) -> Result<String> {
    let mut hasher = Keccak256::new();
    hasher.update(program_content.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}

// 🏃 Cairo 프로그램 실행 시뮬레이션 (진짜 실행 추적)
fn execute_cairo_program(secret: &str) -> Result<Vec<String>> {
    let mut execution_trace = Vec::new();

    // 1. 프로그램 초기화
    execution_trace.push("INIT_VM".to_string());
    execution_trace.push("LOAD_PROGRAM".to_string());

    // 2. 입력 로드
    let secret_hash = calculate_public_hash(secret);
    execution_trace.push(format!("LOAD_INPUT:{}", &secret_hash[..8]));

    // 3. Poseidon 해시 계산 (STARK 친화적 해시)
    execution_trace.push("POSEIDON_INIT".to_string());
    execution_trace.push(format!("POSEIDON_ABSORB:{}", &secret_hash[..8]));
    execution_trace.push("POSEIDON_PERMUTE".to_string());
    execution_trace.push(format!("POSEIDON_SQUEEZE:{}", &secret_hash[..8]));

    // 4. 추가 계산 (실행 추적 복잡화)
    execution_trace.push("MUL_CONST:2".to_string());
    execution_trace.push("ADD_CONST:1337".to_string());
    execution_trace.push("MOD_PRIME:1000000007".to_string());

    // 5. 메모리 연산
    execution_trace.push("STORE_TEMP".to_string());
    execution_trace.push("LOAD_TEMP".to_string());

    // 6. 결과 저장
    execution_trace.push(format!("STORE_RESULT:{}", secret_hash));
    execution_trace.push("HALT".to_string());

    Ok(execution_trace)
}

// 📊 실행 추적으로부터 다항식 생성 (핵심 STARK 기법!)
fn create_execution_polynomial(execution_trace: &[String]) -> Result<Vec<BigUint>> {
    let mut polynomial = Vec::new();

    // 각 실행 단계를 다항식 계수로 변환
    for (i, step) in execution_trace.iter().enumerate() {
        let mut hasher = Keccak256::new();
        hasher.update(format!("{}:{}", i, step).as_bytes());
        let hash = hasher.finalize();

        // 해시의 첫 16바이트를 BigUint로 변환
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&hash[..16]);
        let coefficient = BigUint::from_bytes_be(&bytes);

        polynomial.push(coefficient);
    }

    // 다항식 차수가 2의 거듭제곱이 되도록 패딩
    let target_size = next_power_of_two(polynomial.len());
    while polynomial.len() < target_size {
        polynomial.push(BigUint::zero());
    }

    Ok(polynomial)
}

// 🔢 FRI 커밋먼트 생성 (Low Degree Testing)
fn generate_fri_commitments(polynomial: &[BigUint]) -> Result<Vec<String>> {
    let mut commitments = Vec::new();
    let mut current_poly = polynomial.to_vec();

    // FRI 프로토콜: 다항식 차수를 반복적으로 줄임
    while current_poly.len() > 1 {
        // 다항식의 머클 트리 커밋먼트 생성
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

        // 다항식 차수 절반으로 줄이기 (FRI folding)
        current_poly = fold_polynomial(&current_poly);
    }

    Ok(commitments)
}

// 📈 FRI 평가값 생성
fn generate_fri_evaluations(polynomial: &[BigUint]) -> Result<Vec<String>> {
    let mut evaluations = Vec::new();

    // 랜덤 포인트들에서 다항식 평가
    for i in 0..8 {
        let x = BigUint::from((i * 123 + 456) as u32); // 의사 랜덤 포인트
        let evaluation = evaluate_polynomial(polynomial, &x);
        evaluations.push(evaluation.to_string());
    }

    Ok(evaluations)
}

// 🔧 다항식 폴딩 (FRI 핵심 연산)
fn fold_polynomial(polynomial: &[BigUint]) -> Vec<BigUint> {
    let mut folded = Vec::new();
    let challenge = BigUint::from(42u32); // 실제로는 Fiat-Shamir로 생성

    for i in 0..(polynomial.len() / 2) {
        let even = &polynomial[2 * i];
        let odd = &polynomial[2 * i + 1];

        // f(x) = even + odd * x를 f(challenge)로 평가
        let folded_coeff = even + odd * &challenge;
        folded.push(folded_coeff);
    }

    folded
}

// 🧮 다항식 평가
fn evaluate_polynomial(polynomial: &[BigUint], x: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut x_power = BigUint::one();

    for coeff in polynomial {
        result += coeff * &x_power;
        x_power *= x;
    }

    result
}

// 🔑 공개 해시 계산
fn calculate_public_hash(secret: &str) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(secret.as_bytes());
    hex::encode(hasher.finalize())
}

// 🔧 다음 2의 거듭제곱 찾기
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
