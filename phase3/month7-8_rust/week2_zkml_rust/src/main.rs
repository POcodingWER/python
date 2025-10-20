// 🦀 Week 2: Rust ZKML 실전 프로젝트
// Python ZKML → Rust 포팅 + 성능 비교

mod linear_regression;
mod zkml_circuit;

use std::time::Instant;

fn main() {
    println!("{}", "=".repeat(60));
    println!("🦀 Rust ZKML: Linear Regression + halo2");
    println!("{}", "=".repeat(60));

    // 1. 순수 Rust Linear Regression
    println!("\n[1] 순수 Rust Linear Regression");
    let start = Instant::now();
    linear_regression::run();
    let duration = start.elapsed();
    println!("⏱️  실행 시간: {:?}", duration);

    // 2. halo2 ZK 회로
    println!("\n[2] halo2 ZK 회로");
    let start = Instant::now();
    zkml_circuit::run();
    let duration = start.elapsed();
    println!("⏱️  실행 시간: {:?}", duration);

    println!("\n{}", "=".repeat(60));
    println!("✅ Rust ZKML 완료!");
    println!("{}", "=".repeat(60));
}
