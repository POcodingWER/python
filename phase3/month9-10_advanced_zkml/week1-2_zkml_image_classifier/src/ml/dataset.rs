// 📊 데이터셋 - 간단한 합성 MNIST 스타일 데이터

/// 간단한 합성 데이터 생성
pub fn generate_synthetic_data(num_samples: usize) -> (Vec<Vec<f32>>, Vec<usize>) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut images = Vec::new();
    let mut labels = Vec::new();

    for _ in 0..num_samples {
        let label = rng.gen_range(0..10);
        let image = generate_digit_image(label);

        images.push(image);
        labels.push(label);
    }

    (images, labels)
}

/// 특정 숫자의 간단한 이미지 생성
fn generate_digit_image(digit: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut image = vec![0.0; 784]; // 28x28

    // 숫자마다 다른 패턴 생성
    match digit {
        0 => {
            // 원 모양
            for y in 8..20 {
                for x in 8..20 {
                    let dx = x as f32 - 14.0;
                    let dy = y as f32 - 14.0;
                    if dx * dx + dy * dy < 36.0 && dx * dx + dy * dy > 16.0 {
                        image[y * 28 + x] = rng.gen_range(0.7..1.0);
                    }
                }
            }
        }
        1 => {
            // 세로 선
            for y in 6..22 {
                for x in 12..16 {
                    image[y * 28 + x] = rng.gen_range(0.7..1.0);
                }
            }
        }
        2 => {
            // 숫자 2: 위 가로선 + 오른쪽 세로 + 중간 가로선 + 왼쪽 세로 + 아래 가로선
            // 위 가로선
            for x in 8..20 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            // 오른쪽 위 세로선
            for y in 8..14 {
                image[y * 28 + 18] = rng.gen_range(0.7..1.0);
            }
            // 중간 가로선
            for x in 8..20 {
                image[14 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            // 왼쪽 아래 세로선
            for y in 14..20 {
                image[y * 28 + 8] = rng.gen_range(0.7..1.0);
            }
            // 아래 가로선
            for x in 8..20 {
                image[20 * 28 + x] = rng.gen_range(0.7..1.0);
            }
        }
        3 => {
            // 오른쪽 세로선 + 가로선들
            for y in 8..20 {
                image[y * 28 + 18] = rng.gen_range(0.7..1.0);
            }
            for x in 8..20 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0);
                image[14 * 28 + x] = rng.gen_range(0.7..1.0);
                image[20 * 28 + x] = rng.gen_range(0.7..1.0);
            }
        }
        4 => {
            // 왼쪽 위 세로 + 가로 + 오른쪽 세로
            for y in 8..14 {
                image[y * 28 + 8] = rng.gen_range(0.7..1.0);
            }
            for x in 8..20 {
                image[14 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            for y in 8..20 {
                image[y * 28 + 18] = rng.gen_range(0.7..1.0);
            }
        }
        5 => {
            // 위, 중간, 아래 가로선 + 세로선
            for x in 8..20 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0);
                image[14 * 28 + x] = rng.gen_range(0.7..1.0);
                image[20 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            for y in 8..14 {
                image[y * 28 + 8] = rng.gen_range(0.7..1.0);
            }
            for y in 14..20 {
                image[y * 28 + 18] = rng.gen_range(0.7..1.0);
            }
        }
        6 => {
            // 5와 비슷하지만 아래 왼쪽 추가
            for x in 8..20 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0);
                image[14 * 28 + x] = rng.gen_range(0.7..1.0);
                image[20 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            for y in 8..20 {
                image[y * 28 + 8] = rng.gen_range(0.7..1.0);
            }
            for y in 14..20 {
                image[y * 28 + 18] = rng.gen_range(0.7..1.0);
            }
        }
        7 => {
            // 위 가로선 + 오른쪽 대각선
            for x in 8..20 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            for i in 0..12 {
                let y = 8 + i;
                let x = 18 - i;
                if x >= 8 && x < 20 {
                    image[y * 28 + x] = rng.gen_range(0.7..1.0);
                }
            }
        }
        8 => {
            // 숫자 8: 위 사각형 + 아래 사각형
            // 위 사각형
            for x in 10..18 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0); // 위
                image[13 * 28 + x] = rng.gen_range(0.7..1.0); // 아래
            }
            for y in 8..14 {
                image[y * 28 + 10] = rng.gen_range(0.7..1.0); // 왼쪽
                image[y * 28 + 17] = rng.gen_range(0.7..1.0); // 오른쪽
            }
            // 아래 사각형
            for x in 10..18 {
                image[14 * 28 + x] = rng.gen_range(0.7..1.0); // 위
                image[20 * 28 + x] = rng.gen_range(0.7..1.0); // 아래
            }
            for y in 14..21 {
                image[y * 28 + 10] = rng.gen_range(0.7..1.0); // 왼쪽
                image[y * 28 + 17] = rng.gen_range(0.7..1.0); // 오른쪽
            }
        }
        9 => {
            // 6을 뒤집은 모양
            for x in 8..20 {
                image[8 * 28 + x] = rng.gen_range(0.7..1.0);
                image[14 * 28 + x] = rng.gen_range(0.7..1.0);
                image[20 * 28 + x] = rng.gen_range(0.7..1.0);
            }
            for y in 8..14 {
                image[y * 28 + 8] = rng.gen_range(0.7..1.0);
            }
            for y in 8..20 {
                image[y * 28 + 18] = rng.gen_range(0.7..1.0);
            }
        }
        _ => {}
    }

    // 노이즈 추가
    for pixel in image.iter_mut() {
        if *pixel > 0.0 {
            *pixel += rng.gen_range(-0.1..0.1);
            if *pixel < 0.0 {
                *pixel = 0.0;
            }
            if *pixel > 1.0 {
                *pixel = 1.0;
            }
        }
    }

    image
}

/// 이미지를 ASCII로 시각화
pub fn visualize_image(image: &[f32]) {
    for y in 0..28 {
        for x in 0..28 {
            let pixel = image[y * 28 + x];
            let char = if pixel > 0.7 {
                "█"
            } else if pixel > 0.4 {
                "▓"
            } else if pixel > 0.2 {
                "▒"
            } else if pixel > 0.1 {
                "░"
            } else {
                " "
            };
            print!("{}", char);
        }
        println!();
    }
}
