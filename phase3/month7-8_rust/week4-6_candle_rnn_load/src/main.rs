// Week 4-6: 저장된 RNN 모델 불러오기 📂
//
// 🎯 목표: 학습 없이 바로 예측!
//
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{linear, Linear, Module, VarBuilder, VarMap};

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("📂 저장된 RNN 모델 불러오기");
    println!("{}", "=".repeat(60));

    // 1. VarMap 생성 & 모델 불러오기
    println!("\n[1] 모델 불러오기");
    let device = Device::Cpu;
    let mut varmap = VarMap::new();

    // 저장된 가중치 불러오기
    varmap.load("../week4-5_candle_rnn_stock/stock_rnn.safetensors")?;
    println!("  ✅ stock_rnn.safetensors 불러오기 완료!");
    println!("  💾 학습된 가중치 로드 완료 (학습 안 해도 됨!)");

    // 2. 모델 생성 (구조는 같아야 함!)
    println!("\n[2] 모델 구조 생성");
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = StockRNN::new(vb)?;
    println!("  🧠 RNN 모델 생성 완료!");

    // 3. 바로 예측! (학습 없이!)
    println!("\n[3] 예측 테스트 (학습 없이 바로!)");

    let test_cases = vec![
        (
            "상승 추세",
            vec![
                10000.0, 10200.0, 10500.0, 10300.0, 10600.0, 10800.0, 11000.0,
            ],
        ),
        (
            "급등 패턴",
            vec![
                10000.0, 10300.0, 10800.0, 11500.0, 12300.0, 13200.0, 14200.0,
            ],
        ),
        (
            "급락 패턴",
            vec![14000.0, 13500.0, 12800.0, 12000.0, 11100.0, 10100.0, 9000.0],
        ),
        (
            "횡보 패턴",
            vec![10000.0, 10100.0, 9900.0, 10050.0, 9950.0, 10000.0, 10100.0],
        ),
    ];

    for (name, prices) in test_cases {
        println!("\n  📊 {}", name);
        println!("    과거 7일:");
        for (i, price) in prices.iter().enumerate() {
            let day = match i {
                0 => "월",
                1 => "화",
                2 => "수",
                3 => "목",
                4 => "금",
                5 => "토",
                6 => "일",
                _ => "?",
            };
            println!("      {}: {:>7.0}원", day, price);
        }

        // 예측
        let x = create_sequence(&prices)?;
        let pred = model.forward(&x)?;
        let pred_val = pred.i((0, 0))?.to_scalar::<f32>()?;

        // 역정규화
        let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;
        let predicted_price = pred_val * range + min;

        // 추세 계산
        let trend = (prices[6] - prices[0]) / 6.0;
        let expected = prices[6] + trend;

        println!("    🔮 AI 예측: {:>7.0}원", predicted_price);
        println!("    📈 추세 예측: {:>7.0}원", expected);
        println!("    ✅ 오차: {:>7.0}원", (predicted_price - expected).abs());
    }

    println!("\n{}", "=".repeat(60));
    println!("✅ 불러오기 성공! 학습 없이 바로 예측 가능!");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// 주식 예측 RNN (저장할 때와 같은 구조!)
struct StockRNN {
    rnn_layer: Linear,
    output_layer: Linear,
    hidden_size: usize,
}

impl StockRNN {
    fn new(vb: VarBuilder) -> Result<Self> {
        let hidden_size = 32; // 저장할 때와 동일!
        let rnn_layer = linear(1 + hidden_size, hidden_size, vb.pp("rnn"))?;
        let output_layer = linear(hidden_size, 1, vb.pp("output"))?;

        Ok(Self {
            rnn_layer,
            output_layer,
            hidden_size,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let device = x.device();
        let batch_size = x.dim(0)?;
        let seq_len = x.dim(1)?;

        let mut hidden = Tensor::zeros(&[batch_size, self.hidden_size], DType::F32, device)?;

        for t in 0..seq_len {
            let x_t = x.i((.., t, ..))?;
            let combined = Tensor::cat(&[&x_t, &hidden], 1)?;
            hidden = self.rnn_layer.forward(&combined)?.tanh()?;
        }

        self.output_layer.forward(&hidden)
    }
}

/// 시퀀스 생성 (정규화)
fn create_sequence(prices: &[f32]) -> Result<Tensor> {
    let device = Device::Cpu;

    let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;

    let normalized: Vec<f32> = prices.iter().map(|p| (p - min) / range).collect();

    Tensor::from_vec(normalized, &[1, prices.len(), 1], &device)
}
