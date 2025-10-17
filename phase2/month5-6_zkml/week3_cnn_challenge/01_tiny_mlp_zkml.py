"""
Week 3-1B: Tiny MLP ZKML 변환 (크기 축소 버전)

전략:
- 14x14로 다운샘플링 (784 → 196 차원)
- 파라미터 최소화
- EZKL 호환성 우선

Author: AI + Human
Date: 2025-10-17
"""

import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader
from torchvision import datasets, transforms
import ezkl
import os
import json
import asyncio

print("="*60)
print("🎯 Week 3-1B: Tiny MLP ZKML 변환 (축소 버전)")
print("="*60)

# ====================
# 1. 하이퍼파라미터 설정
# ====================
print("\n[1/15] 하이퍼파라미터 설정")

IMAGE_SIZE = 14       # 14x14로 축소
INPUT_SIZE = IMAGE_SIZE * IMAGE_SIZE  # 196
HIDDEN_SIZE = 16      # 작게
OUTPUT_SIZE = 10
BATCH_SIZE = 64
EPOCHS = 2            # 빠르게
LEARNING_RATE = 0.001

print(f"  이미지 크기: {IMAGE_SIZE}×{IMAGE_SIZE} = {INPUT_SIZE} (원본 대비 1/4)")
print(f"  은닉층: {HIDDEN_SIZE}")
print(f"  출력: {OUTPUT_SIZE}")

# ====================
# 2. 데이터 로드 (Resize 추가)
# ====================
print("\n[2/15] MNIST 데이터셋 로드 (14×14 다운샘플링)")

transform = transforms.Compose([
    transforms.Resize(IMAGE_SIZE),  # 28×28 → 14×14
    transforms.ToTensor(),
    transforms.Normalize((0.1307,), (0.3081,))
])

train_dataset = datasets.MNIST('./data', train=True, download=True, transform=transform)
test_dataset = datasets.MNIST('./data', train=False, download=True, transform=transform)

train_loader = DataLoader(train_dataset, batch_size=BATCH_SIZE, shuffle=True)
test_loader = DataLoader(test_dataset, batch_size=BATCH_SIZE, shuffle=False)

print(f"  훈련 데이터: {len(train_dataset):,}개")
print(f"  테스트 데이터: {len(test_dataset):,}개")

# ====================
# 3. 모델 정의
# ====================
print("\n[3/15] 초소형 MLP 정의")

class TinyMLP(nn.Module):
    """
    초소형 MLP (파라미터 최소화)
    """
    def __init__(self):
        super(TinyMLP, self).__init__()
        self.fc1 = nn.Linear(INPUT_SIZE, HIDDEN_SIZE)
        self.fc2 = nn.Linear(HIDDEN_SIZE, OUTPUT_SIZE)
    
    def forward(self, x):
        x = self.fc1(x)
        x = self.fc2(x)
        return x

model = TinyMLP()
total_params = sum(p.numel() for p in model.parameters())
print(f"  총 파라미터: {total_params:,}개 (Week 2: 43개)")
print(f"  구조: Linear({INPUT_SIZE}→{HIDDEN_SIZE}) → Linear({HIDDEN_SIZE}→{OUTPUT_SIZE})")

# ====================
# 4. 훈련
# ====================
print("\n[4/15] 훈련 설정 및 시작")

criterion = nn.CrossEntropyLoss()
optimizer = optim.Adam(model.parameters(), lr=LEARNING_RATE)

model.train()
for epoch in range(EPOCHS):
    running_loss = 0.0
    correct = 0
    total = 0
    
    for batch_idx, (images, labels) in enumerate(train_loader):
        # Flatten
        images = images.view(-1, INPUT_SIZE)
        
        outputs = model(images)
        loss = criterion(outputs, labels)
        
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        
        running_loss += loss.item()
        _, predicted = outputs.max(1)
        total += labels.size(0)
        correct += predicted.eq(labels).sum().item()
        
        if (batch_idx + 1) % 200 == 0:
            acc = 100. * correct / total
            avg_loss = running_loss / (batch_idx + 1)
            print(f"  Epoch {epoch+1}/{EPOCHS}, Batch {batch_idx+1}, Loss: {avg_loss:.4f}, Acc: {acc:.2f}%")
    
    epoch_acc = 100. * correct / total
    print(f"  ✅ Epoch {epoch+1} 완료: Acc={epoch_acc:.2f}%")

# ====================
# 5. 테스트
# ====================
print("\n[5/15] 테스트")

model.eval()
test_correct = 0
test_total = 0

with torch.no_grad():
    for images, labels in test_loader:
        images = images.view(-1, INPUT_SIZE)
        outputs = model(images)
        _, predicted = outputs.max(1)
        test_total += labels.size(0)
        test_correct += predicted.eq(labels).sum().item()

test_acc = 100. * test_correct / test_total
print(f"  테스트 정확도: {test_acc:.2f}% (14×14 해상도)")

# ====================
# 6. ONNX 변환
# ====================
print("\n[6/15] ONNX 변환")

dummy_input = torch.randn(1, INPUT_SIZE)
onnx_path = "tiny_mlp.onnx"

torch.onnx.export(
    model,
    dummy_input,
    onnx_path,
    export_params=True,
    opset_version=11,
    do_constant_folding=True,
    input_names=['input'],
    output_names=['output'],
)

