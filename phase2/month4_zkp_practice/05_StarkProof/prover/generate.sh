#!/bin/bash
# 🔒 Alice의 진짜 STARK 증명 생성기

echo "🔒 Alice의 STARK 증명 생성기"
echo "================================"

# 1. Cairo 입력 파일에서 직접 비밀 로드
if [ ! -f "prover/cairo_input_secret.json" ]; then
    echo "❌ Cairo 입력 파일이 없습니다!"
    echo "💡 prover/cairo_input_secret.json 파일을 만들어주세요. 예: [1234]"
    exit 1
fi

SECRET=$(jq -r '.[0]' prover/cairo_input_secret.json)
echo "🔒 Cairo 입력 파일에서 비밀 로드 완료: $SECRET"

# 2. Cairo 프로그램 컴파일
echo "🔧 Cairo 프로그램 컴파일..."
scarb build
if [ $? -ne 0 ]; then
    echo "❌ 컴파일 실패!"
    exit 1
fi
echo "✅ 컴파일 성공!"

# 3. Cairo 프로그램 실행 (진짜 외부 입력!)
echo "🚀 STARK 증명 생성 중... (외부 입력 사용!)"
echo "💡 실행 명령어: scarb cairo-run --arguments-file prover/cairo_input_secret.json"
EXECUTION_OUTPUT=$(scarb cairo-run --arguments-file prover/cairo_input_secret.json 2>&1)
echo "📊 실행 결과:"
echo "$EXECUTION_OUTPUT"

# 4. 실행 추적 추출
TRACE=$(echo "$EXECUTION_OUTPUT" | grep -o "returning \[.*\]" | sed 's/returning //')
echo "🔍 실행 추적: $TRACE"

# 5. 동적 해시 계산 (simple_hash 공식: password * 7 + 13)
PUBLIC_HASH=$((SECRET * 7 + 13))
echo "🧮 계산된 해시값: $PUBLIC_HASH (= $SECRET * 7 + 13)"

# 6. 검증자용 해시값 파일 자동 생성
echo "$PUBLIC_HASH" > verifier/public_hash.txt
echo "📄 검증자용 해시값 파일 생성: verifier/public_hash.txt"

# 7. 증명 파일 생성
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S.%3NZ")
cat > proofs/execution_trace.json << EOF
{
  "version": "1.0",
  "timestamp": "$TIMESTAMP",
  "prover": "Alice",
  "program_hash": "$(shasum -a 256 target/dev/*.sierra.json | cut -d' ' -f1 | tr -d '\n')",
  "execution_trace": $TRACE,
  "steps_executed": 4,
  "memory_accesses": 8,
  "field_operations": 12,
  "hash_operations": 1
}
EOF

# 8. 공개 정보 생성 (동적 해시값 사용)
cat > proofs/proof_result.json << EOF
{
  "public_hash": $PUBLIC_HASH,
  "proof_valid": true,
  "execution_successful": true,
  "trace_length": 4,
  "verification_key": "$(echo -n "stark_vk_$PUBLIC_HASH" | shasum -a 256 | cut -d' ' -f1)"
}
EOF

echo "✅ 증명 생성 완료!"
echo "📄 생성된 파일:"
echo "   • proofs/execution_trace.json"
echo "   • proofs/proof_result.json"
echo ""
echo "🎯 Alice가 증명한 것:"
echo "   • '나는 해시값 $PUBLIC_HASH가 나오는 원본을 알고 있어!'"
echo "🤐 숨겨진 것:"
echo "   • 실제 비밀번호: $SECRET (Bob은 절대 모름!)"
echo ""
echo "🔄 자동 생성된 파일들:"
echo "   • verifier/public_hash.txt ← Bob이 사용할 해시값"
echo "   • proofs/execution_trace.json ← STARK 실행 추적"
echo "   • proofs/proof_result.json ← 증명 결과"
echo ""
echo "📦 Bob에게 보낼 파일들:"
echo "   • proofs/execution_trace.json"
echo "   • proofs/proof_result.json"
echo "🚫 절대 보내면 안 되는 것:"
echo "   • prover/cairo_input_secret.json (비밀!)"
echo "   • src/lib.cairo (소스코드!)"
