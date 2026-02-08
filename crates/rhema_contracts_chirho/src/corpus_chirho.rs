// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Canonical corpus contracts — the internal representation that all
//! engine operations work against, independent of SWORD binary format.

use serde::{Deserialize, Serialize};

use crate::ids_chirho::{CorpusIdChirho, LemmaIdChirho, StrongNumberChirho, TokenPositionChirho};
use crate::keys_chirho::VerseRefChirho;
use crate::morphology_chirho::MorphCodeChirho;

/// The language of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageChirho {
    HebrewChirho,
    GreekChirho,
    AramaicChirho,
    LatinChirho,
    EnglishChirho,
    OtherChirho,
}

/// A single canonical token within the corpus.
/// Carries all metadata extracted during ingestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalTokenChirho {
    /// Absolute position in the corpus (unique, monotonic).
    pub position_chirho: TokenPositionChirho,
    /// Surface form as it appears in the text.
    pub surface_chirho: String,
    /// Dictionary form (lemma).
    pub lemma_chirho: Option<LemmaIdChirho>,
    /// Strong's concordance number.
    pub strong_chirho: Option<StrongNumberChirho>,
    /// Morphology code (Robinson, OSHM, etc.).
    pub morph_chirho: Option<MorphCodeChirho>,
    /// Language of this token.
    pub language_chirho: LanguageChirho,
    /// Verse reference this token belongs to.
    pub verse_ref_chirho: VerseRefChirho,
    /// Word index within the verse (0-based).
    pub word_index_chirho: u16,
}

/// Module type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModuleTypeChirho {
    BibleChirho,
    CommentaryChirho,
    LexiconChirho,
    GenBookChirho,
    DevotionalChirho,
    GlossaryChirho,
    ImageChirho,
}

/// Metadata about an ingested module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadataChirho {
    pub name_chirho: String,
    pub description_chirho: String,
    pub module_type_chirho: ModuleTypeChirho,
    pub language_chirho: String,
    pub versification_chirho: String,
    pub encoding_chirho: String,
    pub corpus_id_chirho: CorpusIdChirho,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn make_token_chirho() -> CanonicalTokenChirho {
        CanonicalTokenChirho {
            position_chirho: TokenPositionChirho(42),
            surface_chirho: "loved".to_string(),
            lemma_chirho: Some(LemmaIdChirho::new_chirho("agapao")),
            strong_chirho: Some(StrongNumberChirho::new_chirho("G25").unwrap()),
            morph_chirho: Some(MorphCodeChirho::new_chirho("V-AAI-3S")),
            language_chirho: LanguageChirho::GreekChirho,
            verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            word_index_chirho: 5,
        }
    }

    #[test]
    fn test_canonical_token_fields_chirho() {
        let token_chirho = make_token_chirho();
        assert_eq!(token_chirho.surface_chirho, "loved");
        assert_eq!(token_chirho.position_chirho.0, 42);
        assert_eq!(token_chirho.word_index_chirho, 5);
        assert_eq!(token_chirho.language_chirho, LanguageChirho::GreekChirho);
    }

    #[test]
    fn test_canonical_token_optional_fields_chirho() {
        let mut token_chirho = make_token_chirho();
        assert!(token_chirho.lemma_chirho.is_some());
        assert!(token_chirho.strong_chirho.is_some());
        assert!(token_chirho.morph_chirho.is_some());

        token_chirho.lemma_chirho = None;
        token_chirho.strong_chirho = None;
        token_chirho.morph_chirho = None;
        assert!(token_chirho.lemma_chirho.is_none());
    }

    #[test]
    fn test_canonical_token_serde_chirho() {
        let token_chirho = make_token_chirho();
        let json_chirho = serde_json::to_string(&token_chirho).unwrap();
        let parsed_chirho: CanonicalTokenChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.surface_chirho, "loved");
        assert_eq!(parsed_chirho.position_chirho.0, 42);
    }

    #[test]
    fn test_language_enum_coverage_chirho() {
        let languages_chirho = [
            LanguageChirho::HebrewChirho,
            LanguageChirho::GreekChirho,
            LanguageChirho::AramaicChirho,
            LanguageChirho::LatinChirho,
            LanguageChirho::EnglishChirho,
            LanguageChirho::OtherChirho,
        ];
        assert_eq!(languages_chirho.len(), 6);
        assert_ne!(LanguageChirho::HebrewChirho, LanguageChirho::GreekChirho);
    }

    #[test]
    fn test_module_type_enum_coverage_chirho() {
        let types_chirho = [
            ModuleTypeChirho::BibleChirho,
            ModuleTypeChirho::CommentaryChirho,
            ModuleTypeChirho::LexiconChirho,
            ModuleTypeChirho::GenBookChirho,
            ModuleTypeChirho::DevotionalChirho,
            ModuleTypeChirho::GlossaryChirho,
            ModuleTypeChirho::ImageChirho,
        ];
        assert_eq!(types_chirho.len(), 7);
    }

    #[test]
    fn test_module_metadata_serde_chirho() {
        let meta_chirho = ModuleMetadataChirho {
            name_chirho: "KJV".to_string(),
            description_chirho: "King James Version".to_string(),
            module_type_chirho: ModuleTypeChirho::BibleChirho,
            language_chirho: "en".to_string(),
            versification_chirho: "KJV".to_string(),
            encoding_chirho: "UTF-8".to_string(),
            corpus_id_chirho: CorpusIdChirho(1),
        };
        let json_chirho = serde_json::to_string(&meta_chirho).unwrap();
        let parsed_chirho: ModuleMetadataChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.name_chirho, "KJV");
        assert_eq!(parsed_chirho.module_type_chirho, ModuleTypeChirho::BibleChirho);
    }
}
