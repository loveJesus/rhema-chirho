// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the query crate.

use thiserror::Error;

/// Errors arising during query parsing and planning.
#[derive(Debug, Error)]
pub enum QueryErrorChirho {
    #[error("Parse error at position {position_chirho}: {message_chirho}")]
    ParseErrorChirho {
        position_chirho: usize,
        message_chirho: String,
    },

    #[error("Empty query")]
    EmptyQueryChirho,

    #[error("Unmatched quote at position {position_chirho}")]
    UnmatchedQuoteChirho { position_chirho: usize },

    #[error("Unmatched bracket at position {position_chirho}")]
    UnmatchedBracketChirho { position_chirho: usize },

    #[error("Invalid Strong's number: '{value_chirho}'")]
    InvalidStrongChirho { value_chirho: String },

    #[error("Invalid proximity distance: '{value_chirho}'")]
    InvalidProximityChirho { value_chirho: String },

    #[error("Invalid morphology syntax: '{value_chirho}'")]
    InvalidMorphSyntaxChirho { value_chirho: String },
}