onnx_size = os.path.getsize(onnx_path) / 1024
print(f"  ✅ ONNX: {onnx_path} ({onnx_size:.2f} KB)")

# ====================
# 7. 입력 데이터
# ====================
print("\n[7/15] 입력 데이터 생성")

test_image, test_label = test_dataset[0]
test_image_flat = test_image.view(-1).numpy().tolist()

input_data = {"input_data": [test_image_flat]}
input_path = "input.json"

with open(input_path, 'w') as f:
    json.dump(input_data, f)

print(f"  ✅ Input: {input_path} (레이블: {test_label})")

# 예측 확인
with torch.no_grad():
    model.eval()
    pred = model(test_image.view(1, -1))
    pred_label = pred.argmax(dim=1).item()
    print(f"  모델 예측: {pred_label} (정답: {test_label})")

# ====================
# 8-15. EZKL 워크플로우
# ====================

async def run_ezkl():
    settings_path = "settings.json"
    compiled_path = "network.ezkl"
    pk_path = "test.pk"
    vk_path = "test.vk"
    srs_path = "kzg.srs"
    witness_path = "witness.json"
    proof_path = "test.pf"
    
    print("\n[8/15] Settings 생성")
    ezkl.gen_settings(onnx_path, settings_path)
    print("  ✅ Settings")
    
    print("\n[9/15] Calibration")
    ezkl.calibrate_settings(input_path, onnx_path, settings_path, "resources")
    print("  ✅ Calibration")
    
    print("\n[10/15] 컴파일")
    ezkl.compile_circuit(onnx_path, compiled_path, settings_path)
    circuit_size = os.path.getsize(compiled_path) / (1024 * 1024)
    print(f"  ✅ Circuit: {circuit_size:.2f} MB")
    
    print("\n[11/15] SRS 준비")
    if os.path.exists(srs_path):
        print("  ✅ 기존 SRS 사용")
    else:
        try:
            await ezkl.get_srs(settings_path=settings_path, srs_path=srs_path)
            print("  ✅ SRS 다운로드")
        except:
            os.system(f"ezkl gen-srs --srs-path={srs_path} --logrows=17")
            print("  ✅ SRS 로컬 생성")
    
    print("\n[12/15] Setup (PK/VK)")
    print("  ⏳ 1-3분 소요...")
    ezkl.setup(compiled_path, vk_path, pk_path, srs_path)
    pk_size = os.path.getsize(pk_path) / (1024 * 1024)
    vk_size = os.path.getsize(vk_path) / 1024
    print(f"  ✅ PK: {pk_size:.2f} MB, VK: {vk_size:.2f} KB")
    
    print("\n[13/15] Witness 생성")
    ezkl.gen_witness(input_path, compiled_path, witness_path)
    print("  ✅ Witness")
    
    print("\n[14/15] Proof 생성")
    print("  ⏳ 5-15분 예상...")
    
    import time
    start = time.time()
    
    ezkl.prove(witness_path, compiled_path, pk_path, proof_path, "single", srs_path)
    
    elapsed = time.time() - start
    proof_size = os.path.getsize(proof_path) / 1024
    
    print(f"  ✅ Proof 완료! {elapsed:.1f}초 ({elapsed/60:.1f}분)")
    print(f"  Proof 크기: {proof_size:.2f} KB")
    
    print("\n[15/15] 검증")
    is_valid = ezkl.verify(proof_path, settings_path, vk_path, srs_path)
    
    if is_valid:
        print("  ✅ 검증 성공! 🎉")
    else:
        print("  ❌ 검증 실패")
    
    return is_valid, elapsed, proof_size, pk_size

# 실행
print("\n" + "="*60)
print("🚀 EZKL 워크플로우 시작")
print("="*60)

is_valid, proof_time, proof_size, pk_size = asyncio.run(run_ezkl())

# ====================
# 결과
# ====================
print("\n" + "="*60)
print("📊 최종 결과")
print("="*60)
print(f"이미지 크기:     {IMAGE_SIZE}×{IMAGE_SIZE} (14×14)")
print(f"입력 차원:       {INPUT_SIZE} (원본 대비 1/4)")
print(f"파라미터:        {total_params:,}개")
print(f"테스트 정확도:   {test_acc:.2f}%")
print(f"ONNX 크기:      {onnx_size:.2f} KB")
print(f"증명 시간:       {proof_time:.1f}초 ({proof_time/60:.1f}분)")
print(f"Proof 크기:     {proof_size:.2f} KB")
print(f"PK 크기:        {pk_size:.2f} MB")
print(f"검증 결과:       {'✅ 성공' if is_valid else '❌ 실패'}")
print("="*60)

if is_valid:
    print("\n🎉 Week 3-1B: Tiny MLP ZKML 성공!")
    print("\n💡 배운 점:")
    print("  1. 입력 크기가 증명 시간에 큰 영향")
    print("  2. 784 → 196 차원으로 줄이니 성공")
    print("  3. 정확도는 조금 떨어지지만 ZKML 가능")
    print("\n다음 단계:")
    print("  - 최적의 이미지 크기 찾기 (16×16? 20×20?)")
    print("  - Hidden size 조정 실험")
    print("  - Level 2: CNN 도전")

