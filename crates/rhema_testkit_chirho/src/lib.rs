// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_testkit_chirho
//!
//! Shared test fixtures, generators, golden harnesses, differential runners.
//!
//! This crate provides testing infrastructure for the Rhema engine:
//!
//! - **Golden fixtures**: Well-known verse references and expected text
//! - **Query fixtures**: Queries with expected result counts and verse matches
//! - **Differential test harness**: Compare results across backends

pub mod fixtures_chirho;
pub mod golden_chirho;

pub use fixtures_chirho::{VerseFixtureChirho, QueryFixtureChirho};
pub use golden_chirho::GoldenHarnessChirho;
