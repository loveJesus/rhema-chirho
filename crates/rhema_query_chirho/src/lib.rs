// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_query_chirho
//!
//! Query parser, IR, planner, and operators.
//!
//! This crate provides the query parsing and planning infrastructure, converting
//! user queries into an intermediate representation ([`QueryNodeChirho`]) and
//! optimizing execution plans.
//!
//! ## Query Syntax
//!
//! - Simple term: `love`
//! - Phrase: `"God so loved"`
//! - Boolean: `love AND world`, `grace OR mercy`, `NOT sin`
//! - Strong's: `strong:G26` or `strong:H430`
//! - Lemma: `lemma:agape`
//! - Proximity: `love NEAR/5 world`
//! - Scope: `[John] love` (restrict to book of John)

pub mod parser_chirho;
pub mod planner_chirho;
pub mod error_chirho;

pub use parser_chirho::QueryParserChirho;
pub use planner_chirho::QueryPlannerChirho;
pub use error_chirho::QueryErrorChirho;
