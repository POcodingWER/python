# EZKL - 쉬운 영지식 추론 🧠🔐

> **원본 저장소**: https://github.com/zkonduit/ezkl  
> **라이센스**: 상업적 사용 시 Zkonduit Inc. 문의 필요  
> **번역일**: 2025년 10월 15일

<h1 align="center">
	<br>
	 💭
	<br>
	<br>
EZKL
	<br>
	<br>
	<br>
</h1>

> 쉬운 영지식 추론 (Easy Zero-Knowledge Inference)

[![Test](https://github.com/zkonduit/ezkl/workflows/Rust/badge.svg)](https://github.com/zkonduit/ezkl/actions?query=workflow%3ARust)

## 📋 개요

`ezkl`은 딥러닝 모델과 기타 계산 그래프를 ZK-SNARK(ZKML)로 추론하기 위한 라이브러리 및 명령줄 도구입니다.

### 🔄 워크플로우

1. **계산 그래프 정의**: PyTorch나 TensorFlow에서 신경망 등 임의의 연산 집합을 정의합니다.
2. **ONNX 내보내기**: 최종 연산 그래프를 [.onnx](https://onnx.ai/) 파일로, 샘플 입력을 `.json` 파일로 내보냅니다.
3. **ZK 회로 생성**: `ezkl`이 `.onnx`와 `.json` 파일을 사용하여 다음과 같은 증명이 가능한 ZK-SNARK 회로를 생성합니다:

### 🎯 증명 가능한 명제들

> **"공개된 신경망을 개인 데이터로 실행했고 이 결과를 얻었습니다"**

[![Notebook](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_public_network_output.ipynb)

> **"개인 신경망을 공개 데이터로 실행했고 이 결과를 얻었습니다"**

[![Notebook](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_public_input_output.ipynb)

> **"공개된 신경망을 공개 데이터로 올바르게 실행했고 이 결과를 얻었습니다"**

[![Notebook](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb)

백엔드에서는 공동 개발된 [Halo2](https://github.com/privacy-scaling-explorations/halo2)를 증명 시스템으로 사용합니다.

생성된 증명은 온체인(이더리움 가상 머신), 브라우저, 또는 디바이스에서 훨씬 적은 계산 자원으로 검증할 수 있습니다.

### 🤝 커뮤니티

- 질문이 있으시면 [Discussions](https://github.com/zkonduit/ezkl/discussions)에 토론 주제를 열어주세요. 또는 ✨[EZKL 커뮤니티 텔레그램 그룹](https://t.me/+QRzaRvTPIthlYWMx)💫에 참여하세요.

- 더 자세한 기술 문서는 [블로그](https://blog.ezkl.xyz/)를 확인하세요.

- ezkl로 무엇을 만들 수 있는지 보려면 [cryptoidol.tech](https://cryptoidol.tech/)를 확인하세요. 여기서 ezkl은 당신의 노래를 영원히 평가하는 AI를 만드는 데 사용됩니다.

---

## 🚀 시작하기 ⚙️

가장 쉬운 시작 방법은 노트북을 사용해보는 것입니다.

### 🐍 Python 설치

Python 바인딩을 설치하려면 다음을 실행하세요:

```bash
pip install ezkl
```

GPU 버전의 경우:

```bash
pip install ezkl-gpu
```

**Google Colab 예제**: 신경망을 훈련하고 온체인 추론 검증기를 배포하여 다른 스마트 컨트랙트에서 사용하는 방법을 학습하세요. [![Notebook](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/ezkl_demo.ipynb)

더 많은 노트북 튜토리얼은 `examples/notebooks`에서 찾을 수 있습니다.

### 💻 CLI 설치

CLI를 설치하려면:

```shell
curl https://raw.githubusercontent.com/zkonduit/ezkl/main/install_ezkl_cli.sh | bash
```

자세한 내용은 [문서](https://docs.ezkl.xyz)를 참조하세요. CLI는 오버헤드가 적어 Python보다 빠릅니다. 더 빠른 속도와 편의성을 원한다면 튜닝된 클러스터로 지원되는 [원격 증명 서비스](https://ei40vx5x6j0.typeform.com/to/sFv1oxvb)를 확인하세요.

자동 생성된 Rust 문서를 빌드하고 브라우저에서 로컬로 열려면: `cargo doc --open`

## 🔨 프로젝트 빌드

### 🦀 Rust CLI

소스에서 라이브러리를 설치할 수 있습니다:

```bash
cargo install --locked --path .
```

`ezkl`은 이제 solc 설치를 자동으로 관리합니다.

### 🐍 Python 바인딩 빌드

Python 바인딩이 존재하며 `maturin`을 사용하여 빌드할 수 있습니다. `rust`와 `cargo`가 설치되어 있어야 합니다.

```bash
python -m venv .env
source .env/bin/activate
pip install -r requirements.txt
maturin develop --release --features python-bindings
# 튜토리얼 전용 의존성
pip install torch pandas numpy seaborn jupyter onnx kaggle py-solc-x web3 librosa tensorflow keras tf2onnx
```

## 🚀 GPU 가속

NVIDIA GPU에 액세스할 수 있다면 `icicle` 기능으로 빌드하고 다음 환경 변수를 설정하여 가속을 활성화할 수 있습니다:

```sh
export ENABLE_ICICLE_GPU=true
```

GPU 가속은 [Icicle](https://github.com/ingonyama-zk/icicle)에서 제공됩니다.

CPU로 다시 실행하려면 이전 환경 변수를 false 값으로 전환하는 대신 **설정 해제**해야 합니다:

```sh
unset ENABLE_ICICLE_GPU
```

**참고**: 위 환경 변수가 설정되어 있어도 k <= 8인 회로에서는 icicle이 비활성화됩니다. icicle이 활성화되는 `k` 값을 변경하려면 `ICICLE_SMALL_K` 환경 변수를 설정할 수 있습니다.

## 🌍 기여하기

기여에 관심이 있지만 어디서 시작해야 할지 확실하지 않다면 관리자 중 한 명에게 연락하세요:

- dante (alexander-camuto)
- jason (jasonmorton)

더 광범위하게는:

- 기여 방법에 대한 아이디어는 현재 열린 이슈를 참조하세요.

- PR의 경우 [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) 명명 규칙을 사용합니다.

- 버그를 보고하거나 새로운 기능을 요청하려면 [Issues 내에서 새 이슈를 생성](https://github.com/zkonduit/ezkl/issues)하여 더 큰 커뮤니티에 알리세요.

귀하가 의도적으로 작업에 포함하기 위해 제출한 모든 기여는 [CLA](https://github.com/zkonduit/ezkl/blob/main/cla.md)에 명시된 조건에 따라 Zkonduit Inc.에 라이센스가 부여되며, 기여를 의도적으로 제출함으로써 이에 동의하게 됩니다. 특히, 귀하는 기여를 제출할 권리가 있으며 저희가 이를 배포할 수 있습니다.

---

## 📚 한국어 사용자를 위한 추가 정보

### ⚠️ 라이센스 주의사항

- 이 프로젝트는 명시적인 오픈소스 라이센스가 없습니다
- 상업적 사용 전에 Zkonduit Inc.에 문의하는 것을 권장합니다
- 학습 및 연구 목적으로는 자유롭게 사용 가능합니다

### 🎯 ZKML 학습 순서

1. **기초**: `examples/notebooks/simple_demo.ipynb`부터 시작
2. **실습**: 선형 회귀, CNN 등 단계적 학습
3. **응용**: 실제 프로젝트에 적용

### 🔧 한국어 커뮤니티

- 한국 ZKML 개발자들과의 교류를 원한다면 관련 커뮤니티를 찾아보세요
- 이 번역본은 학습 목적으로 제작되었습니다

**🚀 즐거운 ZKML 학습 되세요!** 🧠🔐
