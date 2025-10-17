"""
Week 3-1: MNIST MLP ZKML 변환

목표:
- MNIST 손글씨 숫자 분류 (0-9)
- CNN 없이 MLP만 사용 (이미지 flatten)
- EZKL 호환성 확인

전략:
- 28x28 이미지 → 784 차원 flatten
- Linear만 사용 (ReLU 없음)
- 파라미터 최소화로 증명 시간 단축

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
print("🎯 Week 3-1: MNIST MLP ZKML 변환")
print("="*60)

# ====================
# 1. 하이퍼파라미터 설정
# ====================
print("\n[1/15] 하이퍼파라미터 설정")

INPUT_SIZE = 28 * 28  # 784
HIDDEN_SIZE = 32      # Week 2보다 크게 (더 복잡한 문제)
OUTPUT_SIZE = 10      # 0-9 숫자
BATCH_SIZE = 64
EPOCHS = 3            # 빠른 실험을 위해
LEARNING_RATE = 0.001

print(f"  입력: {INPUT_SIZE} (28x28 이미지)")
print(f"  은닉층: {HIDDEN_SIZE}")
print(f"  출력: {OUTPUT_SIZE} (10 클래스)")
print(f"  배치 크기: {BATCH_SIZE}")
print(f"  에폭: {EPOCHS}")

# ====================
# 2. 데이터 로드
# ====================
print("\n[2/15] MNIST 데이터셋 로드")

transform = transforms.Compose([
    transforms.ToTensor(),
    transforms.Normalize((0.1307,), (0.3081,))  # MNIST 평균/표준편차
])

# 훈련 데이터
train_dataset = datasets.MNIST(
    root='./data',
    train=True,
    download=True,
    transform=transform
)

# 테스트 데이터
test_dataset = datasets.MNIST(
    root='./data',
    train=False,
    download=True,
    transform=transform
)

train_loader = DataLoader(train_dataset, batch_size=BATCH_SIZE, shuffle=True)
test_loader = DataLoader(test_dataset, batch_size=BATCH_SIZE, shuffle=False)

print(f"  훈련 데이터: {len(train_dataset):,}개")
print(f"  테스트 데이터: {len(test_dataset):,}개")

# ====================
# 3. 모델 정의
# ====================
print("\n[3/15] 모델 정의 (Linear만 사용)")

class MNIST_MLP(nn.Module):
    """
    간단한 MLP 모델 (ReLU 없음)
    
    구조:
    - Linear(784, 32)
    - Linear(32, 10)
    
    중요: view() 제거 → 입력을 미리 flatten
    """
    def __init__(self):
        super(MNIST_MLP, self).__init__()
        self.fc1 = nn.Linear(INPUT_SIZE, HIDDEN_SIZE)
        self.fc2 = nn.Linear(HIDDEN_SIZE, OUTPUT_SIZE)
    
    def forward(self, x):
        # 입력이 이미 (batch_size, 784) 형태라고 가정
        # view() 제거 → ONNX Reshape 연산 회피
        x = self.fc1(x)
        x = self.fc2(x)  # Softmax는 손실함수에 포함
        return x

model = MNIST_MLP()

# 파라미터 수 계산
total_params = sum(p.numel() for p in model.parameters())
print(f"  총 파라미터: {total_params:,}개")
print(f"  구조: Linear(784→{HIDDEN_SIZE}) → Linear({HIDDEN_SIZE}→10)")

# ====================
# 4. 훈련 설정
# ====================
print("\n[4/15] 훈련 설정")

criterion = nn.CrossEntropyLoss()
optimizer = optim.Adam(model.parameters(), lr=LEARNING_RATE)

print(f"  손실 함수: CrossEntropyLoss")
print(f"  옵티마이저: Adam (lr={LEARNING_RATE})")

# ====================
# 5. 훈련
# ====================
print("\n[5/15] 모델 훈련 시작")

model.train()
for epoch in range(EPOCHS):
    running_loss = 0.0
    correct = 0
    total = 0
    
    for batch_idx, (images, labels) in enumerate(train_loader):
        # Flatten 이미지 (batch_size, 1, 28, 28) → (batch_size, 784)
        images = images.view(-1, INPUT_SIZE)
        
        # Forward
        outputs = model(images)
        loss = criterion(outputs, labels)
        
        # Backward
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        
        # 통계
        running_loss += loss.item()
        _, predicted = outputs.max(1)
        total += labels.size(0)
        correct += predicted.eq(labels).sum().item()
        
        # 진행 상황 (매 100 배치)
        if (batch_idx + 1) % 100 == 0:
            acc = 100. * correct / total
            avg_loss = running_loss / (batch_idx + 1)
            print(f"  Epoch {epoch+1}/{EPOCHS}, Batch {batch_idx+1}/{len(train_loader)}, "
                  f"Loss: {avg_loss:.4f}, Acc: {acc:.2f}%")
    
    # 에폭 결과
    epoch_loss = running_loss / len(train_loader)
    epoch_acc = 100. * correct / total
    print(f"  ✅ Epoch {epoch+1} 완료: Loss={epoch_loss:.4f}, Acc={epoch_acc:.2f}%")

# ====================
# 6. 테스트
# ====================
print("\n[6/15] 모델 테스트")

model.eval()
test_correct = 0
test_total = 0

with torch.no_grad():
    for images, labels in test_loader:
        # Flatten 이미지
        images = images.view(-1, INPUT_SIZE)
        
        outputs = model(images)
        _, predicted = outputs.max(1)
        test_total += labels.size(0)
        test_correct += predicted.eq(labels).sum().item()

test_acc = 100. * test_correct / test_total
print(f"  테스트 정확도: {test_acc:.2f}%")

# ====================
# 7. ONNX 변환
# ====================
print("\n[7/15] ONNX 변환")

# 더미 입력 (1, 784) - 이미 flatten된 형태
dummy_input = torch.randn(1, INPUT_SIZE)

onnx_path = "mnist_mlp.onnx"
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
print(f"  ONNX 파일 생성: {onnx_path}")
print(f"  파일 크기: {onnx_size:.2f} KB")

# ====================
# 8. EZKL 입력 데이터 생성
# ====================
print("\n[8/15] EZKL 입력 데이터 생성")

# 테스트 이미지 1개 선택 (숫자 7)
test_image, test_label = test_dataset[0]

# Flatten (1, 28, 28) → (784,)
test_image_flat = test_image.view(-1).numpy().tolist()

input_data = {
    "input_data": [test_image_flat]
}

input_path = "input.json"
with open(input_path, 'w') as f:
    json.dump(input_data, f)

print(f"  입력 파일 생성: {input_path}")
print(f"  테스트 이미지: 레이블 {test_label}")

# 예측 확인
with torch.no_grad():
    model.eval()
    # Flatten해서 입력
    test_image_input = test_image.view(1, -1)
    pred = model(test_image_input)
    pred_label = pred.argmax(dim=1).item()
    print(f"  모델 예측: {pred_label} (정답: {test_label})")

# ====================
# 9-15. EZKL 워크플로우
# ====================

async def run_ezkl_workflow():
    """EZKL 전체 워크플로우 실행"""
    
    settings_path = "settings.json"
    compiled_model_path = "network.ezkl"
    pk_path = "test.pk"
    vk_path = "test.vk"
    srs_path = "kzg.srs"
    witness_path = "witness.json"
    proof_path = "test.pf"
    
    # 9. Settings 생성
    print("\n[9/15] EZKL Settings 생성")
    ezkl.gen_settings(onnx_path, settings_path)
    print(f"  ✅ Settings 생성: {settings_path}")
    
    # 10. Calibrate Settings
    print("\n[10/15] Settings Calibration")
    ezkl.calibrate_settings(input_path, onnx_path, settings_path, "resources")
    print(f"  ✅ Calibration 완료")
    
    # 11. 컴파일
    print("\n[11/15] ZK 회로 컴파일")
    ezkl.compile_circuit(onnx_path, compiled_model_path, settings_path)
    circuit_size = os.path.getsize(compiled_model_path) / (1024 * 1024)
    print(f"  ✅ 컴파일 완료: {compiled_model_path} ({circuit_size:.2f} MB)")
    
    # 12. SRS 준비
    print("\n[12/15] SRS (Structured Reference String) 준비")
    if os.path.exists(srs_path):
        print(f"  ✅ 기존 SRS 사용: {srs_path}")
    else:
        print(f"  ⏳ SRS 다운로드 시도 (시간 소요 가능)...")
        try:
            await ezkl.get_srs(settings_path=settings_path, srs_path=srs_path)
            print(f"  ✅ SRS 다운로드 완료")
        except Exception as e:
            print(f"  ⚠️  다운로드 실패, 로컬 생성 시도...")
            os.system(f"ezkl gen-srs --srs-path={srs_path} --logrows=17")
            print(f"  ✅ SRS 로컬 생성 완료")
    
    # 13. Setup (PK/VK 생성)
    print("\n[13/15] Proving Key & Verification Key 생성")
    print("  ⏳ 시간이 오래 걸릴 수 있습니다 (1-3분)...")
    ezkl.setup(compiled_model_path, vk_path, pk_path, srs_path)
    
    pk_size = os.path.getsize(pk_path) / (1024 * 1024)
    vk_size = os.path.getsize(vk_path) / 1024
    print(f"  ✅ PK 생성: {pk_size:.2f} MB")
    print(f"  ✅ VK 생성: {vk_size:.2f} KB")
    
    # 14. Witness 생성
    print("\n[14/15] Witness 생성")
    ezkl.gen_witness(input_path, compiled_model_path, witness_path)
    witness_size = os.path.getsize(witness_path) / 1024
    print(f"  ✅ Witness 생성: {witness_size:.2f} KB")
    
    # 15. Proof 생성
    print("\n[15/15] ZK Proof 생성")
    print("  ⏳ 증명 생성 중 (5-15분 예상)...")
    
    import time
    start_time = time.time()
    
    ezkl.prove(
        witness_path,
        compiled_model_path,
        pk_path,
        proof_path,
        "single",
        srs_path
    )
    
    elapsed_time = time.time() - start_time
    proof_size = os.path.getsize(proof_path) / 1024
    
    print(f"  ✅ Proof 생성 완료!")
    print(f"  증명 시간: {elapsed_time:.1f}초 ({elapsed_time/60:.1f}분)")
    print(f"  Proof 크기: {proof_size:.2f} KB")
    
    # 16. Verify
    print("\n[16/15] ZK Proof 검증")
    is_valid = ezkl.verify(
        proof_path,
        settings_path,
        vk_path,
        srs_path
    )
    
    if is_valid:
        print("  ✅ 검증 성공! 🎉")
    else:
        print("  ❌ 검증 실패")
    
    return is_valid, elapsed_time, proof_size, pk_size

# 실행
print("\n" + "="*60)
print("🚀 EZKL 워크플로우 시작")
print("="*60)

is_valid, proof_time, proof_size, pk_size = asyncio.run(run_ezkl_workflow())

# ====================
# 최종 결과
# ====================
print("\n" + "="*60)
print("📊 최종 결과")
print("="*60)
print(f"모델 정확도:    {test_acc:.2f}%")
print(f"파라미터 수:     {total_params:,}개")
print(f"ONNX 크기:      {onnx_size:.2f} KB")
print(f"증명 시간:       {proof_time:.1f}초 ({proof_time/60:.1f}분)")
print(f"Proof 크기:     {proof_size:.2f} KB")
print(f"PK 크기:        {pk_size:.2f} MB")
print(f"검증 결과:       {'✅ 성공' if is_valid else '❌ 실패'}")
print("="*60)

if is_valid:
    print("\n🎉 Week 3-1: MNIST MLP ZKML 변환 성공!")
    print("\n다음 단계:")
    print("  1. Level 2: 초간단 CNN 도전 (Conv2d 1개)")
    print("  2. Level 3: CIFAR-10 CNN 포팅")
else:
    print("\n⚠️  검증 실패. 디버깅 필요.")

