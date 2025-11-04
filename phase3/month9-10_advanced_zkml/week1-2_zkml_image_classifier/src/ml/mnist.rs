// 📊 MNIST 데이터 다운로드 및 로딩
// Week 5-3 DCGAN 코드에서 가져옴

use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

/// MNIST 데이터 다운로드
pub fn download_mnist() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = "mnist_data";
    fs::create_dir_all(data_dir)?;

    let base_url = "https://ossci-datasets.s3.amazonaws.com/mnist/";
    let files = ["train-images-idx3-ubyte.gz", "train-labels-idx1-ubyte.gz"];

    println!("📥 MNIST 데이터 확인 중...");

    for file in &files {
        let path = format!("{}/{}", data_dir, file);

        // 파일이 이미 있으면 스킵!
        if Path::new(&path).exists() {
            println!("  ✅ {} 이미 존재 (다운로드 스킵)", file);
            continue;
        }

        println!("  ⬇️  {} 다운로드 중...", file);
        let url = format!("{}{}", base_url, file);

        let response = reqwest::blocking::get(&url)?;

        if !response.status().is_success() {
            return Err(format!("다운로드 실패: HTTP {}", response.status()).into());
        }

        let bytes = response.bytes()?;

        let mut file_handle = File::create(&path)?;
        file_handle.write_all(&bytes)?;

        println!("  ✅ {} 다운로드 완료! ({}KB)", file, bytes.len() / 1024);
    }

    println!("✅ MNIST 데이터 준비 완료!");
    Ok(())
}

/// MNIST 이미지 로딩
fn load_mnist_images(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut gz = GzDecoder::new(file);
    let mut data = Vec::new();
    gz.read_to_end(&mut data)?;

    // Skip header (16 bytes)
    Ok(data[16..].to_vec())
}

/// MNIST 레이블 로딩
fn load_mnist_labels(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut gz = GzDecoder::new(file);
    let mut data = Vec::new();
    gz.read_to_end(&mut data)?;

    // Skip header (8 bytes)
    Ok(data[8..].to_vec())
}

/// MNIST 데이터 준비 (모든 클래스)
pub fn prepare_mnist_data() -> Result<(Vec<Vec<f32>>, Vec<usize>), Box<dyn std::error::Error>> {
    println!("📊 MNIST 데이터 로딩 중...");

    let images = load_mnist_images("mnist_data/train-images-idx3-ubyte.gz")?;
    let labels = load_mnist_labels("mnist_data/train-labels-idx1-ubyte.gz")?;

    let num_images = labels.len();
    println!("✅ MNIST 이미지 {}개 로딩 완료!", num_images);

    // 이미지 변환 (0-255 → 0.0-1.0)
    let mut image_vec = Vec::new();
    let mut label_vec = Vec::new();

    for (i, &label) in labels.iter().enumerate() {
        let start = i * 28 * 28;
        let end = start + 28 * 28;
        let image: Vec<f32> = images[start..end]
            .iter()
            .map(|&x| x as f32 / 255.0) // 0.0 ~ 1.0로 정규화
            .collect();

        image_vec.push(image);
        label_vec.push(label as usize);
    }

    Ok((image_vec, label_vec))
}

/// MNIST 데이터 준비 (특정 클래스만)
pub fn prepare_mnist_data_filtered(
    target_digit: usize,
) -> Result<(Vec<Vec<f32>>, Vec<usize>), Box<dyn std::error::Error>> {
    println!("📊 MNIST 데이터 로딩 중 (숫자 {}만)...", target_digit);

    let images = load_mnist_images("mnist_data/train-images-idx3-ubyte.gz")?;
    let labels = load_mnist_labels("mnist_data/train-labels-idx1-ubyte.gz")?;

    // 특정 숫자만 필터링
    let mut image_vec = Vec::new();
    let mut label_vec = Vec::new();

    for (i, &label) in labels.iter().enumerate() {
        if label as usize == target_digit {
            let start = i * 28 * 28;
            let end = start + 28 * 28;
            let image: Vec<f32> = images[start..end]
                .iter()
                .map(|&x| x as f32 / 255.0) // 0.0 ~ 1.0로 정규화
                .collect();

            image_vec.push(image);
            label_vec.push(label as usize);
        }
    }

    println!(
        "✅ 숫자 {} 이미지 {}개 로딩 완료!",
        target_digit,
        image_vec.len()
    );

    Ok((image_vec, label_vec))
}
