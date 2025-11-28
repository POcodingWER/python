use serde::{Deserialize, Serialize};

/// Bag-of-Words + Multi-layer Dense Network (고성능!)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentModel {
    pub vocab_size: usize,
    pub embedding_dim: usize, // 호환성 유지

    // Dense Layer 1 (vocab_size -> hidden1)
    pub hidden1_size: usize,
    pub dense1_weights: Vec<Vec<f32>>,
    pub dense1_bias: Vec<f32>,

    // Dense Layer 2 (hidden1 -> hidden2)
    pub hidden2_size: usize,
    pub dense2_weights: Vec<Vec<f32>>,
    pub dense2_bias: Vec<f32>,

    // Dense Layer 3 (hidden2 -> hidden3)
    pub hidden3_size: usize,
    pub dense3_weights: Vec<Vec<f32>>,
    pub dense3_bias: Vec<f32>,

    // Output layer (hidden3 -> 3)
    pub output_weights: Vec<Vec<f32>>,
    pub output_bias: Vec<f32>,
}

impl SentimentModel {
    /// 새로운 Bag-of-Words + Dense Network 생성 (Xavier 초기화)
    pub fn new(vocab_size: usize, _embedding_dim: usize, hidden_size: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Vocab이 너무 크면 제한 (메모리 절약)
        let limited_vocab_size = vocab_size.min(10000); // 최대 1만개 단어만 사용

        let hidden1_size = hidden_size * 2; // 128
        let hidden2_size = hidden_size; // 64
        let hidden3_size = hidden_size / 2; // 32

        // Dense Layer 1: limited_vocab_size -> hidden1
        let scale1 = (2.0 / (limited_vocab_size + hidden1_size) as f32).sqrt();
        let dense1_weights = (0..limited_vocab_size)
            .map(|_| {
                (0..hidden1_size)
                    .map(|_| rng.gen::<f32>() * scale1 - scale1 / 2.0)
                    .collect()
            })
            .collect();
        let dense1_bias = vec![0.0; hidden1_size];

        // Dense Layer 2: hidden1 -> hidden2
        let scale2 = (2.0 / (hidden1_size + hidden2_size) as f32).sqrt();
        let dense2_weights = (0..hidden1_size)
            .map(|_| {
                (0..hidden2_size)
                    .map(|_| rng.gen::<f32>() * scale2 - scale2 / 2.0)
                    .collect()
            })
            .collect();
        let dense2_bias = vec![0.0; hidden2_size];

        // Dense Layer 3: hidden2 -> hidden3
        let scale3 = (2.0 / (hidden2_size + hidden3_size) as f32).sqrt();
        let dense3_weights = (0..hidden2_size)
            .map(|_| {
                (0..hidden3_size)
                    .map(|_| rng.gen::<f32>() * scale3 - scale3 / 2.0)
                    .collect()
            })
            .collect();
        let dense3_bias = vec![0.0; hidden3_size];

        // Output Layer: hidden3 -> 2 (부정/긍정)
        let scale_out = (2.0 / (hidden3_size + 2) as f32).sqrt();
        let output_weights = (0..hidden3_size)
            .map(|_| {
                (0..2)
                    .map(|_| rng.gen::<f32>() * scale_out - scale_out / 2.0)
                    .collect()
            })
            .collect();
        let output_bias = vec![0.0; 2];

        Self {
            vocab_size: limited_vocab_size, // 제한된 vocab 사용
            embedding_dim: 0,               // 사용 안 함
            hidden1_size,
            dense1_weights,
            dense1_bias,
            hidden2_size,
            dense2_weights,
            dense2_bias,
            hidden3_size,
            dense3_weights,
            dense3_bias,
            output_weights,
            output_bias,
        }
    }

    /// ReLU 활성화 함수
    fn relu(&self, x: f32) -> f32 {
        x.max(0.0)
    }

