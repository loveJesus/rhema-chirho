// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the execution crate.

use thiserror::Error;

/// Errors arising during query execution.
#[derive(Debug, Error)]
pub enum ExecErrorChirho {
    #[error("Search backend error: {reason_chirho}")]
    SearchFailedChirho { reason_chirho: String },

    #[error("Module not loaded for search: '{name_chirho}'")]
    ModuleNotLoadedChirho { name_chirho: String },

    #[error("Unsupported plan step: {detail_chirho}")]
    UnsupportedStepChirho { detail_chirho: String },

    #[error("Ingest error: {0}")]
    IngestChirho(#[from] rhema_ingest_chirho::IngestErrorChirho),

    #[error("SWORD error: {0}")]
    SwordChirho(String),

    #[error("Cross-reference store error: {reason_chirho}")]
    XrefStoreChirho { reason_chirho: String },

    #[error("Domain store error: {reason_chirho}")]
    DomainStoreChirho { reason_chirho: String },
}
