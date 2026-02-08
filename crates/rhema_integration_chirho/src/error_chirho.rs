// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the integration API.

use thiserror::Error;

/// Errors from the integration layer.
#[derive(Debug, Error)]
pub enum IntegrationErrorChirho {
    #[error("Engine error: {0}")]
    EngineChirho(#[from] rhema_core_chirho::CoreErrorChirho),

    #[error("Query parse error: {0}")]
    QueryParseChirho(#[from] rhema_query_chirho::QueryErrorChirho),

    #[error("Module '{name_chirho}' not found")]
    ModuleNotFoundChirho { name_chirho: String },

    #[error("Invalid verse reference: {detail_chirho}")]
    InvalidRefChirho { detail_chirho: String },

    #[error("{0}")]
    OtherChirho(String),
}
