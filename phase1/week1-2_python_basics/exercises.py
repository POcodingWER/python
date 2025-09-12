# 📝 파이썬 기초 연습문제와 해답

"""
파이썬 기초 학습을 위한 연습문제 모음입니다.
각 문제를 차례대로 해결해보세요!
"""

print("🎯 파이썬 기초 연습문제")
print("="*50)

# 문제 1: 기본 변수와 출력
print("\n📌 문제 1: 자기소개 프로그램")
print("이름, 나이, 취미를 변수에 저장하고 멋진 자기소개를 출력하세요.")

# 해답 1
name = "김파이썬"
age = 20
hobby = "코딩"

print(f"""
안녕하세요! 🙋‍♂️
제 이름은 {name}이고, {age}살입니다.
제 취미는 {hobby}입니다.
파이썬을 열심히 배우고 있어요! 🐍
""")

# 문제 2: 조건문 활용
print("\n📌 문제 2: 성적 등급 계산기")
print("점수를 입력받아 등급을 출력하세요.")
print("90점 이상: A, 80점 이상: B, 70점 이상: C, 나머지: F")

# 해답 2
def get_grade(score):
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    else:
        return "F"

# 테스트
test_scores = [95, 85, 75, 60]
for score in test_scores:
    grade = get_grade(score)
    print(f"점수: {score}점 → 등급: {grade}")

# 문제 3: 반복문 활용
print("\n📌 문제 3: 구구단 출력")
print("원하는 단의 구구단을 출력하세요.")

# 해답 3
def print_multiplication_table(dan):
    print(f"\n{dan}단:")
    for i in range(1, 10):
        result = dan * i
        print(f"{dan} × {i} = {result}")

# 테스트
print_multiplication_table(7)

# 문제 4: 리스트 활용
print("\n📌 문제 4: 숫자 리스트 통계")
print("숫자 리스트의 합계, 평균, 최대값, 최소값을 구하세요.")

# 해답 4
def analyze_numbers(numbers):
    total = sum(numbers)
    average = total / len(numbers)
    maximum = max(numbers)
    minimum = min(numbers)
    
    print(f"숫자 리스트: {numbers}")
    print(f"합계: {total}")
    print(f"평균: {average:.2f}")
    print(f"최대값: {maximum}")
    print(f"최소값: {minimum}")

# 테스트
test_numbers = [10, 25, 30, 15, 40, 35, 20]
analyze_numbers(test_numbers)

# 문제 5: 딕셔너리 활용
print("\n📌 문제 5: 학생 성적 관리")
print("학생들의 성적을 딕셔너리로 관리하고 통계를 내세요.")

# 해답 5
students = {
    "김철수": [90, 85, 88],
    "이영희": [92, 90, 89],
    "박민수": [78, 82, 80],
    "최지은": [95, 93, 94]
}

print("\n학생별 성적 현황:")
for name, scores in students.items():
    average = sum(scores) / len(scores)
    print(f"{name}: {scores} → 평균: {average:.1f}점")

# 문제 6: 함수 활용
print("\n📌 문제 6: 소수 판별 프로그램")
print("입력받은 숫자가 소수인지 판별하는 함수를 만드세요.")

# 해답 6
def is_prime(n):
    """소수 판별 함수"""
    if n < 2:
        return False
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            return False
    return True

# 테스트
test_numbers = [2, 3, 4, 17, 25, 29]
print("\n소수 판별 결과:")
for num in test_numbers:
    result = "소수" if is_prime(num) else "소수가 아님"
    print(f"{num}: {result}")

# 문제 7: 종합 문제 - 단어 게임
print("\n📌 문제 7: 끝말잇기 게임")
print("간단한 끝말잇기 게임을 만들어보세요.")

# 해답 7
def word_chain_game():
    """끝말잇기 게임"""
    words = []
    
    print("\n🎮 끝말잇기 게임 시작!")
    print("'종료'를 입력하면 게임이 끝납니다.")
    
    while True:
        if not words:
            word = input("첫 번째 단어를 입력하세요: ")
        else:
            last_char = words[-1][-1]
            word = input(f"'{last_char}'로 시작하는 단어를 입력하세요: ")
        
        if word == "종료":
            break
        
        if words and word[0] != words[-1][-1]:
            print("❌ 끝말이 맞지 않습니다!")
            continue
        
        if word in words:
            print("❌ 이미 사용한 단어입니다!")
            continue
        
        words.append(word)
        print(f"✅ 좋습니다! 현재 단어들: {' → '.join(words)}")
    
    print(f"\n게임 종료! 총 {len(words)}개의 단어를 사용했습니다.")
    print(f"사용한 단어들: {' → '.join(words)}")

# 게임 실행 (주석 해제해서 플레이)
print("\n🎮 끝말잇기 게임을 시작하려면 아래 주석을 해제하세요!")
print("# word_chain_game()")
word_chain_game()  # ← 이 줄의 # 를 제거하면 게임 시작!

print("\n🎉 모든 연습문제를 완료했습니다!")
print("파이썬 기초를 마스터하셨네요! 👏")
