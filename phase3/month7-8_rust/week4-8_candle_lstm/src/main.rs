// Week 4-8: LSTM 장단기 메모리(Long Short-Term Memory) - RNN의 업그레이드! 🚀
// 참고: https://wikidocs.net/22888
//
// 🎯 목표: 장기 기억 문제 해결!
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📚 LSTM이란? (Long Short-Term Memory)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 🧠 사람의 기억과 비교:
//
//   단기 기억 (Short-Term Memory):
//   - "방금 본 전화번호" → 금방 잊어버림
//   - "오늘 점심 메뉴" → 저녁엔 기억 안 남
//
//   장기 기억 (Long-Term Memory):
//   - "어릴 적 추억" → 평생 기억
//   - "중요한 사건" → 오래 기억
//
//   LSTM = 둘 다 가능! 💪
//   - 중요한 건 오래 기억 (장기)
//   - 덜 중요한 건 빨리 잊음 (단기)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🆚 RNN vs LSTM 비교
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// RNN (기본):
//   과거 7일:  ████████ (기억 잘함)
//   과거 30일: ░░░░░░░░ (기억 희미)
//   과거 100일: ........ (거의 못 기억)
//
//   문제: Gradient Vanishing (기울기 소실)
//   → 오래된 정보가 점점 희미해짐
//
// LSTM (개선):
//   과거 7일:  ████████ (완벽)
//   과거 30일: ████████ (완벽)
//   과거 100일: ███████░ (여전히 좋음!)
//
//   해결: Cell State (장기 기억 저장소)
//   → 중요한 정보는 오래 유지!
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🚪 LSTM의 3개 게이트 (문지기)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 1️⃣ Forget Gate (잊을 것 결정):
//    "이 정보 계속 기억할까? 아니면 잊을까?"
//
//    예시:
//    - 주식이 계속 오르는 중 → 기억! (중요)
//    - 일시적 노이즈 → 잊기! (덜 중요)
//
// 2️⃣ Input Gate (기억할 것 결정):
//    "새로운 정보 중 뭘 기억할까?"
//
//    예시:
//    - 큰 가격 변동 → 기억! (중요)
//    - 작은 변동 → 무시! (덜 중요)
//
// 3️⃣ Output Gate (출력할 것 결정):
//    "지금 뭘 출력할까?"
//
//    예시:
//    - 예측에 필요한 정보만 선택적으로 출력
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 💾 Cell State (장기 기억 저장소)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// Cell State = 컨베이어 벨트 🏭
//
//   [과거] ──────────────────────────> [현재]
//          ↓ 잊기    ↓ 추가
//        Forget    Input
//         Gate     Gate
//
//   - 중요한 정보는 계속 흘러감
//   - 덜 중요한 건 중간에 제거
//   - 새로운 중요 정보는 추가
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📊 실전 예시: 주식 가격 예측
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 과거 30일 데이터:
//   Day 1:  10,000원 (시작)
//   Day 15: 11,400원 (중간)
//   Day 30: 12,900원 (최근)
//
// LSTM 처리 과정:
//
//   1. Day 1 입력:
//      Forget Gate: "새로운 시작, 다 기억!"
//      Input Gate:  "10,000원 기억!"
//      Cell State:  [10,000원 추세 시작]
//
//   2. Day 2-14 입력:
//      Forget Gate: "상승 추세 계속 → 기억 유지!"
//      Input Gate:  "계속 오르는 중 → 추가 기억!"
//      Cell State:  [상승 추세 강화]
//
//   3. Day 15 입력:
//      Forget Gate: "여전히 상승 → 기억!"
//      Input Gate:  "11,400원 → 추세 확인!"
//      Cell State:  [장기 상승 추세 확정]
//
//   4. Day 16-30 입력:
//      Forget Gate: "장기 추세 유지!"
//      Input Gate:  "계속 상승 → 신뢰도 ↑"
//      Cell State:  [강한 상승 추세]
//
//   5. 예측:
//      Output Gate: "30일 추세 분석 → 13,000원!"
//      결과: 12,967원 (오차 33원!)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔑 핵심 포인트
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//
// 1. Cell State = 장기 기억 (30일, 100일도 OK!)
// 2. Hidden State = 단기 기억 (현재 상태)
// 3. 3개 게이트 = 똑똑한 정보 관리자
// 4. Sigmoid = 0~1 사이 값 (얼마나 기억/잊을지)
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{
    linear, loss, ops, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap,
};
use rand::Rng;

