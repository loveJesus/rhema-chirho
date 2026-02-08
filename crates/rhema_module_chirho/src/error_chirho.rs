// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the module crate.

use thiserror::Error;

/// Errors arising during module read/write operations.
#[derive(Debug, Error)]
pub enum ModuleErrorChirho {
    #[error("SQLite error: {0}")]
    SqliteChirho(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoChirho(#[from] std::io::Error),

    #[error("Module not found: '{path_chirho}'")]
    NotFoundChirho { path_chirho: String },

    #[error("Missing metadata key: '{key_chirho}'")]
    MissingMetaChirho { key_chirho: String },

    #[error("Schema migration failed: {reason_chirho}")]
    MigrationChirho { reason_chirho: String },

    #[error("Invalid module data: {reason_chirho}")]
    InvalidDataChirho { reason_chirho: String },
}
