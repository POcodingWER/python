#!/usr/bin/env python3
"""
🚀 간단한 ZKML 테스트
EZKL의 기본 기능만 테스트해보기
"""

import torch
import torch.nn as nn
import onnx
import json
import os

print("🧠 간단한 ZKML 테스트")
print("=" * 40)

# 1. 매우 간단한 모델 생성
class VerySimpleModel(nn.Module):
    def __init__(self):
        super(VerySimpleModel, self).__init__()
        self.linear = nn.Linear(1, 1, bias=False)  # 편향 없이 더 간단하게
        
    def forward(self, x):
        return self.linear(x)

# 2. 모델 생성 및 가중치 설정
model = VerySimpleModel()
with torch.no_grad():
    model.linear.weight.fill_(2.0)  # y = 2x

print(f"📊 모델: y = 2x")
print(f"   가중치: {model.linear.weight.item():.2f}")

# 3. 테스트
test_input = torch.tensor([[3.0]], dtype=torch.float32)
expected_output = model(test_input)
print(f"🧮 테스트: x=3.0 → y={expected_output.item():.2f}")

# 4. ONNX 변환
model.eval()
onnx_path = "very_simple.onnx"

torch.onnx.export(
    model,
    test_input,
    onnx_path,
    export_params=True,
    opset_version=11,
    input_names=['input'],
    output_names=['output']
)

print(f"✅ ONNX 모델 저장: {onnx_path}")

# 5. ONNX 검증
try:
    onnx_model = onnx.load(onnx_path)
    onnx.checker.check_model(onnx_model)
    print("✅ ONNX 모델 검증 성공")
except Exception as e:
    print(f"❌ ONNX 검증 실패: {e}")

# 6. 입력 데이터 생성
input_data = {
    "input_data": [[3.0]]
}

with open("simple_input.json", "w") as f:
    json.dump(input_data, f, indent=2)

print("✅ 입력 데이터 생성: simple_input.json")

# 7. 파일 크기 확인
if os.path.exists(onnx_path):
    size = os.path.getsize(onnx_path)
    print(f"📦 ONNX 모델 크기: {size} bytes")

print("\n🎯 EZKL 명령어 (수동 실행):")
print("1. python -c \"import ezkl; ezkl.gen_settings('very_simple.onnx', 'settings.json')\"")
print("2. python -c \"import ezkl; ezkl.calibrate_settings('simple_input.json', 'very_simple.onnx', 'settings.json')\"")

print("\n🚀 기본 준비 완료! 이제 EZKL 워크플로우를 수동으로 실행해보세요!")
