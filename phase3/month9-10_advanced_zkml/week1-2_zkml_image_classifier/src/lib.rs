// 🖼️ ZKML 이미지 분류기
// 순수 Rust + Halo2로 구현

pub mod ml;
pub mod zk;

// Re-exports
pub use ml::*;
pub use zk::circuit::{MLInferenceCircuit, MLProof};
pub use zk::prove::generate_proof_halo2;
pub use zk::verify::verify_proof;
