// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Capability traits — interfaces that crates implement.
//! These are the stable contracts between crates.

use crate::corpus_chirho::CanonicalTokenChirho;
use crate::keys_chirho::VerseRefChirho;
use crate::query_chirho::QueryChirho;
use crate::result_chirho::QueryResultChirho;

/// Trait for any component that can execute queries.
pub trait QueryExecutorChirho: Send + Sync {
    /// Execute a query and return results.
    fn execute_chirho(
        &self,
        query_chirho: &QueryChirho,
    ) -> Result<QueryResultChirho, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for any component that can read verses from a corpus.
pub trait CorpusReaderChirho: Send + Sync {
    /// Read all tokens for a verse.
    fn read_verse_chirho(
        &self,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, Box<dyn std::error::Error + Send + Sync>>;

    /// Read rendered text for a verse.
    fn read_verse_text_chirho(
        &self,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    /// Read all verses in a chapter.
    fn read_chapter_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u16,
    ) -> Result<Vec<(VerseRefChirho, String)>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for ingesting external formats into canonical corpus.
pub trait IngestAdapterChirho: Send + Sync {
    /// Name of this adapter (e.g., "SWORD", "USFM", "USX").
    fn adapter_name_chirho(&self) -> &str;

    /// Whether this adapter can handle the given source.
    fn can_ingest_chirho(&self, source_path_chirho: &str) -> bool;
}

/// WASM tier classification for features.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmTierChirho {
    /// Must ship in WASM phase 1.
    Tier1Chirho,
    /// Deferred to WASM phase 2+.
    Tier2Chirho,
    /// Research / not required in WASM.
    NativeOnlyChirho,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_wasm_tier_equality_chirho() {
        assert_eq!(WasmTierChirho::Tier1Chirho, WasmTierChirho::Tier1Chirho);
        assert_ne!(WasmTierChirho::Tier1Chirho, WasmTierChirho::Tier2Chirho);
        assert_ne!(WasmTierChirho::Tier2Chirho, WasmTierChirho::NativeOnlyChirho);
    }

    #[test]
    fn test_query_executor_is_object_safe_chirho() {
        // Verify the trait can be used as a trait object
        fn _accept_executor_chirho(_e_chirho: &dyn QueryExecutorChirho) {}
    }

    #[test]
    fn test_corpus_reader_is_object_safe_chirho() {
        fn _accept_reader_chirho(_r_chirho: &dyn CorpusReaderChirho) {}
    }

    #[test]
    fn test_ingest_adapter_is_object_safe_chirho() {
        fn _accept_adapter_chirho(_a_chirho: &dyn IngestAdapterChirho) {}
    }
}
