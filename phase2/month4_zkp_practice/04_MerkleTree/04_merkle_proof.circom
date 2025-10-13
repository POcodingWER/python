pragma circom 2.0.0;

/*
    머클트리 증명 ZKP 회로 - 새로운 도전!
    
    목표: 특정 데이터가 머클트리에 포함되어 있음을 증명하되, 
          실제 데이터와 위치는 비공개로 유지
    
    Private Input: 
        - leaf (증명하려는 데이터)
        - pathElements (머클 경로의 해시들)
        - pathIndices (좌/우 방향 정보)
    
    Public Input:
        - root (머클트리 루트 해시)
    
    Public Output:
        - isValid (1: 유효한 증명, 0: 무효한 증명)
    
    핵심 로직: leaf부터 시작해서 pathElements를 따라 올라가며 root 계산
*/

include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/switcher.circom";
include "circomlib/circuits/comparators.circom";

template MerkleTreeInclusionProof(levels) {
    // 신호 선언
    signal input leaf;                    // 비공개: 증명하려는 데이터
    signal input pathElements[levels];    // 비공개: 머클 경로의 해시들
    signal input pathIndices[levels];     // 비공개: 각 레벨에서의 방향 (0: 왼쪽, 1: 오른쪽)
    signal input root;                    // 공개: 머클트리 루트 해시
    signal output isValid;                // 공개: 증명 유효성 (1 또는 0)
    
    // 중간 해시 계산을 위한 신호들
    signal computedHashes[levels + 1];
    
    // 시작점: leaf 값
    computedHashes[0] <== leaf;
    
    // 각 레벨에서 해시 계산
    component hashers[levels];
    component switchers[levels];
    
    for (var i = 0; i < levels; i++) {
        // 방향에 따라 입력 순서 결정
        // pathIndices[i] = 0: 현재해시가 왼쪽 (hash(current, pathElement))
        // pathIndices[i] = 1: 현재해시가 오른쪽 (hash(pathElement, current))
        switchers[i] = Switcher();
        switchers[i].sel <== pathIndices[i];
        switchers[i].L <== computedHashes[i];      // 현재 해시
        switchers[i].R <== pathElements[i];       // 형제 노드
        
        // Poseidon 해시 함수로 다음 레벨 해시 계산
        hashers[i] = Poseidon(2);
        hashers[i].inputs[0] <== switchers[i].outL;
        hashers[i].inputs[1] <== switchers[i].outR;
        
        computedHashes[i + 1] <== hashers[i].out;
    }
    
    // 최종 계산된 루트가 주어진 루트와 일치하는지 확인
    component rootCheck = IsEqual();
    rootCheck.in[0] <== computedHashes[levels];
    rootCheck.in[1] <== root;
    
    isValid <== rootCheck.out;
}

// 1레벨 머클트리 (진짜 머클트리!)
component main = MerkleTreeInclusionProof(1);

