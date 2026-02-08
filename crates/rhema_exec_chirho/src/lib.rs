// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_exec_chirho
//!
//! Query execution runtime — dispatches planned queries against search backends
//! and produces [`QueryResultChirho`] responses.
//!
//! The executor takes a [`QueryPlanChirho`] from the planner and runs it
//! against rsword_chirho's search infrastructure (Tantivy or regex fallback).

pub mod executor_chirho;
pub mod error_chirho;

pub use executor_chirho::QueryExecutorImplChirho;
pub use error_chirho::ExecErrorChirho;
