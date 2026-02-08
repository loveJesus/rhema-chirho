// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! The Rhema Library — the stable, high-level entry point for all consumers.
//!
//! This is the primary API surface that GUI apps, CLI tools, REST servers,
//! and AI integrations should use. It wraps the engine and query parser
//! behind a simple, consumer-friendly interface with DTOs.

use std::path::Path;

use rhema_contracts_chirho::corpus_chirho::ModuleMetadataChirho;
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_core_chirho::RhemaEngineChirho;
use rhema_query_chirho::QueryParserChirho;

use crate::dto_chirho::{
    ChapterTextChirho, ModuleInfoChirho, VerseTextChirho,
};
use crate::error_chirho::IntegrationErrorChirho;

/// The Rhema Library — high-level API for all consumers.
pub struct RhemaLibraryChirho {
    engine_chirho: RhemaEngineChirho,
}

impl RhemaLibraryChirho {
    /// Initialize with system default SWORD module paths.
    pub fn init_chirho() -> Result<Self, IntegrationErrorChirho> {
        let engine_chirho = RhemaEngineChirho::new_chirho()?;
        Ok(Self { engine_chirho })
    }

    /// Initialize scanning a specific directory for modules.
    pub fn init_with_path_chirho(
        path_chirho: &Path,
    ) -> Result<Self, IntegrationErrorChirho> {
        let engine_chirho = RhemaEngineChirho::with_path_chirho(path_chirho)?;
        Ok(Self { engine_chirho })
    }

    // ── Module Discovery ───────────────────────────────────────────

    /// List all available Bible modules.
    pub fn list_bibles_chirho(&self) -> Vec<ModuleInfoChirho> {
        self.engine_chirho
            .list_bibles_chirho()
            .into_iter()
            .filter_map(|name_chirho| self.get_module_info_chirho(&name_chirho))
            .collect()
    }

    /// List all available commentary modules.
    pub fn list_commentaries_chirho(&self) -> Vec<ModuleInfoChirho> {
        self.engine_chirho
            .list_commentaries_chirho()
            .into_iter()
            .filter_map(|name_chirho| self.get_module_info_chirho(&name_chirho))
            .collect()
    }

    /// List all available lexicon/dictionary modules.
    pub fn list_lexicons_chirho(&self) -> Vec<ModuleInfoChirho> {
        self.engine_chirho
            .list_lexicons_chirho()
            .into_iter()
            .filter_map(|name_chirho| self.get_module_info_chirho(&name_chirho))
            .collect()
    }

    /// List all available modules.
    pub fn list_all_modules_chirho(&self) -> Vec<ModuleInfoChirho> {
        self.engine_chirho
            .list_modules_chirho()
            .into_iter()
            .filter_map(|name_chirho| self.get_module_info_chirho(&name_chirho))
            .collect()
    }

    /// Get info about a specific module.
    pub fn get_module_info_chirho(&self, name_chirho: &str) -> Option<ModuleInfoChirho> {
        self.engine_chirho
            .get_module_metadata_chirho(name_chirho)
            .map(metadata_to_info_chirho)
    }

    /// Total module count.
    pub fn module_count_chirho(&self) -> usize {
        self.engine_chirho.module_count_chirho()
    }

    // ── Verse Reading ──────────────────────────────────────────────

    /// Read a single verse from a module.
    pub fn read_verse_chirho(
        &self,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: u16,
        verse_chirho: u16,
    ) -> Result<VerseTextChirho, IntegrationErrorChirho> {
        let verse_ref_chirho = VerseRefChirho::new_chirho(book_chirho, chapter_chirho, verse_chirho)
            .map_err(|e_chirho| IntegrationErrorChirho::InvalidRefChirho {
                detail_chirho: e_chirho.to_string(),
            })?;

        let text_chirho = self
            .engine_chirho
            .read_verse_chirho(Some(module_chirho), &verse_ref_chirho)?;

        Ok(VerseTextChirho {
            book_chirho: book_chirho.to_string(),
            chapter_chirho,
            verse_chirho,
            text_chirho,
        })
    }

    /// Read an entire chapter from a module.
    pub fn read_chapter_chirho(
        &self,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: u16,
    ) -> Result<ChapterTextChirho, IntegrationErrorChirho> {
        let raw_verses_chirho = self
            .engine_chirho
            .read_chapter_chirho(Some(module_chirho), book_chirho, chapter_chirho)?;

        let verses_chirho: Vec<VerseTextChirho> = raw_verses_chirho
            .into_iter()
            .map(|(ref_chirho, text_chirho)| VerseTextChirho {
                book_chirho: ref_chirho.book_chirho.clone(),
                chapter_chirho: ref_chirho.chapter_chirho,
                verse_chirho: ref_chirho.verse_chirho,
                text_chirho,
            })
            .collect();

        Ok(ChapterTextChirho {
            module_chirho: module_chirho.to_string(),
            book_chirho: book_chirho.to_string(),
            chapter_chirho,
            verses_chirho,
        })
    }

    // ── Query / Search ─────────────────────────────────────────────

    /// Parse a query string (validates syntax without executing).
    pub fn validate_query_chirho(
        &self,
        query_text_chirho: &str,
    ) -> Result<String, IntegrationErrorChirho> {
        let query_chirho = QueryParserChirho::parse_chirho(query_text_chirho)?;
        Ok(format!("{:?}", query_chirho.root_chirho))
    }

    // ── Engine Access ──────────────────────────────────────────────

    /// Get a reference to the underlying engine for advanced operations.
    pub fn engine_chirho(&self) -> &RhemaEngineChirho {
        &self.engine_chirho
    }

    /// Get a mutable reference to the underlying engine.
    pub fn engine_mut_chirho(&mut self) -> &mut RhemaEngineChirho {
        &mut self.engine_chirho
    }
}

/// Convert internal metadata to the public DTO.
fn metadata_to_info_chirho(meta_chirho: &ModuleMetadataChirho) -> ModuleInfoChirho {
    ModuleInfoChirho {
        name_chirho: meta_chirho.name_chirho.clone(),
        description_chirho: meta_chirho.description_chirho.clone(),
        module_type_chirho: format!("{:?}", meta_chirho.module_type_chirho),
        language_chirho: meta_chirho.language_chirho.clone(),
        versification_chirho: meta_chirho.versification_chirho.clone(),
    }
}
