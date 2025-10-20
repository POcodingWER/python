// Week 1: Rust 기초 - 메인 프로그램
// 각 예제를 모듈로 분리하여 학습

mod basics;
mod ownership;
mod structs;

fn main() {
    println!("{}", "=".repeat(60));
    println!("🦀 Week 1: Rust 기초 학습");
    println!("{}", "=".repeat(60));

    // Day 1-2: 기본 문법
    println!("\n[Day 1-2] 기본 문법");
    basics::run();

    // Day 3-4: 소유권 시스템
    println!("\n[Day 3-4] 소유권 시스템");
    ownership::run();

    // Day 5-6: 구조체
    println!("\n[Day 5-6] 구조체 & 메서드");
    structs::run();

    println!("\n{}", "=".repeat(60));
    println!("✅ Week 1 예제 완료!");
    println!("{}", "=".repeat(60));
}
