// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! The Rhema engine — top-level entry point for the biblical scholarship engine.
//!
//! Manages module discovery, corpus loading, and provides the unified interface
//! for all consumers (GUI, CLI, REST, AI).

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Mutex;

use lru::LruCache;
use rhema_contracts_chirho::capability_chirho::CorpusReaderChirho;
use rhema_contracts_chirho::corpus_chirho::{
    CanonicalTokenChirho, ModuleMetadataChirho,
};
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_ingest_chirho::sword_adapter_chirho::SwordAdapterChirho;
use rhema_ingest_chirho::token_extractor_chirho::TokenExtractorChirho;

use crate::error_chirho::CoreErrorChirho;

/// Default number of chapters to keep in the LRU cache.
const DEFAULT_CACHE_SIZE_CHIRHO: usize = 64;

/// Cache key: (module_name, book, chapter).
type ChapterCacheKeyChirho = (String, String, u16);

/// The main Rhema engine. Manages corpora, modules, and provides
/// the unified interface for all consumers (GUI, CLI, REST, AI).
pub struct RhemaEngineChirho {
    /// SWORD module adapter for loading SWORD-format modules.
    sword_adapter_chirho: SwordAdapterChirho,
    /// Metadata cache for discovered modules.
    module_metadata_chirho: HashMap<String, ModuleMetadataChirho>,
    /// Currently active module name (for quick access).
    active_module_chirho: Option<String>,
    /// LRU chapter cache — avoids redundant SWORD I/O for recently read chapters.
    chapter_cache_chirho: Mutex<LruCache<ChapterCacheKeyChirho, Vec<(VerseRefChirho, String)>>>,
}

impl RhemaEngineChirho {
    /// Create a new Rhema engine with system default SWORD module paths.
    pub fn new_chirho() -> Result<Self, CoreErrorChirho> {
        log::info!("Initializing Rhema Chirho engine");
        let sword_adapter_chirho = SwordAdapterChirho::with_system_paths_chirho()?;

        let mut engine_chirho = Self {
            sword_adapter_chirho,
            module_metadata_chirho: HashMap::new(),
            active_module_chirho: None,
            chapter_cache_chirho: Mutex::new(LruCache::new(
                NonZeroUsize::new(DEFAULT_CACHE_SIZE_CHIRHO)
                    .expect("cache size must be > 0"),
            )),
        };

        engine_chirho.refresh_metadata_chirho();
        log::info!(
            "Rhema engine initialized with {} modules",
            engine_chirho.module_count_chirho()
        );

        Ok(engine_chirho)
    }

    /// Create a new Rhema engine scanning a specific path for modules.
    pub fn with_path_chirho(path_chirho: &Path) -> Result<Self, CoreErrorChirho> {
        log::info!(
            "Initializing Rhema Chirho engine with path: {}",
            path_chirho.display()
        );
        let sword_adapter_chirho = SwordAdapterChirho::with_path_chirho(path_chirho)?;

        let mut engine_chirho = Self {
            sword_adapter_chirho,
            module_metadata_chirho: HashMap::new(),
            active_module_chirho: None,
            chapter_cache_chirho: Mutex::new(LruCache::new(
                NonZeroUsize::new(DEFAULT_CACHE_SIZE_CHIRHO)
                    .expect("cache size must be > 0"),
            )),
        };

        engine_chirho.refresh_metadata_chirho();
        Ok(engine_chirho)
    }

    // ── Module Discovery ───────────────────────────────────────────

    /// List all available module names.
    pub fn list_modules_chirho(&self) -> Vec<String> {
        self.sword_adapter_chirho.list_modules_chirho()
    }

    /// List all Bible modules.
    pub fn list_bibles_chirho(&self) -> Vec<String> {
        self.sword_adapter_chirho.list_bibles_chirho()
    }

    /// List all commentary modules.
    pub fn list_commentaries_chirho(&self) -> Vec<String> {
        self.sword_adapter_chirho.list_commentaries_chirho()
    }

    /// List all lexicon/dictionary modules.
    pub fn list_lexicons_chirho(&self) -> Vec<String> {
        self.sword_adapter_chirho.list_lexicons_chirho()
    }

    /// Total number of discovered modules.
    pub fn module_count_chirho(&self) -> usize {
        self.sword_adapter_chirho.module_count_chirho()
    }

    /// Check if a module is available.
    pub fn has_module_chirho(&self, name_chirho: &str) -> bool {
        self.sword_adapter_chirho.has_module_chirho(name_chirho)
    }

    /// Get metadata for a specific module.
    pub fn get_module_metadata_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<&ModuleMetadataChirho> {
        self.module_metadata_chirho.get(name_chirho)
    }

