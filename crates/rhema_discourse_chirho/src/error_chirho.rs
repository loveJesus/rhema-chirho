// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the discourse analysis crate.

use thiserror::Error;

/// Errors arising from discourse analysis operations.
#[derive(Debug, Error)]
pub enum DiscourseErrorChirho {
    #[error("Storage error: {reason_chirho}")]
    StorageChirho { reason_chirho: String },

    #[error("Validation error: {reason_chirho}")]
    ValidationChirho { reason_chirho: String },

    #[error("Proposition not found: {id_chirho}")]
    PropositionNotFoundChirho { id_chirho: i64 },

    #[error("Arc not found: {id_chirho}")]
    ArcNotFoundChirho { id_chirho: i64 },

    #[error("LLM generation error: {reason_chirho}")]
    LlmGenerationChirho { reason_chirho: String },

    #[error("Import/export error: {reason_chirho}")]
    ImportExportChirho { reason_chirho: String },

    #[error("SQLite error: {0}")]
    SqliteChirho(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    JsonChirho(#[from] serde_json::Error),

    #[error("AI error: {0}")]
    AiChirho(#[from] rhema_ai_chirho::AiErrorChirho),
}

/// Result type alias for discourse operations.
pub type DiscourseResultChirho<T> = Result<T, DiscourseErrorChirho>;
