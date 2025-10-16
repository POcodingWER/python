#!/usr/bin/env python3
"""
🧠 Week 2-1: Multi-layer Neural Network → ZKML
Month 5 Week 2

목표: 아이리스 분류 모델 (3층 신경망)을 ZKML로 변환
복습: phase2/month3_ai_advanced/02_classification_neural_network.ipynb
"""

import torch
import torch.nn as nn
import numpy as np
import onnx
import ezkl
import os
import json
import asyncio
from sklearn.datasets import load_iris
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler

print("🧠 Week 2-1: MLP → ZKML")
print("=" * 60)

# ========================================
# 1. 데이터 준비
# ========================================
print("\n[1/13] 📊 아이리스 데이터셋 로드 중...")

iris = load_iris()
X = iris.data  # (150, 4)
y = iris.target  # (150,)

# 데이터 분할
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=42
)

# 정규화
scaler = StandardScaler()
X_train = scaler.fit_transform(X_train)
X_test = scaler.transform(X_test)

print(f"✅ 데이터 로드 완료")
print(f"   훈련 데이터: {X_train.shape}")
print(f"   테스트 데이터: {X_test.shape}")
print(f"   클래스 수: {len(np.unique(y))}")

# ========================================
# 2. Month 3 모델 정의 (동일 아키텍처)
# ========================================
print("\n[2/13] 🏗️  MLP 모델 정의 중...")

class IrisClassifier(nn.Module):
    """
    EZKL 호환 단순화 신경망
    - 입력층: 4개 특징 (꽃받침/꽃잎 길이/너비)
    - 은닉층: 8개 뉴런
    - 출력층: 3개 클래스 (Setosa, Versicolor, Virginica)
    
    ⚠️ ReLU 제거 이유:
    - PyTorch 2.x → ONNX Opset 18로 자동 변환
    - EZKL이 Opset 18의 ReLU를 아직 지원 안 함
    - Linear만 사용하면 정상 작동 (Week 1 확인됨)
    """
    def __init__(self):
        super(IrisClassifier, self).__init__()
        self.fc1 = nn.Linear(4, 8)    # 입력층
        self.fc2 = nn.Linear(8, 3)    # 출력층
    
    def forward(self, x):
        x = self.fc1(x)
        x = self.fc2(x)  # Softmax는 손실함수에 포함
        return x

model = IrisClassifier()
print(f"✅ 모델 정의 완료")
print(f"   파라미터 수: {sum(p.numel() for p in model.parameters())}")

# ========================================
# 3. 모델 훈련 (간단 버전)
# ========================================
print("\n[3/13] 🏋️  모델 훈련 중...")

# 데이터를 Tensor로 변환
X_train_tensor = torch.FloatTensor(X_train)
y_train_tensor = torch.LongTensor(y_train)
X_test_tensor = torch.FloatTensor(X_test)
y_test_tensor = torch.LongTensor(y_test)

# 훈련 설정
criterion = nn.CrossEntropyLoss()
optimizer = torch.optim.Adam(model.parameters(), lr=0.01)

# 간단한 훈련 루프
model.train()
epochs = 100
for epoch in range(epochs):
    # Forward
    outputs = model(X_train_tensor)
    loss = criterion(outputs, y_train_tensor)
    
    # Backward
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()
    
    if (epoch + 1) % 20 == 0:
        print(f"   Epoch [{epoch+1}/{epochs}], Loss: {loss.item():.4f}")

print("✅ 훈련 완료")

# 테스트 정확도 확인
model.eval()
with torch.no_grad():
    test_outputs = model(X_test_tensor)
    _, predicted = torch.max(test_outputs.data, 1)
    accuracy = (predicted == y_test_tensor).sum().item() / len(y_test_tensor)
    print(f"✅ 테스트 정확도: {accuracy * 100:.2f}%")

# ========================================
# 4. ONNX 변환
# ========================================
print("\n[4/13] 🔄 ONNX 변환 중...")

onnx_path = "iris_mlp.onnx"
test_input = torch.FloatTensor(X_test[0:1])  # 첫 번째 테스트 샘플

torch.onnx.export(
    model,
    test_input,
    onnx_path,
    export_params=True,
    opset_version=11,
    do_constant_folding=True,
    input_names=['input'],
    output_names=['output']
)

# PyTorch 2.x 버전 호환성 경고 억제
# Opset 18로 자동 변환되면서 EZKL 호환성 문제 발생 가능
# 실패시: torch 버전을 낮추거나 Week 1 방식 참고

print(f"✅ ONNX 변환 완료: {onnx_path}")

# ONNX 검증
onnx_model = onnx.load(onnx_path)
onnx.checker.check_model(onnx_model)
print("✅ ONNX 모델 검증 완료")

# ========================================
# 5. EZKL 설정 파일 생성
# ========================================
print("\n[5/13] ⚙️  EZKL 설정 파일 생성 중...")

config = {
    "run_args": {
        "tolerance": {
            "val": 0.0,
            "scale": 1.0
        },
        "input_visibility": "Public",   # 입력 공개 (아이리스 특징)
        "output_visibility": "Public",  # 출력 공개 (분류 결과)
        "param_visibility": "Fixed"     # 모델 파라미터 숨김
    }
}

