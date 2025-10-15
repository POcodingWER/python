#!/usr/bin/env python3
"""
🚀 ZKML 워크플로우 실행기
EZKL을 사용한 완전한 ZK 증명 생성 및 검증
"""

import ezkl
import os
import json
import time

def run_zkml_workflow():
    """EZKL 워크플로우 실행"""
    
    print("🔥 ZKML 워크플로우 시작!")
    print("=" * 50)
    
    model_path = "simple_linear.onnx"
    compiled_model_path = "network.ezkl"
    pk_path = "test.pk"
    vk_path = "test.vk"
    proof_path = "test.pf"
    settings_path = "settings.json"
    srs_path = "kzg.srs"
    witness_path = "witness.json"
    
    try:
        # 1. 설정 생성 및 보정
        print("1️⃣ 설정 생성 중...")
        ezkl.gen_settings(model_path, settings_path)
        print("✅ 설정 생성 완료")
        
        print("2️⃣ 설정 보정 중...")
        ezkl.calibrate_settings(
            "input.json", 
            model_path, 
            settings_path
        )
        print("✅ 설정 보정 완료")
        
        # 2. 회로 컴파일
        print("3️⃣ 회로 컴파일 중...")
        ezkl.compile_circuit(model_path, compiled_model_path, settings_path)
        print("✅ 회로 컴파일 완료")
        
        # 3. SRS (Structured Reference String) 생성
        print("4️⃣ SRS 생성 중...")
        # 설정에서 logrows 값을 읽어서 사용
        with open(settings_path, 'r') as f:
            settings = json.load(f)
        logrows = settings['run_args']['logrows']
        ezkl.get_srs(srs_path, logrows)
        print("✅ SRS 생성 완료")
        
        # 4. 키 생성 (Setup)
        print("5️⃣ 키 생성 중...")
        ezkl.setup(
            compiled_model_path,
            vk_path,
            pk_path,
            srs_path,
        )
        print("✅ 키 생성 완료")
        
        # 5. Witness 생성
        print("6️⃣ Witness 생성 중...")
        ezkl.gen_witness(
            "input.json",
            compiled_model_path,
            witness_path
        )
        print("✅ Witness 생성 완료")
        
        # 6. 증명 생성
        print("7️⃣ 증명 생성 중...")
        start_time = time.time()
        
        ezkl.prove(
            witness_path,
            compiled_model_path,
            pk_path,
            proof_path,
            srs_path,
        )
        
        prove_time = time.time() - start_time
        print(f"✅ 증명 생성 완료 ({prove_time:.2f}초)")
        
        # 7. 증명 검증
        print("8️⃣ 증명 검증 중...")
        start_time = time.time()
        
        result = ezkl.verify(
            proof_path,
            settings_path,
            vk_path,
            srs_path,
        )
        
        verify_time = time.time() - start_time
        print(f"✅ 증명 검증 완료 ({verify_time:.2f}초)")
        
        # 8. 결과 출력
        print("\n🎯 ZKML 워크플로우 결과:")
        print(f"   📊 모델: y = 2x + 1")
        print(f"   🧮 입력: x = 3.0")
        print(f"   📈 예상 출력: y = 7.0")
        print(f"   ✅ 증명 검증: {'성공' if result else '실패'}")
        print(f"   ⏱️ 증명 시간: {prove_time:.2f}초")
        print(f"   ⏱️ 검증 시간: {verify_time:.2f}초")
        
        # 9. 파일 크기 정보
        if os.path.exists(proof_path):
            proof_size = os.path.getsize(proof_path)
            print(f"   📦 증명 크기: {proof_size} bytes")
        
        print("\n🚀 첫 ZKML 성공! 선형 회귀 모델이 ZK 회로로 변환되었어요!")
        
        return result
        
    except Exception as e:
        print(f"❌ 오류 발생: {str(e)}")
        return False

if __name__ == "__main__":
    success = run_zkml_workflow()
    if success:
        print("\n🎉 ZKML 워크플로우 완료!")
    else:
        print("\n💥 ZKML 워크플로우 실패!")
