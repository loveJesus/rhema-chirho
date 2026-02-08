// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rhema_ai_chirho
//!
//! AI integration for the Rhema Chirho biblical scholarship engine.
//!
//! Provides:
//! - **LLM providers** — pluggable trait for OpenAI, Anthropic, local models
//! - **Embedding models** — trait + mock for vector generation
//! - **Vector store** — SQLite-backed portable vector search
//! - **Query expansion** — LLM-powered synonym/Strong's/concept expansion
//! - **Hybrid ranking** — weighted merge and RRF for keyword + semantic results
//! - **Concept mapping** — natural language to biblical theme mapping
//! - **Batch indexing** — embedding pipeline for Bible modules

pub mod error_chirho;
pub mod config_chirho;
pub mod llm_provider_chirho;
pub mod embedding_chirho;
pub mod vector_store_chirho;
pub mod query_expander_chirho;
pub mod hybrid_ranker_chirho;
pub mod concept_mapper_chirho;
pub mod indexing_chirho;

pub use error_chirho::{AiErrorChirho, AiResultChirho};
pub use config_chirho::AiConfigChirho;
pub use llm_provider_chirho::{LlmProviderChirho, MockLlmProviderChirho};
pub use embedding_chirho::{EmbeddingModelChirho, MockEmbeddingModelChirho, cosine_similarity_chirho};
pub use vector_store_chirho::{VectorStoreChirho, SqliteVectorStoreChirho};
pub use query_expander_chirho::QueryExpanderChirho;
pub use hybrid_ranker_chirho::{merge_results_chirho, MergeStrategyChirho};
pub use concept_mapper_chirho::ConceptMapperChirho;
pub use indexing_chirho::{EmbeddingIndexerChirho, IndexingConfigChirho};
