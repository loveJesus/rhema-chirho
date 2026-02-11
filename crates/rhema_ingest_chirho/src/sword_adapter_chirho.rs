// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module adapter — bridges rsword_chirho into the Rhema canonical model.
//!
//! This adapter uses `SwMgrChirho` to discover installed SWORD modules and
//! `LoadedModuleChirho` to read verse data, converting it into the canonical
//! token stream and metadata structures.

use std::path::{Path, PathBuf};

use rhema_contracts_chirho::capability_chirho::IngestAdapterChirho;
use rhema_contracts_chirho::corpus_chirho::{
    CanonicalTokenChirho, LanguageChirho, ModuleMetadataChirho, ModuleTypeChirho,
};
use rhema_contracts_chirho::ids_chirho::CorpusIdChirho;
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rsword_chirho::config_chirho::ModuleConfigChirho;
use rsword_chirho::manager_chirho::LoadedModuleChirho;
use rsword_chirho::SwMgrChirho;

use crate::error_chirho::IngestErrorChirho;
use crate::token_extractor_chirho::TokenExtractorChirho;

/// The SWORD module adapter.
///
/// Wraps rsword_chirho's `SwMgrChirho` and provides the bridge between
/// SWORD binary modules and Rhema's canonical corpus model.
pub struct SwordAdapterChirho {
    manager_chirho: SwMgrChirho,
    _module_paths_chirho: Vec<PathBuf>,
}

impl SwordAdapterChirho {
    /// Create a new SWORD adapter using system default module paths.
    pub fn with_system_paths_chirho() -> Result<Self, IngestErrorChirho> {
        let manager_chirho =
            SwMgrChirho::with_system_paths_chirho().map_err(|e_chirho| {
                IngestErrorChirho::SwordErrorChirho(format!(
                    "Failed to initialize SWORD manager: {}",
                    e_chirho
                ))
            })?;

        Ok(Self {
            manager_chirho,
            _module_paths_chirho: Vec::new(),
        })
    }

    /// Create a new SWORD adapter scanning a specific path for modules.
    pub fn with_path_chirho(path_chirho: &Path) -> Result<Self, IngestErrorChirho> {
        let mut manager_chirho = SwMgrChirho::new_chirho();
        manager_chirho.add_path_chirho(path_chirho);
        manager_chirho
            .load_modules_chirho()
            .map_err(|e_chirho| {
                IngestErrorChirho::SwordErrorChirho(format!(
                    "Failed to load modules from {}: {}",
                    path_chirho.display(),
                    e_chirho
                ))
            })?;

        Ok(Self {
            manager_chirho,
            _module_paths_chirho: vec![path_chirho.to_path_buf()],
        })
    }

    /// List all discovered module names.
    pub fn list_modules_chirho(&self) -> Vec<String> {
        self.manager_chirho
            .get_module_names_chirho()
            .into_iter()
            .map(|s_chirho| s_chirho.to_string())
            .collect()
    }

    /// List module names filtered by type (e.g., "Biblical Texts").
    pub fn list_modules_by_type_chirho(&self, type_filter_chirho: &str) -> Vec<String> {
        self.manager_chirho
            .get_modules_by_type_chirho(type_filter_chirho)
            .into_iter()
            .map(|cfg_chirho| cfg_chirho.name_chirho.clone())
            .collect()
    }

    /// List all Bible modules.
    pub fn list_bibles_chirho(&self) -> Vec<String> {
        self.manager_chirho
            .get_bibles_chirho()
            .into_iter()
            .map(|cfg_chirho| cfg_chirho.name_chirho.clone())
            .collect()
    }

    /// List all commentary modules.
    pub fn list_commentaries_chirho(&self) -> Vec<String> {
        self.manager_chirho
            .get_commentaries_chirho()
            .into_iter()
            .map(|cfg_chirho| cfg_chirho.name_chirho.clone())
            .collect()
    }

    /// List all lexicon/dictionary modules.
    pub fn list_lexicons_chirho(&self) -> Vec<String> {
        self.manager_chirho
            .get_lexicons_chirho()
            .into_iter()
            .map(|cfg_chirho| cfg_chirho.name_chirho.clone())
            .collect()
    }

