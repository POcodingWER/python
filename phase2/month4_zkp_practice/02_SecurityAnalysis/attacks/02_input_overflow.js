#!/usr/bin/env node

/**
 * 🚨 공격 시나리오 2: 입력 데이터 오버플로우 공격
 *
 * 목표: 극단적인 입력값으로 시스템의 한계를 테스트하고
 *       예상치 못한 동작이나 취약점을 찾아내기
 */

const fs = require("fs");
const { execSync } = require("child_process");

console.log("🚨 ZKP 입력 오버플로우 공격 시뮬레이션");
console.log("======================================\n");

// 공격 시나리오들
const overflowAttacks = [
  {
    name: "매우 큰 수 입력",
    description: "JavaScript Number.MAX_SAFE_INTEGER를 초과하는 값",
    input: {
      a: "9007199254740992", // Number.MAX_SAFE_INTEGER + 1
      b: "9007199254740993",
    },
    expectedResult: "81129638414606681695789005144064",
  },

  {
    name: "음수 입력",
    description: "음수 값으로 예상치 못한 동작 유발",
    input: {
      a: "-5",
      b: "7",
    },
    expectedResult: "-35",
  },

  {
    name: "0 입력",
    description: "0 곱셈으로 특수 케이스 테스트",
    input: {
      a: "0",
      b: "999999999",
    },
    expectedResult: "0",
  },

  {
    name: "매우 큰 소수",
    description: "큰 소수로 암호학적 연산 부하 테스트",
    input: {
      a: "2147483647", // 2^31 - 1 (Mersenne prime)
      b: "2147483647",
    },
    expectedResult: "4611686014132420609",
  },

  {
    name: "16진수 문자열",
    description: "잘못된 형식의 입력으로 파싱 오류 유발",
    input: {
      a: "0xFF",
      b: "0x10",
    },
    shouldFail: true,
  },

  {
    name: "문자열 입력",
    description: "숫자가 아닌 문자열로 타입 혼동 공격",
    input: {
      a: "hello",
      b: "world",
    },
    shouldFail: true,
  },

  {
    name: "null/undefined 입력",
    description: "null 값으로 예외 상황 유발",
    input: {
      a: null,
      b: "5",
    },
    shouldFail: true,
  },

  {
    name: "부동소수점 입력",
    description: "소수점 값으로 정수 제약 우회 시도",
    input: {
      a: "3.14159",
      b: "2.71828",
    },
    shouldFail: true,
  },
];

// 성능 측정 함수
function measurePerformance(testName, fn) {
  const start = process.hrtime.bigint();
  try {
    const result = fn();
    const end = process.hrtime.bigint();
    const duration = Number(end - start) / 1000000; // Convert to milliseconds
    console.log(`   ⏱️  실행 시간: ${duration.toFixed(2)}ms`);
    return result;
  } catch (error) {
    const end = process.hrtime.bigint();
    const duration = Number(end - start) / 1000000;
    console.log(`   ⏱️  실행 시간: ${duration.toFixed(2)}ms (오류 발생)`);
    throw error;
  }
}

