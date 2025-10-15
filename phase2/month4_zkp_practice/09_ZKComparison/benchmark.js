#!/usr/bin/env node

const fs = require("fs-extra");
const path = require("path");
const { execSync } = require("child_process");
const chalk = require("chalk");
const Table = require("cli-table3");
const now = require("performance-now");

class ZKPBenchmark {
  constructor() {
    this.results = {
      snark: {},
      stark: {},
      halo2: {},
    };
    this.testData = {
      leaf: "123",
      pathElements: [],
      pathIndices: [],
      root: "456",
    };

    // 결과 저장 디렉토리 생성
    fs.ensureDirSync("./results");
  }

  async runBenchmark() {
    console.log(chalk.blue.bold("\n🚀 ZKP 프로토콜 성능 벤치마크 시작!\n"));

    const protocols = this.getProtocolsToTest();

    for (const protocol of protocols) {
      console.log(
        chalk.yellow(`\n📊 ${protocol.toUpperCase()} 벤치마크 실행 중...`)
      );
      await this.benchmarkProtocol(protocol);
    }

    this.generateReport();
    this.saveResults();
  }

  getProtocolsToTest() {
    const args = process.argv.slice(2);
    const protocolArg = args.find((arg) => arg.startsWith("--protocol="));

    if (protocolArg) {
      const protocol = protocolArg.split("=")[1];
      return [protocol];
    }

    return ["snark", "stark", "halo2"];
  }

  async benchmarkProtocol(protocol) {
    try {
      const result = {
        protocol: protocol,
        proveTime: 0,
        verifyTime: 0,
        proofSize: 0,
        memoryUsage: 0,
        success: false,
        error: null,
      };

      // 메모리 사용량 측정 시작
      const initialMemory = process.memoryUsage();

      switch (protocol) {
        case "snark":
          await this.benchmarkSNARK(result);
          break;
        case "stark":
          await this.benchmarkSTARK(result);
          break;
        case "halo2":
          await this.benchmarkHalo2(result);
          break;
      }

      // 메모리 사용량 계산
      const finalMemory = process.memoryUsage();
      result.memoryUsage =
        (finalMemory.heapUsed - initialMemory.heapUsed) / 1024 / 1024; // MB

      this.results[protocol] = result;

      if (result.success) {
        console.log(chalk.green(`✅ ${protocol.toUpperCase()} 벤치마크 완료`));
      } else {
        console.log(
          chalk.red(
            `❌ ${protocol.toUpperCase()} 벤치마크 실패: ${result.error}`
          )
        );
      }
    } catch (error) {
      console.error(
        chalk.red(`❌ ${protocol.toUpperCase()} 벤치마크 오류:`, error.message)
      );
      this.results[protocol] = {
        protocol: protocol,
        success: false,
        error: error.message,
      };
    }
  }

  async benchmarkSNARK(result) {
    const snarkPath = "../04_MerkleTree";

    if (!fs.existsSync(snarkPath)) {
      throw new Error("SNARK 프로젝트 폴더를 찾을 수 없습니다");
    }

    // SNARK 전체 과정 시간 측정 (회로 컴파일 + 신뢰 설정)
    try {
      // Step 1: 회로 컴파일 (필요시)
      if (!fs.existsSync(path.join(snarkPath, "04_merkle_proof_js"))) {
        execSync("npm run step1", {
          cwd: snarkPath,
          stdio: "pipe",
          timeout: 30000,
        });
      }

      // Step 2-3: 신뢰 설정 (필요시)
      if (!fs.existsSync(path.join(snarkPath, "merkle_proof_0001.zkey"))) {
        execSync(
          "npm run step2-1 && npm run step2-2 && npm run step2-3 && npm run step3-1 && npm run step3-2 && npm run step3-3",
          {
            cwd: snarkPath,
            stdio: "pipe",
            timeout: 120000,
          }
        );
      }
    } catch (error) {
      throw new Error(`SNARK 증명 생성 실패: ${error.message}`);
    }

    // 증명 생성 시간 측정
    const proveStart = now();
    try {
      execSync("npm run step4-1-valid", {
        cwd: snarkPath,
        stdio: "pipe",
        timeout: 60000,
      });
      result.proveTime = now() - proveStart;
    } catch (error) {
      throw new Error(`SNARK 증명 생성 실패: ${error.message}`);
    }

    // 검증 시간 측정
    const verifyStart = now();
    try {
      execSync("npm run step4-2-valid", {
        cwd: snarkPath,
        stdio: "pipe",
        timeout: 10000,
      });
      result.verifyTime = now() - verifyStart;
    } catch (error) {
      throw new Error(`SNARK 검증 실패: ${error.message}`);
    }

    // 증명 크기 측정
    const proofPath = path.join(snarkPath, "proof_valid.json");
    if (fs.existsSync(proofPath)) {
      const proofStats = fs.statSync(proofPath);
      result.proofSize = proofStats.size / 1024; // KB
    }

    result.success = true;
  }

