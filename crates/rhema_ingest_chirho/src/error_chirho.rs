// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the ingest crate.

use thiserror::Error;

/// Errors arising during module ingestion.
#[derive(Debug, Error)]
pub enum IngestErrorChirho {
    #[error("Module not found: '{name_chirho}'")]
    ModuleNotFoundChirho { name_chirho: String },

    #[error("Failed to load module '{name_chirho}': {reason_chirho}")]
    ModuleLoadFailedChirho {
        name_chirho: String,
        reason_chirho: String,
    },

    #[error("Failed to read entry at key '{key_chirho}': {reason_chirho}")]
    EntryReadFailedChirho {
        key_chirho: String,
        reason_chirho: String,
    },

    #[error("Unsupported module type: '{module_type_chirho}'")]
    UnsupportedModuleTypeChirho { module_type_chirho: String },

    #[error("SWORD manager error: {0}")]
    SwordErrorChirho(String),

    #[error("Contract error: {0}")]
    ContractChirho(#[from] rhema_contracts_chirho::error_chirho::ContractErrorChirho),
}
