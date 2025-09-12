# 🚀 파이썬 기초 빠른 참조 가이드

"""
파이썬 기초 문법 한눈에 보기
필요할 때마다 참고하세요!
"""

# ========================================
# 1. 변수와 데이터 타입
# ========================================

# 기본 데이터 타입
name = "홍길동"          # 문자열 (str)
age = 25                # 정수 (int) 
height = 175.5          # 실수 (float)
is_student = True       # 불린 (bool)

# 타입 확인
print(type(name))       # <class 'str'>

# 타입 변환
str_num = "123"
int_num = int(str_num)  # 문자열 → 정수
float_num = float(str_num)  # 문자열 → 실수

# ========================================
# 2. 연산자
# ========================================

# 산술 연산자
a, b = 10, 3
print(a + b)    # 덧셈: 13
print(a - b)    # 뺄셈: 7
print(a * b)    # 곱셈: 30
print(a / b)    # 나눗셈: 3.333...
print(a // b)   # 몫: 3
print(a % b)    # 나머지: 1
print(a ** b)   # 거듭제곱: 1000

# 비교 연산자
print(a == b)   # 같음: False
print(a != b)   # 다름: True
print(a > b)    # 큼: True
print(a < b)    # 작음: False

# 논리 연산자
x, y = True, False
print(x and y)  # 그리고: False
print(x or y)   # 또는: True
print(not x)    # 부정: False

# ========================================
# 3. 문자열
# ========================================

text = "Hello Python"

# 문자열 연산
greeting = "안녕" + "하세요"       # 연결
repeat = "★" * 5                # 반복

# 문자열 메서드
print(text.upper())             # 대문자: HELLO PYTHON
print(text.lower())             # 소문자: hello python
print(text.replace("Python", "World"))  # 바꾸기
print(len(text))                # 길이: 12

# 문자열 포맷팅
name = "김철수"
age = 20
print(f"이름: {name}, 나이: {age}")  # f-string

# ========================================
# 4. 조건문
# ========================================

score = 85

# 기본 if문
if score >= 80:
    print("합격")
else:
    print("불합격")

# 다중 조건
if score >= 90:
    grade = "A"
elif score >= 80:
    grade = "B"
elif score >= 70:
    grade = "C"
else:
    grade = "F"

# ========================================
# 5. 반복문
# ========================================

# for문 - 범위
for i in range(5):          # 0, 1, 2, 3, 4
    print(i)

for i in range(1, 6):       # 1, 2, 3, 4, 5
    print(i)

for i in range(0, 10, 2):   # 0, 2, 4, 6, 8
    print(i)

# for문 - 리스트
fruits = ["사과", "바나나", "오렌지"]
for fruit in fruits:
    print(fruit)

# while문
count = 0
while count < 5:
    print(count)
    count += 1

# break와 continue
for i in range(10):
    if i == 3:
        continue    # 3 건너뛰기
    if i == 7:
        break       # 7에서 중단
    print(i)

# ========================================
# 6. 함수
# ========================================

# 기본 함수
def greet():
    print("안녕하세요!")

# 매개변수가 있는 함수
def greet_person(name):
    print(f"안녕하세요, {name}님!")

# 리턴값이 있는 함수
def add(a, b):
    return a + b

# 기본값 매개변수
def greet_with_title(name, title="님"):
    return f"안녕하세요, {name}{title}!"

# 여러 리턴값
def calculate(a, b):
    return a + b, a - b, a * b

# 함수 호출
greet()                             # 안녕하세요!
greet_person("김철수")              # 안녕하세요, 김철수님!
result = add(5, 3)                  # 8
message = greet_with_title("이영희") # 안녕하세요, 이영희님!
plus, minus, multiply = calculate(10, 3)  # 13, 7, 30

# ========================================
# 7. 리스트 (List)
# ========================================

# 리스트 생성
numbers = [1, 2, 3, 4, 5]
fruits = ["사과", "바나나", "오렌지"]
mixed = [1, "hello", True, 3.14]

# 인덱싱과 슬라이싱
print(numbers[0])       # 첫 번째: 1
print(numbers[-1])      # 마지막: 5
print(numbers[1:4])     # 슬라이싱: [2, 3, 4]

# 리스트 메서드
fruits.append("포도")           # 추가
fruits.insert(1, "딸기")        # 삽입
fruits.remove("바나나")         # 제거
popped = fruits.pop()          # 마지막 제거 후 리턴

# 리스트 연산
print(len(numbers))            # 길이: 5
print(1 in numbers)            # 포함 여부: True
combined = numbers + [6, 7]    # 연결

# ========================================
# 8. 딕셔너리 (Dictionary)
# ========================================

# 딕셔너리 생성
person = {
    "name": "김철수",
    "age": 25,
    "city": "서울"
}

# 값 접근
print(person["name"])           # 김철수
print(person.get("age"))        # 25

# 값 수정
person["age"] = 26
person["job"] = "개발자"        # 새 키 추가

# 딕셔너리 메서드
keys = person.keys()            # 키 목록
values = person.values()        # 값 목록
items = person.items()          # 키-값 쌍

# 딕셔너리 반복
for key in person:
    print(f"{key}: {person[key]}")

for key, value in person.items():
    print(f"{key}: {value}")

# ========================================
# 9. 튜플 (Tuple)
# ========================================

# 튜플 생성 (수정 불가)
coordinates = (10, 20)
rgb = (255, 128, 0)

# 튜플 언패킹
x, y = coordinates              # x=10, y=20
r, g, b = rgb                   # r=255, g=128, b=0

# ========================================
# 10. 집합 (Set)
# ========================================

# 집합 생성 (중복 제거)
numbers = {1, 2, 3, 3, 4, 5}    # {1, 2, 3, 4, 5}

# 집합 연산
set1 = {1, 2, 3}
set2 = {3, 4, 5}

union = set1 | set2             # 합집합: {1, 2, 3, 4, 5}
intersection = set1 & set2      # 교집합: {3}
difference = set1 - set2        # 차집합: {1, 2}

# ========================================
# 11. 예외 처리
# ========================================

# try-except 기본
try:
    result = 10 / 0
except ZeroDivisionError:
    print("0으로 나눌 수 없습니다!")

# 여러 예외 처리
try:
    number = int(input("숫자 입력: "))
    result = 10 / number
except ValueError:
    print("올바른 숫자를 입력하세요!")
except ZeroDivisionError:
    print("0으로 나눌 수 없습니다!")
finally:
    print("프로그램 종료")

# ========================================
# 12. 파일 처리
# ========================================

# 파일 쓰기
with open("data.txt", "w", encoding="utf-8") as f:
    f.write("안녕하세요!\n")
    f.write("파이썬 학습 중입니다.")

# 파일 읽기
with open("data.txt", "r", encoding="utf-8") as f:
    content = f.read()
    print(content)

# 줄별로 읽기
with open("data.txt", "r", encoding="utf-8") as f:
    for line in f:
        print(line.strip())

# ========================================
# 13. 유용한 내장 함수
# ========================================

numbers = [1, 5, 3, 9, 2]

print(len(numbers))             # 길이: 5
print(max(numbers))             # 최대값: 9
print(min(numbers))             # 최소값: 1
print(sum(numbers))             # 합계: 20
print(sorted(numbers))          # 정렬: [1, 2, 3, 5, 9]

# enumerate - 인덱스와 값을 함께
for i, value in enumerate(numbers):
    print(f"{i}: {value}")

# zip - 여러 리스트를 묶기
names = ["김철수", "이영희", "박민수"]
ages = [25, 23, 27]
for name, age in zip(names, ages):
    print(f"{name}: {age}세")

# ========================================
# 14. 리스트 컴프리헨션
# ========================================

# 기본 형태
squares = [x**2 for x in range(5)]     # [0, 1, 4, 9, 16]

# 조건 포함
evens = [x for x in range(10) if x % 2 == 0]  # [0, 2, 4, 6, 8]

# 문자열 처리
words = ["apple", "banana", "cherry"]
upper_words = [word.upper() for word in words]

# ========================================
# 15. 자주 사용하는 패턴
# ========================================

# 사용자 입력 받기
# name = input("이름을 입력하세요: ")
# age = int(input("나이를 입력하세요: "))

# 랜덤 숫자 생성
import random
random_num = random.randint(1, 100)    # 1~100 사이
random_choice = random.choice(fruits)   # 리스트에서 랜덤 선택

# 현재 시간
from datetime import datetime
now = datetime.now()
print(now.strftime("%Y-%m-%d %H:%M:%S"))

print("\n🎉 파이썬 기초 문법 완료!")
print("이제 실전 프로젝트에 도전해보세요! 🚀")
