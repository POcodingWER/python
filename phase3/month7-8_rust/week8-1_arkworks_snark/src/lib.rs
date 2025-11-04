// 🔐 arkworks: Rust ZK 암호학 라이브러리
// 타원곡선 연산 + SNARK 프로토콜

use ark_ec::{pairing::Pairing, CurveGroup};
use ark_ff::{Field, PrimeField};
use ark_std::rand::RngCore;
use serde::{Deserialize, Serialize};

// 회로 모듈
pub mod circuit;

// ============================================================================
// 1. 타원곡선 기본 연산
// ============================================================================

/// 필드 연산 데모
pub fn field_operations<F: Field>(a: F, b: F) -> (F, F, F, F) {
    let add = a + b;
    let sub = a - b;
    let mul = a * b;
    let inv = a.inverse().unwrap_or(F::zero());

    (add, sub, mul, inv)
}

/// 곡선 포인트 연산 데모
pub fn curve_operations<G: CurveGroup>(p: G, q: G, scalar: G::ScalarField) -> (G, G, G) {
    let add = p + q;
    let double_p = p.double();
    let scalar_mul = p * scalar;

    (add, double_p, scalar_mul)
}

/// Pairing 연산 데모
pub fn pairing_demo<E: Pairing>(g1: E::G1, g2: E::G2) -> E::TargetField {
    E::pairing(g1, g2).0
}

// ============================================================================
// 2. 데이터 구조
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveStats {
    pub curve_name: String,
    pub field_size: usize,
    pub scalar_field_size: usize,
    pub security_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub operation: String,
    pub curve: String,
    pub time_ns: u128,
    pub iterations: u64,
}

// ============================================================================
// 3. 유틸리티 함수
// ============================================================================

/// BN254 곡선 정보
pub fn bn254_stats() -> CurveStats {
    CurveStats {
        curve_name: "BN254".to_string(),
        field_size: 254,
        scalar_field_size: 254,
        security_level: 128,
    }
}

/// BLS12-381 곡선 정보
pub fn bls12_381_stats() -> CurveStats {
    CurveStats {
        curve_name: "BLS12-381".to_string(),
        field_size: 381,
        scalar_field_size: 255,
        security_level: 128,
    }
}

/// 랜덤 필드 원소 생성
pub fn random_field_element<F: Field, R: RngCore>(rng: &mut R) -> F {
    F::rand(rng)
}

/// 랜덤 곡선 포인트 생성
pub fn random_curve_point<G: CurveGroup, R: RngCore>(rng: &mut R) -> G {
    G::rand(rng)
}

// ============================================================================
// 4. 벤치마크 헬퍼
// ============================================================================

use std::time::Instant;

pub fn benchmark_operation<F>(name: &str, iterations: u64, mut op: F) -> BenchmarkResult
where
    F: FnMut(),
{
    let start = Instant::now();

    for _ in 0..iterations {
        op();
    }

    let elapsed = start.elapsed();

    BenchmarkResult {
        operation: name.to_string(),
        curve: "Generic".to_string(),
        time_ns: elapsed.as_nanos(),
        iterations,
    }
}

pub fn print_benchmark_result(result: &BenchmarkResult) {
    let avg_ns = result.time_ns / result.iterations as u128;
    let avg_us = avg_ns as f64 / 1000.0;

    println!("📊 {}", result.operation);
    println!(
        "   • 총 시간: {:.2} ms",
        result.time_ns as f64 / 1_000_000.0
    );
    println!("   • 평균 시간: {:.2} μs", avg_us);
    println!("   • 반복 횟수: {}", result.iterations);
}