fn main() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("🧠 LSTM: 장기 기억 신경망");
    println!("{}", "=".repeat(60));

    // 1. 장기 패턴 데이터 생성 (30일!)
    println!("\n[1] 장기 시계열 데이터 생성");
    let (train_x, train_y) = generate_long_term_data(200, 30)?;
    println!("  📊 200개 시퀀스 생성!");
    println!("  📏 입력: 과거 30일 → 출력: 다음 1일");
    println!("  💡 RNN은 30일 기억 어려움, LSTM은 가능!");

    // 2. LSTM 모델 생성
    println!("\n[2] LSTM 모델 생성");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = SimpleLSTM::new(vb)?;
    println!("  🧠 LSTM 모델 생성 완료!");
    println!("  구조: LSTM(1→32) → Linear(32→1)");
    println!("  💡 3개 게이트: Forget, Input, Output");

    // 3. 학습
    println!("\n[3] 학습 시작!");
    train_lstm(&model, &varmap, &train_x, &train_y, 100)?;

    // 4. 예측 테스트
    println!("\n[4] 예측 테스트");

    // 테스트 케이스 1: 상승 추세
    let test_prices = vec![
        10000.0, 10100.0, 10200.0, 10300.0, 10400.0, 10500.0, 10600.0, 10700.0, 10800.0, 10900.0,
        11000.0, 11100.0, 11200.0, 11300.0, 11400.0, 11500.0, 11600.0, 11700.0, 11800.0, 11900.0,
        12000.0, 12100.0, 12200.0, 12300.0, 12400.0, 12500.0, 12600.0, 12700.0, 12800.0, 12900.0,
    ];

    println!("\n  📊 상승 추세 (30일 데이터)");
    println!("    첫날:  {:>7.0}원", test_prices[0]);
    println!("    15일:  {:>7.0}원", test_prices[14]);
    println!("    30일:  {:>7.0}원", test_prices[29]);

    let test_seq = create_sequence(&test_prices)?;
    let prediction = model.forward(&test_seq)?;
    let pred_val = prediction.i((0, 0))?.to_scalar::<f32>()?;

    let min = test_prices.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = test_prices
        .iter()
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;
    let pred_price = pred_val * range + min;

    let expected = 13000.0;
    println!("    🔮 LSTM 예측: {:>7.0}원", pred_price);
    println!("    📈 실제 추세: {:>7.0}원", expected);
    println!("    ✅ 오차: {:>7.0}원", (pred_price - expected).abs());

    // 5. 모델 저장
    println!("\n[5] 모델 저장");
    varmap.save("lstm_model.safetensors")?;
    println!("  ✅ lstm_model.safetensors 저장 완료!");

    println!("\n{}", "=".repeat(60));
    println!("✅ LSTM 완료! 장기 기억 가능! 🎉");
    println!("{}", "=".repeat(60));

    Ok(())
}

/// LSTM 모델
struct SimpleLSTM {
    // Forget Gate (잊을 것 결정)
    forget_gate: Linear,
    // Input Gate (새로 기억할 것 결정)
    input_gate: Linear,
    input_transform: Linear,
    // Output Gate (출력할 것 결정)
    output_gate: Linear,
    // 최종 출력
    output_layer: Linear,
    hidden_size: usize,
}