// 공격 실행 함수
async function runOverflowAttacks() {
  console.log("🎯 전제조건 확인...");

  // 필요한 파일들이 있는지 확인
  const requiredFiles = [
    "verification_key.json",
    "multiplier_final.zkey",
    "01_multiplier_js/01_multiplier.wasm",
  ];
  const missingFiles = requiredFiles.filter((file) => !fs.existsSync(file));

  if (missingFiles.length > 0) {
    console.log("❌ 필요한 파일이 없습니다:");
    missingFiles.forEach((file) => console.log(`   - ${file}`));
    console.log("\n먼저 다음 명령을 실행하세요:");
    console.log("pnpm run all");
    return;
  }

  console.log("✅ 환경 준비 완료\n");

  let successCount = 0;
  let failureCount = 0;
  let performanceData = [];

  // 각 공격 시나리오 실행
  for (let i = 0; i < overflowAttacks.length; i++) {
    const attack = overflowAttacks[i];
    console.log(`\n🚨 공격 ${i + 1}: ${attack.name}`);
    console.log(`📋 설명: ${attack.description}`);
    console.log("─".repeat(60));

    try {
      // 공격 입력 파일 생성
      const inputFile = `attack_input_${i + 1}.json`;
      fs.writeFileSync(inputFile, JSON.stringify(attack.input, null, 2));
      console.log(`📝 공격 입력 생성: ${JSON.stringify(attack.input)}`);

      // Witness 계산 시도
      console.log("🧮 Witness 계산 시도...");
      const witnessFile = `attack_witness_${i + 1}.wtns`;

      const witnessResult = measurePerformance("witness calculation", () => {
        return execSync(
          `snarkjs wtns calculate 01_multiplier_js/01_multiplier.wasm ${inputFile} ${witnessFile}`,
          { encoding: "utf8", stdio: "pipe" }
        );
      });

      // 증명 생성 시도
      console.log("📝 증명 생성 시도...");
      const proofFile = `attack_proof_${i + 1}.json`;
      const publicFile = `attack_public_${i + 1}.json`;

      const proofResult = measurePerformance("proof generation", () => {
        return execSync(
          `snarkjs groth16 prove multiplier_final.zkey ${witnessFile} ${proofFile} ${publicFile}`,
          { encoding: "utf8", stdio: "pipe" }
        );
      });

      // 결과 검증
      const publicData = JSON.parse(fs.readFileSync(publicFile, "utf8"));
      const actualResult = publicData[0];

      console.log(`🔍 계산 결과: ${actualResult}`);

      if (attack.shouldFail) {
        console.log("❌ 예상과 다름: 실패해야 하는 입력이 성공함");
        console.log("   → 입력 검증 로직 강화 필요");
        failureCount++;
      } else {
        if (attack.expectedResult && actualResult === attack.expectedResult) {
          console.log("✅ 정상 처리: 예상 결과와 일치");
          successCount++;
        } else if (attack.expectedResult) {
          console.log(
            `⚠️  결과 불일치: 예상 ${attack.expectedResult}, 실제 ${actualResult}`
          );
          console.log("   → 큰 수 연산에서 정밀도 손실 가능성");
          failureCount++;
        } else {
          console.log("✅ 처리 완료: 시스템이 입력을 정상 처리");
          successCount++;
        }
      }

      // 검증 시도
      console.log("🔍 증명 검증 시도...");
      const verifyResult = measurePerformance("verification", () => {
        return execSync(
          `snarkjs groth16 verify verification_key.json ${publicFile} ${proofFile}`,
          { encoding: "utf8", stdio: "pipe" }
        );
      });

      if (verifyResult.includes("OK!")) {
        console.log("✅ 검증 성공: 증명이 유효함");
      }

      // 성능 데이터 수집
      performanceData.push({
        attack: attack.name,
        input: attack.input,
        result: actualResult,
        success: !attack.shouldFail,
      });

      // 임시 파일 정리
      [inputFile, witnessFile, proofFile, publicFile].forEach((file) => {
        if (fs.existsSync(file)) fs.unlinkSync(file);
      });
    } catch (error) {
      if (attack.shouldFail) {
        console.log("✅ 예상된 실패: 시스템이 올바르게 거부함");
        console.log(`   → 오류: ${error.message.split("\n")[0]}`);
        successCount++;
      } else {
        console.log("❌ 예상치 못한 실패: 정상 입력이 거부됨");
        console.log(`   → 오류: ${error.message.split("\n")[0]}`);
        failureCount++;
      }

      // 임시 파일 정리
      [
        `attack_input_${i + 1}.json`,
        `attack_witness_${i + 1}.wtns`,
        `attack_proof_${i + 1}.json`,
        `attack_public_${i + 1}.json`,
      ].forEach((file) => {
        if (fs.existsSync(file)) fs.unlinkSync(file);
      });
    }

    console.log("─".repeat(60));
  }

  // 최종 결과 요약
  console.log("\n🎯 오버플로우 공격 시뮬레이션 완료!");
  console.log("=====================================");
  console.log(`📊 결과 요약:`);
  console.log(`   ✅ 성공적 방어: ${successCount}/${overflowAttacks.length}`);
  console.log(`   ❌ 취약점 발견: ${failureCount}/${overflowAttacks.length}`);

  if (performanceData.length > 0) {
    console.log("\n📈 성능 분석:");
    performanceData.forEach((data) => {
      console.log(`   - ${data.attack}: ${data.success ? "✅" : "❌"}`);
    });
  }

  console.log("\n🛡️ 보안 권장사항:");
  console.log("   1. 입력값 범위 검증 강화");
  console.log("   2. 타입 검증 로직 추가");
  console.log("   3. 큰 수 연산 정밀도 검토");
  console.log("   4. 예외 처리 메커니즘 개선");
}

// 스크립트 실행
if (require.main === module) {
  runOverflowAttacks().catch(console.error);
}

module.exports = { overflowAttacks, runOverflowAttacks };
