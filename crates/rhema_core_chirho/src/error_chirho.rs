// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the core engine crate.

use thiserror::Error;

/// Errors arising from core engine operations.
#[derive(Debug, Error)]
pub enum CoreErrorChirho {
    #[error("Module not found: '{name_chirho}'")]
    ModuleNotFoundChirho { name_chirho: String },

    #[error("No module loaded — call load_module_chirho first")]
    NoModuleLoadedChirho,

    #[error("Ingest error: {0}")]
    IngestChirho(#[from] rhema_ingest_chirho::IngestErrorChirho),

    #[error("Contract error: {0}")]
    ContractChirho(#[from] rhema_contracts_chirho::error_chirho::ContractErrorChirho),

    #[error("Engine initialization failed: {reason_chirho}")]
    InitFailedChirho { reason_chirho: String },

    #[error("{0}")]
    OtherChirho(#[from] anyhow::Error),
}
