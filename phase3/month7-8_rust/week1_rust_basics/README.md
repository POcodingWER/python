# Week 1: Rust 기초 🦀

> **목표**: Rust 언어의 핵심 개념 이해 및 소유권 시스템 마스터

---

## 📋 학습 목표

### ✅ **Day 1-2: Rust 기본 문법**

- [x] Hello World 실행 ✅
- [ ] 변수와 데이터 타입
- [ ] 함수 정의
- [ ] 제어 구조 (if, loop, while, for)
- [ ] 표준 입출력

### ✅ **Day 3-4: 소유권 시스템** (핵심!)

- [ ] 소유권(Ownership) 이해
- [ ] 빌림(Borrowing) & 참조
- [ ] 슬라이스(Slice)
- [ ] 메모리 안전성 보장 원리

### ✅ **Day 5-6: 구조체 & 열거형**

- [ ] Struct 정의 및 사용
- [ ] Enum과 패턴 매칭
- [ ] Option<T> & Result<T, E>
- [ ] 에러 처리

### ✅ **Day 7: 실습 프로젝트**

- [ ] 간단한 계산기
- [ ] 벡터 연산 (AI 준비)
- [ ] 파일 I/O

---

## 🎯 실습 예제

### 1. 기본 문법

```rust
// src/01_basics.rs
fn main() {
    // 변수
    let x = 5;
    let mut y = 10;  // 가변 변수

    // 함수
    let result = add(x, y);
    println!("Result: {}", result);

    // 제어 구조
    if result > 10 {
        println!("Big!");
    }

    // 반복문
    for i in 0..5 {
        println!("{}", i);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // return 생략 가능
}
```

### 2. 소유권 (핵심!)

```rust
// src/02_ownership.rs
fn main() {
    // 소유권 이동
    let s1 = String::from("hello");
    let s2 = s1;  // s1은 더 이상 사용 불가!
    // println!("{}", s1);  // 에러!

    // 빌림 (참조)
    let s3 = String::from("world");
    let len = calculate_length(&s3);  // 빌림
    println!("{} length: {}", s3, len);  // s3 여전히 사용 가능
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 3. 구조체

```rust
// src/03_struct.rs
#[derive(Debug)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = Vector2D::new(1.0, 2.0);

    println!("v1: {:?}", v1);
    println!("Magnitude: {}", v1.magnitude());

    let v3 = v1.add(&v2);
    println!("v1 + v2 = {:?}", v3);
}
```

### 4. 열거형 & 에러 처리

```rust
// src/04_enum_error.rs
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // ? 연산자: 에러 전파
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("test.txt") {
        Ok(contents) => println!("File: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## 💡 Python vs Rust 비교

| 개념          | Python           | Rust                            |
| ------------- | ---------------- | ------------------------------- |
| **변수**      | `x = 5`          | `let x = 5;`                    |
| **가변 변수** | 기본 가변        | `let mut x = 5;`                |
| **함수**      | `def add(a, b):` | `fn add(a: i32, b: i32) -> i32` |
| **리스트**    | `[1, 2, 3]`      | `vec![1, 2, 3]`                 |
| **딕셔너리**  | `{"a": 1}`       | `HashMap::new()`                |
| **에러 처리** | `try/except`     | `Result<T, E>`                  |
| **None**      | `None`           | `Option::None`                  |

---

## 🚀 실행 방법

```bash
# 컴파일 및 실행
cargo run

# 빌드만
cargo build

# 릴리즈 빌드 (최적화)
cargo build --release

# 테스트
cargo test

# 코드 포맷팅
cargo fmt

# Linter
cargo clippy
```

---

## 📚 학습 자료

### 필수

- [The Rust Book (한국어)](https://doc.rust-kr.org/ch01-00-getting-started.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### 추천

- [Rustlings](https://github.com/rust-lang/rustlings) - 인터랙티브 연습
- [Exercism Rust Track](https://exercism.org/tracks/rust)

---

## ✅ Week 1 체크리스트

### Day 1-2

- [x] Rust 설치 확인 ✅
- [x] Hello World 실행 ✅
- [ ] 변수, 함수, 제어 구조 학습
- [ ] 10개 이상 예제 작성

### Day 3-4

- [ ] 소유권 개념 완전 이해
- [ ] 빌림 & 참조 실습
- [ ] 메모리 안전성 체감

### Day 5-6

- [ ] 구조체 5개 이상 작성
- [ ] Enum & Match 마스터
- [ ] Result/Option 에러 처리

### Day 7

- [ ] 벡터 연산 라이브러리 (AI 준비)
- [ ] 간단한 계산기 완성
- [ ] Week 1 회고 작성

---

**시작일**: 2025-10-20  
**목표 완료일**: 2025-10-27 (7일)  
**다음 단계**: Week 2 - Rust 고급 문법 🚀
