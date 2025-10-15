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
    output_names=['output'],
    dynamic_axes={
        'input': {0: 'batch_size'},
        'output': {0: 'batch_size'}
    }
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
            "input_visibility": "public",
            "output_visibility": "public",
            "param_visibility": "fixed"
        }
    }
    
    with open("settings.json", "w") as f:
        json.dump(config, f, indent=2)
    
    print("✅ EZKL 설정 파일 생성: settings.json")

create_ezkl_config()

# 7. 입력 데이터 JSON 파일 생성
input_data = {
    "input_data": [[[3.0]]]  # EZKL 형식에 맞게 3차원 배열
}

with open("input.json", "w") as f:
    json.dump(input_data, f, indent=2)

print("✅ 입력 데이터 파일 생성: input.json")

print("\n🎯 다음 단계:")
print("1. ezkl gen-settings 실행")
print("2. ezkl calibrate-settings 실행")
print("3. ezkl compile-circuit 실행")
print("4. ezkl setup 실행")
print("5. ezkl prove 실행")
print("6. ezkl verify 실행")

print("\n🚀 ZKML 첫 예제 준비 완료!")
