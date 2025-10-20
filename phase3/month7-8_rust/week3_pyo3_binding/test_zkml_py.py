#!/usr/bin/env python3
"""
🦀 PyO3 테스트: Rust 함수를 Python에서 호출
"""

import time
import zkml_py

print("=" * 60)
print("🦀 PyO3: Rust → Python 바인딩 테스트")
print("=" * 60)

# 1. 간단한 함수 호출
print("\n[1] 간단한 함수 호출")
result = zkml_py.add(10.0, 20.0)
print(f"  zkml_py.add(10.0, 20.0) = {result}")

# 2. Linear Regression 모델
print("\n[2] Linear Regression 모델")
model = zkml_py.LinearModel(2.0, 1.0)
print(f"  모델: {model}")

# 단일 예측
x = 3.0
y = model.predict(x)
print(f"  예측: x={x} → y={y}")

# 배치 예측
inputs = [1.0, 2.0, 3.0, 4.0, 5.0]
outputs = model.predict_batch(inputs)
print(f"  배치 예측:")
for x, y in zip(inputs, outputs):
    print(f"    x={x} → y={y}")

# 3. 성능 비교: 벡터 내적
print("\n[3] 성능 비교: 벡터 내적 (100만 차원)")

# 벡터 생성
n = 1_000_000
a = [float(i) for i in range(n)]
b = [float(i) for i in range(n)]

# Rust (PyO3)
start = time.time()
result_rust = zkml_py.dot_product(a, b)
time_rust = time.time() - start

# Python (순수)
start = time.time()
result_python = sum(x * y for x, y in zip(a, b))
time_python = time.time() - start

print(f"  Rust (PyO3):   {time_rust*1000:.2f}ms")
print(f"  Python (순수): {time_python*1000:.2f}ms")
print(f"  성능 향상:     {time_python/time_rust:.1f}배 ⚡")
print(f"  결과 일치:     {result_rust == result_python} ✅")

# 4. 대량 예측 성능 비교 (개별 호출)
print("\n[4] Linear Regression 100만 회 예측 (개별 호출)")

# Rust (PyO3)
start = time.time()
for i in range(1_000_000):
    _ = model.predict(float(i))
time_rust = time.time() - start

# Python (순수)
def python_predict(x, weight=2.0, bias=1.0):
    return weight * x + bias

start = time.time()
for i in range(1_000_000):
    _ = python_predict(float(i))
time_python = time.time() - start

print(f"  Rust (PyO3):   {time_rust*1000:.2f}ms")
print(f"  Python (순수): {time_python*1000:.2f}ms")
print(f"  차이:          {time_python/time_rust:.2f}배")
print(f"  ⚠️ PyO3 오버헤드: 100만 번 Python ↔ Rust 전환")

# 5. 배치 예측 성능 비교 (올바른 방법!)
print("\n[5] Linear Regression 100만 회 예측 (배치 처리) ⭐")

# 데이터 준비
data = [float(i) for i in range(1_000_000)]

# Rust (PyO3) - 배치
start = time.time()
_ = model.predict_batch(data)
time_rust_batch = time.time() - start

# Python (순수) - 배치
start = time.time()
_ = [python_predict(x) for x in data]
time_python_batch = time.time() - start

print(f"  Rust (PyO3):   {time_rust_batch*1000:.2f}ms")
print(f"  Python (순수): {time_python_batch*1000:.2f}ms")
print(f"  성능 향상:     {time_python_batch/time_rust_batch:.1f}배 ⚡")
print(f"  💡 배치 처리로 오버헤드 제거!")

print("\n" + "=" * 60)
print("✅ PyO3 테스트 완료!")
print("=" * 60)


