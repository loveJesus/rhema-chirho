// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the AI integration crate.

use thiserror::Error;

/// Errors arising from AI operations.
#[derive(Debug, Error)]
pub enum AiErrorChirho {
    #[error("LLM provider error: {reason_chirho}")]
    LlmProviderChirho { reason_chirho: String },

    #[error("Embedding error: {reason_chirho}")]
    EmbeddingChirho { reason_chirho: String },

    #[error("Vector store error: {reason_chirho}")]
    VectorStoreChirho { reason_chirho: String },

    #[error("Query expansion error: {reason_chirho}")]
    QueryExpansionChirho { reason_chirho: String },

    #[error("Configuration error: {reason_chirho}")]
    ConfigChirho { reason_chirho: String },

    #[error("SQLite error: {0}")]
    SqliteChirho(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    JsonChirho(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoChirho(#[from] std::io::Error),
}

/// Result type alias for AI operations.
pub type AiResultChirho<T> = Result<T, AiErrorChirho>;
