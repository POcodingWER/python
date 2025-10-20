// Day 5-6: 구조체 & 메서드 (AI를 위한 벡터 연산)

#[derive(Debug, Clone)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    // 생성자 (associated function)
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    // 메서드 (self를 받음)
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let mag = self.magnitude();
        Vector2D {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

pub fn run() {
    println!("  🧮 벡터 연산 (AI 준비)");
    vector_operations();

    println!("\n  📦 열거형 & 패턴 매칭");
    enums_and_matching();

    println!("\n  ⚠️  에러 처리");
    error_handling();
}

fn vector_operations() {
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = Vector2D::new(1.0, 2.0);

    println!("    v1 = {:?}", v1);
    println!("    v2 = {:?}", v2);

    // 크기
    println!("    |v1| = {:.2}", v1.magnitude());

    // 정규화
    let v1_norm = v1.normalize();
    println!("    v1 정규화 = {:?}", v1_norm);
    println!("    |v1_norm| = {:.2}", v1_norm.magnitude());

    // 덧셈
    let v3 = v1.add(&v2);
    println!("    v1 + v2 = {:?}", v3);

    // 내적
    let dot_product = v1.dot(&v2);
    println!("    v1 · v2 = {:.2}", dot_product);
}

// 열거형: Python의 Enum과 유사하지만 훨씬 강력!
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enums_and_matching() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("    메시지 처리:");
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);

    // Option<T>: Rust의 null 대체
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("\n    Option 처리:");
    println!("      some_number = {:?}", some_number);
    println!("      no_number = {:?}", no_number);

    // match로 Option 처리
    match some_number {
        Some(n) => println!("      값이 있음: {}", n),
        None => println!("      값이 없음"),
    }

    // if let (간단한 경우)
    if let Some(n) = some_number {
        println!("      if let: 값 = {}", n);
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("      Quit 메시지");
        }
        Message::Move { x, y } => {
            println!("      Move 메시지: x={}, y={}", x, y);
        }
        Message::Write(text) => {
            println!("      Write 메시지: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("      ChangeColor 메시지: RGB({}, {}, {})", r, g, b);
        }
    }
}

// Result<T, E>: 에러 처리
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("0으로 나눌 수 없습니다"))
    } else {
        Ok(a / b)
    }
}

fn error_handling() {
    // Result 처리 방법 1: match
    match divide(10.0, 2.0) {
        Ok(result) => println!("    10.0 / 2.0 = {}", result),
        Err(e) => println!("    에러: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("    10.0 / 0.0 = {}", result),
        Err(e) => println!("    에러: {}", e),
    }

    // Result 처리 방법 2: unwrap_or
    let result1 = divide(20.0, 4.0).unwrap_or(0.0);
    let result2 = divide(20.0, 0.0).unwrap_or(-1.0);

    println!("    unwrap_or: {} / {}", result1, result2);

    // Result 처리 방법 3: ? 연산자 (함수에서만)
    // 에러를 자동으로 전파
    println!("    계산 체인:");
    if let Ok(result) = calculate_chain() {
        println!("      성공: {}", result);
    }
}

fn calculate_chain() -> Result<f64, String> {
    let a = divide(100.0, 2.0)?; // 50.0
    let b = divide(a, 5.0)?; // 10.0
    let c = divide(b, 2.0)?; // 5.0
    Ok(c)
}
