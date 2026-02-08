// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_index_chirho
//!
//! Full-text indexing engine for the Rhema biblical scholarship platform.
//!
//! This crate provides Tantivy-based indexing with structured fields for
//! books, chapters, verses, Strong's numbers, and verse text. It supports:
//!
//! - **Module indexing** — build a search index from any SWORD module
//! - **Full-text search** — boolean queries, phrases, fuzzy, regex
//! - **Strong's lookup** — search by Strong's concordance numbers
//! - **Book filtering** — scope searches to specific books
//! - **Manifest versioning** — detect stale indexes after schema changes

pub mod error_chirho;
pub mod indexer_chirho;
pub mod manifest_chirho;
pub mod schema_chirho;
pub mod searcher_chirho;

pub use error_chirho::IndexErrorChirho;
pub use indexer_chirho::ModuleIndexerChirho;
pub use manifest_chirho::IndexManifestChirho;
pub use schema_chirho::RhemaSchemaChirho;
pub use searcher_chirho::{IndexHitChirho, IndexSearcherChirho};
