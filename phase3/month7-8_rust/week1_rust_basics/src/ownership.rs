// Day 3-4: 소유권 시스템 (Rust의 핵심!)

pub fn run() {
    println!("  🔑 소유권 (Ownership)");
    ownership_basics();

    println!("\n  📌 빌림 (Borrowing)");
    borrowing();

    println!("\n  🔪 슬라이스 (Slice)");
    slices();
}

fn ownership_basics() {
    // 1. 각 값은 하나의 소유자만 가짐
    let s1 = String::from("hello");
    println!("    s1 = {}", s1);

    // 2. 소유권 이동 (move)
    let s2 = s1; // s1의 소유권이 s2로 이동
    println!("    s2 = {}", s2);
    // println!("{}", s1);  // 에러! s1은 더 이상 유효하지 않음

    // 3. 복사 (Copy trait)
    let x = 5;
    let y = x; // 정수는 Copy trait 구현 → 복사됨
    println!("    x = {}, y = {} (둘 다 사용 가능)", x, y);

    // 4. 클론 (명시적 복사)
    let s3 = String::from("world");
    let s4 = s3.clone(); // 깊은 복사
    println!("    s3 = {}, s4 = {} (둘 다 사용 가능)", s3, s4);

    // 5. 함수와 소유권
    let s5 = String::from("function");
    takes_ownership(s5); // s5의 소유권이 함수로 이동
                         // println!("{}", s5);  // 에러! s5는 더 이상 유효하지 않음

    let x2 = 10;
    makes_copy(x2); // 정수는 복사됨
    println!("    x2 = {} (여전히 사용 가능)", x2);
}

fn takes_ownership(s: String) {
    println!("    함수가 소유권 받음: {}", s);
} // s가 여기서 drop됨 (메모리 해제)

fn makes_copy(x: i32) {
    println!("    함수가 복사본 받음: {}", x);
}

fn borrowing() {
    // 1. 불변 참조 (immutable reference)
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1: s1을 빌림
    println!("    '{}' 길이 = {} (s1 여전히 사용 가능)", s1, len);

    // 2. 가변 참조 (mutable reference)
    let mut s2 = String::from("hello");
    change(&mut s2); // &mut s2: 가변 빌림
    println!("    변경 후: {}", s2);

    // 3. 참조 규칙
    let mut s3 = String::from("test");

    // 규칙 1: 불변 참조는 여러 개 가능
    let r1 = &s3;
    let r2 = &s3;
    println!("    r1 = {}, r2 = {}", r1, r2);

    // 규칙 2: 가변 참조는 하나만 가능 (동시에)
    let r3 = &mut s3;
    r3.push_str("!");
    println!("    r3 = {}", r3);
    // let r4 = &mut s3;  // 에러! 가변 참조는 하나만

    // 규칙 3: 가변 참조와 불변 참조는 동시에 불가능
    // let r5 = &s3;  // 에러! r3(가변)이 아직 사용 중
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s는 참조이므로 drop되지 않음

fn change(s: &mut String) {
    s.push_str(", world");
}

fn slices() {
    // 슬라이스: 컬렉션의 일부를 참조
    let s = String::from("hello world");

    let hello = &s[0..5]; // 또는 &s[..5]
    let world = &s[6..11]; // 또는 &s[6..]
    let full = &s[..]; // 전체

    println!("    hello = {}", hello);
    println!("    world = {}", world);
    println!("    full = {}", full);

    // 배열 슬라이스
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("    배열 슬라이스 = {:?}", slice);

    // 실용 예제: 첫 단어 찾기
    let sentence = String::from("hello rust world");
    let first = first_word(&sentence);
    println!("    첫 단어 = {}", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
