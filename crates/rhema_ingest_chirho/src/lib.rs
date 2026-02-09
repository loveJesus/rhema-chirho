// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_ingest_chirho
//!
//! SWORD/OSIS/TEI/IMP importers and normalization into canonical corpus.
//!
//! This crate provides importers for various Bible module formats and normalizes
//! them into the canonical Rhema corpus format for indexing and search.
//!
//! The primary adapter is [`SwordAdapterChirho`], which uses rsword_chirho's
//! `SwMgrChirho` to discover and load SWORD modules, then converts them into
//! the canonical token stream defined in `rhema_contracts_chirho`.

pub mod sword_adapter_chirho;
pub mod error_chirho;
pub mod token_extractor_chirho;
pub mod xref_extractor_chirho;

pub use sword_adapter_chirho::SwordAdapterChirho;
pub use error_chirho::IngestErrorChirho;
