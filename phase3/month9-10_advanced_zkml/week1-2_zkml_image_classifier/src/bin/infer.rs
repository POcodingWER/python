// 🔮 추론 테스트

use zkml_classifier::{SimpleClassifier, prepare_mnist_data};
use rand::seq::SliceRandom;

/// 간단한 이미지 시각화 (ASCII)
fn visualize_image(image: &[f32]) {
    for y in 0..28 {
        for x in 0..28 {
            let idx = y * 28 + x;
            let pixel = image[idx];
            let char = if pixel > 0.8 {
                '█'
            } else if pixel > 0.6 {
                '▓'
            } else if pixel > 0.4 {
                '▒'
            } else if pixel > 0.2 {
                '░'
            } else {
                ' '
            };
            print!("{}", char);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔮 ZKML 이미지 분류기 - 추론 테스트               ║");
    println!("║                 (실제 MNIST 데이터)                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. 모델 로드
    println!("📂 [1/3] 모델 로드 중...");
    let model = SimpleClassifier::load("models/classifier.json")?;
    println!("   ✅ 모델 로드 완료!\n");

    // 2. 테스트 데이터 로딩
    println!("📊 [2/3] MNIST 테스트 데이터 로딩 중...");
    let (images, labels) = prepare_mnist_data()?;
    
    // 랜덤으로 10개 샘플 선택
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..images.len()).collect();
    indices.shuffle(&mut rng);
    let test_indices = &indices[..10];
    
    println!("   ✅ 10개 랜덤 샘플 선택 완료!\n");

    // 3. 추론 테스트
    println!("🔮 [3/3] 추론 테스트 중...\n");
    let mut correct = 0;
    
    for (i, &idx) in test_indices.iter().enumerate() {
        let image = &images[idx];
        let label = labels[idx];
        let predicted = model.predict(image);
        let is_correct = predicted == label;
        
        if is_correct {
            correct += 1;
        }
        
        println!("샘플 {} (MNIST #{}):", i + 1, idx);
        visualize_image(image);
        println!("실제: {}, 예측: {} {}", 
            label, 
            predicted, 
            if is_correct { "✅" } else { "❌" }
        );
        println!();
    }

    let accuracy = correct as f32 / test_indices.len() as f32 * 100.0;
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          정확도: {:.1}% ({}/{})                           ║", accuracy, correct, test_indices.len());
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}

