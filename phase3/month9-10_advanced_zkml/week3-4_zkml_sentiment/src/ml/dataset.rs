use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSample {
    pub text: String,
    pub label: i32, // 0: Negative, 1: Neutral, 2: Positive
}

/// 레이블 분포 확인
pub fn label_distribution(dataset: &[SentimentSample]) -> HashMap<i32, usize> {
    let mut dist = HashMap::new();
    for sample in dataset {
        *dist.entry(sample.label).or_insert(0) += 1;
    }
    dist
}

/// NSMC (Naver Sentiment Movie Corpus) 다운로드
pub fn download_nsmc() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = "data";
    std::fs::create_dir_all(data_dir)?;

    let train_url = "https://raw.githubusercontent.com/e9t/nsmc/master/ratings_train.txt";
    let test_url = "https://raw.githubusercontent.com/e9t/nsmc/master/ratings_test.txt";

    let train_path = format!("{}/ratings_train.txt", data_dir);
    let test_path = format!("{}/ratings_test.txt", data_dir);

    // 이미 다운로드되어 있으면 스킵
    if std::path::Path::new(&train_path).exists() && std::path::Path::new(&test_path).exists() {
        println!("   ✅ NSMC 데이터셋이 이미 존재합니다!");
        return Ok(());
    }

    println!("   📥 NSMC 데이터셋 다운로드 중...");

    // Train 데이터 다운로드
    println!("      - ratings_train.txt 다운로드 중...");
    let train_content = reqwest::blocking::get(train_url)?.text()?;
    let mut train_file = File::create(&train_path)?;
    train_file.write_all(train_content.as_bytes())?;
    println!("      ✅ ratings_train.txt 다운로드 완료!");

    // Test 데이터 다운로드
    println!("      - ratings_test.txt 다운로드 중...");
    let test_content = reqwest::blocking::get(test_url)?.text()?;
    let mut test_file = File::create(&test_path)?;
    test_file.write_all(test_content.as_bytes())?;
    println!("      ✅ ratings_test.txt 다운로드 완료!");

    Ok(())
}

/// NSMC 데이터 로드 (TSV 형식)
pub fn load_nsmc(
    file_path: &str,
    max_samples: Option<usize>,
) -> Result<Vec<SentimentSample>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut samples = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue; // 헤더 스킵
        }

        if let Some(max) = max_samples {
            if samples.len() >= max {
                break;
            }
        }

        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() >= 3 {
            let text = parts[1].to_string();
            let label_str = parts[2];

            // NSMC는 0(부정), 1(긍정)
            // 우리 모델도 0(부정), 1(긍정) 2개 클래스로 단순화!
            if let Ok(nsmc_label) = label_str.parse::<i32>() {
                let label = nsmc_label; // 0 -> 0 (부정), 1 -> 1 (긍정)

                // 빈 텍스트 제외
                if !text.trim().is_empty() {
                    samples.push(SentimentSample { text, label });
                }
            }
        }
    }

    Ok(samples)
}

/// NSMC 데이터 준비 (다운로드 + 로드)
pub fn prepare_nsmc_data(
    max_train: Option<usize>,
    max_test: Option<usize>,
) -> Result<(Vec<SentimentSample>, Vec<SentimentSample>), Box<dyn std::error::Error>> {
    println!("📊 NSMC 데이터셋 준비 중...");

    // 다운로드
    download_nsmc()?;

    // 로드
    println!("   📂 학습 데이터 로딩 중...");
    let train_data = load_nsmc("data/ratings_train.txt", max_train)?;
    println!("      ✅ 학습 데이터: {}개", train_data.len());

    println!("   📂 테스트 데이터 로딩 중...");
    let test_data = load_nsmc("data/ratings_test.txt", max_test)?;
    println!("      ✅ 테스트 데이터: {}개", test_data.len());

    Ok((train_data, test_data))
}
