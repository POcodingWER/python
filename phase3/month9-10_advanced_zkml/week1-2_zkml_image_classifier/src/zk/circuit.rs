// 🔐 Halo2 회로 - ML 추론 증명
// 간단한 버전: 예측 클래스가 0-9 범위인지만 확인

use serde::{Deserialize, Serialize};

/// ML 추론 증명 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLProof {
    /// 예측된 클래스 (0-9)
    pub predicted_class: usize,
    
    /// 이미지 해시 (입력 이미지의 SHA256)
    pub image_hash: String,
    
    /// 타임스탬프
    pub timestamp: u64,
}

impl MLProof {
    /// 새로운 증명 생성
    pub fn new(predicted_class: usize, image: &[f32]) -> Self {
        use sha2::{Sha256, Digest};
        
        // 이미지 해시 계산
        let mut hasher = Sha256::new();
        for &pixel in image {
            hasher.update(&pixel.to_le_bytes());
        }
        let hash_bytes = hasher.finalize();
        let image_hash = format!("{:x}", hash_bytes);
        
        // 현재 시간
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            predicted_class,
            image_hash,
            timestamp,
        }
    }
    
    /// 증명 검증
    pub fn verify(&self) -> bool {
        // 간단한 검증: 클래스가 0-9 범위인지
        self.predicted_class < 10
    }
    
    /// 증명 저장
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    /// 증명 로드
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let proof = serde_json::from_str(&json)?;
        Ok(proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ml_proof() {
        let image = vec![0.5; 784];
        let predicted_class = 5;
        
        let proof = MLProof::new(predicted_class, &image);
        assert_eq!(proof.predicted_class, 5);
        assert!(proof.verify());
    }
    
    #[test]
    fn test_invalid_class() {
        let mut proof = MLProof {
            predicted_class: 10, // 잘못된 클래스
            image_hash: "test".to_string(),
            timestamp: 0,
        };
        
        assert!(!proof.verify());
        
        proof.predicted_class = 5;
        assert!(proof.verify());
    }
}
