// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_integration_chirho
//!
//! Stable library integration APIs for GUI/AI/CLI/REST wrappers.
//!
//! This crate provides the high-level, consumer-facing API surface
//! for the Rhema biblical scholarship engine. All external consumers
//! (GUI apps, CLI tools, REST servers, AI integrations) should use
//! this crate rather than depending on internal crates directly.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rhema_integration_chirho::RhemaLibraryChirho;
//!
//! let lib_chirho = RhemaLibraryChirho::init_chirho().unwrap();
//! println!("Modules: {:?}", lib_chirho.list_bibles_chirho());
//!
//! let verse_chirho = lib_chirho.read_verse_chirho("KJV", "John", 3, 16).unwrap();
//! println!("{}", verse_chirho.text_chirho);
//! ```

pub mod library_chirho;
pub mod dto_chirho;
pub mod error_chirho;

pub use library_chirho::RhemaLibraryChirho;
pub use error_chirho::IntegrationErrorChirho;
