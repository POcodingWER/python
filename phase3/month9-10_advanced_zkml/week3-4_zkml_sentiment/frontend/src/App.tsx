import React, { useState } from 'react';
import './App.css';

interface InferResponse {
  sentiment: number;
  sentiment_label: string;
  probabilities: number[];
  text_hash: string;
}

interface ProveResponse {
  proof: any;
}

interface VerifyResponse {
  valid: boolean;
  message: string;
}

const App: React.FC = () => {
  const [text, setText] = useState<string>('');
  const [result, setResult] = useState<InferResponse | null>(null);
  const [proof, setProof] = useState<any>(null);
  const [verifyResult, setVerifyResult] = useState<VerifyResponse | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [proving, setProving] = useState<boolean>(false);

  const API_URL = 'http://localhost:8080';

  // 감성 분석 실행
  const handleAnalyze = async () => {
    if (!text.trim()) {
      alert('텍스트를 입력해주세요!');
      return;
    }

    setLoading(true);
    setResult(null);
    setProof(null);
    setVerifyResult(null);

    try {
      const response = await fetch(`${API_URL}/api/infer`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text }),
      });

      if (!response.ok) throw new Error('API 요청 실패');

      const data: InferResponse = await response.json();
      setResult(data);
    } catch (error) {
      console.error('Error:', error);
      alert('감성 분석 실패! 서버가 실행 중인지 확인해주세요.');
    } finally {
      setLoading(false);
    }
  };

  // ZK 증명 생성
  const handleProve = async () => {
    if (!result) {
      alert('먼저 감성 분석을 실행해주세요!');
      return;
    }

    setProving(true);
    setVerifyResult(null);

    try {
      const response = await fetch(`${API_URL}/api/prove`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text }),
      });

      if (!response.ok) throw new Error('증명 생성 실패');

      const data: ProveResponse = await response.json();
      setProof(data.proof);

      // 자동으로 검증 실행
      await handleVerify(data.proof);
    } catch (error) {
      console.error('Error:', error);
      alert('ZK 증명 생성 실패!');
    } finally {
      setProving(false);
    }
  };

  // ZK 증명 검증
  const handleVerify = async (proofToVerify?: any) => {
    const targetProof = proofToVerify || proof;
    if (!targetProof) {
      alert('증명이 없습니다!');
      return;
    }

    try {
      const response = await fetch(`${API_URL}/api/verify`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ proof: targetProof }),
      });

      if (!response.ok) throw new Error('검증 실패');

      const data: VerifyResponse = await response.json();
      setVerifyResult(data);
    } catch (error) {
      console.error('Error:', error);
      alert('ZK 증명 검증 실패!');
    }
  };

  // 샘플 텍스트
  const sampleTexts = [
    '이 영화 정말 재밌어요 최고예요',
    '완전 쓰레기 영화 시간 낭비',
    '배우들 연기가 너무 좋았어요',
    '스토리가 엉망이고 지루해요',
  ];

  return (
    <div className="app">
      <div className="container">
        <header className="header">
          <h1>🤖 ZKML 한국어 감성 분석</h1>
          <p>Zero-Knowledge Machine Learning으로 감성을 분석합니다</p>
        </header>

        <div className="main-content">
          {/* 입력 영역 */}
          <div className="input-section">
            <h2>📝 텍스트 입력</h2>
            <textarea
              className="text-input"
              placeholder="영화 리뷰나 감상평을 입력해주세요..."
              value={text}
              onChange={(e) => setText(e.target.value)}
              rows={5}
            />

            <div className="sample-buttons">
              <p>샘플 텍스트:</p>
              {sampleTexts.map((sample, idx) => (
                <button
                  key={idx}
                  className="sample-btn"
                  onClick={() => setText(sample)}
                >
                  {sample}
                </button>
              ))}
            </div>

            <button
              className="analyze-btn"
              onClick={handleAnalyze}
              disabled={loading}
            >
              {loading ? '분석 중...' : '🔍 감성 분석 실행'}
            </button>
          </div>

          {/* 결과 영역 */}
          {result && (
            <div className="result-section">
              <h2>📊 분석 결과</h2>
              <div className={`sentiment-result ${result.sentiment === 1 ? 'positive' : 'negative'}`}>
                <div className="sentiment-icon">
                  {result.sentiment === 1 ? '😊' : '😞'}
                </div>
                <div className="sentiment-label">
                  {result.sentiment_label}
                </div>
              </div>

              <div className="probabilities">
                <div className="prob-bar">
                  <span>부정 (Negative)</span>
                  <div className="bar">
                    <div
                      className="bar-fill negative"
                      style={{ width: `${result.probabilities[0] * 100}%` }}
                    />
                  </div>
                  <span>{(result.probabilities[0] * 100).toFixed(2)}%</span>
                </div>
                <div className="prob-bar">
                  <span>긍정 (Positive)</span>
                  <div className="bar">
                    <div
                      className="bar-fill positive"
                      style={{ width: `${result.probabilities[1] * 100}%` }}
                    />
                  </div>
                  <span>{(result.probabilities[1] * 100).toFixed(2)}%</span>
                </div>
              </div>

              <button
                className="prove-btn"
                onClick={handleProve}
                disabled={proving}
              >
                {proving ? '증명 생성 중...' : '🔐 ZK 증명 생성'}
              </button>
            </div>
          )}

          {/* ZK 증명 결과 */}
          {proof && (
            <div className="proof-section">
              <h2>🔐 Zero-Knowledge Proof</h2>
              <div className="proof-info">
                <div className="info-row">
                  <span className="label">예측 클래스:</span>
                  <span className="value">{proof.predicted_sentiment}</span>
                </div>
                <div className="info-row">
                  <span className="label">Text Hash:</span>
                  <span className="value hash">{proof.text_hash.substring(0, 16)}...</span>
                </div>
                <div className="info-row">
                  <span className="label">Model Hash:</span>
                  <span className="value hash">{proof.model_hash.substring(0, 16)}...</span>
                </div>
                <div className="info-row">
                  <span className="label">Halo2 Proof:</span>
                  <span className="value">
                    {proof.halo2_proof ? '✅ 생성됨' : '❌ 없음'}
                  </span>
                </div>
              </div>

              {verifyResult && (
                <div className={`verify-result ${verifyResult.valid ? 'valid' : 'invalid'}`}>
                  <div className="verify-icon">
                    {verifyResult.valid ? '✅' : '❌'}
                  </div>
                  <div className="verify-message">
                    {verifyResult.message}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        <footer className="footer">
          <p>Powered by Halo2 Zero-Knowledge Proofs 🔐</p>
          <p>Bag-of-Words + Dense Network (Vocab: 5000)</p>
        </footer>
      </div>
    </div>
  );
};

export default App;

