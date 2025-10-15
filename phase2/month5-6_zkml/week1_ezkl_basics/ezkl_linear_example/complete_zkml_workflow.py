#!/usr/bin/env python3
"""
🚀 완전한 ZKML 워크플로우 - EZKL 공식 예제 기반
scikit-learn 선형 회귀 → ZKML 증명 생성 및 검증
"""

import ezkl
import os
import json
import time

def run_complete_zkml():
    """완전한 ZKML 워크플로우 실행"""
    
    print("🔥 완전한 ZKML 워크플로우 시작!")
    print("=" * 50)
    
    # 파일 경로 설정
    model_path = "network.onnx"
    compiled_model_path = "network.ezkl"
    pk_path = "test.pk"
    vk_path = "test.vk"
    proof_path = "test.pf"
    settings_path = "settings.json"
    srs_path = "kzg.srs"
    witness_path = "witness.json"
    input_path = "input.json"
    
    try:
        # 입력 데이터 확인
        with open(input_path, 'r') as f:
            input_data = json.load(f)
        
        print("📊 입력 데이터:")
        print(f"   입력: {input_data['input_data'][0]}")
        print(f"   예상 출력: {input_data['output_data'][0]}")
        
        # 1. 설정 생성
        print("\n1️⃣ 설정 생성 중...")
        ezkl.gen_settings(model_path, settings_path)
        print("✅ 설정 생성 완료")
        
        # 2. 설정 보정
        print("2️⃣ 설정 보정 중...")
        ezkl.calibrate_settings(input_path, model_path, settings_path)
        print("✅ 설정 보정 완료")
        
        # 3. 회로 컴파일
        print("3️⃣ 회로 컴파일 중...")
        ezkl.compile_circuit(model_path, compiled_model_path, settings_path)
        print("✅ 회로 컴파일 완료")
        
        # 4. SRS 생성
        print("4️⃣ SRS 생성 중...")
        with open(settings_path, 'r') as f:
            settings = json.load(f)
        logrows = settings['run_args']['logrows']
        
        # 비동기 문제 해결을 위해 try-except 사용
        try:
            ezkl.get_srs(srs_path, logrows)
            print("✅ SRS 생성 완료")
        except Exception as e:
            print(f"⚠️ SRS 생성 스킵 (비동기 이슈): {e}")
            # 기존 SRS 파일이 있는지 확인
            if not os.path.exists(srs_path):
                print("❌ SRS 파일이 없어서 워크플로우를 중단합니다.")
                return False
        
        # 5. 키 생성 (Setup)
        print("5️⃣ 키 생성 중...")
        try:
            ezkl.setup(compiled_model_path, vk_path, pk_path, srs_path)
            print("✅ 키 생성 완료")
        except Exception as e:
            print(f"⚠️ 키 생성 실패: {e}")
            return False
        
        # 6. Witness 생성
        print("6️⃣ Witness 생성 중...")
        ezkl.gen_witness(input_path, compiled_model_path, witness_path)
        print("✅ Witness 생성 완료")
        
        # 7. 증명 생성
        print("7️⃣ 증명 생성 중...")
        start_time = time.time()
        
        ezkl.prove(witness_path, compiled_model_path, pk_path, proof_path, srs_path)
        
        prove_time = time.time() - start_time
        print(f"✅ 증명 생성 완료 ({prove_time:.2f}초)")
        
        # 8. 증명 검증
        print("8️⃣ 증명 검증 중...")
        start_time = time.time()
        
        result = ezkl.verify(proof_path, settings_path, vk_path, srs_path)
        
        verify_time = time.time() - start_time
        print(f"✅ 증명 검증 완료 ({verify_time:.2f}초)")
        
        # 9. 결과 출력
        print("\n🎯 ZKML 워크플로우 결과:")
        print(f"   📊 모델: scikit-learn 선형 회귀")
        print(f"   🧮 입력: {input_data['input_data'][0]}")
        print(f"   📈 예상 출력: {input_data['output_data'][0]}")
        print(f"   ✅ 증명 검증: {'성공' if result else '실패'}")
        print(f"   ⏱️ 증명 시간: {prove_time:.2f}초")
        print(f"   ⏱️ 검증 시간: {verify_time:.2f}초")
        
        # 10. 파일 크기 정보
        if os.path.exists(proof_path):
            proof_size = os.path.getsize(proof_path)
            print(f"   📦 증명 크기: {proof_size} bytes")
        
        if os.path.exists(model_path):
            model_size = os.path.getsize(model_path)
            print(f"   📦 모델 크기: {model_size} bytes")
        
        print("\n🚀 완전한 ZKML 성공! scikit-learn → ZKML 변환 완료!")
        
        return result
        
    except Exception as e:
        print(f"❌ 오류 발생: {str(e)}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = run_complete_zkml()
    if success:
        print("\n🎉 ZKML 워크플로우 완료!")
        print("🔥 이제 진짜 AI 모델이 ZK 증명으로 검증되었어요!")
    else:
        print("\n💥 ZKML 워크플로우 실패!")
        print("🔧 비동기 이슈나 환경 문제일 가능성이 높아요.")
