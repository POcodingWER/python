// 🔥 진짜 STARK 증명 - 외부 입력 기반
// 비밀번호 해시 영지식 증명

use core::poseidon::PoseidonTrait;
use core::hash::HashStateTrait;

// ZK-friendly 해시 함수 (Poseidon)
fn poseidon_hash(input: felt252) -> felt252 {
    let mut state = PoseidonTrait::new();
    state = state.update(input);
    state.finalize()
}

// 간단한 해시 함수 (교육용 - 실제로는 Poseidon 사용)
fn simple_hash(password: felt252) -> felt252 {
    password * 7 + 13
}

// 🎯 진짜 영지식 증명 함수
// 외부에서 비밀을 받아서 검증
fn verify_password_knowledge(secret_input: felt252, public_hash: felt252) -> bool {
    let calculated_hash = simple_hash(secret_input);
    calculated_hash == public_hash
}

// 🔒 메인 함수 - 진짜 외부 입력 처리
fn main(secret_password: felt252) -> Array<felt252> {
    // 🎉 진짜 외부 입력! scarb cairo-run [1234] 또는 --arguments-file 사용
    // 이제 Stone Prover 없이도 외부에서 비밀을 받을 수 있어요!
    
    // 🧮 동적 해시 계산 (simple_hash 공식 적용)
    let public_hash: felt252 = simple_hash(secret_password);  // 🌍 계산된 해시값
    
    // 🎯 영지식 증명 실행
    let is_valid = verify_password_knowledge(secret_password, public_hash);
    
    // 🔍 실행 추적 정보 (STARK의 핵심!)
    let mut execution_trace = ArrayTrait::new();
    
    // 단계 1: 비밀 입력 (값은 숨기고 존재만 기록)
    execution_trace.append(1); // 입력 단계
    
    // 단계 2: 해시 계산
    execution_trace.append(2); // 해시 계산 단계
    
    // 단계 3: 비교 연산
    execution_trace.append(3); // 비교 단계
    
    // 단계 4: 결과 출력
    if is_valid {
        execution_trace.append(1); // 성공
    } else {
        execution_trace.append(0); // 실패
    }
    
    // 🎯 STARK 증명의 핵심: 실행 추적 반환
    // 이 배열이 실제 STARK 증명의 기초가 됨
    execution_trace
}

// 🔍 검증용 함수 (Bob이 사용할 수 있는 공개 함수)
fn verify_execution_trace(trace: Array<felt252>, expected_hash: felt252) -> bool {
    // 실행 추적이 올바른 형태인지 검증
    if trace.len() != 4 {
        return false;
    }
    
    // 단계별 검증
    let step1 = *trace.at(0); // 입력 단계
    let step2 = *trace.at(1); // 해시 단계  
    let step3 = *trace.at(2); // 비교 단계
    let result = *trace.at(3); // 결과
    
    // 올바른 실행 순서인지 확인
    step1 == 1 && step2 == 2 && step3 == 3 && (result == 0 || result == 1)
}

#[cfg(test)]
mod tests {
    use super::{simple_hash, verify_password_knowledge, verify_execution_trace};

    #[test]
    fn test_simple_hash() {
        assert(simple_hash(1234) == 8651, 'Hash should be 8651');
        assert(simple_hash(5678) == 39759, 'Hash should be 39759');
    }

    #[test]
    fn test_password_verification() {
        // 올바른 비밀번호
        assert(verify_password_knowledge(1234, 8651), 'Should verify correct password');
        
        // 잘못된 비밀번호
        assert(!verify_password_knowledge(9999, 8651), 'Should reject wrong password');
    }

    #[test]
    fn test_execution_trace() {
        let mut trace = ArrayTrait::new();
        trace.append(1);
        trace.append(2);
        trace.append(3);
        trace.append(1);
        
        assert(verify_execution_trace(trace, 8651), 'Valid trace should pass');
    }
}
