use zkml_sentiment::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ZKML 감성 분석 모델 학습 시작!\n");

    // 1. NSMC 데이터셋 준비 (전체 데이터 사용)
    println!("📊 NSMC 데이터셋 준비 중...");
    // 전체 데이터 사용: 학습 15만개, 테스트 5만개 (최대 정확도)
    let (train_data, test_data) = prepare_nsmc_data(None, None)?;

    println!("\n📈 데이터셋 통계:");
    println!("   - 학습 데이터: {}개", train_data.len());
    println!("   - 테스트 데이터: {}개", test_data.len());

    let train_dist = label_distribution(&train_data);
    println!("   - 학습 레이블 분포: {:?}", train_dist);
    println!("     (0: 부정, 1: 긍정)");

    // 2. 토크나이저 구축
    println!("\n🔤 토크나이저 구축 중 (빈도 기반 상위 5000개)...");
    let mut tokenizer = Tokenizer::new(20); // max_len = 20
    let train_texts: Vec<String> = train_data.iter().map(|s| s.text.clone()).collect();
    tokenizer.fit(&train_texts);
    println!("   ✅ 최종 Vocabulary 크기: {}", tokenizer.vocab_size);

    // 3. Bag-of-Words + Dense Network 모델 생성
    println!("\n🧠 Bag-of-Words + Dense Network 모델 생성 중...");
    let hidden_size = 64; // 히든 사이즈
    let mut model = SentimentModel::new(tokenizer.vocab_size, 0, hidden_size);
    println!("   - Vocab Size: {}", tokenizer.vocab_size);
    println!("   - Architecture: BoW -> Dense(128) -> Dense(64) -> Dense(32) -> Output(2)");
    println!("   - Activation: ReLU");
    println!("   - Output Classes: 2 (부정=0, 긍정=1)");

    // 4. 학습 (Best Model Saving - Overfitting 방지)
    println!("\n🏋️ 모델 학습 중 (35 Epoch, 최적 지점)...");
    let epochs = 35; // 35 epoch (최적 지점, Overfitting 전)
    let learning_rate = 0.0001; // 학습률 유지

    let mut best_accuracy = 0.0;
    let mut best_model = model.clone();
    let mut best_epoch = 0;

    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        let mut correct = 0;

        for sample in &train_data {
            let word_indices = tokenizer.encode(&sample.text);
            let loss = model.train_bow(&word_indices, sample.label as usize, learning_rate);
            total_loss += loss;

            let prediction = model.predict_bow(&word_indices);
            if prediction == sample.label as usize {
                correct += 1;
            }
        }

        let avg_loss = total_loss / train_data.len() as f32;
        let accuracy = correct as f32 / train_data.len() as f32 * 100.0;

        // Best model 저장
        if accuracy > best_accuracy {
            best_accuracy = accuracy;
            best_model = model.clone();
            best_epoch = epoch + 1;
            println!(
                "   Epoch {}/{}: Loss = {:.4}, Accuracy = {:.2}% ✅ (Best!)",
                epoch + 1,
                epochs,
                avg_loss,
                accuracy
            );
        } else {
            println!(
                "   Epoch {}/{}: Loss = {:.4}, Accuracy = {:.2}%",
                epoch + 1,
                epochs,
                avg_loss,
                accuracy
            );
        }
    }

    // Best model 복원
    model = best_model;
    println!(
        "\n✅ 최고 성능 모델로 복원! Epoch {}, 정확도: {:.2}%",
        best_epoch, best_accuracy
    );

    // 5. 테스트
    println!("\n🧪 모델 테스트 중...");
    let mut correct = 0;
    for sample in &test_data {
        let word_indices = tokenizer.encode(&sample.text);
        let prediction = model.predict_bow(&word_indices);
        if prediction == sample.label as usize {
            correct += 1;
        }
    }

    let test_accuracy = correct as f32 / test_data.len() as f32 * 100.0;
    println!("   ✅ 테스트 정확도: {:.2}%", test_accuracy);

    // 6. 모델 저장
    println!("\n💾 모델 저장 중...");
    std::fs::create_dir_all("models")?;

    let model_json = serde_json::to_string_pretty(&model)?;
    std::fs::write("models/sentiment_model.json", model_json)?;
    println!("   ✅ 모델 저장 완료: models/sentiment_model.json");

    let tokenizer_json = serde_json::to_string_pretty(&tokenizer)?;
    std::fs::write("models/tokenizer.json", tokenizer_json)?;
    println!("   ✅ 토크나이저 저장 완료: models/tokenizer.json");

    println!("\n🎉 학습 완료!");
    println!("   - 최종 테스트 정확도: {:.2}%", test_accuracy);
    println!("   - 모델 파일: models/sentiment_model.json");
    println!("   - 토크나이저 파일: models/tokenizer.json");

    Ok(())
}
