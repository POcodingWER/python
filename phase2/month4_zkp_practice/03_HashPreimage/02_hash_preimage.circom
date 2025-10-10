pragma circom 2.0.0;

include "circomlib/circuits/sha256/sha256.circom";
include "circomlib/circuits/bitify.circom";

/*
 * Hash Preimage 증명 회로
 * 
 * 목적: 해시값을 알고 있을 때, 원본 데이터를 알고 있다는 것을 증명
 * 응용: 패스워드 검증, 비밀 데이터 증명, 커밋-리빌 스킴
 * 
 * 입력:
 * - preimage: 원본 데이터 (private)
 * - hash: 해시 결과값 (public)
 * 
 * 증명: SHA256(preimage) == hash
 */

template HashPreimage() {
    // 입력 신호 정의
    signal input preimage[8];  // 256비트 원본 데이터 (32바이트를 8개 32비트로, private)
    signal output hash[8];     // 256비트 해시 결과 (public)
    
    // SHA256 컴포넌트 인스턴스화
    component sha256 = Sha256(256);
    
    // preimage를 비트로 변환
    component preimage_bits[8];
    for (var i = 0; i < 8; i++) {
        preimage_bits[i] = Num2Bits(32);
        preimage_bits[i].in <== preimage[i];
        
        // SHA256 입력에 연결 (리틀 엔디안)
        for (var j = 0; j < 32; j++) {
            sha256.in[i * 32 + j] <== preimage_bits[i].out[j];
        }
    }
    
    // SHA256 해시 계산 결과를 32비트 청크로 변환
    component hash_bits[8];
    for (var i = 0; i < 8; i++) {
        hash_bits[i] = Bits2Num(32);
        
        // SHA256 출력을 32비트씩 그룹화
        for (var j = 0; j < 32; j++) {
            hash_bits[i].in[j] <== sha256.out[i * 32 + j];
        }
        
        hash[i] <== hash_bits[i].out;
    }
}

/*
 * 패스워드 검증 템플릿
 * 
 * 실용적 응용: 서버에 패스워드 해시만 저장하고,
 * 사용자가 패스워드를 알고 있다는 것을 ZK로 증명
 */
template PasswordVerify() {
    signal input password[8];        // 128비트 패스워드 (간단화, private)
    signal output password_hash[8];  // 256비트 해시 결과 (public)
    
    // 패스워드를 256비트로 패딩 (뒤쪽을 0으로 채움)
    signal padded_password[8];
    padded_password[0] <== password[0];
    padded_password[1] <== password[1];
    padded_password[2] <== password[2];
    padded_password[3] <== password[3];
    padded_password[4] <== password[4];
    padded_password[5] <== password[5];
    padded_password[6] <== password[6];
    padded_password[7] <== password[7];
    
    // HashPreimage 컴포넌트 사용
    component hash_verifier = HashPreimage();
    for (var i = 0; i < 8; i++) {
        hash_verifier.preimage[i] <== padded_password[i];
        password_hash[i] <== hash_verifier.hash[i];
    }
}

// 메인 컴포넌트 (테스트용)
component main = HashPreimage();