impl SimpleLSTM {
    fn new(vb: VarBuilder) -> Result<Self> {
        let hidden_size = 32;
        let input_size = 1 + hidden_size; // 입력 + hidden state

        // 3개 게이트 + 변환 레이어
        let forget_gate = linear(input_size, hidden_size, vb.pp("forget"))?;
        let input_gate = linear(input_size, hidden_size, vb.pp("input"))?;
        let input_transform = linear(input_size, hidden_size, vb.pp("transform"))?;
        let output_gate = linear(input_size, hidden_size, vb.pp("output"))?;
        let output_layer = linear(hidden_size, 1, vb.pp("out"))?;

        Ok(Self {
            forget_gate,
            input_gate,
            input_transform,
            output_gate,
            output_layer,
            hidden_size,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let device = x.device();
        let batch_size = x.dim(0)?;
        let seq_len = x.dim(1)?;

        // Hidden state & Cell state 초기화
        let mut hidden = Tensor::zeros(&[batch_size, self.hidden_size], DType::F32, device)?;
        let mut cell = Tensor::zeros(&[batch_size, self.hidden_size], DType::F32, device)?;

        // 시퀀스 처리 (30일 데이터를 하나씩 처리)
        for t in 0..seq_len {
            // 현재 시점의 입력 (예: Day 1의 가격)
            let x_t = x.i((.., t, ..))?;

            // 입력 + 이전 hidden state 결합
            // [현재 입력, 이전 기억] → 게이트들이 판단할 재료
            let combined = Tensor::cat(&[&x_t, &hidden], 1)?;

            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // 1️⃣ Forget Gate: "이전 기억 중 뭘 잊을까?" 🚪
            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // sigmoid(0~1) = 잊을 비율
            // 0.0 = 완전히 잊기, 1.0 = 완전히 기억
            // 예: 일시적 노이즈 → 0.2 (80% 잊기)
            //     중요한 추세 → 0.9 (10%만 잊기)
            let f_t = ops::sigmoid(&self.forget_gate.forward(&combined)?)?;

            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // 2️⃣ Input Gate: "새 정보 중 뭘 기억할까?" 🚪
            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // i_t = 기억할 비율 (0~1)
            // c_tilde = 새로운 후보 정보 (-1~1)
            // 예: 큰 가격 변동 → i_t=0.9, c_tilde=0.8
            //     작은 변동 → i_t=0.1, c_tilde=0.1
            let i_t = ops::sigmoid(&self.input_gate.forward(&combined)?)?;
            let c_tilde = self.input_transform.forward(&combined)?.tanh()?;

            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // 3️⃣ Cell State 업데이트 (장기 기억 저장소!) 💾
            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // 공식: cell = (f_t * cell) + (i_t * c_tilde)
            //            ↑ 이전 기억 중   ↑ 새 정보 중
            //              남길 것         추가할 것
            //
            // 예시:
            //   이전 cell = [상승 추세: 0.7]
            //   f_t = 0.9 (90% 유지)
            //   i_t = 0.8, c_tilde = 0.6 (새 상승 정보)
            //   새 cell = (0.9 * 0.7) + (0.8 * 0.6) = 0.63 + 0.48 = 1.11
            //   → 상승 추세 더 강화!
            cell = ((&f_t * &cell)? + (&i_t * &c_tilde)?)?;

            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // 4️⃣ Output Gate: "지금 뭘 출력할까?" 🚪
            // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            // o_t = 출력할 비율 (0~1)
            // hidden = 다음 시점으로 전달될 단기 기억
            // 예: 예측에 필요한 정보만 선택적으로 출력
            let o_t = ops::sigmoid(&self.output_gate.forward(&combined)?)?;
            hidden = (&o_t * &cell.tanh()?)?;
        }

        // 최종 출력
        self.output_layer.forward(&hidden)
    }
}

/// 장기 시계열 데이터 생성
fn generate_long_term_data(n: usize, seq_len: usize) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;
    let mut rng = rand::thread_rng();
    let mut sequences = Vec::new();
    let mut targets = Vec::new();

    for _ in 0..n {
        let base_price = rng.gen_range(8000.0..12000.0);
        let pattern = rng.gen_range(0..3);

        let mut prices = vec![base_price];

        // 장기 패턴 생성
        for i in 1..=seq_len {
            let prev = prices[i - 1];
            let next: f32 = match pattern {
                0 => {
                    // 장기 상승 추세
                    prev + rng.gen_range(50.0..150.0)
                }
                1 => {
                    // 장기 하락 추세
                    prev - rng.gen_range(50.0..150.0)
                }
                _ => {
                    // 장기 횡보 (작은 변동)
                    prev + rng.gen_range(-50.0..50.0)
                }
            };
            prices.push(next.max(1000.0));
        }

        // 정규화
        let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;

        for i in 0..seq_len {
            let normalized = (prices[i] - min) / range;
            sequences.push(normalized);
        }

        let target_normalized = (prices[seq_len] - min) / range;
        targets.push(target_normalized);
    }

    let x = Tensor::from_vec(sequences, &[n, seq_len, 1], &device)?;
    let y = Tensor::from_vec(targets, &[n, 1], &device)?;

    Ok((x, y))
}

/// 시퀀스 생성
fn create_sequence(prices: &[f32]) -> Result<Tensor> {
    let device = Device::Cpu;

    let min = prices.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;

    let normalized: Vec<f32> = prices.iter().map(|p| (p - min) / range).collect();

    Tensor::from_vec(normalized, &[1, prices.len(), 1], &device)
}

/// LSTM 학습
fn train_lstm(
    model: &SimpleLSTM,
    varmap: &VarMap,
    train_x: &Tensor,
    train_y: &Tensor,
    epochs: usize,
) -> Result<()> {
    let params = ParamsAdamW {
        lr: 0.01,
        ..Default::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), params)?;

    for epoch in 0..epochs {
        let predictions = model.forward(train_x)?;
        let loss = loss::mse(&predictions, train_y)?;
        optimizer.backward_step(&loss)?;

        if epoch % 10 == 0 {
            let loss_val = loss.to_vec0::<f32>()?;
            println!("  Epoch {:3}/{}  Loss: {:.6}", epoch, epochs, loss_val);
        }
    }

    println!("  🎉 학습 완료!");
    Ok(())
}
