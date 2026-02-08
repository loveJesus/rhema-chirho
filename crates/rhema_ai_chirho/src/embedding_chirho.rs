// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Embedding model abstraction — trait + implementations.

use async_trait::async_trait;

use crate::error_chirho::AiResultChirho;

/// Trait for embedding model implementations.
#[async_trait]
pub trait EmbeddingModelChirho: Send + Sync {
    /// Generate an embedding vector for a single text.
    async fn embed_chirho(&self, text_chirho: &str) -> AiResultChirho<Vec<f32>>;

    /// Generate embeddings for a batch of texts.
    async fn embed_batch_chirho(&self, texts_chirho: &[String]) -> AiResultChirho<Vec<Vec<f32>>>;

    /// Return the embedding dimension.
    fn dimension_chirho(&self) -> usize;

    /// Return the model name.
    fn model_name_chirho(&self) -> &str;
}

/// Compute cosine similarity between two vectors.
pub fn cosine_similarity_chirho(a_chirho: &[f32], b_chirho: &[f32]) -> f32 {
    if a_chirho.len() != b_chirho.len() || a_chirho.is_empty() {
        return 0.0;
    }

    let dot_chirho: f32 = a_chirho
        .iter()
        .zip(b_chirho.iter())
        .map(|(x_chirho, y_chirho)| x_chirho * y_chirho)
        .sum();

    let norm_a_chirho: f32 = a_chirho.iter().map(|x_chirho| x_chirho * x_chirho).sum::<f32>().sqrt();
    let norm_b_chirho: f32 = b_chirho.iter().map(|x_chirho| x_chirho * x_chirho).sum::<f32>().sqrt();

    if norm_a_chirho == 0.0 || norm_b_chirho == 0.0 {
        return 0.0;
    }

    dot_chirho / (norm_a_chirho * norm_b_chirho)
}

/// Mock embedding model for testing — generates deterministic pseudo-embeddings.
pub struct MockEmbeddingModelChirho {
    dim_chirho: usize,
}

impl MockEmbeddingModelChirho {
    pub fn new_chirho(dim_chirho: usize) -> Self {
        Self { dim_chirho }
    }
}

#[async_trait]
impl EmbeddingModelChirho for MockEmbeddingModelChirho {
    async fn embed_chirho(&self, text_chirho: &str) -> AiResultChirho<Vec<f32>> {
        // Generate a deterministic pseudo-embedding based on text hash.
        let hash_chirho = simple_hash_chirho(text_chirho);
        let mut embedding_chirho = vec![0.0f32; self.dim_chirho];
        for (i_chirho, val_chirho) in embedding_chirho.iter_mut().enumerate() {
            // Use hash + index to produce a deterministic float in [-1, 1].
            let seed_chirho = hash_chirho.wrapping_add(i_chirho as u64);
            *val_chirho = ((seed_chirho % 2000) as f32 / 1000.0) - 1.0;
        }
        // Normalize to unit vector.
        let norm_chirho: f32 = embedding_chirho
            .iter()
            .map(|x_chirho| x_chirho * x_chirho)
            .sum::<f32>()
            .sqrt();
        if norm_chirho > 0.0 {
            for v_chirho in &mut embedding_chirho {
                *v_chirho /= norm_chirho;
            }
        }
        Ok(embedding_chirho)
    }

    async fn embed_batch_chirho(&self, texts_chirho: &[String]) -> AiResultChirho<Vec<Vec<f32>>> {
        let mut results_chirho = Vec::with_capacity(texts_chirho.len());
        for text_chirho in texts_chirho {
            results_chirho.push(self.embed_chirho(text_chirho).await?);
        }
        Ok(results_chirho)
    }

    fn dimension_chirho(&self) -> usize {
        self.dim_chirho
    }

    fn model_name_chirho(&self) -> &str {
        "mock-embedding"
    }
}

/// Simple deterministic hash for test embeddings.
fn simple_hash_chirho(s_chirho: &str) -> u64 {
    let mut hash_chirho: u64 = 5381;
    for byte_chirho in s_chirho.bytes() {
        hash_chirho = hash_chirho
            .wrapping_mul(33)
            .wrapping_add(u64::from(byte_chirho));
    }
    hash_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical_chirho() {
        let a_chirho = vec![1.0, 0.0, 0.0];
        let b_chirho = vec![1.0, 0.0, 0.0];
        let sim_chirho = cosine_similarity_chirho(&a_chirho, &b_chirho);
        assert!((sim_chirho - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal_chirho() {
        let a_chirho = vec![1.0, 0.0];
        let b_chirho = vec![0.0, 1.0];
        let sim_chirho = cosine_similarity_chirho(&a_chirho, &b_chirho);
        assert!(sim_chirho.abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite_chirho() {
        let a_chirho = vec![1.0, 0.0];
        let b_chirho = vec![-1.0, 0.0];
        let sim_chirho = cosine_similarity_chirho(&a_chirho, &b_chirho);
        assert!((sim_chirho + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_empty_chirho() {
        let sim_chirho = cosine_similarity_chirho(&[], &[]);
        assert_eq!(sim_chirho, 0.0);
    }

    #[test]
    fn test_cosine_similarity_mismatched_chirho() {
        let a_chirho = vec![1.0, 0.0];
        let b_chirho = vec![1.0];
        let sim_chirho = cosine_similarity_chirho(&a_chirho, &b_chirho);
        assert_eq!(sim_chirho, 0.0);
    }

    #[tokio::test]
    async fn test_mock_embedding_deterministic_chirho() {
        let model_chirho = MockEmbeddingModelChirho::new_chirho(384);
        let e1_chirho = model_chirho.embed_chirho("test text").await.unwrap();
        let e2_chirho = model_chirho.embed_chirho("test text").await.unwrap();
        assert_eq!(e1_chirho, e2_chirho);
        assert_eq!(e1_chirho.len(), 384);
    }

    #[tokio::test]
    async fn test_mock_embedding_normalized_chirho() {
        let model_chirho = MockEmbeddingModelChirho::new_chirho(128);
        let e_chirho = model_chirho.embed_chirho("hello world").await.unwrap();
        let norm_chirho: f32 = e_chirho.iter().map(|x_chirho| x_chirho * x_chirho).sum::<f32>().sqrt();
        assert!((norm_chirho - 1.0).abs() < 1e-4);
    }

    #[tokio::test]
    async fn test_mock_embedding_batch_chirho() {
        let model_chirho = MockEmbeddingModelChirho::new_chirho(64);
        let texts_chirho = vec!["hello".to_string(), "world".to_string()];
        let embeddings_chirho = model_chirho.embed_batch_chirho(&texts_chirho).await.unwrap();
        assert_eq!(embeddings_chirho.len(), 2);
        assert_eq!(embeddings_chirho[0].len(), 64);
    }

    #[tokio::test]
    async fn test_mock_different_texts_different_embeddings_chirho() {
        let model_chirho = MockEmbeddingModelChirho::new_chirho(64);
        let e1_chirho = model_chirho.embed_chirho("love").await.unwrap();
        let e2_chirho = model_chirho.embed_chirho("hate").await.unwrap();
        // Different texts should produce different embeddings.
        assert_ne!(e1_chirho, e2_chirho);
    }
}
