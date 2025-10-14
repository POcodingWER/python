// 🌀 Halo2 재귀적 증명 집계 - 증명 생성기
// 여러 개의 증명을 1개로 압축!

use halo_password_proof::{aggregate_proofs, generate_individual_proof, IndividualProof};
use serde_json;
use std::fs;

// 📦 JSON 입력 구조
#[derive(Debug, serde::Deserialize)]
struct SecretInput {
    id: u32,
    name: String,
    secret_password: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌀 Halo2 재귀적 증명 집계 시스템");
    println!("================================");
    println!("🎯 목표: 여러 증명을 1개 증명으로 압축!");
    println!("");

    // 1️⃣ JSON 파일에서 비밀 입력들 로드
    println!("1️⃣ JSON 파일에서 비밀 입력들 로드 중...");

    let secrets_input: Vec<SecretInput> = match fs::read_to_string("prover/secrets_input.json") {
        Ok(content) => {
            println!("📄 prover/secrets_input.json 로드 완료!");
            serde_json::from_str(&content)?
        }
        Err(_) => {
            println!("❌ prover/secrets_input.json 파일이 없습니다!");
            println!("💡 파일을 만들어주세요. 예:");
            println!("   [{{\"id\": 1, \"name\": \"Alice\", \"secret_password\": 1111}}]");
            return Ok(());
        }
    };

    println!("📊 로드된 비밀 개수: {}", secrets_input.len());
    println!("");

    // 2️⃣ 개별 증명들 생성
    println!("2️⃣ 개별 증명들 생성 중...");
    let mut individual_proofs: Vec<IndividualProof> = Vec::new();

    for secret_data in secrets_input {
        let proof = generate_individual_proof(secret_data.id, secret_data.secret_password);
        println!(
            "   🔒 {} (ID: {}) 증명 생성: 해시={}",
            secret_data.name, secret_data.id, proof.public_hash
        );
        individual_proofs.push(proof);
    }

    println!("✅ {} 개의 개별 증명 생성 완료!", individual_proofs.len());
    println!("");

    // 3️⃣ Halo2 재귀적 집계 실행
    println!("3️⃣ Halo2 재귀적 증명 집계 실행...");
    println!(
        "🌀 {} 개 증명 → 1개 증명으로 압축 중...",
        individual_proofs.len()
    );

    let aggregated_proof = aggregate_proofs(individual_proofs.clone());

    println!("");
    println!("🎉 재귀적 집계 완료!");
    println!("📊 집계 결과:");
    println!("   • 원본 증명 개수: {}", aggregated_proof.proof_count);
    println!(
        "   • 집계된 해시: {}...",
        &hex::encode(&aggregated_proof.recursive_halo2_proof)[..16]
    );
    println!(
        "   • 재귀적 증명: {}...",
        &aggregated_proof.verification_key_hash[..16]
    );
    println!("");

    // 4️⃣ 파일 저장
    fs::create_dir_all("proofs")?;

    // 개별 증명들 저장
    for (_i, proof) in individual_proofs.iter().enumerate() {
        let filename = format!("proofs/individual_proof_{}.json", proof.id);
        fs::write(&filename, serde_json::to_string_pretty(proof)?)?;
        println!("📄 개별 증명 {} 저장: {}", proof.id, filename);
    }

    // 집계된 증명 저장
    fs::write(
        "proofs/aggregated_proof.json",
        serde_json::to_string_pretty(&aggregated_proof)?,
    )?;
    println!("📄 집계된 증명 저장: proofs/aggregated_proof.json");

    // 공개 해시값들 저장 (검증자용)
    let public_hashes: Vec<u64> = individual_proofs.iter().map(|p| p.public_hash).collect();
    fs::write(
        "verifier/expected_hashes.json",
        serde_json::to_string_pretty(&public_hashes)?,
    )?;
    println!("📄 검증자용 해시값들 저장: verifier/expected_hashes.json");

    println!("");
    println!("🎯 Halo2 재귀적 집계의 장점:");
    println!(
        "   ✅ 저장 공간: {} 개 파일 → 1개 파일",
        individual_proofs.len()
    );
    println!(
        "   ✅ 검증 시간: {} 번 검증 → 1번 검증",
        individual_proofs.len()
    );
    println!(
        "   ✅ 네트워크: {} 번 전송 → 1번 전송",
        individual_proofs.len()
    );
    println!("   ✅ 가스비: {} 배 절약!", individual_proofs.len());

    println!("");
    println!("📦 Bob에게 보낼 파일:");
    println!("   • proofs/aggregated_proof.json (집계된 1개 증명만!)");
    println!("🚫 개별 증명들은 보내지 않음:");
    println!("   • proofs/individual_proof_*.json (불필요!)");

    Ok(())
}
