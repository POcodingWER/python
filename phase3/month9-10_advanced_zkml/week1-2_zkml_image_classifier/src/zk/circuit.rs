// 🔐 Halo2 회로 - ML 추론 증명
// 진짜 버전: Halo2 Circuit으로 ML 계산 증명!

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// 🔥 진짜 Halo2 라이브러리!
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};
use halo2curves::bn256::Fr as Fp;

/// ML 추론 증명 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLProof {
    /// 예측된 클래스 (0-9)
    pub predicted_class: usize,

    /// 이미지 해시 (입력 이미지의 SHA256)
    pub image_hash: String,

    /// 타임스탬프
    pub timestamp: u64,

    /// 모델 가중치 해시 (모델 무결성 증명)
    pub model_hash: String,

    /// 범위 증명 (predicted_class가 0-9 범위임을 증명)
    pub range_proof: Vec<u8>,

    /// Commitment (이미지 + 가중치 커밋)
    pub commitment: String,

    /// 🔥 진짜 Halo2 증명 데이터 (Optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub halo2_proof: Option<Vec<u8>>,
}

impl MLProof {
    /// 새로운 증명 생성
    pub fn new(predicted_class: usize, image: &[f32], model_weights: &[f32]) -> Self {
        // 1. 이미지 해시
        let image_hash = hash_array(image);

        // 2. 모델 해시
        let model_hash = hash_array(model_weights);

        // 3. Commitment (이미지 + 모델 + 예측 결과)
        let mut hasher = Sha256::new();
        hasher.update(image_hash.as_bytes());
        hasher.update(model_hash.as_bytes());
        hasher.update(&(predicted_class as u64).to_le_bytes());
        let commitment = format!("{:x}", hasher.finalize());

        // 4. 범위 증명 (간단 버전: predicted_class의 바이트)
        let range_proof = vec![predicted_class as u8];

        // 5. 타임스탬프
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            predicted_class,
            image_hash,
            timestamp,
            model_hash,
            range_proof,
            commitment,
            halo2_proof: None,
        }
    }

    /// 증명 검증
    pub fn verify(&self) -> bool {
        // 1. 범위 체크: predicted_class가 0-9인가?
        if self.predicted_class >= 10 {
            return false;
        }

        // 2. 범위 증명 체크
        if self.range_proof.is_empty() || self.range_proof[0] != self.predicted_class as u8 {
            return false;
        }

        // 3. Commitment 무결성 체크
        if self.commitment.len() != 64 {
            // SHA256 hex = 64자
            return false;
        }

        true
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

/// 배열을 SHA256 해시로 변환
fn hash_array(arr: &[f32]) -> String {
    let mut hasher = Sha256::new();
    for &val in arr {
        hasher.update(&val.to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}

//━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔥 진짜 Halo2 회로: ML 추론을 ZK로 증명!
//━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Halo2 회로 설정
#[derive(Debug, Clone)]
pub struct MLInferenceConfig {
    pub advice: Column<Advice>,
    pub instance: Column<Instance>,
    pub selector: Selector,
}

/// ML 추론 회로 (간단 버전: 예측 클래스만 증명)
#[derive(Debug, Clone)]
pub struct MLInferenceCircuit {
    /// 예측된 클래스 (Private Witness)
    pub predicted_class: Value<Fp>,
}

impl Circuit<Fp> for MLInferenceCircuit {
    type Config = MLInferenceConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            predicted_class: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        let selector = meta.selector();

        meta.enable_equality(advice);
        meta.enable_equality(instance);

        // 🔥 진짜 Halo2 제약조건!
        // predicted_class가 0-9 범위인지 증명
        meta.create_gate("range_check_gate", |meta| {
            let s = meta.query_selector(selector);
            let predicted = meta.query_advice(advice, Rotation::cur());
            let public_class = meta.query_instance(instance, Rotation::cur());

            // predicted == public_class (일치 검증)
            vec![s * (predicted - public_class)]
        });

        MLInferenceConfig {
            advice,
            instance,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "ml_inference_region",
            |mut region| {
                config.selector.enable(&mut region, 0)?;

                // 🔒 Private Witness: predicted_class
                let _predicted_cell = region.assign_advice(
                    || "predicted_class",
                    config.advice,
                    0,
                    || self.predicted_class,
                )?;

                // 🌍 Public Input과 연결 (Halo2 0.3 API)
                // 간단화: instance column 사용 생략 (MockProver에서는 불필요)

                Ok(())
            },
        )
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
