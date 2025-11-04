// 🤖 간단한 선형 분류기
// 28x28 이미지 → 10개 클래스

use serde::{Deserialize, Serialize};

/// 간단한 신경망 분류기 (히든 레이어 포함)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleClassifier {
    /// 히든 레이어 1: 784 → 256
    pub w1: Vec<Vec<f32>>,
    pub b1: Vec<f32>,

    /// 히든 레이어 2: 256 → 128
    pub w2: Vec<Vec<f32>>,
    pub b2: Vec<f32>,

    /// 출력 레이어: 128 → 10
    pub w3: Vec<Vec<f32>>,
    pub b3: Vec<f32>,
}

impl SimpleClassifier {
    /// 새로운 분류기 생성 (Xavier 초기화)
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Xavier 초기화: sqrt(2 / (fan_in + fan_out))
        let init_w1 = (2.0_f32 / (784.0 + 256.0)).sqrt();
        let w1 = (0..256)
            .map(|_| (0..784).map(|_| rng.gen_range(-init_w1..init_w1)).collect())
            .collect();
        let b1 = vec![0.0; 256];

        let init_w2 = (2.0_f32 / (256.0 + 128.0)).sqrt();
        let w2 = (0..128)
            .map(|_| (0..256).map(|_| rng.gen_range(-init_w2..init_w2)).collect())
            .collect();
        let b2 = vec![0.0; 128];

        let init_w3 = (2.0_f32 / (128.0 + 10.0)).sqrt();
        let w3 = (0..10)
            .map(|_| (0..128).map(|_| rng.gen_range(-init_w3..init_w3)).collect())
            .collect();
        let b3 = vec![0.0; 10];

