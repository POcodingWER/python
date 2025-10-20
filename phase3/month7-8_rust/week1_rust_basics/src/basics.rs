// Day 1-2: Rust 기본 문법

pub fn run() {
    println!("  📚 변수와 데이터 타입");
    variables();

    println!("\n  🔧 함수");
    functions();

    println!("\n  🔄 제어 구조");
    control_flow();

    println!("\n  📊 벡터 (AI 준비)");
    vectors();
}

fn variables() {
    // 불변 변수 (기본)
    let x = 5;
    println!("    불변 변수 x = {}", x);

    // 가변 변수
    let mut y = 10;
    println!("    가변 변수 y = {}", y);
    y = 20;
    println!("    y 변경 후 = {}", y);

    // 타입 명시
    let z: f64 = 3.14;
    println!("    실수 z = {}", z);

    // 튜플
    let tuple = (1, 2.5, "hello");
    println!("    튜플 = {:?}", tuple);
}

fn functions() {
    let a = 10;
    let b = 20;

    let sum = add(a, b);
    let product = multiply(a, b);

    println!("    {} + {} = {}", a, b, sum);
    println!("    {} × {} = {}", a, b, product);

    // 표현식 vs 문장
    let result = {
        let x = 3;
        x + 1 // 세미콜론 없음 = 표현식 (반환값)
    };
    println!("    블록 표현식 결과 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // return 생략 가능
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn control_flow() {
    // if 표현식
    let number = 7;
    let description = if number < 5 {
        "작음"
    } else if number < 10 {
        "중간"
    } else {
        "큼"
    };
    println!("    {} is {}", number, description);

    // loop (무한 루프)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // loop도 값을 반환 가능!
        }
    };
    println!("    loop 결과 = {}", result);

    // while
    let mut n = 3;
    print!("    while 카운트다운: ");
    while n > 0 {
        print!("{} ", n);
        n -= 1;
    }
    println!("발사! 🚀");

    // for (range)
    print!("    for 0..5: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
}

fn vectors() {
    // Python의 list와 유사
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("    벡터: {:?}", numbers);

    // 추가
    numbers.push(6);
    println!("    push(6): {:?}", numbers);

    // 접근
    println!("    numbers[0] = {}", numbers[0]);

    // 반복
    print!("    각 요소 × 2: ");
    for n in &numbers {
        print!("{} ", n * 2);
    }
    println!();

    // map (Python의 list comprehension과 유사)
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("    map으로 × 2: {:?}", doubled);

    // filter
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("    짝수만: {:?}", evens);
}
