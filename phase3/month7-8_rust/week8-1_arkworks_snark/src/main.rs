// 🔐 타원곡선 연산 데모
// BN254, BLS12-381 비교

use ark_bls12_381::{Bls12_381, Fr as BlsFr, G1Projective as BlsG1};
use ark_bn254::{Bn254, Fr as BnFr, G1Projective as BnG1};
use ark_ec::CurveGroup;
use ark_ff::Field;
use ark_std::UniformRand;
use arkworks_snark::{
    benchmark_operation, bls12_381_stats, bn254_stats, curve_operations, field_operations,
    print_benchmark_result,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔐 arkworks 타원곡선 연산 데모                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut rng = ark_std::test_rng();

    // ========================================================================
    // 1. 곡선 정보
    // ========================================================================
    println!("📊 [1/4] 타원곡선 정보");
    println!("{}", "─".repeat(60));

    let bn254 = bn254_stats();
    println!("🔵 BN254:");
    println!("   • 필드 크기: {} bits", bn254.field_size);
    println!("   • 스칼라 필드: {} bits", bn254.scalar_field_size);
    println!("   • 보안 수준: {} bits", bn254.security_level);
    println!("   • 용도: zkSNARK 표준 (Ethereum, Polygon)");

    println!();

    let bls12_381 = bls12_381_stats();
    println!("🟣 BLS12-381:");
    println!("   • 필드 크기: {} bits", bls12_381.field_size);
    println!("   • 스칼라 필드: {} bits", bls12_381.scalar_field_size);
    println!("   • 보안 수준: {} bits", bls12_381.security_level);
    println!("   • 용도: Ethereum 2.0, Zcash");

    println!();

    // ========================================================================
    // 2. 필드 연산 (BN254)
    // ========================================================================
    println!("🔢 [2/4] 필드 연산 (BN254)");
    println!("{}", "─".repeat(60));

    let a = BnFr::from(12345u64);
    let b = BnFr::from(67890u64);

    let (add, sub, mul, inv) = field_operations(a, b);

    println!("   a = 12345");
    println!("   b = 67890");
    println!();
    println!("   a + b = {:?}", add);
    println!("   a - b = {:?}", sub);
    println!("   a * b = {:?}", mul);
    println!("   a⁻¹   = {:?}", inv);

    println!();

    // ========================================================================
    // 3. 곡선 포인트 연산 (BN254)
    // ========================================================================
    println!("📐 [3/4] 곡선 포인트 연산 (BN254)");
    println!("{}", "─".repeat(60));

    let p = BnG1::rand(&mut rng);
    let q = BnG1::rand(&mut rng);
    let scalar = BnFr::rand(&mut rng);

    let (add, double_p, scalar_mul) = curve_operations(p, q, scalar);

    println!("   P = 랜덤 포인트");
    println!("   Q = 랜덤 포인트");
    println!("   s = 랜덤 스칼라");
    println!();
    println!("   P + Q     = {:?}", add.into_affine());
    println!("   2P        = {:?}", double_p.into_affine());
    println!("   s * P     = {:?}", scalar_mul.into_affine());

    println!();

    // ========================================================================
    // 4. 성능 벤치마크
    // ========================================================================
    println!("⚡ [4/4] 성능 벤치마크");
    println!("{}", "─".repeat(60));

    // BN254 필드 곱셈
    let result = benchmark_operation("BN254 필드 곱셈", 10000, || {
        let a = BnFr::rand(&mut rng);
        let b = BnFr::rand(&mut rng);
        let _ = a * b;
    });
    print_benchmark_result(&result);
    println!();

    // BN254 곡선 덧셈
    let result = benchmark_operation("BN254 곡선 덧셈", 10000, || {
        let p = BnG1::rand(&mut rng);
        let q = BnG1::rand(&mut rng);
        let _ = p + q;
    });
    print_benchmark_result(&result);
    println!();

    // BN254 스칼라 곱셈
    let result = benchmark_operation("BN254 스칼라 곱셈", 1000, || {
        let p = BnG1::rand(&mut rng);
        let s = BnFr::rand(&mut rng);
        let _ = p * s;
    });
    print_benchmark_result(&result);
    println!();

    // BLS12-381 필드 곱셈
    let result = benchmark_operation("BLS12-381 필드 곱셈", 10000, || {
        let a = BlsFr::rand(&mut rng);
        let b = BlsFr::rand(&mut rng);
        let _ = a * b;
    });
    print_benchmark_result(&result);
    println!();

    // BLS12-381 곡선 덧셈
    let result = benchmark_operation("BLS12-381 곡선 덧셈", 10000, || {
        let p = BlsG1::rand(&mut rng);
        let q = BlsG1::rand(&mut rng);
        let _ = p + q;
    });
    print_benchmark_result(&result);
    println!();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    ✅ 데모 완료! ✅                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎓 배운 내용:");
    println!("  1. ✅ 타원곡선 기본 개념 (BN254, BLS12-381)");
    println!("  2. ✅ 필드 연산 (덧셈, 곱셈, 역원)");
    println!("  3. ✅ 곡선 포인트 연산 (덧셈, 배가, 스칼라 곱)");
    println!("  4. ✅ 성능 측정 (μs 단위)");
    println!();
    println!("🚀 다음 단계:");
    println!("   cargo run --bin groth16_demo");
    println!();
}
