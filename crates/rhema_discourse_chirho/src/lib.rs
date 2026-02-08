// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_discourse_chirho
//!
//! Discourse analysis for biblical scholarship — arcing, bracketing, phrasing.
//!
//! Provides:
//! - **Types** — 22 relationship types, propositions, arc structures, brackets, phrases
//! - **Storage** — SQLite-backed persistence for discourse analyses
//! - **Detection** — Heuristic proposition detection from passage text
//! - **Suggestion** — Rule-based relationship suggestion from discourse markers
//! - **Validation** — Structural integrity checks (no cycles, valid references)
//! - **LLM Generation** — AI-powered complete arc generation and improvement
//! - **Search** — Query discourse analyses by relationship type or proposition text
//! - **Import/Export** — JSON round-trip serialization

pub mod types_chirho;
pub mod error_chirho;
pub mod storage_chirho;
pub mod sqlite_store_chirho;
pub mod detector_chirho;
pub mod suggester_chirho;
pub mod validator_chirho;
pub mod llm_generator_chirho;
pub mod search_chirho;
pub mod import_export_chirho;

pub use types_chirho::*;
pub use error_chirho::{DiscourseErrorChirho, DiscourseResultChirho};
pub use storage_chirho::DiscourseStoreChirho;
pub use sqlite_store_chirho::SqliteDiscourseStoreChirho;
pub use detector_chirho::HeuristicPropositionDetectorChirho;
pub use suggester_chirho::RuleBasedSuggesterChirho;
pub use validator_chirho::{validate_arc_chirho, ValidationResultChirho};
pub use import_export_chirho::{export_to_json_chirho, import_from_json_chirho};
