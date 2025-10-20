// 순수 Rust Linear Regression (PyTorch 없이)
// y = weight * x + bias

#[derive(Debug, Clone)]
pub struct LinearModel {
    pub weight: f64,
    pub bias: f64,
}

impl LinearModel {
    pub fn new(weight: f64, bias: f64) -> Self {
        LinearModel { weight, bias }
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.weight * x + self.bias
    }

    pub fn predict_batch(&self, inputs: &[f64]) -> Vec<f64> {
        inputs.iter().map(|&x| self.predict(x)).collect()
    }
}

pub fn run() {
    // Python 코드와 동일: y = 2x + 1
    let model = LinearModel::new(2.0, 1.0);

    println!("  📊 모델: y = {}x + {}", model.weight, model.bias);

    // 단일 예측
    let x = 3.0;
    let y = model.predict(x);
    println!("  🧮 예측: x={} → y={}", x, y);

    // 배치 예측
    let inputs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let outputs = model.predict_batch(&inputs);

    println!("  📈 배치 예측:");
    for (x, y) in inputs.iter().zip(outputs.iter()) {
        println!("     x={:.1} → y={:.1}", x, y);
    }

    // 성능 테스트: 100만 번 예측
    let start = std::time::Instant::now();
    let mut sum = 0.0;
    for i in 0..1_000_000 {
        sum += model.predict(i as f64);
    }
    let duration = start.elapsed();

    println!("  ⚡ 성능: 1,000,000회 예측 = {:?}", duration);
    println!("     (합계: {:.2e} - 오버플로우 방지용)", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_model() {
        let model = LinearModel::new(2.0, 1.0);
        assert_eq!(model.predict(3.0), 7.0);
        assert_eq!(model.predict(0.0), 1.0);
        assert_eq!(model.predict(-1.0), -1.0);
    }

    #[test]
    fn test_batch_prediction() {
        let model = LinearModel::new(2.0, 1.0);
        let inputs = vec![1.0, 2.0, 3.0];
        let outputs = model.predict_batch(&inputs);
        assert_eq!(outputs, vec![3.0, 5.0, 7.0]);
    }
}