    /// Get metadata for all modules.
    pub fn get_all_metadata_chirho(&self) -> Vec<&ModuleMetadataChirho> {
        self.module_metadata_chirho.values().collect()
    }

    // ── Active Module ──────────────────────────────────────────────

    /// Set the active module for quick access.
    pub fn set_active_module_chirho(
        &mut self,
        name_chirho: &str,
    ) -> Result<(), CoreErrorChirho> {
        if !self.has_module_chirho(name_chirho) {
            return Err(CoreErrorChirho::ModuleNotFoundChirho {
                name_chirho: name_chirho.to_string(),
            });
        }
        self.active_module_chirho = Some(name_chirho.to_string());
        Ok(())
    }

    /// Get the currently active module name.
    pub fn active_module_chirho(&self) -> Option<&str> {
        self.active_module_chirho.as_deref()
    }

    // ── Verse Reading ──────────────────────────────────────────────

    /// Read a single verse from the active module (or a named module).
    ///
    /// If the chapter containing this verse is already cached, the verse
    /// is returned from the cache without any SWORD I/O.
    pub fn read_verse_chirho(
        &self,
        module_name_chirho: Option<&str>,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<String, CoreErrorChirho> {
        let name_chirho = self.resolve_module_name_chirho(module_name_chirho)?;

        // Try to serve from the chapter cache first.
        let cache_key_chirho = (
            name_chirho.to_string(),
            verse_ref_chirho.book_chirho.clone(),
            verse_ref_chirho.chapter_chirho,
        );

        {
            let mut cache_chirho = self.chapter_cache_chirho.lock().unwrap();
            if let Some(chapter_verses_chirho) = cache_chirho.get(&cache_key_chirho) {
                let found_chirho = chapter_verses_chirho
                    .iter()
                    .find(|(r_chirho, _)| r_chirho == verse_ref_chirho)
                    .map(|(_, text_chirho)| text_chirho.clone());

                if let Some(text_chirho) = found_chirho {
                    log::debug!(
                        "Cache hit for verse {}:{}:{} in {}",
                        verse_ref_chirho.book_chirho,
                        verse_ref_chirho.chapter_chirho,
                        verse_ref_chirho.verse_chirho,
                        name_chirho,
                    );
                    return Ok(text_chirho);
                }
            }
        }

        // Cache miss — read the full chapter and cache it, then return the verse.
        let chapter_chirho = self
            .load_and_cache_chapter_chirho(name_chirho, &verse_ref_chirho.book_chirho, verse_ref_chirho.chapter_chirho)?;

        chapter_chirho
            .iter()
            .find(|(r_chirho, _)| r_chirho == verse_ref_chirho)
            .map(|(_, text_chirho)| text_chirho.clone())
            .ok_or_else(|| CoreErrorChirho::OtherChirho(
                anyhow::anyhow!(
                    "Verse {} not found in chapter",
                    verse_ref_chirho.verse_chirho
                ),
            ))
    }

    /// Read all verses in a chapter from the active module (or a named module).
    ///
    /// Results are cached in the LRU cache. Subsequent calls for the same
    /// module/book/chapter are served from memory.
    pub fn read_chapter_chirho(
        &self,
        module_name_chirho: Option<&str>,
        book_chirho: &str,
        chapter_chirho: u16,
    ) -> Result<Vec<(VerseRefChirho, String)>, CoreErrorChirho> {
        let name_chirho = self.resolve_module_name_chirho(module_name_chirho)?;
        self.load_and_cache_chapter_chirho(name_chirho, book_chirho, chapter_chirho)
    }

    /// Read rendered text for a verse (convenience for the CorpusReaderChirho trait).
    pub fn read_verse_text_chirho(
        &self,
        module_name_chirho: Option<&str>,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<String, CoreErrorChirho> {
        self.read_verse_chirho(module_name_chirho, verse_ref_chirho)
    }

    // ── Cache Management ────────────────────────────────────────────

    /// Clear the entire chapter cache.
    pub fn clear_cache_chirho(&self) {
        self.chapter_cache_chirho.lock().unwrap().clear();
        log::debug!("Chapter cache cleared");
    }

    /// Number of chapters currently in the cache.
    pub fn cache_len_chirho(&self) -> usize {
        self.chapter_cache_chirho.lock().unwrap().len()
    }

    // ── Token Extraction ───────────────────────────────────────────

    /// Ingest a chapter into canonical tokens.
    pub fn ingest_chapter_tokens_chirho(
        &self,
        module_name_chirho: Option<&str>,
        book_chirho: &str,
        chapter_chirho: u16,
        extractor_chirho: &mut TokenExtractorChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, CoreErrorChirho> {
        let name_chirho = self.resolve_module_name_chirho(module_name_chirho)?;
        self.sword_adapter_chirho
            .ingest_chapter_chirho(name_chirho, book_chirho, chapter_chirho, extractor_chirho)
            .map_err(CoreErrorChirho::IngestChirho)
    }

    // ── SWORD Adapter Access ───────────────────────────────────────

    /// Direct access to the SWORD adapter for advanced operations.
    pub fn sword_adapter_chirho(&self) -> &SwordAdapterChirho {
        &self.sword_adapter_chirho
    }

    // ── Private Helpers ────────────────────────────────────────────

    /// Load a chapter from the SWORD adapter and insert it into the LRU cache.
    /// Returns a clone of the cached data.
    fn load_and_cache_chapter_chirho(
        &self,
        module_name_chirho: &str,
        book_chirho: &str,
        chapter_chirho: u16,
    ) -> Result<Vec<(VerseRefChirho, String)>, CoreErrorChirho> {
        let cache_key_chirho = (
            module_name_chirho.to_string(),
            book_chirho.to_string(),
            chapter_chirho,
        );

        // Check cache first.
        {
            let mut cache_chirho = self.chapter_cache_chirho.lock().unwrap();
            if let Some(cached_chirho) = cache_chirho.get(&cache_key_chirho) {
                let result_chirho = cached_chirho.clone();
                let cache_len_chirho = cache_chirho.len();
                log::debug!(
                    "Cache hit for {} {} ch.{} ({} entries in cache)",
                    module_name_chirho,
                    book_chirho,
                    chapter_chirho,
                    cache_len_chirho,
                );
                return Ok(result_chirho);
            }
        }

        // Cache miss — read from SWORD.
        log::debug!(
            "Cache miss for {} {} ch.{}, loading from SWORD",
            module_name_chirho,
            book_chirho,
            chapter_chirho,
        );
        let verses_chirho = self
            .sword_adapter_chirho
            .read_chapter_chirho(module_name_chirho, book_chirho, chapter_chirho)
            .map_err(CoreErrorChirho::IngestChirho)?;

        // Insert into cache.
        {
            let mut cache_chirho = self.chapter_cache_chirho.lock().unwrap();
            cache_chirho.put(cache_key_chirho, verses_chirho.clone());
        }

        Ok(verses_chirho)
    }

    /// Refresh module metadata cache from the SWORD adapter.
    fn refresh_metadata_chirho(&mut self) {
        self.module_metadata_chirho.clear();
        for meta_chirho in self.sword_adapter_chirho.get_all_metadata_chirho() {
            self.module_metadata_chirho
                .insert(meta_chirho.name_chirho.clone(), meta_chirho);
        }
    }

    /// Resolve a module name: use the given name, or the active module,
    /// or return an error.
    fn resolve_module_name_chirho<'a>(
        &'a self,
        explicit_chirho: Option<&'a str>,
    ) -> Result<&'a str, CoreErrorChirho> {
        explicit_chirho
            .or(self.active_module_chirho.as_deref())
            .ok_or(CoreErrorChirho::NoModuleLoadedChirho)
    }
}

impl CorpusReaderChirho for RhemaEngineChirho {
    fn read_verse_chirho(
        &self,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, Box<dyn std::error::Error + Send + Sync>> {
        let name_chirho = self
            .resolve_module_name_chirho(None)
            .map_err(|e_chirho| Box::new(e_chirho) as Box<dyn std::error::Error + Send + Sync>)?;

        let mut extractor_chirho = TokenExtractorChirho::new_chirho();
        let tokens_chirho = self
            .sword_adapter_chirho
            .ingest_chapter_chirho(
                name_chirho,
                &verse_ref_chirho.book_chirho,
                verse_ref_chirho.chapter_chirho,
                &mut extractor_chirho,
            )
            .map_err(|e_chirho| Box::new(e_chirho) as Box<dyn std::error::Error + Send + Sync>)?;

        // Filter to just the requested verse
        Ok(tokens_chirho
            .into_iter()
            .filter(|t_chirho| t_chirho.verse_ref_chirho == *verse_ref_chirho)
            .collect())
    }

    fn read_verse_text_chirho(
        &self,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.read_verse_chirho(None, verse_ref_chirho)
            .map_err(|e_chirho| Box::new(e_chirho) as Box<dyn std::error::Error + Send + Sync>)
    }

    fn read_chapter_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u16,
    ) -> Result<Vec<(VerseRefChirho, String)>, Box<dyn std::error::Error + Send + Sync>> {
        self.read_chapter_chirho(None, book_chirho, chapter_chirho)
            .map_err(|e_chirho| Box::new(e_chirho) as Box<dyn std::error::Error + Send + Sync>)
    }
}
