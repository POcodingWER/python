// ✅ 증명 검증

use crate::zk::circuit::MLProof;

/// ML 추론 증명 검증
pub fn verify_proof(proof: &MLProof) -> Result<bool, Box<dyn std::error::Error>> {
    // 기본 검증
    if !proof.verify() {
        return Ok(false);
    }
    
    // 타임스탬프 검증 (24시간 이내)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    
    let age = now.saturating_sub(proof.timestamp);
    if age > 86400 { // 24시간
        return Ok(false);
    }
    
    Ok(true)
}
