use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use zkml_sentiment::*;

/// 서버 상태 (모델 & 토크나이저)
struct AppState {
    model: Mutex<SentimentModel>,
    tokenizer: Mutex<Tokenizer>,
}

#[derive(Deserialize)]
struct InferRequest {
    text: String,
}

#[derive(Serialize)]
struct InferResponse {
    sentiment: usize,
    sentiment_label: String,
    probabilities: Vec<f32>,
    text_hash: String,
}

#[derive(Deserialize)]
struct ProveRequest {
    text: String,
}

#[derive(Serialize)]
struct ProveResponse {
    proof: SentimentProof,
}

#[derive(Deserialize)]
struct VerifyRequest {
    proof: SentimentProof,
}

#[derive(Serialize)]
struct VerifyResponse {
    valid: bool,
    message: String,
}

/// Health check
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "service": "ZKML Sentiment Analysis",
        "version": "0.1.0",
        "status": "running",
        "endpoints": {
            "GET /": "Health check & API documentation",
            "POST /api/infer": "Sentiment analysis inference",
            "POST /api/prove": "Generate ZK proof",
            "POST /api/verify": "Verify ZK proof"
        }
    }))
}

/// 감성 분석 추론
#[post("/api/infer")]
async fn infer(
    data: web::Data<AppState>,
    req: web::Json<InferRequest>,
) -> impl Responder {
    let model = data.model.lock().unwrap();
    let tokenizer = data.tokenizer.lock().unwrap();

    // BoW 방식으로 인코딩
    let word_indices = tokenizer.encode(&req.text);
    let probs = model.forward_bow(&word_indices);
    let prediction = model.predict_bow(&word_indices);

    let sentiment_label = match prediction {
        0 => "부정 (Negative)",
        1 => "긍정 (Positive)",
        _ => "Unknown",
    };

    // Text hash
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(req.text.as_bytes());
    let text_hash = format!("{:x}", hasher.finalize());

    HttpResponse::Ok().json(InferResponse {
        sentiment: prediction,
        sentiment_label: sentiment_label.to_string(),
        probabilities: probs,
        text_hash,
    })
}

/// ZK 증명 생성
#[post("/api/prove")]
async fn prove(
    data: web::Data<AppState>,
    req: web::Json<ProveRequest>,
) -> impl Responder {
    let model = data.model.lock().unwrap();
    let tokenizer = data.tokenizer.lock().unwrap();

    // BoW 방식으로 추론
    let word_indices = tokenizer.encode(&req.text);
    let prediction = model.predict_bow(&word_indices);

    // 모델 가중치 수집
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

    // ZK 증명 생성
    match generate_proof_halo2(prediction, &req.text, &model_weights) {
        Ok(proof) => HttpResponse::Ok().json(ProveResponse { proof }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Proof generation failed: {}", e)
        })),
    }
}

/// ZK 증명 검증
#[post("/api/verify")]
async fn verify(req: web::Json<VerifyRequest>) -> impl Responder {
    match verify_proof(&req.proof) {
        Ok(_) => HttpResponse::Ok().json(VerifyResponse {
            valid: true,
            message: "Proof is valid!".to_string(),
        }),
        Err(e) => HttpResponse::Ok().json(VerifyResponse {
            valid: false,
            message: format!("Proof verification failed: {}", e),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 ZKML Sentiment Analysis Server Starting...\n");

    // 모델 로드
    println!("📂 Loading model...");
    let model_json = std::fs::read_to_string("models/sentiment_model.json")
        .expect("Failed to read model file");
    let model: SentimentModel =
        serde_json::from_str(&model_json).expect("Failed to parse model");
    println!("   ✅ Model loaded!");

    // 토크나이저 로드
    println!("📂 Loading tokenizer...");
    let tokenizer_json = std::fs::read_to_string("models/tokenizer.json")
        .expect("Failed to read tokenizer file");
    let tokenizer: Tokenizer =
        serde_json::from_str(&tokenizer_json).expect("Failed to parse tokenizer");
    println!("   ✅ Tokenizer loaded! (vocab_size: {})", tokenizer.vocab_size);

    let app_state = web::Data::new(AppState {
        model: Mutex::new(model),
        tokenizer: Mutex::new(tokenizer),
    });

    println!("\n🌐 Server running at http://localhost:8080");
    println!("📚 API Documentation: http://localhost:8080\n");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(index)
            .service(infer)
            .service(prove)
            .service(verify)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

