// 🔐 Groth16 SNARK 데모
// Setup → Prove → Verify 전체 파이프라인

use ark_bn254::{Bn254, Fr};
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_std::rand::thread_rng;
use arkworks_snark::circuit::{
    compute_password_hash, compute_square, PasswordCircuit, SquareCircuit,
};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔐 Groth16 SNARK 데모                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut rng = thread_rng();

    // ========================================================================
    // 예시 1: 비밀번호 해시 증명
    // ========================================================================
    println!("🔑 [1/2] 비밀번호 해시 증명");
    println!("{}", "─".repeat(60));
    println!("   회로: secret * 7 + 13 = hash");
    println!("   목표: secret 값을 숨기면서 hash가 맞음을 증명");
    println!();

    // 비밀 값
    let secret = Fr::from(42u64);
    let hash = compute_password_hash(secret);
    println!("   🔒 비밀 값: {}", secret);
    println!("   ✅ 공개 해시: {}", hash);
    println!();

    // 1. Setup
    println!("   ⚙️  [1/3] Setup (신뢰 설정)...");
    let start = Instant::now();

    let circuit = PasswordCircuit {
        secret: Some(secret),
        hash: Some(hash),
    };

    let pk =
        Groth16::<Bn254>::generate_random_parameters_with_reduction(circuit.clone(), &mut rng)?;
    let vk = pk.vk.clone();
    let setup_time = start.elapsed();
    println!(
        "      ✅ Setup 완료! ({:.2}ms)",
        setup_time.as_secs_f64() * 1000.0
    );
    println!();

    // 2. Prove
    println!("   🔐 [2/3] Prove (증명 생성)...");
    let start = Instant::now();

    let proof = Groth16::<Bn254>::create_random_proof_with_reduction(circuit, &pk, &mut rng)?;
    let prove_time = start.elapsed();
    println!(
        "      ✅ 증명 생성 완료! ({:.2}ms)",
        prove_time.as_secs_f64() * 1000.0
    );
    println!();

    // 3. Verify
    println!("   ✅ [3/3] Verify (증명 검증)...");
    let start = Instant::now();

    let pvk = prepare_verifying_key(&vk);
    let public_inputs = vec![hash];
    let is_valid = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs)?;
    let verify_time = start.elapsed();

    println!(
        "      ✅ 검증 결과: {}",
        if is_valid {
            "성공! ✅"
        } else {
            "실패! ❌"
        }
    );
    println!(
        "      ⚡ 검증 시간: {:.2}ms",
        verify_time.as_secs_f64() * 1000.0
    );
    println!();

    println!("   📊 성능 요약:");
    println!("      • Setup:  {:.2}ms", setup_time.as_secs_f64() * 1000.0);
    println!("      • Prove:  {:.2}ms", prove_time.as_secs_f64() * 1000.0);
    println!(
        "      • Verify: {:.2}ms",
        verify_time.as_secs_f64() * 1000.0
    );
    println!();

    // ========================================================================
    // 예시 2: 제곱 증명
    // ========================================================================
    println!("🔢 [2/2] 제곱 증명");
    println!("{}", "─".repeat(60));
    println!("   회로: x² = y");
    println!("   목표: x 값을 숨기면서 x² = 25 임을 증명");
    println!();

    // 비밀 값
    let x = Fr::from(5u64);
    let y = compute_square(x);
    println!("   🔒 비밀 값 x: {}", x);
    println!("   ✅ 공개 값 y: {}", y);
    println!();

    // 1. Setup
    println!("   ⚙️  [1/3] Setup...");
    let start = Instant::now();

    let circuit = SquareCircuit {
        x: Some(x),
        y: Some(y),
    };

    let pk =
        Groth16::<Bn254>::generate_random_parameters_with_reduction(circuit.clone(), &mut rng)?;
    let vk = pk.vk.clone();
    let setup_time = start.elapsed();
    println!(
        "      ✅ Setup 완료! ({:.2}ms)",
        setup_time.as_secs_f64() * 1000.0
    );
    println!();

    // 2. Prove
    println!("   🔐 [2/3] Prove...");
    let start = Instant::now();

    let proof = Groth16::<Bn254>::create_random_proof_with_reduction(circuit, &pk, &mut rng)?;
    let prove_time = start.elapsed();
    println!(
        "      ✅ 증명 생성 완료! ({:.2}ms)",
        prove_time.as_secs_f64() * 1000.0
    );
    println!();

    // 3. Verify
    println!("   ✅ [3/3] Verify...");
    let start = Instant::now();

    let pvk = prepare_verifying_key(&vk);
    let public_inputs = vec![y];
    let is_valid = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs)?;
    let verify_time = start.elapsed();

    println!(
        "      ✅ 검증 결과: {}",
        if is_valid {
            "성공! ✅"
        } else {
            "실패! ❌"
        }
    );
    println!(
        "      ⚡ 검증 시간: {:.2}ms",
        verify_time.as_secs_f64() * 1000.0
    );
    println!();

    println!("   📊 성능 요약:");
    println!("      • Setup:  {:.2}ms", setup_time.as_secs_f64() * 1000.0);
    println!("      • Prove:  {:.2}ms", prove_time.as_secs_f64() * 1000.0);
    println!(
        "      • Verify: {:.2}ms",
        verify_time.as_secs_f64() * 1000.0
    );
    println!();

    // ========================================================================
    // 요약
    // ========================================================================
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    ✅ 데모 완료! ✅                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎓 배운 내용:");
    println!("  1. ✅ Groth16 SNARK 전체 파이프라인");
    println!("  2. ✅ Setup → Prove → Verify");
    println!("  3. ✅ 비밀 값을 숨기면서 증명");
    println!("  4. ✅ 초고속 검증 (< 5ms)");
    println!();
    println!("💡 핵심:");
    println!("  • 검증자는 비밀 값(secret, x)을 모름!");
    println!("  • 공개 값(hash, y)만으로 검증 성공!");
    println!("  • 이것이 바로 영지식 증명! 🔐");
    println!();

    Ok(())
}
