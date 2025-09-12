# 🎮 파이썬 인터랙티브 데모

print("🐍 파이썬 기초 학습 - 인터랙티브 데모")
print("="*50)

# 1. 간단한 계산기
print("\n1️⃣ 간단한 계산기 데모")
a, b = 10, 3
print(f"{a} + {b} = {a + b}")
print(f"{a} - {b} = {a - b}")
print(f"{a} * {b} = {a * b}")
print(f"{a} / {b} = {a / b:.2f}")

# 2. 조건문 예제
print("\n2️⃣ 성적 등급 시스템")
scores = [95, 85, 75, 65]
for score in scores:
    if score >= 90:
        grade = "A"
    elif score >= 80:
        grade = "B"
    elif score >= 70:
        grade = "C"
    else:
        grade = "F"
    print(f"점수 {score}점 → 등급: {grade}")

# 3. 반복문 예제
print("\n3️⃣ 구구단 (5단)")
for i in range(1, 6):
    print(f"5 × {i} = {5 * i}")

# 4. 리스트 다루기
print("\n4️⃣ 과일 리스트 관리")
fruits = ["사과", "바나나", "오렌지"]
print(f"원래 과일들: {fruits}")

fruits.append("딸기")
fruits.append("포도")
print(f"과일 추가 후: {fruits}")
print(f"총 과일 개수: {len(fruits)}개")

# 5. 딕셔너리 활용
print("\n5️⃣ 학생 정보 관리")
student = {
    "이름": "김파이썬",
    "나이": 20,
    "학과": "컴퓨터공학",
    "성적": [90, 85, 92]
}

print("학생 정보:")
for key, value in student.items():
    if key == "성적":
        avg = sum(value) / len(value)
        print(f"  {key}: {value} (평균: {avg:.1f}점)")
    else:
        print(f"  {key}: {value}")

# 6. 함수 예제
print("\n6️⃣ 함수 활용 - 원의 넓이 계산")
def circle_area(radius):
    """원의 넓이를 계산하는 함수"""
    pi = 3.14159
    return pi * radius ** 2

radii = [3, 5, 7]
for r in radii:
    area = circle_area(r)
    print(f"반지름 {r}cm → 넓이: {area:.2f}cm²")

# 7. 간단한 게임
print("\n7️⃣ 숫자 맞추기 게임 (데모)")
import random

target = random.randint(1, 10)
guesses = [3, 7, target]  # 시뮬레이션용

print("1~10 사이의 숫자를 맞춰보세요!")
for i, guess in enumerate(guesses, 1):
    print(f"시도 {i}: {guess}")
    if guess == target:
        print(f"🎉 정답! {target}을 맞췄습니다!")
        break
    elif guess < target:
        print("더 큰 수를 시도해보세요!")
    else:
        print("더 작은 수를 시도해보세요!")

print("\n✨ 데모 완료! 파이썬의 다양한 기능들을 확인해보았습니다.")
print("📚 더 자세한 학습은 Jupyter 노트북 파일들을 확인해주세요!")