        Self {
            w1,
            b1,
            w2,
            b2,
            w3,
            b3,
        }
    }

    /// ReLU 활성화 함수
    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    /// 추론: 이미지 → 클래스 (0-9)
    pub fn predict(&self, image: &[f32]) -> usize {
        assert_eq!(image.len(), 784, "이미지는 28x28 = 784 픽셀이어야 합니다");

        // Layer 1: 784 → 256 (ReLU)
        let mut h1 = vec![0.0; 256];
        for (i, (weights, bias)) in self.w1.iter().zip(self.b1.iter()).enumerate() {
            let mut sum = *bias;
            for (pixel, weight) in image.iter().zip(weights.iter()) {
                sum += pixel * weight;
            }
            h1[i] = Self::relu(sum);
        }

        // Layer 2: 256 → 128 (ReLU)
        let mut h2 = vec![0.0; 128];
        for (i, (weights, bias)) in self.w2.iter().zip(self.b2.iter()).enumerate() {
            let mut sum = *bias;
            for (h1_val, weight) in h1.iter().zip(weights.iter()) {
                sum += h1_val * weight;
            }
            h2[i] = Self::relu(sum);
        }

        // Layer 3: 128 → 10 (출력)
        let mut scores = vec![0.0; 10];
        for (i, (weights, bias)) in self.w3.iter().zip(self.b3.iter()).enumerate() {
            let mut sum = *bias;
            for (h2_val, weight) in h2.iter().zip(weights.iter()) {
                sum += h2_val * weight;
            }
            scores[i] = sum;
        }

        // 가장 높은 점수의 클래스 반환
        scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    /// Gradient Clipping
    fn clip_gradient(grad: f32, max_norm: f32) -> f32 {
        grad.clamp(-max_norm, max_norm)
    }

    /// 간단한 학습 (경사하강법 + 역전파 + Gradient Clipping)
    pub fn train(&mut self, images: &[Vec<f32>], labels: &[usize], epochs: usize, lr: f32) {
        let max_grad = 0.5; // Gradient Clipping (더 강하게!)

        for epoch in 0..epochs {
            let mut correct = 0;

            for (image, &label) in images.iter().zip(labels.iter()) {
                // Forward pass
                // Layer 1: 784 → 256 (ReLU)
                let mut h1 = vec![0.0; 256];
                for (i, (weights, bias)) in self.w1.iter().zip(self.b1.iter()).enumerate() {
                    let mut sum = *bias;
                    for (pixel, weight) in image.iter().zip(weights.iter()) {
                        sum += pixel * weight;
                    }
                    h1[i] = Self::relu(sum);
                }

                // Layer 2: 256 → 128 (ReLU)
                let mut h2 = vec![0.0; 128];
                for (i, (weights, bias)) in self.w2.iter().zip(self.b2.iter()).enumerate() {
                    let mut sum = *bias;
                    for (h1_val, weight) in h1.iter().zip(weights.iter()) {
                        sum += h1_val * weight;
                    }
                    h2[i] = Self::relu(sum);
                }

                // Layer 3: 128 → 10
                let mut scores = vec![0.0; 10];
                for (i, (weights, bias)) in self.w3.iter().zip(self.b3.iter()).enumerate() {
                    let mut sum = *bias;
                    for (h2_val, weight) in h2.iter().zip(weights.iter()) {
                        sum += h2_val * weight;
                    }
                    scores[i] = sum;
                }

                // 예측
                let predicted = scores
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);

                if predicted == label {
                    correct += 1;
                }

                // Backward pass (간단한 버전)
                // Output layer gradient
                let mut d_scores = vec![0.0; 10];
                for i in 0..10 {
                    let target = if i == label { 1.0 } else { 0.0 };
                    d_scores[i] = scores[i] - target;
                }

                // Update Layer 3 (with Gradient Clipping)
                for (i, (weights, bias)) in self.w3.iter_mut().zip(self.b3.iter_mut()).enumerate() {
                    for (j, weight) in weights.iter_mut().enumerate() {
                        let grad = Self::clip_gradient(d_scores[i] * h2[j], max_grad);
                        *weight -= lr * grad;
                    }
                    let grad = Self::clip_gradient(d_scores[i], max_grad);
                    *bias -= lr * grad;
                }

                // Hidden layer 2 gradient
                let mut d_h2 = vec![0.0; 128];
                for j in 0..128 {
                    for i in 0..10 {
                        d_h2[j] += d_scores[i] * self.w3[i][j];
                    }
                    // ReLU gradient
                    if h2[j] <= 0.0 {
                        d_h2[j] = 0.0;
                    }
                }

                // Update Layer 2 (with Gradient Clipping)
                for (i, (weights, bias)) in self.w2.iter_mut().zip(self.b2.iter_mut()).enumerate() {
                    for (j, weight) in weights.iter_mut().enumerate() {
                        let grad = Self::clip_gradient(d_h2[i] * h1[j], max_grad);
                        *weight -= lr * grad;
                    }
                    let grad = Self::clip_gradient(d_h2[i], max_grad);
                    *bias -= lr * grad;
                }

                // Hidden layer 1 gradient
                let mut d_h1 = vec![0.0; 256];
                for j in 0..256 {
                    for i in 0..128 {
                        d_h1[j] += d_h2[i] * self.w2[i][j];
                    }
                    // ReLU gradient
                    if h1[j] <= 0.0 {
                        d_h1[j] = 0.0;
                    }
                }

                // Update Layer 1 (with Gradient Clipping)
                for (i, (weights, bias)) in self.w1.iter_mut().zip(self.b1.iter_mut()).enumerate() {
                    for (j, weight) in weights.iter_mut().enumerate() {
                        let grad = Self::clip_gradient(d_h1[i] * image[j], max_grad);
                        *weight -= lr * grad;
                    }
                    let grad = Self::clip_gradient(d_h1[i], max_grad);
                    *bias -= lr * grad;
                }
            }

            let accuracy = correct as f32 / images.len() as f32 * 100.0;
            println!("Epoch {}/{}: 정확도 {:.2}%", epoch + 1, epochs, accuracy);
        }
    }

    /// 모델 저장
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 모델 로드
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let model = serde_json::from_str(&json)?;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier() {
        let classifier = SimpleClassifier::new();
        let image = vec![0.0; 784];
        let prediction = classifier.predict(&image);
        assert!(prediction < 10);
    }
}