    /// Bag-of-Words: 단어 빈도를 벡터로 변환 (TF-IDF 스타일)
    fn text_to_bow(&self, word_indices: &[usize]) -> Vec<f32> {
        let mut bow = vec![0.0; self.vocab_size];
        for &idx in word_indices {
            if idx < self.vocab_size {
                bow[idx] += 1.0;
            }
        }
        // 정규화 제거 - 빈도 그대로 사용 (더 강한 신호)
        bow
    }

    /// Softmax 함수
    fn softmax(&self, logits: &[f32]) -> Vec<f32> {
        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_logits: Vec<f32> = logits.iter().map(|&x| (x - max_logit).exp()).collect();
        let sum_exp: f32 = exp_logits.iter().sum();
        exp_logits.iter().map(|&x| x / sum_exp).collect()
    }

    /// Forward pass: word_indices -> sentiment prediction
    pub fn forward_bow(&self, word_indices: &[usize]) -> Vec<f32> {
        // Bag-of-Words
        let bow = self.text_to_bow(word_indices);

        // Dense Layer 1
        let mut hidden1 = self.dense1_bias.clone();
        for i in 0..self.hidden1_size {
            for j in 0..self.vocab_size {
                hidden1[i] += bow[j] * self.dense1_weights[j][i];
            }
            hidden1[i] = self.relu(hidden1[i]);
        }

        // Dense Layer 2
        let mut hidden2 = self.dense2_bias.clone();
        for i in 0..self.hidden2_size {
            for j in 0..self.hidden1_size {
                hidden2[i] += hidden1[j] * self.dense2_weights[j][i];
            }
            hidden2[i] = self.relu(hidden2[i]);
        }

        // Dense Layer 3
        let mut hidden3 = self.dense3_bias.clone();
        for i in 0..self.hidden3_size {
            for j in 0..self.hidden2_size {
                hidden3[i] += hidden2[j] * self.dense3_weights[j][i];
            }
            hidden3[i] = self.relu(hidden3[i]);
        }

        // Output layer (2 classes)
        let mut logits = self.output_bias.clone();
        for i in 0..2 {
            for j in 0..self.hidden3_size {
                logits[i] += hidden3[j] * self.output_weights[j][i];
            }
        }

        // Softmax
        self.softmax(&logits)
    }

    /// Forward pass (호환성 유지)
    pub fn forward(&self, embeddings: &[Vec<f32>]) -> Vec<f32> {
        // embeddings는 사용 안 함, 대신 word_indices 필요
        // 임시로 빈 벡터 반환
        vec![0.33, 0.33, 0.34]
    }

    /// 예측 (Bag-of-Words)
    pub fn predict_bow(&self, word_indices: &[usize]) -> usize {
        let probs = self.forward_bow(word_indices);
        probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(1)
    }

    /// 예측 (호환성 유지)
    pub fn predict(&self, _embeddings: &[Vec<f32>]) -> usize {
        1 // Default: Neutral
    }

