// 🔐 증명 생성

use crate::zk::circuit::MLProof;

/// ML 추론 증명 생성
pub fn generate_proof(
    image: &[f32],
    predicted_class: usize,
) -> Result<MLProof, Box<dyn std::error::Error>> {
    // 증명 생성
    let proof = MLProof::new(predicted_class, image);
    
    // 검증
    if !proof.verify() {
        return Err("증명 검증 실패: 잘못된 클래스".into());
    }
    
    Ok(proof)
}
