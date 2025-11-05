import React, { useState, useRef, useEffect } from 'react';
import './App.css';

// 🎨 타입 정의
interface InferResponse {
  predicted_class: number;
  confidence: number;
}

interface ProveResponse {
  predicted_class: number;
  proof_path: string;
  halo2_proof_size: number;
}

interface VerifyResponse {
  valid: boolean;
  predicted_class: number | null;
  message: string;
}

const API_BASE = 'http://localhost:8080';

function App() {
  // 🎨 State
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isDrawing, setIsDrawing] = useState(false);
  const [lastPos, setLastPos] = useState<{ x: number; y: number } | null>(null);
  const [prediction, setPrediction] = useState<number | null>(null);
  const [confidence, setConfidence] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);
  const [proofStatus, setProofStatus] = useState<string>('');
  const [verifyStatus, setVerifyStatus] = useState<string>('');
  const [proofSize, setProofSize] = useState<number>(0);

  // 🎨 Canvas 초기화
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // 배경 흰색으로
    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }, []);

  // 🎨 그리기 시작
  const startDrawing = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    setIsDrawing(true);
    setLastPos({ x, y });

    // 첫 점 찍기
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.fillStyle = 'black';
    ctx.beginPath();
    ctx.arc(x, y, 12, 0, Math.PI * 2);
    ctx.fill();
  };

  // 🎨 그리기 (부드러운 선)
  const draw = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDrawing || !lastPos) return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // 선 그리기 (부드럽게)
    ctx.strokeStyle = 'black';
    ctx.lineWidth = 24;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    ctx.beginPath();
    ctx.moveTo(lastPos.x, lastPos.y);
    ctx.lineTo(x, y);
    ctx.stroke();

    setLastPos({ x, y });
  };

  // 🎨 그리기 종료
  const stopDrawing = () => {
    setIsDrawing(false);
    setLastPos(null);
  };

  // 🎨 Canvas 초기화
  const clearCanvas = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // 상태 초기화
    setPrediction(null);
    setConfidence(null);
    setProofStatus('');
    setVerifyStatus('');
    setProofSize(0);
  };

  // 🎨 Canvas → 28x28 이미지 데이터 변환
  const getImageData = (): number[] => {
    const canvas = canvasRef.current;
    if (!canvas) return [];

    const ctx = canvas.getContext('2d');
    if (!ctx) return [];

    // 280x280 → 28x28로 다운샘플링
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const pixels: number[] = [];

    for (let y = 0; y < 28; y++) {
      for (let x = 0; x < 28; x++) {
        // 10x10 블록의 평균값 계산
        let sum = 0;
        for (let dy = 0; dy < 10; dy++) {
          for (let dx = 0; dx < 10; dx++) {
            const sx = x * 10 + dx;
            const sy = y * 10 + dy;
            const idx = (sy * canvas.width + sx) * 4;
            // RGB 평균 (흑백이므로 R만 사용)
            sum += 255 - imageData.data[idx]; // 반전 (흰 배경 → 검은 배경)
          }
        }
        pixels.push(sum / 100 / 255); // 0~1로 정규화
      }
    }

    return pixels;
  };

  // 🔥 AI 추론
  const handleInfer = async () => {
    setLoading(true);
    setPrediction(null);
    setConfidence(null);

    try {
      const imageData = getImageData();
      const response = await fetch(`${API_BASE}/api/infer`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ image: imageData }),
      });

      const data: InferResponse = await response.json();
      setPrediction(data.predicted_class);
      setConfidence(data.confidence);
    } catch (error) {
      console.error('추론 실패:', error);
      alert('추론 실패! 서버가 실행 중인지 확인하세요.');
    } finally {
      setLoading(false);
    }
  };

  // 🔐 ZK 증명 생성
  const handleProve = async () => {
    setLoading(true);
    setProofStatus('🔐 ZK 증명 생성 중...');

    try {
      const imageData = getImageData();
      const response = await fetch(`${API_BASE}/api/prove`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ image: imageData }),
      });

      const data: ProveResponse = await response.json();
      setPrediction(data.predicted_class);
      setProofSize(data.halo2_proof_size);
      setProofStatus(`✅ 증명 생성 완료! (${data.halo2_proof_size} bytes Halo2 proof)`);

      // 자동으로 검증
      await handleVerify();
    } catch (error) {
      console.error('증명 생성 실패:', error);
      setProofStatus('❌ 증명 생성 실패!');
    } finally {
      setLoading(false);
    }
  };

  // ✅ ZK 증명 검증
  const handleVerify = async () => {
    setVerifyStatus('✅ ZK 증명 검증 중...');

    try {
      const response = await fetch(`${API_BASE}/api/verify`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ proof_path: 'proofs/proof.json' }),
      });

      const data: VerifyResponse = await response.json();
      setVerifyStatus(data.message);
    } catch (error) {
      console.error('검증 실패:', error);
      setVerifyStatus('❌ 검증 실패!');
    }
  };

  // 🎲 랜덤 MNIST 이미지 로드
  const handleLoadRandom = async () => {
    setLoading(true);

    try {
      const response = await fetch(`${API_BASE}/api/random`);
      const data = await response.json();

      // Canvas에 그리기
      const canvas = canvasRef.current;
      if (!canvas) return;

      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      // 배경 흰색
      ctx.fillStyle = 'white';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // 28x28 이미지를 280x280으로 확대
      for (let y = 0; y < 28; y++) {
        for (let x = 0; x < 28; x++) {
          const value = data.image[y * 28 + x];
          const gray = Math.floor((1 - value) * 255); // 반전
          ctx.fillStyle = `rgb(${gray}, ${gray}, ${gray})`;
          ctx.fillRect(x * 10, y * 10, 10, 10);
        }
      }

      // 상태 초기화
      setPrediction(null);
      setConfidence(null);
      setProofStatus('');
      setVerifyStatus('');
    } catch (error) {
      console.error('랜덤 이미지 로드 실패:', error);
      alert('랜덤 이미지 로드 실패!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>🧠 ZKML 이미지 분류기</h1>
        <p className="subtitle">Zero-Knowledge Machine Learning with Halo2</p>
      </header>

      <main className="App-main">
        {/* Canvas 그림판 */}
        <section className="canvas-section">
          <h2>✏️ 숫자를 그려보세요 (0-9)</h2>
          <canvas
            ref={canvasRef}
            width={280}
            height={280}
            onMouseDown={startDrawing}
            onMouseMove={draw}
            onMouseUp={stopDrawing}
            onMouseLeave={stopDrawing}
            className="drawing-canvas"
          />
          <div className="button-group">
            <button onClick={clearCanvas} className="btn btn-secondary">
              🗑️ 지우기
            </button>
            <button onClick={handleLoadRandom} className="btn btn-secondary" disabled={loading}>
              🎲 랜덤 MNIST
            </button>
          </div>
        </section>

        {/* AI 추론 */}
        <section className="inference-section">
          <h2>🤖 AI 추론</h2>
          <button onClick={handleInfer} className="btn btn-primary" disabled={loading}>
            {loading ? '⏳ 추론 중...' : '🚀 예측하기'}
          </button>

          {prediction !== null && (
            <div className="result-box">
              <div className="prediction-result">
                <span className="prediction-label">예측:</span>
                <span className="prediction-value">{prediction}</span>
              </div>
              {confidence !== null && (
                <div className="confidence">
                  신뢰도: {(confidence * 100).toFixed(1)}%
                </div>
              )}
            </div>
          )}
        </section>

        {/* ZK 증명 */}
        <section className="proof-section">
          <h2>🔐 Zero-Knowledge 증명</h2>
          <button onClick={handleProve} className="btn btn-accent" disabled={loading}>
            {loading ? '⏳ 증명 생성 중...' : '🔒 ZK 증명 생성'}
          </button>

          {proofStatus && (
            <div className="status-box">
              <p>{proofStatus}</p>
              {proofSize > 0 && (
                <div className="proof-details">
                  <span className="proof-badge">Halo2 Proof</span>
                  <span className="proof-size">{proofSize} bytes</span>
                </div>
              )}
            </div>
          )}

          {verifyStatus && (
            <div className={`status-box ${verifyStatus.includes('✅') ? 'success' : 'error'}`}>
              <p>{verifyStatus}</p>
            </div>
          )}
        </section>

        {/* 설명 */}
        <section className="info-section">
          <h3>💡 이 프로젝트는?</h3>
          <ul>
            <li>✅ <strong>Rust</strong>로 구현한 순수 신경망 (784→256→128→10)</li>
            <li>✅ <strong>Halo2</strong> ZK-SNARK로 ML 추론 증명</li>
            <li>✅ <strong>MNIST</strong> 손글씨 숫자 분류 (~98% 정확도)</li>
            <li>✅ <strong>영지식 증명</strong>: 입력 데이터를 공개하지 않고 올바른 계산 증명</li>
          </ul>
        </section>
      </main>

      <footer className="App-footer">
        <p>
          Made with 🔥 by <strong>ZKML Portfolio Project</strong>
        </p>
        <p className="tech-stack">
          React + TypeScript + Rust + Actix-web + Halo2
        </p>
      </footer>
    </div>
  );
}

export default App;
