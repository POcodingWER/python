#!/usr/bin/env node

/**
 * 🚀 ZKP 성능 벤치마크 분석 도구
 *
 * 목표: ZKP 시스템의 각 단계별 성능을 측정하고
 *       병목 지점을 식별하여 최적화 방안 제시
 */

const fs = require("fs");
const { execSync } = require("child_process");
const path = require("path");

console.log("🚀 ZKP 성능 벤치마크 분석");
console.log("==========================\n");

// 성능 측정 클래스
class PerformanceBenchmark {
  constructor() {
    this.results = [];
    this.memoryUsage = [];
  }

  // 메모리 사용량 측정
  measureMemory(label) {
    const usage = process.memoryUsage();
    this.memoryUsage.push({
      label,
      timestamp: Date.now(),
      rss: usage.rss,
      heapTotal: usage.heapTotal,
      heapUsed: usage.heapUsed,
      external: usage.external,
    });
  }

  // 시간 측정 래퍼
  async measureTime(label, fn) {
    this.measureMemory(`${label} - 시작`);
    const start = process.hrtime.bigint();

    try {
      const result = await fn();
      const end = process.hrtime.bigint();
      const duration = Number(end - start) / 1000000; // ms

      this.measureMemory(`${label} - 완료`);

      this.results.push({
        label,
        duration,
        success: true,
        timestamp: Date.now(),
      });

      console.log(`✅ ${label}: ${duration.toFixed(2)}ms`);
      return result;
    } catch (error) {
      const end = process.hrtime.bigint();
      const duration = Number(end - start) / 1000000;

      this.results.push({
        label,
        duration,
        success: false,
        error: error.message,
        timestamp: Date.now(),
      });

      console.log(`❌ ${label}: ${duration.toFixed(2)}ms (실패)`);
      throw error;
    }
  }

  // 파일 크기 측정
  getFileSize(filePath) {
    try {
      const stats = fs.statSync(filePath);
      return stats.size;
    } catch {
      return 0;
    }
  }

  // 결과 분석 및 리포트 생성
  generateReport() {
    const report = {
      summary: {
        totalTests: this.results.length,
        successfulTests: this.results.filter((r) => r.success).length,
        totalTime: this.results.reduce((sum, r) => sum + r.duration, 0),
        averageTime:
          this.results.length > 0
            ? this.results.reduce((sum, r) => sum + r.duration, 0) /
              this.results.length
            : 0,
      },
      detailed: this.results,
      memory: this.memoryUsage,
      fileAnalysis: this.analyzeFiles(),
      bottlenecks: this.identifyBottlenecks(),
      recommendations: this.generateRecommendations(),
    };

    return report;
  }

  // 파일 분석
  analyzeFiles() {
    const files = [
      "01_multiplier.r1cs",
      "01_multiplier.wasm",
      "pot12_final.ptau",
      "multiplier_final.zkey",
      "verification_key.json",
      "witness.wtns",
      "proof.json",
      "public.json",
    ];

    return files.map((file) => ({
      name: file,
      size: this.getFileSize(file),
      sizeKB: (this.getFileSize(file) / 1024).toFixed(2),
      exists: fs.existsSync(file),
    }));
  }

  // 병목 지점 식별
  identifyBottlenecks() {
    const sortedResults = this.results
      .filter((r) => r.success)
      .sort((a, b) => b.duration - a.duration);

    return {
      slowest: sortedResults.slice(0, 3),
      fastest: sortedResults.slice(-3).reverse(),
      averageDuration:
        sortedResults.reduce((sum, r) => sum + r.duration, 0) /
        sortedResults.length,
    };
  }

  // 최적화 권장사항 생성
  generateRecommendations() {
    const bottlenecks = this.identifyBottlenecks();
    const recommendations = [];

    if (bottlenecks.slowest.length > 0) {
      const slowest = bottlenecks.slowest[0];

      if (
        slowest.label.includes("Powers of Tau") ||
        slowest.label.includes("Phase 2")
      ) {
        recommendations.push({
          category: "Trusted Setup 최적화",
          suggestion: "더 작은 파워 사이즈 사용 또는 사전 생성된 파일 재사용",
          impact: "high",
        });
      }

      if (
        slowest.label.includes("ZKey 생성") ||
        slowest.label.includes("ZKey 기여")
      ) {
        recommendations.push({
          category: "Key 생성 최적화",
          suggestion: "병렬 처리 또는 하드웨어 가속 활용",
          impact: "medium",
        });
      }

      if (slowest.label.includes("증명 생성")) {
        recommendations.push({
          category: "증명 생성 최적화",
          suggestion: "회로 복잡도 감소 또는 배치 처리",
          impact: "medium",
        });
      }
    }

    // 메모리 사용량 분석
    const maxMemory = Math.max(...this.memoryUsage.map((m) => m.heapUsed));
    if (maxMemory > 100 * 1024 * 1024) {
      // 100MB 이상
      recommendations.push({
        category: "메모리 최적화",
        suggestion: "메모리 사용량이 높습니다. 스트리밍 처리 고려",
        impact: "medium",
      });
    }

    return recommendations;
  }
}

// 벤치마크 테스트 시나리오들
const benchmarkScenarios = [
  {
    name: "기본 시나리오",
    input: { a: "3", b: "11" },
    description: "표준 입력값으로 기본 성능 측정",
  },
  {
    name: "큰 수 시나리오",
    input: { a: "999999", b: "888888" },
    description: "큰 수 연산 성능 측정",
  },
  {
    name: "소수 시나리오",
    input: { a: "97", b: "101" },
    description: "소수 곱셈 성능 측정",
  },
  {
    name: "0 포함 시나리오",
    input: { a: "0", b: "12345" },
    description: "0 곱셈 최적화 확인",
  },
  {
    name: "1 곱셈 시나리오",
    input: { a: "1", b: "987654321" },
    description: "1 곱셈 최적화 확인",
  },
];

