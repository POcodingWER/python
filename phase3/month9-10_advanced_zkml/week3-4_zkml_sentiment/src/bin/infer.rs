use zkml_sentiment::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 ZKML 감성 분석 추론 테스트!\n");

    // 1. 모델 로드
    println!("📂 모델 로딩 중...");
    let model_json = std::fs::read_to_string("models/sentiment_model.json")?;
    let model: SentimentModel = serde_json::from_str(&model_json)?;
    println!("   ✅ 모델 로드 완료!");

    // 2. 토크나이저 로드
    println!("📂 토크나이저 로딩 중...");
    let tokenizer_json = std::fs::read_to_string("models/tokenizer.json")?;
    let tokenizer: Tokenizer = serde_json::from_str(&tokenizer_json)?;
    println!(
        "   ✅ 토크나이저 로드 완료! (vocab_size: {})",
        tokenizer.vocab_size
    );

    // 3. 한국어 테스트 문장들
    let test_texts = vec![
        "이 영화 정말 재밌어요 최고예요",
        "완전 쓰레기 영화 시간 낭비",
        "배우들 연기가 너무 좋았어요",
        "스토리가 엉망이고 지루해요",
        "감동적인 영화 꼭 보세요",
        "돈 아깝네요 별로예요",
        "와 진짜 명작이네요 강추합니다",
        "최악의 영화 돈과 시간 버렸어요",
    ];

    println!("\n🧪 추론 테스트 (Bag-of-Words + Dense Network):\n");

    for text in test_texts {
        // BoW 방식으로 인코딩
        let word_indices = tokenizer.encode(text);
        let probs = model.forward_bow(&word_indices);
        let prediction = model.predict_bow(&word_indices);

        let sentiment_label = match prediction {
            0 => "😞 부정 (Negative)",
            1 => "😊 긍정 (Positive)",
            _ => "❓ Unknown",
        };

        println!("📝 Text: \"{}\"", text);
        println!("   Prediction: {} (class {})", sentiment_label, prediction);
        println!(
            "   Probabilities: [부정: {:.2}%, 긍정: {:.2}%]",
            probs[0] * 100.0,
            probs[1] * 100.0
        );
        println!();
    }

    println!("✅ 추론 테스트 완료!");

    Ok(())
}
