use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenizer {
    pub vocab: HashMap<String, usize>,
    pub vocab_size: usize,
    pub max_len: usize,
}

impl Tokenizer {
    /// 새로운 토크나이저 생성
    pub fn new(max_len: usize) -> Self {
        let mut vocab = HashMap::new();
        vocab.insert("<PAD>".to_string(), 0);
        vocab.insert("<UNK>".to_string(), 1);

        Self {
            vocab,
            vocab_size: 2,
            max_len,
        }
    }

    /// 데이터셋으로부터 vocabulary 구축 (빈도 기반 상위 N개만)
    pub fn fit(&mut self, texts: &[String]) {
        use std::collections::HashMap;

        // 1. 단어 빈도 계산
        let mut word_counts: HashMap<String, usize> = HashMap::new();
        for text in texts {
            let words = self.tokenize_text(text);
            for word in words {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }

        let total_words = word_counts.len(); // 이동 전에 길이 저장!

        // 2. 빈도순 정렬
        let mut word_freq: Vec<(String, usize)> = word_counts.into_iter().collect();
        word_freq.sort_by(|a, b| b.1.cmp(&a.1)); // 빈도 높은 순

        // 3. 상위 5000개만 vocabulary에 추가
        let max_vocab = 5000;
        for (word, _count) in word_freq.into_iter().take(max_vocab) {
            if !self.vocab.contains_key(&word) {
                self.vocab.insert(word, self.vocab_size);
                self.vocab_size += 1;
            }
        }

        println!(
            "   ✅ Vocabulary 제한: {} → {}개 (빈도 기반)",
            total_words,
            self.vocab_size - 2
        );
    }

    /// 텍스트를 단어로 분리 (한글 지원 - 공백 기반)
    fn tokenize_text(&self, text: &str) -> Vec<String> {
        // 한글은 대소문자 구분이 없으므로 그대로 사용
        text.split_whitespace().map(|s| s.to_string()).collect()
    }

    /// 텍스트를 인덱스 시퀀스로 변환
    pub fn encode(&self, text: &str) -> Vec<usize> {
        let words = self.tokenize_text(text);
        let mut indices = Vec::new();

        for word in words.iter().take(self.max_len) {
            let idx = self.vocab.get(word).copied().unwrap_or(1); // <UNK> = 1
            indices.push(idx);
        }

        // Padding
        while indices.len() < self.max_len {
            indices.push(0); // <PAD> = 0
        }

        indices
    }

    /// 인덱스 시퀀스를 텍스트로 변환 (디버깅용)
    pub fn decode(&self, indices: &[usize]) -> String {
        let reverse_vocab: HashMap<usize, String> =
            self.vocab.iter().map(|(k, v)| (*v, k.clone())).collect();

        indices
            .iter()
            .filter(|&&idx| idx != 0) // <PAD> 제거
            .filter_map(|&idx| reverse_vocab.get(&idx))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// 텍스트를 임베딩 벡터로 변환 (간단한 one-hot 기반)
    pub fn text_to_embedding(&self, text: &str, embedding_dim: usize) -> Vec<Vec<f32>> {
        let indices = self.encode(text);
        let mut embeddings = Vec::new();

        for &idx in &indices {
            // 간단한 랜덤 임베딩 (실제로는 학습된 임베딩 사용)
            let mut embedding = vec![0.0; embedding_dim];
            if idx > 0 {
                // <PAD>가 아닌 경우
                let seed = idx as f32;
                for i in 0..embedding_dim {
                    embedding[i] = ((seed + i as f32) * 0.1).sin();
                }
            }
            embeddings.push(embedding);
        }

        embeddings
    }
}
