#!/bin/bash

# 🔐 ZKP 단계별 실행 스크립트
# 사용법: ./run_step.sh [단계번호]

set -e  # 에러 발생 시 스크립트 중단

# 색상 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 도움말 함수
show_help() {
    echo -e "${CYAN}🎯 ZKP 단계별 실행 스크립트${NC}"
    echo -e "${CYAN}=========================${NC}"
    echo ""
    echo -e "${YELLOW}사용법:${NC}"
    echo "  ./run_step.sh [단계번호]"
    echo ""
    echo -e "${YELLOW}사용 가능한 단계:${NC}"
    echo -e "  ${GREEN}1${NC}     - 1단계: Circom 회로 컴파일"
    echo -e "  ${GREEN}2-1${NC}   - 2-1단계: Powers of Tau 시작"
    echo -e "  ${GREEN}2-2${NC}   - 2-2단계: 엔트로피 기여"
    echo -e "  ${GREEN}2-3${NC}   - 2-3단계: Phase 2 준비"
    echo -e "  ${GREEN}3-1${NC}   - 3-1단계: ZKey 생성"
    echo -e "  ${GREEN}3-2${NC}   - 3-2단계: ZKey 기여"
    echo -e "  ${GREEN}3-3${NC}   - 3-3단계: 검증키 추출"
    echo -e "  ${GREEN}4-1${NC}   - 4-1단계: Witness 계산"
    echo -e "  ${GREEN}4-2${NC}   - 4-2단계: 증명 생성"
    echo -e "  ${GREEN}5${NC}     - 5단계: 증명 검증"
    echo -e "  ${GREEN}all${NC}   - 모든 단계 자동 실행"
    echo -e "  ${GREEN}clean${NC} - 생성 파일 모두 삭제"
    echo -e "  ${GREEN}status${NC}- 현재 상태 확인"
    echo ""
    echo -e "${YELLOW}예시:${NC}"
    echo "  ./run_step.sh 1      # 1단계만 실행"
    echo "  ./run_step.sh 2-1    # 2-1단계만 실행"
    echo "  ./run_step.sh all    # 모든 단계 실행"
    echo "  ./run_step.sh clean  # 정리"
}

# 상태 확인 함수
check_status() {
    echo -e "${BLUE}📊 현재 상태:${NC}"
    echo "===================="
    
    files_found=false
    
    # 각 파일 타입별로 확인
    if ls *.r1cs 2>/dev/null; then
        echo -e "${GREEN}✅ R1CS 파일 존재${NC}"
        files_found=true
    fi
    
    if ls *.wasm 2>/dev/null; then
        echo -e "${GREEN}✅ WASM 파일 존재${NC}"
        files_found=true
    fi
    
    if ls *.ptau 2>/dev/null; then
        echo -e "${GREEN}✅ PTAU 파일 존재${NC}"
        files_found=true
    fi
    
    if ls *.zkey 2>/dev/null; then
        echo -e "${GREEN}✅ ZKEY 파일 존재${NC}"
        files_found=true
    fi
    
    if ls *.wtns 2>/dev/null; then
        echo -e "${GREEN}✅ Witness 파일 존재${NC}"
        files_found=true
    fi
    
    if ls proof.json 2>/dev/null; then
        echo -e "${GREEN}✅ 증명 파일 존재${NC}"
        files_found=true
    fi
    
    if ls verification_key.json 2>/dev/null; then
        echo -e "${GREEN}✅ 검증키 파일 존재${NC}"
        files_found=true
    fi
    
    if [ -d "01_multiplier_js" ]; then
        echo -e "${GREEN}✅ JS 디렉토리 존재${NC}"
        files_found=true
    fi
    
    if [ "$files_found" = false ]; then
        echo -e "${YELLOW}⚠️  생성된 파일이 없습니다.${NC}"
    fi
}

# 정리 함수
clean_files() {
    echo -e "${YELLOW}🧹 정리 중...${NC}"
    rm -f *.ptau *.zkey *.wtns *.r1cs *.wasm *.sym *.json
    rm -rf 01_multiplier_js/
    echo -e "${GREEN}✅ 모든 생성 파일 삭제 완료!${NC}"
}

