// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the index crate.

use thiserror::Error;

/// Errors arising from indexing and search operations.
#[derive(Debug, Error)]
pub enum IndexErrorChirho {
    #[error("Index I/O error: {0}")]
    IoChirho(#[from] std::io::Error),

    #[error("Tantivy error: {0}")]
    TantivyChirho(#[from] tantivy::TantivyError),

    #[error("Tantivy query parse error: {0}")]
    QueryParseChirho(#[from] tantivy::query::QueryParserError),

    #[error("Index not found for module '{module_name_chirho}'")]
    IndexNotFoundChirho { module_name_chirho: String },

    #[error("Manifest error: {message_chirho}")]
    ManifestChirho { message_chirho: String },

    #[error("Ingest error: {0}")]
    IngestChirho(#[from] rhema_ingest_chirho::IngestErrorChirho),

    #[error("{0}")]
    OtherChirho(#[from] anyhow::Error),
}
