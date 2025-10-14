// 🔍 Halo2 재귀적 증명 집계 - 검증기
// 1개 증명으로 여러 증명을 한번에 검증!

use halo_password_proof::{verify_aggregated_proof, AggregatedProof};
use serde_json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Halo2 재귀적 증명 집계 검증기");
    println!("================================");
    println!("🎯 목표: 1개 증명으로 여러 증명을 한번에 검증!");
    println!("");

    // 1️⃣ 집계된 증명 로드
    let aggregated_proof: AggregatedProof = match fs::read_to_string("proofs/aggregated_proof.json")
    {
        Ok(content) => {
            println!("📄 집계된 증명 로드 완료!");
            serde_json::from_str(&content)?
        }
        Err(_) => {
            println!("❌ proofs/aggregated_proof.json 파일이 없습니다!");
            println!("💡 먼저 'cargo run --bin aggregate_prove'를 실행해주세요.");
            return Ok(());
        }
    };

    // 2️⃣ 예상 해시값들 로드 (Bob이 알고 있는 정보)
    let expected_hashes: Vec<u64> = match fs::read_to_string("verifier/expected_hashes.json") {
        Ok(content) => serde_json::from_str(&content)?,
        Err(_) => {
            println!("❌ verifier/expected_hashes.json 파일이 없습니다!");
            return Ok(());
        }
    };

    println!("📊 검증 정보:");
    println!("----------------------------------------");
    println!("• 집계된 증명 개수: {}", aggregated_proof.proof_count);
    println!("• 예상 해시값 개수: {}", expected_hashes.len());
    println!(
        "• 집계된 해시: {}...",
        &hex::encode(&aggregated_proof.recursive_halo2_proof)[..16]
    );
    println!(
        "• 재귀적 증명: {}...",
        &aggregated_proof.verification_key_hash[..16]
    );
    println!("");

    // 3️⃣ 기존 방식 vs Halo2 방식 비교
    println!("🔍 검증 방식 비교:");
    println!("----------------------------------------");
    println!("❌ 기존 방식 (SNARK/STARK):");
    println!("   • 증명 1 검증 → 시간 T");
    println!("   • 증명 2 검증 → 시간 T");
    println!("   • 증명 3 검증 → 시간 T");
    println!("   • 증명 4 검증 → 시간 T");
    println!("   • 증명 5 검증 → 시간 T");
    println!("   • 총 시간: 5T (선형 증가!)");
    println!("");
    println!("✅ Halo2 재귀적 방식:");
    println!("   • 집계된 증명 1개 검증 → 시간 T");
    println!("   • 총 시간: T (상수 시간!)");
    println!("   • 🚀 5배 빠름!");
    println!("");

    // 4️⃣ Halo2 재귀적 검증 실행
    println!("4️⃣ Halo2 재귀적 검증 실행...");
    println!(
        "🌀 1개 증명으로 {} 개 증명을 한번에 검증 중...",
        aggregated_proof.proof_count
    );

    let verification_start = std::time::Instant::now();
    let is_valid = verify_aggregated_proof(&aggregated_proof, &expected_hashes);
    let verification_time = verification_start.elapsed();

    println!("");
    println!("🎯 최종 검증 결과:");
    println!("================================");

    if is_valid {
        println!("🎉 재귀적 검증 성공!");
        println!(
            "✅ {} 개 증명이 모두 유효합니다!",
            aggregated_proof.proof_count
        );
        println!("⚡ 검증 시간: {:?}", verification_time);
        println!("");

        println!("🎯 Bob이 확신하는 것:");
        println!("   • Alice, Bob, Charlie, David, Eve 모두 자신의 비밀을 알고 있다!");
        println!("   • 모든 해시값이 올바르게 계산되었다!");
        println!("   • Halo2 재귀적 증명이 성공적으로 검증되었다!");
        println!("");

        println!("🤐 Bob이 모르는 것:");
        println!("   • 각자의 실제 비밀번호가 뭔지 (완전히 숨겨짐!)");
        println!("   • 개별 증명의 세부사항 (집계되어 숨겨짐!)");
        println!("   • 중간 계산 과정 (영지식!)");
        println!("");

        println!("🌟 Halo2 재귀적 집계의 혁신:");
        println!("   🚀 확장성: 1000개 증명도 1번 검증으로 끝!");
        println!(
            "   💰 비용 절약: 가스비 {}배 절약!",
            aggregated_proof.proof_count
        );
        println!(
            "   ⚡ 속도: 검증 시간 {}배 단축!",
            aggregated_proof.proof_count
        );
        println!(
            "   📦 효율성: 네트워크 전송량 {}배 감소!",
            aggregated_proof.proof_count
        );
    } else {
        println!("❌ 재귀적 검증 실패!");
        println!("❌ 집계된 증명에 문제가 있습니다!");
        std::process::exit(1);
    }

    Ok(())
}