    /// Get total number of discovered modules.
    pub fn module_count_chirho(&self) -> usize {
        self.manager_chirho.module_count_chirho()
    }

    /// Check if a module is available.
    pub fn has_module_chirho(&self, name_chirho: &str) -> bool {
        self.manager_chirho.has_module_chirho(name_chirho)
    }

    /// Get metadata for a module by name.
    pub fn get_module_metadata_chirho(
        &self,
        name_chirho: &str,
    ) -> Result<ModuleMetadataChirho, IngestErrorChirho> {
        let config_chirho = self
            .manager_chirho
            .get_module_chirho(name_chirho)
            .ok_or_else(|| IngestErrorChirho::ModuleNotFoundChirho {
                name_chirho: name_chirho.to_string(),
            })?;

        Ok(config_to_metadata_chirho(config_chirho))
    }

    /// Get metadata for all discovered modules.
    pub fn get_all_metadata_chirho(&self) -> Vec<ModuleMetadataChirho> {
        self.manager_chirho
            .get_modules_chirho()
            .values()
            .map(config_to_metadata_chirho)
            .collect()
    }

    /// Read a single verse from a module.
    pub fn read_verse_chirho(
        &self,
        module_name_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<String, IngestErrorChirho> {
        let loaded_chirho = self.load_module_chirho(module_name_chirho)?;
        let key_chirho = format!("{}", verse_ref_chirho);

        loaded_chirho
            .read_entry_chirho(&key_chirho)
            .map_err(|e_chirho: rsword_chirho::ErrorChirho| IngestErrorChirho::EntryReadFailedChirho {
                key_chirho: key_chirho.clone(),
                reason_chirho: e_chirho.to_string(),
            })
    }

    /// Read all verses in a chapter from a module.
    pub fn read_chapter_chirho(
        &self,
        module_name_chirho: &str,
        book_chirho: &str,
        chapter_chirho: u16,
    ) -> Result<Vec<(VerseRefChirho, String)>, IngestErrorChirho> {
        let loaded_chirho = self.load_module_chirho(module_name_chirho)?;

        let raw_verses_chirho = loaded_chirho
            .read_chapter_batch_chirho(book_chirho, chapter_chirho as u32, 200)
            .map_err(|e_chirho: rsword_chirho::ErrorChirho| IngestErrorChirho::EntryReadFailedChirho {
                key_chirho: format!("{} {}", book_chirho, chapter_chirho),
                reason_chirho: e_chirho.to_string(),
            })?;

        let mut result_chirho = Vec::with_capacity(raw_verses_chirho.len());
        for (verse_num_chirho, text_chirho) in raw_verses_chirho {
            let verse_ref_chirho =
                VerseRefChirho::new_chirho(book_chirho, chapter_chirho, verse_num_chirho as u16)
                    .map_err(IngestErrorChirho::ContractChirho)?;
            result_chirho.push((verse_ref_chirho, text_chirho));
        }

        Ok(result_chirho)
    }

    /// Ingest an entire chapter into canonical tokens.
    pub fn ingest_chapter_chirho(
        &self,
        module_name_chirho: &str,
        book_chirho: &str,
        chapter_chirho: u16,
        extractor_chirho: &mut TokenExtractorChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, IngestErrorChirho> {
        let config_chirho = self
            .manager_chirho
            .get_module_chirho(module_name_chirho)
            .ok_or_else(|| IngestErrorChirho::ModuleNotFoundChirho {
                name_chirho: module_name_chirho.to_string(),
            })?;

        let language_chirho = detect_language_chirho(config_chirho);
        let verses_chirho = self.read_chapter_chirho(module_name_chirho, book_chirho, chapter_chirho)?;

        let mut all_tokens_chirho = Vec::new();
        for (verse_ref_chirho, text_chirho) in &verses_chirho {
            let tokens_chirho =
                extractor_chirho.extract_verse_chirho(text_chirho, verse_ref_chirho, language_chirho)?;
            all_tokens_chirho.extend(tokens_chirho);
        }

        Ok(all_tokens_chirho)
    }

    /// Load a module driver from the manager.
    fn load_module_chirho(
        &self,
        name_chirho: &str,
    ) -> Result<LoadedModuleChirho, IngestErrorChirho> {
        self.manager_chirho
            .load_module_chirho(name_chirho)
            .map_err(|e_chirho| IngestErrorChirho::ModuleLoadFailedChirho {
                name_chirho: name_chirho.to_string(),
                reason_chirho: e_chirho.to_string(),
            })
    }

    /// Get a reference to the underlying SwMgrChirho.
    pub fn manager_chirho(&self) -> &SwMgrChirho {
        &self.manager_chirho
    }
}

impl IngestAdapterChirho for SwordAdapterChirho {
    fn adapter_name_chirho(&self) -> &str {
        "SWORD"
    }

    fn can_ingest_chirho(&self, source_path_chirho: &str) -> bool {
        // Check if the path contains a mods.d directory (SWORD module repository)
        let path_chirho = Path::new(source_path_chirho);
        path_chirho.join("mods.d").is_dir()
            || path_chirho.join("modules").is_dir()
            || path_chirho.extension().is_some_and(|ext_chirho| ext_chirho == "conf")
    }
}

/// Convert a SWORD ModuleConfigChirho into our canonical ModuleMetadataChirho.
fn config_to_metadata_chirho(config_chirho: &ModuleConfigChirho) -> ModuleMetadataChirho {
    let module_type_chirho = if config_chirho.is_bible_chirho() {
        ModuleTypeChirho::BibleChirho
    } else if config_chirho.is_commentary_chirho() {
        ModuleTypeChirho::CommentaryChirho
    } else if config_chirho.is_lexicon_chirho() {
        ModuleTypeChirho::LexiconChirho
    } else if config_chirho.is_genbook_chirho() {
        ModuleTypeChirho::GenBookChirho
    } else {
        ModuleTypeChirho::BibleChirho // default fallback
    };

    ModuleMetadataChirho {
        name_chirho: config_chirho.name_chirho.clone(),
        description_chirho: config_chirho
            .description_chirho()
            .unwrap_or("Unknown")
            .to_string(),
        module_type_chirho,
        language_chirho: config_chirho
            .language_chirho()
            .unwrap_or("en")
            .to_string(),
        versification_chirho: config_chirho.versification_chirho().to_string(),
        encoding_chirho: config_chirho.encoding_chirho().to_string(),
        corpus_id_chirho: CorpusIdChirho(0), // Assigned during full ingest
    }
}

/// Detect the primary language from a SWORD module config.
fn detect_language_chirho(config_chirho: &ModuleConfigChirho) -> LanguageChirho {
    match config_chirho.language_chirho().unwrap_or("en") {
        "he" | "hbo" => LanguageChirho::HebrewChirho,
        "grc" | "el" => LanguageChirho::GreekChirho,
        "arc" => LanguageChirho::AramaicChirho,
        "la" => LanguageChirho::LatinChirho,
        "en" => LanguageChirho::EnglishChirho,
        _ => LanguageChirho::OtherChirho,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_config_to_metadata_chirho() {
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestBible".to_string());
        config_chirho.set_chirho("ModDrv", "RawText");
        config_chirho.set_chirho("Lang", "en");
        config_chirho.set_chirho("Description", "Test Bible Module");
        config_chirho.set_chirho("Versification", "KJV");

        let meta_chirho = config_to_metadata_chirho(&config_chirho);
        assert_eq!(meta_chirho.name_chirho, "TestBible");
        assert_eq!(meta_chirho.language_chirho, "en");
        assert_eq!(meta_chirho.module_type_chirho, ModuleTypeChirho::BibleChirho);
    }

    #[test]
    fn test_detect_language_chirho() {
        let mut config_chirho = ModuleConfigChirho::new_chirho("Test".to_string());
        config_chirho.set_chirho("Lang", "he");
        assert_eq!(
            detect_language_chirho(&config_chirho),
            LanguageChirho::HebrewChirho
        );

        config_chirho.set_chirho("Lang", "grc");
        assert_eq!(
            detect_language_chirho(&config_chirho),
            LanguageChirho::GreekChirho
        );
    }
}
