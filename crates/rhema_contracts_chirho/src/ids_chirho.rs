// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Strong newtypes for identifiers. These replace raw strings throughout
//! the engine so invalid states fail at compile time.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Immutable corpus version identifier. Tied to a specific ingest snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorpusIdChirho(pub u64);

/// Unique token position within a corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TokenPositionChirho(pub u64);

/// Module name, validated on construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleNameChirho(String);

impl ModuleNameChirho {
    /// Create a new module name. Must be non-empty and ASCII-safe.
    pub fn new_chirho(name_chirho: &str) -> Result<Self, crate::error_chirho::ContractErrorChirho> {
        if name_chirho.is_empty() {
            return Err(crate::error_chirho::ContractErrorChirho::EmptyIdentifierChirho {
                field_chirho: "module_name".to_string(),
            });
        }
        Ok(Self(name_chirho.to_string()))
    }

    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleNameChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}", self.0)
    }
}

/// Strong's concordance number (e.g., H430, G26). Validated format.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StrongNumberChirho(String);

impl StrongNumberChirho {
    /// Create from string like "H430" or "G26".
    pub fn new_chirho(raw_chirho: &str) -> Result<Self, crate::error_chirho::ContractErrorChirho> {
        let trimmed_chirho = raw_chirho.trim();
        if trimmed_chirho.len() < 2 {
            return Err(crate::error_chirho::ContractErrorChirho::InvalidFormatChirho {
                field_chirho: "strong_number".to_string(),
                value_chirho: raw_chirho.to_string(),
                expected_chirho: "H/G followed by digits (e.g., H430, G26)".to_string(),
            });
        }
        let prefix_chirho = trimmed_chirho.as_bytes()[0];
        if prefix_chirho != b'H' && prefix_chirho != b'G' {
            return Err(crate::error_chirho::ContractErrorChirho::InvalidFormatChirho {
                field_chirho: "strong_number".to_string(),
                value_chirho: raw_chirho.to_string(),
                expected_chirho: "Must start with H (Hebrew) or G (Greek)".to_string(),
            });
        }
        if !trimmed_chirho[1..].chars().all(|c_chirho| c_chirho.is_ascii_digit()) {
            return Err(crate::error_chirho::ContractErrorChirho::InvalidFormatChirho {
                field_chirho: "strong_number".to_string(),
                value_chirho: raw_chirho.to_string(),
                expected_chirho: "Digits after H/G prefix".to_string(),
            });
        }
        Ok(Self(trimmed_chirho.to_string()))
    }

    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }

    pub fn is_hebrew_chirho(&self) -> bool {
        self.0.starts_with('H')
    }

    pub fn is_greek_chirho(&self) -> bool {
        self.0.starts_with('G')
    }

    pub fn number_chirho(&self) -> u32 {
        self.0[1..].parse().unwrap_or(0)
    }
}

impl fmt::Display for StrongNumberChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}", self.0)
    }
}

/// Lemma identifier (dictionary form of a word).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LemmaIdChirho(pub String);

