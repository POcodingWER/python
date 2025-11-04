// 🔍 ZKML 증명 검증

use std::fs;
use zkml_inference::{verify_zkml_proof, ZKProofData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔍 ZKML 증명 검증                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. 증명 데이터 로드
    println!("📁 [1/3] 증명 데이터 로드 중...");

    let proof_data: ZKProofData = match fs::read_to_string("proofs/zkml_proof.json") {
        Ok(content) => {
            println!("   ✅ proofs/zkml_proof.json 로드 완료!");
            serde_json::from_str(&content)?
        }
        Err(_) => {
            println!("   ❌ proofs/zkml_proof.json 파일이 없습니다!");
            println!("\n💡 먼저 증명을 생성하세요:");
            println!("   cargo run --bin prove");
            return Ok(());
        }
    };

    println!("   📊 증명 정보:");
    println!("      • 증명 해시: {}...", &proof_data.proof_hash[..16]);
    println!("      • 입력 해시: {}...", &proof_data.input_hash[..16]);
    println!("      • 타임스탬프: {}", proof_data.timestamp);

    // 2. 예상 출력 로드
    println!("\n🌍 [2/3] 예상 출력 로드 중...");

    let expected_output: f32 = match fs::read_to_string("proofs/expected_output.txt") {
        Ok(content) => {
            let output = content.trim().parse()?;
            println!("   ✅ proofs/expected_output.txt 로드 완료!");
            println!("   🌍 예상 출력: {:.2}", output);
            output
        }
        Err(_) => {
            println!("   ❌ proofs/expected_output.txt 파일이 없습니다!");
            return Ok(());
        }
    };

    // 3. ZK 증명 검증
    println!("\n🔥 [3/3] ZK 증명 검증 중...");
    println!("   (Halo2 회로 검증 중...)");

    let is_valid = verify_zkml_proof(proof_data.clone(), expected_output)?;

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    if is_valid {
        println!("║              ✅ ZKML 증명 검증 성공! ✅                     ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("\n🎉 검증 완료:");
        println!("   • ML 추론이 정확히 실행되었음을 확인");
        println!("   • 입력 데이터는 비공개 유지");
        println!("   • 모델 가중치는 비공개 유지");
        println!("   • 출력만 검증됨: {:.2}", proof_data.output);
    } else {
        println!("║              ❌ ZKML 증명 검증 실패! ❌                     ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("\n⚠️  증명이 유효하지 않거나 조작되었을 수 있습니다!");
    }

    println!("\n📊 검증 결과:");
    println!("   • 예상 출력: {:.2}", expected_output);
    println!(
        "   • 증명 유효성: {}",
        if is_valid { "✅ 유효" } else { "❌ 무효" }
    );
    println!();

    Ok(())
}
