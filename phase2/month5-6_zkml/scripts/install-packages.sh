#!/bin/bash

# 가상환경 절대경로로 활성화
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

source "$PROJECT_DIR/zkml_env/bin/activate"

echo "3️⃣ 필수 패키지 설치 중..."

"$PROJECT_DIR/zkml_env/bin/pip" install --upgrade pip --quiet

if [ -f "$PROJECT_DIR/requirements.txt" ]; then
    "$PROJECT_DIR/zkml_env/bin/pip" install -r "$PROJECT_DIR/requirements.txt"
    echo "✅ requirements.txt에서 패키지 설치 완료!"
else
    # 기본 패키지 먼저 설치
    echo "📦 기본 패키지 설치 중..."
    "$PROJECT_DIR/zkml_env/bin/pip" install numpy
    
    # PyTorch 설치 (CPU 버전)
    echo "🔥 PyTorch 설치 중..."
    "$PROJECT_DIR/zkml_env/bin/pip" install torch torchvision --index-url https://download.pytorch.org/whl/cpu
    
    # ONNX 바이너리 설치
    echo "📊 ONNX 설치 중..."
    "$PROJECT_DIR/zkml_env/bin/pip" install --only-binary :all: onnx onnxruntime
    
    # ONNX 관련 추가 패키지
    echo "📝 ONNX Script 설치 중..."
    "$PROJECT_DIR/zkml_env/bin/pip" install onnxscript
    
    # 나머지 패키지
    echo "🛠️  추가 패키지 설치 중..."
    "$PROJECT_DIR/zkml_env/bin/pip" install ezkl scikit-learn matplotlib jupyter hummingbird-ml
    
    echo "✅ 필수 패키지 설치 완료!"
fi

echo ""
echo "설치된 패키지:"
echo "  - ezkl (ZKML 프레임워크)"
echo "  - torch (PyTorch)"
echo "  - onnx + onnxscript (모델 변환)"
echo "  - jupyter (노트북)"