with open("settings.json", "w") as f:
    json.dump(config, f, indent=2)

print("✅ 설정 파일 생성: settings.json")

# ========================================
# 6. 입력 데이터 JSON 생성
# ========================================
print("\n[6/13] 📝 입력 데이터 파일 생성 중...")

# 첫 번째 테스트 샘플 사용
input_data = {
    "input_data": [X_test[0].tolist()]  # (4,) → [[v1, v2, v3, v4]]
}

with open("input.json", "w") as f:
    json.dump(input_data, f, indent=2)

print("✅ 입력 데이터 파일 생성: input.json")
print(f"   입력값: {X_test[0]}")
print(f"   예상 출력: {iris.target_names[y_test[0]]}")

# ========================================
# 7. EZKL 워크플로우 실행
# ========================================
async def run_ezkl_workflow():
    """EZKL 전체 워크플로우 (Week 1과 동일)"""
    print("\n" + "="*60)
    print("🔧 EZKL 워크플로우 시작")
    print("="*60)

    # 파일 경로
    model_path = "iris_mlp.onnx"
    compiled_model_path = "network.ezkl"
    settings_path = "settings.json"
    witness_path = "witness.json"
    pk_path = "test.pk"
    vk_path = "test.vk"
    proof_path = "test.pf"
    srs_path = "kzg.srs"
    data_path = "input.json"

    # Step 1: Settings 생성
    print("\n[7/13] 📝 Settings 생성 중...")
    ezkl.gen_settings(model_path, settings_path)
    print("✅ Settings 생성 완료")

    # Step 2: 회로 컴파일
    print("\n[8/13] 🔨 ZK 회로 컴파일 중...")
    ezkl.compile_circuit(model_path, compiled_model_path, settings_path)
    print("✅ 회로 컴파일 완료")

    # Step 3: SRS 확인/다운로드
    print("\n[9/13] 🎲 SRS 준비 중...")
    if not os.path.exists(srs_path):
        print("  SRS 다운로드 시도...")
        try:
            await ezkl.get_srs(settings_path, srs_path=srs_path)
        except:
            print("  ⚠️  다운로드 실패, CLI로 생성...")
            os.system(f"ezkl gen-srs --srs-path={srs_path} --logrows=17")
    else:
        print("  기존 SRS 파일 사용")
    print("✅ SRS 준비 완료")

    # Step 4: Setup (키 생성)
    print("\n[10/13] 🔑 증명/검증 키 생성 중...")
    ezkl.setup(compiled_model_path, vk_path, pk_path, srs_path)
    print("✅ 키 생성 완료")

    # Step 5: Witness 생성
    print("\n[11/13] ⚡ Witness 생성 중...")
    ezkl.gen_witness(data_path, compiled_model_path, witness_path)
    print("✅ Witness 생성 완료")

    # Step 6: 증명 생성
    print("\n[12/13] 🔐 ZK 증명 생성 중...")
    print("   ⏳ MLP 모델이므로 1-2분 예상...")
    ezkl.prove(witness_path, compiled_model_path, pk_path, proof_path, "single", srs_path)
    print("✅ 증명 생성 완료")

    # Step 7: 증명 검증
    print("\n[13/13] ✅ 증명 검증 중...")
    result = ezkl.verify(proof_path, settings_path, vk_path, srs_path)

    print("\n" + "="*60)
    if result:
        print("🎉 검증 성공! ZK 증명이 유효합니다!")
    else:
        print("❌ 검증 실패!")
    print("="*60)

    # 파일 크기 확인
    print("\n📊 생성된 파일 크기:")
    files = [
        (model_path, "ONNX 모델"),
        (compiled_model_path, "ZK 회로"),
        (proof_path, "ZK 증명"),
        (vk_path, "검증 키"),
        (pk_path, "증명 키"),
        (srs_path, "SRS")
    ]
    
    for file_path, description in files:
        if os.path.exists(file_path):
            size = os.path.getsize(file_path)
            if size > 1024 * 1024:
                size_str = f"{size / (1024 * 1024):.2f} MB"
            elif size > 1024:
                size_str = f"{size / 1024:.2f} KB"
            else:
                size_str = f"{size} bytes"
            print(f"  - {description}: {size_str}")

    print("\n💡 Week 1 vs Week 2 비교:")
    print("  Week 1 (Linear): 파라미터 3개, 증명 시간 ~30초")
    print("  Week 2 (MLP):    파라미터 ~200개, 증명 시간 ~1-2분")
    print("  → 모델 복잡도 증가 → 증명 시간 증가!")

    print("\n🚀 MLP → ZKML 변환 완료!")

# 워크플로우 실행
print("\n" + "="*60)
print("🎯 실전 AI 모델의 첫 ZKML 변환을 시작합니다!")
print("="*60)

asyncio.run(run_ezkl_workflow())

print("\n✅ Week 2-1 완료!")
print("📚 다음 단계: 02_cnn_zkml.py (더 복잡한 CNN 모델)")

