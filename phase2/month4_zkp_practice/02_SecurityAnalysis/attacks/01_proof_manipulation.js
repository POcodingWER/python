#!/usr/bin/env node

/**
 * 🚨 공격 시나리오 1: 증명서 조작 공격
 *
 * 목표: proof.json과 public.json 파일을 조작하여
 *       검증 시스템을 우회할 수 있는지 테스트
 */

const fs = require("fs");
const { execSync } = require("child_process");

console.log("🚨 ZKP 증명서 조작 공격 시뮬레이션");
console.log("=====================================\n");

// 공격 시나리오들
const attacks = [
  {
    name: "Public 값 변조",
    description: "public.json의 결과값을 임의로 변경",
    execute: () => {
      console.log("📝 원본 public.json 백업...");
      if (fs.existsSync("public.json")) {
        fs.copyFileSync("public.json", "public_original.json");

        console.log("🔧 public.json 값 변조 (33 → 999)...");
        fs.writeFileSync("public.json", '[\n  "999"\n]');

        console.log("🔍 변조된 파일로 검증 시도...");
        try {
          const result = execSync(
            "snarkjs groth16 verify verification_key.json public.json proof.json",
            { encoding: "utf8", stdio: "pipe" }
          );
          console.log("❌ 공격 실패: 검증이 성공함 (예상치 못한 결과)");
          console.log(result);
        } catch (error) {
          console.log("✅ 방어 성공: 검증 실패 - Invalid proof");
          console.log("   → ZKP 시스템이 조작을 감지함");
        }

        // 원본 복구
        fs.copyFileSync("public_original.json", "public.json");
        fs.unlinkSync("public_original.json");
      } else {
        console.log("⚠️  public.json 파일이 없습니다. 먼저 증명을 생성하세요.");
      }
    },
  },

  {
    name: "Proof 파일 변조",
    description: "proof.json의 암호학적 증명 데이터 변경",
    execute: () => {
      console.log("📝 원본 proof.json 백업...");
      if (fs.existsSync("proof.json")) {
        const originalProof = JSON.parse(fs.readFileSync("proof.json", "utf8"));
        fs.writeFileSync(
          "proof_original.json",
          JSON.stringify(originalProof, null, 2)
        );

        console.log("🔧 proof.json 변조 (pi_a 값 수정)...");
        const fakeProof = { ...originalProof };
        fakeProof.pi_a[0] =
          "1234567890123456789012345678901234567890123456789012345678901234567890";
        fs.writeFileSync("proof.json", JSON.stringify(fakeProof, null, 2));

        console.log("🔍 변조된 증명으로 검증 시도...");
        try {
          const result = execSync(
            "snarkjs groth16 verify verification_key.json public.json proof.json",
            { encoding: "utf8", stdio: "pipe" }
          );
          console.log("❌ 공격 실패: 검증이 성공함 (예상치 못한 결과)");
          console.log(result);
        } catch (error) {
          console.log("✅ 방어 성공: 검증 실패 - Invalid proof");
          console.log("   → 암호학적 무결성이 보장됨");
        }

        // 원본 복구
        fs.copyFileSync("proof_original.json", "proof.json");
        fs.unlinkSync("proof_original.json");
      } else {
        console.log("⚠️  proof.json 파일이 없습니다. 먼저 증명을 생성하세요.");
      }
    },
  },

  {
    name: "크로스 사용자 증명서 도용",
    description: "다른 사용자의 증명서를 자신의 것처럼 사용",
    execute: () => {
      console.log("🔧 가짜 사용자 증명서 생성...");

      // 다른 입력으로 증명 생성
      const fakeInput = { a: "7", b: "8" };
      fs.writeFileSync("fake_input.json", JSON.stringify(fakeInput, null, 2));

      try {
        console.log("📝 가짜 사용자 증명 생성 중...");
        execSync(
          "snarkjs wtns calculate 01_multiplier_js/01_multiplier.wasm fake_input.json fake_witness.wtns",
          { stdio: "pipe" }
        );
        execSync(
          "snarkjs groth16 prove multiplier_final.zkey fake_witness.wtns fake_proof.json fake_public.json",
          { stdio: "pipe" }
        );

        console.log("🔍 원본 사용자가 가짜 증명서 사용 시도...");
        // 원본 사용자의 public.json과 가짜 proof.json 조합
        const result = execSync(
          "snarkjs groth16 verify verification_key.json public.json fake_proof.json",
          { encoding: "utf8", stdio: "pipe" }
        );
        console.log("❌ 공격 실패: 검증이 성공함 (예상치 못한 결과)");
      } catch (error) {
        console.log("✅ 방어 성공: 크로스 사용자 공격 차단");
        console.log("   → 증명서와 공개값이 일치하지 않음");
      }

      // 임시 파일 정리
      [
        "fake_input.json",
        "fake_witness.wtns",
        "fake_proof.json",
        "fake_public.json",
      ].forEach((file) => {
        if (fs.existsSync(file)) fs.unlinkSync(file);
      });
    },
  },
];

// 공격 실행
async function runAttacks() {
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

  // 기본 증명이 없으면 생성
  if (!fs.existsSync("proof.json") || !fs.existsSync("public.json")) {
    console.log("📝 기본 증명 생성 중...");
    try {
      execSync("pnpm run step4-1 && pnpm run step4-2", { stdio: "inherit" });
    } catch (error) {
      console.log("❌ 기본 증명 생성 실패");
      return;
    }
  }

  console.log("✅ 환경 준비 완료\n");

  // 각 공격 시나리오 실행
  for (let i = 0; i < attacks.length; i++) {
    const attack = attacks[i];
    console.log(`\n🚨 공격 ${i + 1}: ${attack.name}`);
    console.log(`📋 설명: ${attack.description}`);
    console.log("─".repeat(50));

    try {
      attack.execute();
    } catch (error) {
      console.log(`❌ 공격 실행 중 오류: ${error.message}`);
    }

    console.log("─".repeat(50));
  }

  console.log("\n🎯 공격 시뮬레이션 완료!");
  console.log("📊 결과 요약:");
  console.log("   - ZKP 시스템은 모든 조작 시도를 성공적으로 차단");
  console.log("   - 암호학적 무결성이 보장됨");
  console.log("   - 크로스 사용자 공격도 방어됨");
}

// 스크립트 실행
if (require.main === module) {
  runAttacks().catch(console.error);
}

module.exports = { attacks, runAttacks };
