#!/usr/bin/env python3
"""
🚀 첫 ZKML 예제: 간단한 선형 회귀 → ZK 회로
Month 5 Week 1: EZKL 기초

목표: PyTorch 모델을 ONNX로 변환하고 EZKL로 ZK 회로 생성
"""

import torch
import torch.nn as nn
import numpy as np
import onnx
import ezkl
import os
import json
import asyncio

print("🧠 ZKML 첫 걸음: 선형 회귀 → ZK 회로")
print("=" * 50)

# 1. 간단한 선형 회귀 모델 정의
class SimpleLinearRegression(nn.Module):
    def __init__(self):
        super(SimpleLinearRegression, self).__init__()
        self.linear = nn.Linear(1, 1)  # 입력 1개, 출력 1개
        
    def forward(self, x):
        return self.linear(x)

# 2. 모델 생성 및 가중치 설정
model = SimpleLinearRegression()
# 간단한 가중치 설정: y = 2x + 1
with torch.no_grad():
    model.linear.weight.fill_(2.0)
    model.linear.bias.fill_(1.0)

print(f"📊 모델 구조: y = 2x + 1")
print(f"   가중치: {model.linear.weight.item():.2f}")
print(f"   편향: {model.linear.bias.item():.2f}")

# 3. 테스트 데이터 생성
test_input = torch.tensor([[3.0]], dtype=torch.float32)
expected_output = model(test_input)
print(f"🧮 테스트: x=3.0 → y={expected_output.item():.2f}")

# 4. ONNX 모델로 변환
model.eval()
onnx_path = "simple_linear.onnx"

torch.onnx.export(
    model,
    test_input,
    onnx_path,
    export_params=True,
    opset_version=11,
    do_constant_folding=True,
    input_names=['input'],
    output_names=['output']
    # EZKL을 위해 dynamic_axes 제거 (고정 배치 크기 사용)
)

print(f"✅ ONNX 모델 저장: {onnx_path}")

# 5. ONNX 모델 검증
onnx_model = onnx.load(onnx_path)
onnx.checker.check_model(onnx_model)
print("✅ ONNX 모델 검증 완료")

# 6. EZKL 설정 파일 생성
def create_ezkl_config():
    """EZKL 설정 파일 생성"""
    config = {
        "run_args": {
            "tolerance": {
                "val": 0.0,
                "scale": 1.0
            },
            "input_visibility": "Public",   # 입력 공개
            "output_visibility": "Public",  # 출력 공개
            "param_visibility": "Fixed"     # 파라미터 숨김
        }
    }
    
    with open("settings.json", "w") as f:
        json.dump(config, f, indent=2)
    
    print("✅ EZKL 설정 파일 생성: settings.json")

create_ezkl_config()

# 7. 입력 데이터 JSON 파일 생성 (EZKL 형식)
input_data = {
    "input_data": [[3.0]]  # EZKL이 기대하는 간단한 형식
}

with open("input.json", "w") as f:
    json.dump(input_data, f, indent=2)

print("✅ 입력 데이터 파일 생성: input.json")

# ========================================
# EZKL 워크플로우 자동화
# ========================================
async def run_ezkl_workflow():
    """EZKL 전체 워크플로우 실행 (Async)"""
    print("\n" + "="*50)
    print("🔧 EZKL 워크플로우 시작")
    print("="*50)

    # 파일 경로 정의
    model_path = "simple_linear.onnx"
    compiled_model_path = "network.ezkl"
    settings_path = "settings.json"
    witness_path = "witness.json"
    pk_path = "test.pk"
    vk_path = "test.vk"
    proof_path = "test.pf"
    srs_path = "kzg.srs"
    data_path = "input.json"

    # Step 1: Settings 생성
    print("\n[1/7] 📝 Settings 생성 중...")
    ezkl.gen_settings(model_path, settings_path)
    print("✅ Settings 생성 완료")

    # Step 2: 회로 컴파일
    print("\n[2/7] 🔨 ZK 회로 컴파일 중...")
    ezkl.compile_circuit(model_path, compiled_model_path, settings_path)
    print("✅ 회로 컴파일 완료")

    # Step 3: SRS 확인/다운로드
    print("\n[3/7] 🎲 SRS 준비 중...")
    if not os.path.exists(srs_path):
        print("  SRS 다운로드 시도...")
        try:
            await ezkl.get_srs(settings_path, srs_path=srs_path)
        except:
            print("  ⚠️  다운로드 실패, CLI로 생성 시도...")
            os.system(f"ezkl gen-srs --srs-path={srs_path} --logrows=17")
    else:
        print("  기존 SRS 파일 사용")
    print("✅ SRS 준비 완료")

    # Step 4: Setup (키 생성)
    print("\n[4/7] 🔑 증명/검증 키 생성 중...")
    ezkl.setup(compiled_model_path, vk_path, pk_path, srs_path)
    print("✅ 키 생성 완료")

    # Step 5: Witness 생성
    print("\n[5/7] ⚡ Witness 생성 중...")
    ezkl.gen_witness(data_path, compiled_model_path, witness_path)
    print("✅ Witness 생성 완료")

    # Step 6: 증명 생성
    print("\n[6/7] 🔐 ZK 증명 생성 중...")
    ezkl.prove(witness_path, compiled_model_path, pk_path, proof_path, "single", srs_path)
    print("✅ 증명 생성 완료")

    # Step 7: 증명 검증
    print("\n[7/7] ✅ 증명 검증 중...")
    result = ezkl.verify(proof_path, settings_path, vk_path, srs_path)

    print("\n" + "="*50)
    if result:
        print("🎉 검증 성공! ZK 증명이 유효합니다!")
    else:
        print("❌ 검증 실패!")
    print("="*50)

    print("\n📁 생성된 파일:")
    print(f"  - {model_path} (ONNX 모델)")
    print(f"  - {compiled_model_path} (ZK 회로)")
    print(f"  - {proof_path} (ZK 증명)")
    print(f"  - {vk_path} (검증 키)")
    print(f"  - {pk_path} (증명 키)")

    print("\n🚀 ZKML 전체 워크플로우 완료!")

# 워크플로우 실행
asyncio.run(run_ezkl_workflow())
