use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};
use halo2curves::bn256::Fr;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// ZK Proof 데이터 구조
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentProof {
    pub predicted_sentiment: usize, // 0: Negative, 1: Neutral, 2: Positive
    pub text_hash: String,          // SHA256(text)
    pub model_hash: String,         // SHA256(model)
    pub timestamp: u64,
    pub commitment: String,                        // SHA256(text_hash + model_hash + sentiment)
    pub range_proof: Vec<usize>,                   // [sentiment]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub halo2_proof: Option<Vec<u8>>, // 32 bytes Halo2 proof
}

impl SentimentProof {
    /// 새로운 증명 생성
    pub fn new(
        predicted_sentiment: usize,
        text: &str,
        model_weights: &[f32],
    ) -> Self {
        // Text hash
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        let text_hash = format!("{:x}", hasher.finalize());

        // Model hash
        let mut hasher = Sha256::new();
        for &weight in model_weights {
            hasher.update(weight.to_le_bytes());
        }
        let model_hash = format!("{:x}", hasher.finalize());

        // Commitment
        let mut hasher = Sha256::new();
        hasher.update(text_hash.as_bytes());
        hasher.update(model_hash.as_bytes());
        hasher.update(predicted_sentiment.to_le_bytes());
        let commitment = format!("{:x}", hasher.finalize());

        // Range proof
        let range_proof = vec![predicted_sentiment];

        // Timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            predicted_sentiment,
            text_hash,
            model_hash,
            timestamp,
            commitment,
            range_proof,
            halo2_proof: None,
        }
    }

    /// 기본 검증 (범위 체크, 해시 길이 등)
    pub fn verify(&self) -> bool {
        // 1. Sentiment 범위 체크 (0-2)
        if self.predicted_sentiment > 2 {
            return false;
        }

        // 2. Range proof 일관성
        if self.range_proof.len() != 1 || self.range_proof[0] != self.predicted_sentiment {
            return false;
        }

        // 3. 해시 길이 체크 (SHA256 = 64 hex chars)
        if self.text_hash.len() != 64
            || self.model_hash.len() != 64
            || self.commitment.len() != 64
        {
            return false;
        }

        true
    }
}

/// Halo2 Circuit for Sentiment Analysis
#[derive(Clone, Debug)]
pub struct SentimentCircuit {
    pub sentiment: Value<Fr>,
}

impl Circuit<Fr> for SentimentCircuit {
    type Config = SentimentConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            sentiment: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        let selector = meta.selector();

        meta.enable_equality(advice);
        meta.enable_equality(instance);

        // Range check: sentiment must be 0, 1, or 2
        meta.create_gate("sentiment range check", |meta| {
            let s = meta.query_selector(selector);
            let sentiment = meta.query_advice(advice, Rotation::cur());

            // sentiment * (sentiment - 1) * (sentiment - 2) = 0
            vec![
                s * sentiment.clone()
                    * (sentiment.clone() - halo2_proofs::plonk::Expression::Constant(Fr::one()))
                    * (sentiment - halo2_proofs::plonk::Expression::Constant(Fr::from(2))),
            ]
        });

        SentimentConfig {
            advice,
            instance,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "sentiment analysis",
            |mut region| {
                config.selector.enable(&mut region, 0)?;

                let sentiment_cell = region.assign_advice(
                    || "sentiment",
                    config.advice,
                    0,
                    || self.sentiment,
                )?;

                let instance_cell = region.assign_advice_from_instance(
                    || "public sentiment",
                    config.instance,
                    0,
                    config.advice,
                    0,
                )?;

                region.constrain_equal(sentiment_cell.cell(), instance_cell.cell())?;

                Ok(())
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct SentimentConfig {
    pub advice: Column<Advice>,
    pub instance: Column<Instance>,
    pub selector: Selector,
}