# 단계별 실행 함수들
step1() {
    echo -e "${PURPLE}🚀 1단계: Circom 회로 컴파일${NC}"
    circom 01_multiplier.circom --r1cs --wasm --sym
    echo -e "${GREEN}✅ 1단계 완료! 생성된 파일들:${NC}"
    ls -la *.r1cs *.wasm *.sym 01_multiplier_js/ 2>/dev/null || true
}

step2_1() {
    echo -e "${PURPLE}🔑 2-1단계: Powers of Tau 시작${NC}"
    snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
    echo -e "${GREEN}✅ 2-1단계 완료! pot12_0000.ptau 생성됨${NC}"
}

step2_2() {
    echo -e "${PURPLE}🔐 2-2단계: 엔트로피 기여${NC}"
    echo "사용자만 아는 비밀키" | snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v
    echo -e "${GREEN}✅ 2-2단계 완료! pot12_0001.ptau 생성됨${NC}"
}

step2_3() {
    echo -e "${PURPLE}🎯 2-3단계: Phase 2 준비${NC}"
    snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v
    echo -e "${GREEN}✅ 2-3단계 완료! pot12_final.ptau 생성됨${NC}"
}

step3_1() {
    echo -e "${PURPLE}🔧 3-1단계: ZKey 생성${NC}"
    snarkjs groth16 setup 01_multiplier.r1cs pot12_final.ptau multiplier_0000.zkey
    echo -e "${GREEN}✅ 3-1단계 완료! multiplier_0000.zkey 생성됨${NC}"
}

step3_2() {
    echo -e "${PURPLE}🔑 3-2단계: ZKey 기여${NC}"
    echo "사용자만 아는 비밀키" | snarkjs zkey contribute multiplier_0000.zkey multiplier_final.zkey --name="Key contribution" -v
    echo -e "${GREEN}✅ 3-2단계 완료! multiplier_final.zkey 생성됨${NC}"
}

step3_3() {
    echo -e "${PURPLE}📋 3-3단계: 검증키 추출${NC}"
    snarkjs zkey export verificationkey multiplier_final.zkey verification_key.json
    echo -e "${GREEN}✅ 3-3단계 완료! verification_key.json 생성됨${NC}"
}

step4_1() {
    echo -e "${PURPLE}🧮 4-1단계: Witness 계산${NC}"
    snarkjs wtns calculate 01_multiplier_js/01_multiplier.wasm input.json witness.wtns
    echo -e "${GREEN}✅ 4-1단계 완료! witness.wtns 생성됨${NC}"
}

step4_2() {
    echo -e "${PURPLE}📝 4-2단계: 증명 생성${NC}"
    snarkjs groth16 prove multiplier_final.zkey witness.wtns proof.json public.json
    echo -e "${GREEN}✅ 4-2단계 완료! proof.json, public.json 생성됨${NC}"
}

step5() {
    echo -e "${PURPLE}✅ 5단계: 증명 검증${NC}"
    snarkjs groth16 verify verification_key.json public.json proof.json
    echo -e "${GREEN}🎉 검증 완료! ZKP 성공!${NC}"
}

run_all() {
    echo -e "${CYAN}🚀 모든 단계 자동 실행 시작!${NC}"
    echo "=================================="
    
    step1
    echo ""
    step2_1
    echo ""
    step2_2
    echo ""
    step2_3
    echo ""
    step3_1
    echo ""
    step3_2
    echo ""
    step3_3
    echo ""
    step4_1
    echo ""
    step4_2
    echo ""
    step5
    
    echo ""
    echo -e "${CYAN}🎉 모든 단계 완료!${NC}"
}

# 메인 실행 로직
case "$1" in
    "1")
        step1
        ;;
    "2-1")
        step2_1
        ;;
    "2-2")
        step2_2
        ;;
    "2-3")
        step2_3
        ;;
    "3-1")
        step3_1
        ;;
    "3-2")
        step3_2
        ;;
    "3-3")
        step3_3
        ;;
    "4-1")
        step4_1
        ;;
    "4-2")
        step4_2
        ;;
    "5")
        step5
        ;;
    "all")
        run_all
        ;;
    "clean")
        clean_files
        ;;
    "status")
        check_status
        ;;
    "help"|"-h"|"--help"|"")
        show_help
        ;;
    *)
        echo -e "${RED}❌ 알 수 없는 단계: $1${NC}"
        echo ""
        show_help
        exit 1
        ;;
esac

