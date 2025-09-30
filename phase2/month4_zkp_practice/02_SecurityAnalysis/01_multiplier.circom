pragma circom 2.0.0;

/*
    간단한 곱셈 회로 - 첫 번째 ZK 회로!
    
    목표: a * b = c 를 증명하되, a와 b는 비공개로 유지
    
    Public Input: c (곱셈 결과)
    Private Input: a, b (곱셈 인수들)
    
    제약 조건: a * b === c
*/

template Multiplier() {
    // 신호 선언
    signal input a;    // 비공개 입력 a
    signal input b;    // 비공개 입력 b
    signal output c;   // 공개 출력 c
    
    // 제약 조건: a * b = c
    c <== a * b;
}

// 메인 컴포넌트 인스턴스화
component main = Multiplier();
