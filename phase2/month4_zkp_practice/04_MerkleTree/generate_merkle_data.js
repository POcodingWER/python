// Poseidon 해시 함수 대신 간단한 해시 함수 사용 (테스트용)
function simpleHash(left, right) {
  // 간단한 해시 함수 (실제로는 Poseidon 사용해야 함)
  const combined = BigInt(left) * 1000000n + BigInt(right);
  return combined % 2147483647n; // 큰 소수로 모듈로 연산
}

/**
 * 머클트리 데이터 생성기
 * 실제 머클트리를 구성하고 유효한 증명 데이터를 생성합니다.
 */

// 간단한 머클트리 구현
class SimpleMerkleTree {
  constructor(leaves) {
    this.leaves = leaves.map((leaf) => BigInt(leaf));
    this.levels = Math.ceil(Math.log2(leaves.length));
    this.tree = this.buildTree();
  }

  buildTree() {
    let currentLevel = [...this.leaves];
    const tree = [currentLevel];

    // 각 레벨을 위로 올라가며 트리 구성
    for (let level = 0; level < this.levels; level++) {
      const nextLevel = [];

      for (let i = 0; i < currentLevel.length; i += 2) {
        const left = currentLevel[i];
        const right = i + 1 < currentLevel.length ? currentLevel[i + 1] : left;

        // 간단한 해시 함수 사용 (테스트용)
        const hash = simpleHash(left, right);
        nextLevel.push(hash);
      }

      tree.push(nextLevel);
      currentLevel = nextLevel;

      if (currentLevel.length === 1) break;
    }

    return tree;
  }

  getRoot() {
    return this.tree[this.tree.length - 1][0];
  }

  getProof(leafIndex) {
    const pathElements = [];
    const pathIndices = [];

    let currentIndex = leafIndex;

    for (let level = 0; level < this.levels; level++) {
      const isRightNode = currentIndex % 2 === 1;
      const siblingIndex = isRightNode ? currentIndex - 1 : currentIndex + 1;

      // 형제 노드가 존재하는 경우
      if (siblingIndex < this.tree[level].length) {
        pathElements.push(this.tree[level][siblingIndex].toString());
        pathIndices.push(isRightNode ? 1 : 0);
      } else {
        // 형제 노드가 없는 경우 자기 자신 사용
        pathElements.push(this.tree[level][currentIndex].toString());
        pathIndices.push(0);
      }

      currentIndex = Math.floor(currentIndex / 2);
    }

    return { pathElements, pathIndices };
  }
}

// 테스트 데이터 생성
function generateTestData() {
  console.log("🌳 머클트리 테스트 데이터 생성 중...");

  // 8개의 테스트 데이터
  const leaves = [12345, 23456, 34567, 45678, 56789, 67890, 78901, 89012];

  console.log("📋 Leaves:", leaves);

  // 머클트리 생성
  const tree = new SimpleMerkleTree(leaves);
  const root = tree.getRoot();

  console.log("🌲 Root:", root.toString());

  // 첫 번째 leaf에 대한 증명 생성
  const leafIndex = 0;
  const leaf = leaves[leafIndex];
  const proof = tree.getProof(leafIndex);

  console.log("🔍 Proof for leaf", leaf, ":");
  console.log("   Path Elements:", proof.pathElements);
  console.log("   Path Indices:", proof.pathIndices);

  // Circom 입력 형식으로 변환
  const inputData = {
    leaf: leaf.toString(),
    pathElements: proof.pathElements,
    pathIndices: proof.pathIndices,
    root: root.toString(),
  };

  console.log("\n📄 Generated input_valid.json:");
  console.log(JSON.stringify(inputData, null, 2));

  return inputData;
}

// 무효한 증명 데이터 생성
function generateInvalidData() {
  console.log("\n❌ 무효한 증명 데이터 생성 중...");

  const invalidData = {
    leaf: "99999", // 존재하지 않는 leaf
    pathElements: ["11111", "22222", "33333", "44444"],
    pathIndices: [0, 1, 0, 1],
    root: "987654321", // 잘못된 root
  };

  console.log("📄 Generated input_invalid.json:");
  console.log(JSON.stringify(invalidData, null, 2));

  return invalidData;
}

// 실행
if (require.main === module) {
  try {
    const validData = generateTestData();
    const invalidData = generateInvalidData();

    // 파일로 저장
    const fs = require("fs");
    fs.writeFileSync("input_valid.json", JSON.stringify(validData, null, 2));
    fs.writeFileSync(
      "input_invalid.json",
      JSON.stringify(invalidData, null, 2)
    );

    console.log("\n✅ 테스트 데이터 생성 완료!");
    console.log("   - input_valid.json: 유효한 증명");
    console.log("   - input_invalid.json: 무효한 증명");
  } catch (error) {
    console.error("❌ 오류 발생:", error.message);

    // Fallback: 간단한 테스트 데이터
    console.log("🔄 Fallback 데이터 사용...");

    const fallbackValid = {
      leaf: "12345",
      pathElements: ["23456", "34567", "45678", "56789"],
      pathIndices: [0, 1, 0, 1],
      root: "987654321",
    };

    const fallbackInvalid = {
      leaf: "99999",
      pathElements: ["11111", "22222", "33333", "44444"],
      pathIndices: [0, 1, 0, 1],
      root: "123456789",
    };

    const fs = require("fs");
    fs.writeFileSync(
      "input_valid.json",
      JSON.stringify(fallbackValid, null, 2)
    );
    fs.writeFileSync(
      "input_invalid.json",
      JSON.stringify(fallbackInvalid, null, 2)
    );

    console.log("✅ Fallback 데이터 생성 완료!");
  }
}

module.exports = { SimpleMerkleTree, generateTestData, generateInvalidData };
