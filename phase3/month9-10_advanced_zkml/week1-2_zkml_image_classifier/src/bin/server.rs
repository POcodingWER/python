// 🌐 ZKML 이미지 분류기 - Actix-web 서버

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use zkml_classifier::{generate_proof_halo2, prepare_mnist_data, verify_proof, SimpleClassifier};

// 🎨 API 요청/응답 구조

#[derive(Debug, Deserialize)]
struct InferRequest {
    image: Vec<f32>, // 784개 픽셀 (28x28)
}

#[derive(Debug, Serialize)]
struct InferResponse {
    predicted_class: usize,
    confidence: f32,
}

#[derive(Debug, Deserialize)]
struct ProveRequest {
    image: Vec<f32>, // 784개 픽셀
}

#[derive(Debug, Serialize)]
struct ProveResponse {
    predicted_class: usize,
    proof_path: String,
    halo2_proof_size: usize,
}

#[derive(Debug, Deserialize)]
struct VerifyRequest {
    proof_path: String,
}

#[derive(Debug, Serialize)]
struct VerifyResponse {
    valid: bool,
    predicted_class: Option<usize>,
    message: String,
}

// 🔥 API 엔드포인트

/// GET / - 헬스 체크
async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "ZKML Image Classifier",
        "version": "1.0.0",
        "endpoints": {
            "GET /": "Health check",
            "POST /api/infer": "Inference (image → prediction)",
            "POST /api/prove": "Generate ZK proof",
            "POST /api/verify": "Verify ZK proof",
            "GET /api/random": "Get random MNIST image"
        }
    }))
}

/// POST /api/infer - 추론
async fn infer(req: web::Json<InferRequest>) -> impl Responder {
    println!("📊 추론 요청 받음");

    // 1. 모델 로드
    let model = match SimpleClassifier::load("models/classifier.json") {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("모델 로드 실패: {}", e)
            }))
        }
    };

    // 2. 추론
    let predicted_class = model.predict(&req.image);

    // 3. 신뢰도 계산 (간단 버전)
    let confidence = 0.95; // TODO: 실제 softmax 확률 계산

    println!("   ✅ 예측: {}, 신뢰도: {:.1}%", predicted_class, confidence * 100.0);

    HttpResponse::Ok().json(InferResponse {
        predicted_class,
        confidence,
    })
}

/// POST /api/prove - ZK 증명 생성
async fn prove(req: web::Json<ProveRequest>) -> impl Responder {
    println!("🔐 ZK 증명 생성 요청 받음");

    // 1. 모델 로드
    let model = match SimpleClassifier::load("models/classifier.json") {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("모델 로드 실패: {}", e)
            }))
        }
    };

    // 2. 추론
    let predicted_class = model.predict(&req.image);
    println!("   예측: {}", predicted_class);

    // 3. ZK 증명 생성
    let proof = match generate_proof_halo2(&req.image, predicted_class, &model) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("증명 생성 실패: {}", e)
            }))
        }
    };

    // 4. 증명 저장
    let proof_path = "proofs/proof.json";
    if let Err(e) = proof.save(proof_path) {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("증명 저장 실패: {}", e)
        }));
    }

    let halo2_proof_size = proof.halo2_proof.as_ref().map(|p| p.len()).unwrap_or(0);

    println!("   ✅ 증명 생성 완료! ({}bytes)", halo2_proof_size);

    HttpResponse::Ok().json(ProveResponse {
        predicted_class,
        proof_path: proof_path.to_string(),
        halo2_proof_size,
    })
}

/// POST /api/verify - ZK 증명 검증
async fn verify(req: web::Json<VerifyRequest>) -> impl Responder {
    println!("✅ ZK 증명 검증 요청 받음");

    // 1. 증명 로드
    let proof = match zkml_classifier::MLProof::load(&req.proof_path) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("증명 로드 실패: {}", e)
            }))
        }
    };

    // 2. 검증
    let valid = match verify_proof(&proof) {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("검증 실패: {}", e)
            }))
        }
    };

    let message = if valid {
        "✅ 검증 성공! 올바른 ML 모델로 계산되었습니다.".to_string()
    } else {
        "❌ 검증 실패! 증명이 유효하지 않습니다.".to_string()
    };

    println!("   {}", message);

    HttpResponse::Ok().json(VerifyResponse {
        valid,
        predicted_class: if valid { Some(proof.predicted_class) } else { None },
        message,
    })
}

/// GET /api/random - 랜덤 MNIST 이미지
async fn random_image() -> impl Responder {
    println!("🎲 랜덤 MNIST 이미지 요청");

    // MNIST 데이터 로드
    let (images, labels) = match prepare_mnist_data() {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("MNIST 로드 실패: {}", e)
            }))
        }
    };

    // 랜덤 선택
    let idx = rand::random::<usize>() % images.len();
    let image = &images[idx];
    let label = labels[idx];

    println!("   ✅ 이미지 #{} (라벨: {})", idx, label);

    HttpResponse::Ok().json(serde_json::json!({
        "image": image,
        "label": label,
        "index": idx
    }))
}

// 🚀 메인 함수

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       🔥 ZKML 이미지 분류기 - Actix-web 서버 🔥           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let host = "127.0.0.1";
    let port = 8080;

    println!("🌐 서버 시작 중...");
    println!("   • 주소: http://{}:{}", host, port);
    println!("   • API 문서: http://{}:{}/", host, port);
    println!("");

    HttpServer::new(|| {
        // CORS 설정 (프론트엔드 연동)
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/api/infer", web::post().to(infer))
            .route("/api/prove", web::post().to(prove))
            .route("/api/verify", web::post().to(verify))
            .route("/api/random", web::get().to(random_image))
    })
    .bind((host, port))?
    .run()
    .await
}

