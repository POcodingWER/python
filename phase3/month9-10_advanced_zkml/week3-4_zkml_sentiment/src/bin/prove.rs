use zkml_sentiment::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 ZKML 감성 분석 ZK 증명 생성!\n");

    // 1. 모델 로드
    println!("📂 모델 로딩 중...");
    let model_json = std::fs::read_to_string("models/sentiment_model.json")?;
    let model: SentimentModel = serde_json::from_str(&model_json)?;
    println!("   ✅ 모델 로드 완료!");

    // 2. 토크나이저 로드
    println!("📂 토크나이저 로딩 중...");
    let tokenizer_json = std::fs::read_to_string("models/tokenizer.json")?;
    let tokenizer: Tokenizer = serde_json::from_str(&tokenizer_json)?;
    println!("   ✅ 토크나이저 로드 완료!");

    // 3. 한국어 테스트 텍스트
    let test_text = "이 영화 정말 재밌어요 최고예요";
    println!("\n📝 테스트 텍스트: \"{}\"", test_text);

    // 4. 추론
    println!("\n🤖 추론 실행 중...");
    let embeddings = tokenizer.text_to_embedding(test_text, model.embedding_dim);
    let probs = model.forward(&embeddings);
    let prediction = model.predict(&embeddings);

    let sentiment_label = match prediction {
        0 => "😞 Negative",
        1 => "😐 Neutral",
        2 => "😊 Positive",
        _ => "❓ Unknown",
    };

    println!("   Prediction: {} (class {})", sentiment_label, prediction);
    println!(
        "   Probabilities: [Neg: {:.2}%, Neu: {:.2}%, Pos: {:.2}%]",
        probs[0] * 100.0,
        probs[1] * 100.0,
        probs[2] * 100.0
    );

    // 5. 모델 가중치 수집 (해시 계산용)
    let mut model_weights = Vec::new();
    // Dense1
    for row in &model.dense1_weights {
        model_weights.extend(row);
    }
    model_weights.extend(&model.dense1_bias);
    // Dense2
    for row in &model.dense2_weights {
        model_weights.extend(row);
    }
    model_weights.extend(&model.dense2_bias);
    // Dense3
    for row in &model.dense3_weights {
        model_weights.extend(row);
    }
    model_weights.extend(&model.dense3_bias);
    // Output
    for row in &model.output_weights {
        model_weights.extend(row);
    }
    model_weights.extend(&model.output_bias);

    // 6. Halo2 ZK 증명 생성
    println!("\n🔥 Halo2 ZK 증명 생성 중...");
    let proof = generate_proof_halo2(prediction, test_text, &model_weights)?;

    // 7. 증명 저장
    std::fs::create_dir_all("proofs")?;
    let proof_json = serde_json::to_string_pretty(&proof)?;
    std::fs::write("proofs/proof.json", proof_json)?;

    println!("\n✅ ZK 증명 생성 완료!");
    println!("   - 증명 파일: proofs/proof.json");
    println!("   - Sentiment: {}", sentiment_label);
    println!("   - Halo2 Proof: 32 bytes");

    Ok(())
}

