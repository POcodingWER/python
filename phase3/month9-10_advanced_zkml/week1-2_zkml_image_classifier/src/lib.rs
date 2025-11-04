// 🖼️ ZKML 이미지 분류기
// 순수 Rust + Halo2로 구현

pub mod ml;
pub mod zk;

// Re-exports
pub use ml::*;
pub use zk::circuit::MLProof;
pub use zk::prove::generate_proof;
pub use zk::verify::verify_proof;

