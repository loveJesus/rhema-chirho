// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! AI configuration types.

use serde::{Deserialize, Serialize};

/// Which LLM provider to use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmProviderTypeChirho {
    /// OpenAI API (GPT-4, etc.).
    OpenAiChirho,
    /// Anthropic API (Claude, etc.).
    AnthropicChirho,
    /// Local llama.cpp model.
    LocalLlamaChirho,
    /// Mock provider for testing.
    MockChirho,
}

/// Which embedding model to use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingModelTypeChirho {
    /// Local ONNX model (e.g. all-MiniLM-L6-v2, 384 dimensions).
    LocalOnnxChirho,
    /// OpenAI text-embedding API.
    OpenAiChirho,
    /// Mock embeddings for testing.
    MockChirho,
}

/// Complete AI configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfigChirho {
    /// LLM provider type.
    pub llm_provider_chirho: LlmProviderTypeChirho,
    /// LLM model name (e.g. "gpt-4", "claude-sonnet-4-5-20250929").
    pub llm_model_chirho: String,
    /// LLM API key (if using cloud provider).
    pub llm_api_key_chirho: Option<String>,
    /// Embedding model type.
    pub embedding_model_chirho: EmbeddingModelTypeChirho,
    /// Embedding dimension count.
    pub embedding_dim_chirho: usize,
    /// Path to the SQLite vector store database.
    pub vector_store_path_chirho: Option<String>,
    /// Maximum tokens for LLM responses.
    pub max_tokens_chirho: usize,
    /// Temperature for LLM generation (0.0 - 1.0).
    pub temperature_chirho: f32,
}

impl Default for AiConfigChirho {
    fn default() -> Self {
        Self {
            llm_provider_chirho: LlmProviderTypeChirho::MockChirho,
            llm_model_chirho: "mock".to_string(),
            llm_api_key_chirho: None,
            embedding_model_chirho: EmbeddingModelTypeChirho::MockChirho,
            embedding_dim_chirho: 384,
            vector_store_path_chirho: None,
            max_tokens_chirho: 2048,
            temperature_chirho: 0.3,
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_default_config_chirho() {
        let config_chirho = AiConfigChirho::default();
        assert_eq!(config_chirho.embedding_dim_chirho, 384);
        assert_eq!(config_chirho.max_tokens_chirho, 2048);
        assert!(config_chirho.llm_api_key_chirho.is_none());
    }

    #[test]
    fn test_config_serde_roundtrip_chirho() {
        let config_chirho = AiConfigChirho::default();
        let json_chirho = serde_json::to_string(&config_chirho).unwrap();
        let parsed_chirho: AiConfigChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.embedding_dim_chirho, config_chirho.embedding_dim_chirho);
    }
}