// 메인 벤치마크 실행 함수
async function runBenchmark() {
  const benchmark = new PerformanceBenchmark();

  console.log("🎯 환경 준비 확인...");

  // 필요한 파일들 확인
  const requiredFiles = [
    "01_multiplier.circom",
    "verification_key.json",
    "multiplier_final.zkey",
  ];
  const missingFiles = requiredFiles.filter((file) => !fs.existsSync(file));

  if (missingFiles.length > 0) {
    console.log("❌ 필요한 파일이 없습니다. 전체 설정을 다시 실행합니다...");

    // 전체 설정 실행
    await benchmark.measureTime("전체 ZKP 설정", () => {
      return new Promise((resolve, reject) => {
        try {
          const result = execSync("pnpm run all", {
            encoding: "utf8",
            stdio: "pipe",
          });
          resolve(result);
        } catch (error) {
          reject(error);
        }
      });
    });
  }

  console.log("✅ 환경 준비 완료\n");

  // 각 시나리오별 성능 측정
  for (let i = 0; i < benchmarkScenarios.length; i++) {
    const scenario = benchmarkScenarios[i];
    console.log(`\n🚀 시나리오 ${i + 1}: ${scenario.name}`);
    console.log(`📋 설명: ${scenario.description}`);
    console.log("─".repeat(50));

    try {
      const inputFile = `bench_input_${i + 1}.json`;
      const witnessFile = `bench_witness_${i + 1}.wtns`;
      const proofFile = `bench_proof_${i + 1}.json`;
      const publicFile = `bench_public_${i + 1}.json`;

      // 입력 파일 생성
      fs.writeFileSync(inputFile, JSON.stringify(scenario.input, null, 2));

      // 각 단계별 성능 측정
      await benchmark.measureTime(`${scenario.name} - Witness 계산`, () => {
        return new Promise((resolve, reject) => {
          try {
            const result = execSync(
              `snarkjs wtns calculate 01_multiplier_js/01_multiplier.wasm ${inputFile} ${witnessFile}`,
              { encoding: "utf8", stdio: "pipe" }
            );
            resolve(result);
          } catch (error) {
            reject(error);
          }
        });
      });

      await benchmark.measureTime(`${scenario.name} - 증명 생성`, () => {
        return new Promise((resolve, reject) => {
          try {
            const result = execSync(
              `snarkjs groth16 prove multiplier_final.zkey ${witnessFile} ${proofFile} ${publicFile}`,
              { encoding: "utf8", stdio: "pipe" }
            );
            resolve(result);
          } catch (error) {
            reject(error);
          }
        });
      });

      await benchmark.measureTime(`${scenario.name} - 증명 검증`, () => {
        return new Promise((resolve, reject) => {
          try {
            const result = execSync(
              `snarkjs groth16 verify verification_key.json ${publicFile} ${proofFile}`,
              { encoding: "utf8", stdio: "pipe" }
            );
            resolve(result);
          } catch (error) {
            reject(error);
          }
        });
      });

      // 결과 확인
      const publicData = JSON.parse(fs.readFileSync(publicFile, "utf8"));
      const expectedResult = (
        BigInt(scenario.input.a) * BigInt(scenario.input.b)
      ).toString();
      const actualResult = publicData[0];

      console.log(`🔍 계산 결과: ${actualResult} (예상: ${expectedResult})`);
      console.log(
        `✅ 결과 검증: ${actualResult === expectedResult ? "일치" : "불일치"}`
      );

      // 임시 파일 정리
      [inputFile, witnessFile, proofFile, publicFile].forEach((file) => {
        if (fs.existsSync(file)) fs.unlinkSync(file);
      });
    } catch (error) {
      console.log(`❌ 시나리오 실행 실패: ${error.message.split("\n")[0]}`);
    }

    console.log("─".repeat(50));
  }

  // 최종 리포트 생성
  console.log("\n📊 성능 분석 리포트 생성 중...");
  const report = benchmark.generateReport();

  // 리포트 저장
  const reportPath = path.join(
    __dirname,
    "../benchmarks/performance_report.json"
  );
  fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));

  // 요약 출력
  console.log("\n🎯 성능 벤치마크 완료!");
  console.log("========================");
  console.log(`📊 총 테스트: ${report.summary.totalTests}`);
  console.log(`✅ 성공: ${report.summary.successfulTests}`);
  console.log(`⏱️  총 시간: ${report.summary.totalTime.toFixed(2)}ms`);
  console.log(`📈 평균 시간: ${report.summary.averageTime.toFixed(2)}ms`);

  if (report.bottlenecks.slowest.length > 0) {
    console.log(
      `\n🐌 가장 느린 작업: ${
        report.bottlenecks.slowest[0].label
      } (${report.bottlenecks.slowest[0].duration.toFixed(2)}ms)`
    );
    console.log(
      `⚡ 가장 빠른 작업: ${
        report.bottlenecks.fastest[0].label
      } (${report.bottlenecks.fastest[0].duration.toFixed(2)}ms)`
    );
  }

  if (report.recommendations.length > 0) {
    console.log("\n💡 최적화 권장사항:");
    report.recommendations.forEach((rec, index) => {
      console.log(
        `   ${index + 1}. [${rec.impact.toUpperCase()}] ${rec.category}: ${
          rec.suggestion
        }`
      );
    });
  }

  console.log(`\n📄 상세 리포트: ${reportPath}`);

  return report;
}

// 스크립트 실행
if (require.main === module) {
  runBenchmark().catch(console.error);
}

module.exports = { PerformanceBenchmark, benchmarkScenarios, runBenchmark };
