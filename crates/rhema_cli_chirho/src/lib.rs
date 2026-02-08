// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_cli_chirho
//!
//! CLI tools for the Rhema biblical scholarship engine.
//!
//! Provides a `rhema_chirho` binary with subcommands for:
//! - `lookup` — read verses/chapters from SWORD modules
//! - `search` — query execution with boolean/phrase/Strong's support
//! - `modules` — list available modules
//! - `info` — display module metadata
//! - `parse` — parse and explain a query without executing

pub mod commands_chirho;