impl LemmaIdChirho {
    pub fn new_chirho(lemma_chirho: &str) -> Self {
        Self(lemma_chirho.to_string())
    }

    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LemmaIdChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}", self.0)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    // ── CorpusIdChirho ──────────────────────────────────────────────

    #[test]
    fn test_corpus_id_equality_chirho() {
        assert_eq!(CorpusIdChirho(1), CorpusIdChirho(1));
        assert_ne!(CorpusIdChirho(1), CorpusIdChirho(2));
    }

    #[test]
    fn test_corpus_id_hash_chirho() {
        let mut set_chirho = std::collections::HashSet::new();
        set_chirho.insert(CorpusIdChirho(42));
        assert!(set_chirho.contains(&CorpusIdChirho(42)));
        assert!(!set_chirho.contains(&CorpusIdChirho(43)));
    }

    #[test]
    fn test_corpus_id_serde_chirho() {
        let id_chirho = CorpusIdChirho(99);
        let json_chirho = serde_json::to_string(&id_chirho).unwrap();
        let parsed_chirho: CorpusIdChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(id_chirho, parsed_chirho);
    }

    // ── TokenPositionChirho ──────────────────────────────────────────

    #[test]
    fn test_token_position_ordering_chirho() {
        let a_chirho = TokenPositionChirho(10);
        let b_chirho = TokenPositionChirho(20);
        assert!(a_chirho < b_chirho);
        assert!(b_chirho > a_chirho);
    }

    #[test]
    fn test_token_position_serde_chirho() {
        let pos_chirho = TokenPositionChirho(5000);
        let json_chirho = serde_json::to_string(&pos_chirho).unwrap();
        let parsed_chirho: TokenPositionChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(pos_chirho, parsed_chirho);
    }

    // ── ModuleNameChirho ──────────────────────────────────────────────

    #[test]
    fn test_module_name_valid_chirho() {
        let name_chirho = ModuleNameChirho::new_chirho("KJV").unwrap();
        assert_eq!(name_chirho.as_str_chirho(), "KJV");
    }

    #[test]
    fn test_module_name_empty_rejected_chirho() {
        let result_chirho = ModuleNameChirho::new_chirho("");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_module_name_display_chirho() {
        let name_chirho = ModuleNameChirho::new_chirho("ESV2011").unwrap();
        assert_eq!(format!("{}", name_chirho), "ESV2011");
    }

    #[test]
    fn test_module_name_equality_chirho() {
        let a_chirho = ModuleNameChirho::new_chirho("KJV").unwrap();
        let b_chirho = ModuleNameChirho::new_chirho("KJV").unwrap();
        let c_chirho = ModuleNameChirho::new_chirho("ESV").unwrap();
        assert_eq!(a_chirho, b_chirho);
        assert_ne!(a_chirho, c_chirho);
    }

    #[test]
    fn test_module_name_hash_chirho() {
        let mut set_chirho = std::collections::HashSet::new();
        set_chirho.insert(ModuleNameChirho::new_chirho("KJV").unwrap());
        assert!(set_chirho.contains(&ModuleNameChirho::new_chirho("KJV").unwrap()));
    }

    #[test]
    fn test_module_name_serde_chirho() {
        let name_chirho = ModuleNameChirho::new_chirho("NASB").unwrap();
        let json_chirho = serde_json::to_string(&name_chirho).unwrap();
        let parsed_chirho: ModuleNameChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(name_chirho, parsed_chirho);
    }

    // ── StrongNumberChirho ──────────────────────────────────────────

    #[test]
    fn test_strong_hebrew_valid_chirho() {
        let s_chirho = StrongNumberChirho::new_chirho("H430").unwrap();
        assert!(s_chirho.is_hebrew_chirho());
        assert!(!s_chirho.is_greek_chirho());
        assert_eq!(s_chirho.number_chirho(), 430);
        assert_eq!(s_chirho.as_str_chirho(), "H430");
    }

    #[test]
    fn test_strong_greek_valid_chirho() {
        let s_chirho = StrongNumberChirho::new_chirho("G26").unwrap();
        assert!(s_chirho.is_greek_chirho());
        assert!(!s_chirho.is_hebrew_chirho());
        assert_eq!(s_chirho.number_chirho(), 26);
    }

    #[test]
    fn test_strong_with_whitespace_chirho() {
        let s_chirho = StrongNumberChirho::new_chirho("  H430  ").unwrap();
        assert_eq!(s_chirho.as_str_chirho(), "H430");
    }

    #[test]
    fn test_strong_too_short_chirho() {
        assert!(StrongNumberChirho::new_chirho("H").is_err());
        assert!(StrongNumberChirho::new_chirho("").is_err());
        assert!(StrongNumberChirho::new_chirho("G").is_err());
    }

    #[test]
    fn test_strong_bad_prefix_chirho() {
        assert!(StrongNumberChirho::new_chirho("X430").is_err());
        assert!(StrongNumberChirho::new_chirho("430").is_err());
        assert!(StrongNumberChirho::new_chirho("hebrew430").is_err());
    }

    #[test]
    fn test_strong_non_digit_suffix_chirho() {
        assert!(StrongNumberChirho::new_chirho("H43a").is_err());
        assert!(StrongNumberChirho::new_chirho("G2.6").is_err());
        assert!(StrongNumberChirho::new_chirho("H430x").is_err());
    }

    #[test]
    fn test_strong_display_chirho() {
        let s_chirho = StrongNumberChirho::new_chirho("G3056").unwrap();
        assert_eq!(format!("{}", s_chirho), "G3056");
    }

    #[test]
    fn test_strong_equality_chirho() {
        let a_chirho = StrongNumberChirho::new_chirho("H430").unwrap();
        let b_chirho = StrongNumberChirho::new_chirho("H430").unwrap();
        let c_chirho = StrongNumberChirho::new_chirho("G26").unwrap();
        assert_eq!(a_chirho, b_chirho);
        assert_ne!(a_chirho, c_chirho);
    }

    #[test]
    fn test_strong_serde_chirho() {
        let s_chirho = StrongNumberChirho::new_chirho("H7225").unwrap();
        let json_chirho = serde_json::to_string(&s_chirho).unwrap();
        let parsed_chirho: StrongNumberChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(s_chirho, parsed_chirho);
    }

    // ── LemmaIdChirho ──────────────────────────────────────────────

    #[test]
    fn test_lemma_creation_chirho() {
        let l_chirho = LemmaIdChirho::new_chirho("agape");
        assert_eq!(l_chirho.as_str_chirho(), "agape");
    }

    #[test]
    fn test_lemma_display_chirho() {
        let l_chirho = LemmaIdChirho::new_chirho("logos");
        assert_eq!(format!("{}", l_chirho), "logos");
    }

    #[test]
    fn test_lemma_equality_chirho() {
        let a_chirho = LemmaIdChirho::new_chirho("agape");
        let b_chirho = LemmaIdChirho::new_chirho("agape");
        let c_chirho = LemmaIdChirho::new_chirho("logos");
        assert_eq!(a_chirho, b_chirho);
        assert_ne!(a_chirho, c_chirho);
    }

    #[test]
    fn test_lemma_serde_chirho() {
        let l_chirho = LemmaIdChirho::new_chirho("pistis");
        let json_chirho = serde_json::to_string(&l_chirho).unwrap();
        let parsed_chirho: LemmaIdChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(l_chirho, parsed_chirho);
    }
}