  async benchmarkSTARK(result) {
    const starkPath = "../06_RealStarkProof";

    if (!fs.existsSync(starkPath)) {
      throw new Error("STARK 프로젝트 폴더를 찾을 수 없습니다");
    }

    // 증명 생성 시간 측정
    const proveStart = now();
    try {
      execSync("cargo run --bin prover --release", {
        cwd: starkPath,
        stdio: "pipe",
        timeout: 60000,
      });
      result.proveTime = now() - proveStart;
    } catch (error) {
      throw new Error(`STARK 증명 생성 실패: ${error.message}`);
    }

    // 검증 시간 측정
    const verifyStart = now();
    try {
      execSync("cargo run --bin verifier --release", {
        cwd: starkPath,
        stdio: "pipe",
        timeout: 30000,
      });
      result.verifyTime = now() - verifyStart;
    } catch (error) {
      throw new Error(`STARK 검증 실패: ${error.message}`);
    }

    // 증명 크기 측정
    const proofPath = path.join(starkPath, "proofs/stark_proof.json");
    if (fs.existsSync(proofPath)) {
      const proofStats = fs.statSync(proofPath);
      result.proofSize = proofStats.size / 1024; // KB
    }

    result.success = true;
  }

  async benchmarkHalo2(result) {
    const halo2Path = "../08_HaloProof";

    if (!fs.existsSync(halo2Path)) {
      throw new Error("Halo2 프로젝트 폴더를 찾을 수 없습니다");
    }

    // 증명 생성 시간 측정
    const proveStart = now();
    try {
      execSync("cargo run --bin aggregate_prove --release", {
        cwd: halo2Path,
        stdio: "pipe",
        timeout: 120000,
      });
      result.proveTime = now() - proveStart;
    } catch (error) {
      throw new Error(`Halo2 증명 생성 실패: ${error.message}`);
    }

    // 검증 시간 측정
    const verifyStart = now();
    try {
      execSync("cargo run --bin aggregate_verify --release", {
        cwd: halo2Path,
        stdio: "pipe",
        timeout: 60000,
      });
      result.verifyTime = now() - verifyStart;
    } catch (error) {
      throw new Error(`Halo2 검증 실패: ${error.message}`);
    }

    // 증명 크기 측정 (집계된 증명)
    const proofPath = path.join(halo2Path, "proofs/aggregated_proof.json");
    if (fs.existsSync(proofPath)) {
      const proofStats = fs.statSync(proofPath);
      result.proofSize = proofStats.size / 1024; // KB
    }

    result.success = true;
  }

  generateReport() {
    console.log(chalk.blue.bold("\n📊 ZKP 프로토콜 성능 비교 결과\n"));

    // 성공한 결과만 필터링
    const successfulResults = Object.values(this.results).filter(
      (r) => r.success
    );

    if (successfulResults.length === 0) {
      console.log(chalk.red("❌ 성공한 벤치마크가 없습니다."));
      return;
    }

    // 테이블 생성
    const table = new Table({
      head: [
        chalk.cyan("프로토콜"),
        chalk.cyan("증명 시간 (ms)"),
        chalk.cyan("검증 시간 (ms)"),
        chalk.cyan("증명 크기 (KB)"),
        chalk.cyan("메모리 (MB)"),
      ],
      colWidths: [12, 18, 18, 18, 15],
    });

    successfulResults.forEach((result) => {
      table.push([
        chalk.yellow(result.protocol.toUpperCase()),
        result.proveTime.toFixed(2),
        result.verifyTime.toFixed(2),
        result.proofSize.toFixed(2),
        result.memoryUsage.toFixed(2),
      ]);
    });

    console.log(table.toString());

    // 성능 분석
    this.analyzePerformance(successfulResults);
  }

