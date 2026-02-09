// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_module_chirho
//!
//! Self-contained SQLite module format (`.rhema`) for distributing Bible modules.
//!
//! Each `.rhema` file contains verses, word-level tokens with morphology, lemmas,
//! Strong's numbers, and metadata — everything needed for a single Bible module.
//!
//! ## Architecture
//!
//! - **SQLite** = per-module storage/distribution format
//! - **Tantivy** = cross-module search index (in `rhema_index_chirho`)
//!
//! Modules ship as self-contained SQLite files. Tantivy builds a unified
//! full-text + morph facet index across all installed modules.

pub mod error_chirho;
pub mod schema_chirho;
pub mod writer_chirho;
pub mod reader_chirho;
pub mod morph_query_chirho;
pub mod converter_chirho;
pub mod xref_store_chirho;
pub mod xref_importer_chirho;
pub mod saved_search_store_chirho;
pub mod domain_store_chirho;
pub mod syntax_store_chirho;

pub use error_chirho::ModuleErrorChirho;
pub use writer_chirho::{ModuleWriterChirho, TokenEntryChirho, VerseEntryChirho};
pub use reader_chirho::{ModuleReaderChirho, TokenRowChirho, VerseRowChirho};
pub use morph_query_chirho::{search_morph_chirho, MorphHitChirho};
pub use converter_chirho::{ConversionResultChirho, SwordToRhemaConverterChirho};
pub use xref_store_chirho::XrefStoreChirho;
pub use saved_search_store_chirho::SavedSearchStoreChirho;
pub use domain_store_chirho::DomainStoreChirho;
pub use syntax_store_chirho::SyntaxStoreChirho;