    /// 학습 (Bag-of-Words + Backprop)
    pub fn train_bow(&mut self, word_indices: &[usize], label: usize, learning_rate: f32) -> f32 {
        // Forward pass with intermediate values
        let bow = self.text_to_bow(word_indices);

        // Dense 1
        let mut hidden1 = self.dense1_bias.clone();
        for i in 0..self.hidden1_size {
            for j in 0..self.vocab_size {
                hidden1[i] += bow[j] * self.dense1_weights[j][i];
            }
            hidden1[i] = self.relu(hidden1[i]);
        }

        // Dense 2
        let mut hidden2 = self.dense2_bias.clone();
        for i in 0..self.hidden2_size {
            for j in 0..self.hidden1_size {
                hidden2[i] += hidden1[j] * self.dense2_weights[j][i];
            }
            hidden2[i] = self.relu(hidden2[i]);
        }

        // Dense 3
        let mut hidden3 = self.dense3_bias.clone();
        for i in 0..self.hidden3_size {
            for j in 0..self.hidden2_size {
                hidden3[i] += hidden2[j] * self.dense3_weights[j][i];
            }
            hidden3[i] = self.relu(hidden3[i]);
        }

        // Output (2 classes)
        let mut logits = self.output_bias.clone();
        for i in 0..2 {
            for j in 0..self.hidden3_size {
                logits[i] += hidden3[j] * self.output_weights[j][i];
            }
        }

        let probs = self.softmax(&logits);
        let loss = -probs[label].max(1e-10).ln();

        // Backpropagation
        let mut grad_output = probs;
        grad_output[label] -= 1.0;

        // Gradient Clipping (기울기 폭발 방지) - 적당하게!
        let clip_value = 0.2; // 0.1은 너무 강함, 0.2로 절충 (학습 가능하게)
        for val in &mut grad_output {
            *val = val.max(-clip_value).min(clip_value);
        }

        // Update output layer
        for i in 0..self.hidden3_size {
            for j in 0..2 {
                self.output_weights[i][j] -= learning_rate * grad_output[j] * hidden3[i];
            }
        }
        for i in 0..2 {
            self.output_bias[i] -= learning_rate * grad_output[i];
        }

        // Gradient for hidden3
        let mut grad_hidden3 = vec![0.0; self.hidden3_size];
        for i in 0..self.hidden3_size {
            for j in 0..2 {
                grad_hidden3[i] += grad_output[j] * self.output_weights[i][j];
            }
            if hidden3[i] <= 0.0 {
                grad_hidden3[i] = 0.0;
            }
            // Gradient Clipping
            grad_hidden3[i] = grad_hidden3[i].max(-clip_value).min(clip_value);
        }

        // Update dense3
        for i in 0..self.hidden2_size {
            for j in 0..self.hidden3_size {
                self.dense3_weights[i][j] -= learning_rate * grad_hidden3[j] * hidden2[i];
            }
        }
        for i in 0..self.hidden3_size {
            self.dense3_bias[i] -= learning_rate * grad_hidden3[i];
        }

        // Gradient for hidden2
        let mut grad_hidden2 = vec![0.0; self.hidden2_size];
        for i in 0..self.hidden2_size {
            for j in 0..self.hidden3_size {
                grad_hidden2[i] += grad_hidden3[j] * self.dense3_weights[i][j];
            }
            if hidden2[i] <= 0.0 {
                grad_hidden2[i] = 0.0;
            }
            // Gradient Clipping
            grad_hidden2[i] = grad_hidden2[i].max(-clip_value).min(clip_value);
        }

        // Update dense2
        for i in 0..self.hidden1_size {
            for j in 0..self.hidden2_size {
                self.dense2_weights[i][j] -= learning_rate * grad_hidden2[j] * hidden1[i];
            }
        }
        for i in 0..self.hidden2_size {
            self.dense2_bias[i] -= learning_rate * grad_hidden2[i];
        }

        // Gradient for hidden1
        let mut grad_hidden1 = vec![0.0; self.hidden1_size];
        for i in 0..self.hidden1_size {
            for j in 0..self.hidden2_size {
                grad_hidden1[i] += grad_hidden2[j] * self.dense2_weights[i][j];
            }
            if hidden1[i] <= 0.0 {
                grad_hidden1[i] = 0.0;
            }
            // Gradient Clipping
            grad_hidden1[i] = grad_hidden1[i].max(-clip_value).min(clip_value);
        }

        // Update dense1
        for i in 0..self.vocab_size {
            for j in 0..self.hidden1_size {
                self.dense1_weights[i][j] -= learning_rate * grad_hidden1[j] * bow[i];
            }
        }
        for i in 0..self.hidden1_size {
            self.dense1_bias[i] -= learning_rate * grad_hidden1[i];
        }

        loss
    }

    /// 학습 (호환성 유지)
    pub fn train(&mut self, _embeddings: &[Vec<f32>], _label: usize, _learning_rate: f32) -> f32 {
        0.69 // Dummy loss
    }
}