  analyzePerformance(results) {
    console.log(chalk.blue.bold("\n🔍 성능 분석\n"));

    if (results.length < 2) {
      console.log(chalk.yellow("⚠️ 비교할 프로토콜이 부족합니다."));
      return;
    }

    // 가장 빠른 증명 생성
    const fastestProve = results.reduce((min, r) =>
      r.proveTime < min.proveTime ? r : min
    );
    console.log(
      chalk.green(
        `🚀 가장 빠른 증명 생성: ${fastestProve.protocol.toUpperCase()} (${fastestProve.proveTime.toFixed(
          2
        )}ms)`
      )
    );

    // 가장 빠른 검증
    const fastestVerify = results.reduce((min, r) =>
      r.verifyTime < min.verifyTime ? r : min
    );
    console.log(
      chalk.green(
        `⚡ 가장 빠른 검증: ${fastestVerify.protocol.toUpperCase()} (${fastestVerify.verifyTime.toFixed(
          2
        )}ms)`
      )
    );

    // 가장 작은 증명 크기
    const smallestProof = results.reduce((min, r) =>
      r.proofSize < min.proofSize ? r : min
    );
    console.log(
      chalk.green(
        `📦 가장 작은 증명: ${smallestProof.protocol.toUpperCase()} (${smallestProof.proofSize.toFixed(
          2
        )}KB)`
      )
    );

    // 가장 적은 메모리 사용
    const leastMemory = results.reduce((min, r) =>
      r.memoryUsage < min.memoryUsage ? r : min
    );
    console.log(
      chalk.green(
        `💾 가장 적은 메모리: ${leastMemory.protocol.toUpperCase()} (${leastMemory.memoryUsage.toFixed(
          2
        )}MB)`
      )
    );

    console.log(chalk.blue("\n📈 추천 사용 사례:"));
    console.log(chalk.white("• SNARK: 작은 증명 크기가 중요한 블록체인 환경"));
    console.log(chalk.white("• STARK: 신뢰 설정이 없는 투명성이 중요한 경우"));
    console.log(chalk.white("• Halo2: 재귀적 증명과 확장성이 중요한 경우"));
  }

  saveResults() {
    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
    const filename = `results/benchmark_${timestamp}.json`;

    const reportData = {
      timestamp: new Date().toISOString(),
      results: this.results,
      summary: this.generateSummary(),
    };

    fs.writeJsonSync(filename, reportData, { spaces: 2 });
    console.log(chalk.blue(`\n💾 결과 저장됨: ${filename}`));
  }

  generateSummary() {
    const successfulResults = Object.values(this.results).filter(
      (r) => r.success
    );

    if (successfulResults.length === 0) {
      return { message: "성공한 벤치마크가 없습니다." };
    }

    return {
      totalProtocols: successfulResults.length,
      averageProveTime:
        successfulResults.reduce((sum, r) => sum + r.proveTime, 0) /
        successfulResults.length,
      averageVerifyTime:
        successfulResults.reduce((sum, r) => sum + r.verifyTime, 0) /
        successfulResults.length,
      averageProofSize:
        successfulResults.reduce((sum, r) => sum + r.proofSize, 0) /
        successfulResults.length,
      averageMemoryUsage:
        successfulResults.reduce((sum, r) => sum + r.memoryUsage, 0) /
        successfulResults.length,
    };
  }
}

// 메인 실행
if (require.main === module) {
  const benchmark = new ZKPBenchmark();
  benchmark.runBenchmark().catch((error) => {
    console.error(chalk.red("❌ 벤치마크 실행 오류:"), error);
    process.exit(1);
  });
}

module.exports = ZKPBenchmark;
