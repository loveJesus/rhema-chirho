// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_core_chirho
//!
//! Core engine: canonical corpus primitives, key/reference logic,
//! module loading abstractions, and versification.
//!
//! This crate bridges rsword_chirho (SWORD compatibility) with the
//! rhema_chirho canonical internal model. The [`RhemaEngineChirho`]
//! is the top-level entry point for all consumers.

pub mod engine_chirho;
pub mod error_chirho;

pub use engine_chirho::RhemaEngineChirho;
pub use error_chirho::CoreErrorChirho;
