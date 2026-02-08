// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Token extraction — converts raw verse text (possibly with OSIS/ThML markup)
//! into a sequence of [`CanonicalTokenChirho`] with Strong's numbers, morphology,
//! and lemma data extracted from inline markup.

use rhema_contracts_chirho::corpus_chirho::{CanonicalTokenChirho, LanguageChirho};
use rhema_contracts_chirho::ids_chirho::{
    LemmaIdChirho, StrongNumberChirho, TokenPositionChirho,
};
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_contracts_chirho::morphology_chirho::MorphCodeChirho;

use crate::error_chirho::IngestErrorChirho;

/// Extracts canonical tokens from raw verse text.
///
/// Handles OSIS XML markup to pull Strong's numbers and morphology codes
/// from `<w>` elements. Falls back to plain whitespace tokenization for
/// untagged text.
pub struct TokenExtractorChirho {
    /// Running position counter (monotonically increasing across entire corpus).
    next_position_chirho: u64,
}

impl TokenExtractorChirho {
    /// Create a new extractor starting at position 0.
    pub fn new_chirho() -> Self {
        Self {
            next_position_chirho: 0,
        }
    }

    /// Create a new extractor starting at a given position.
    pub fn with_start_position_chirho(start_chirho: u64) -> Self {
        Self {
            next_position_chirho: start_chirho,
        }
    }

    /// Extract tokens from a verse's raw text.
    ///
    /// Attempts OSIS `<w>` element parsing first. If no markup is found,
    /// falls back to whitespace tokenization.
    pub fn extract_verse_chirho(
        &mut self,
        raw_text_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
        language_chirho: LanguageChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, IngestErrorChirho> {
        let cleaned_chirho = strip_non_word_markup_chirho(raw_text_chirho);

        if cleaned_chirho.contains("<w ") || cleaned_chirho.contains("<W ") {
            self.extract_osis_tagged_chirho(&cleaned_chirho, verse_ref_chirho, language_chirho)
        } else {
            self.extract_plain_chirho(&cleaned_chirho, verse_ref_chirho, language_chirho)
        }
    }

    /// Extract tokens from OSIS-tagged text with `<w>` elements carrying
    /// Strong's and morphology attributes.
    fn extract_osis_tagged_chirho(
        &mut self,
        text_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
        language_chirho: LanguageChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, IngestErrorChirho> {
        let mut tokens_chirho = Vec::new();
        let mut word_index_chirho: u16 = 0;

        // Simple state-machine parse for <w lemma="..." morph="...">word</w>
        let mut remaining_chirho = text_chirho;
        while let Some(start_chirho) = remaining_chirho
            .find("<w ")
            .or_else(|| remaining_chirho.find("<W "))
        {
            remaining_chirho = &remaining_chirho[start_chirho..];

            // Find closing >
            let Some(tag_end_chirho) = remaining_chirho.find('>') else {
                break;
            };
            let tag_chirho = &remaining_chirho[..tag_end_chirho + 1];

            // Find </w>
            let content_start_chirho = tag_end_chirho + 1;
            let close_chirho = remaining_chirho[content_start_chirho..]
                .find("</w>")
                .or_else(|| remaining_chirho[content_start_chirho..].find("</W>"));

            let (surface_chirho, advance_chirho) = if let Some(close_offset_chirho) = close_chirho
            {
                let surface_chirho =
                    &remaining_chirho[content_start_chirho..content_start_chirho + close_offset_chirho];
                (
                    surface_chirho,
                    content_start_chirho + close_offset_chirho + 4,
                )
            } else {
                // Self-closing or malformed — skip
                remaining_chirho = &remaining_chirho[tag_end_chirho + 1..];
                continue;
            };

            let strong_chirho = extract_attr_chirho(tag_chirho, "lemma")
                .and_then(|s_chirho| normalize_strong_chirho(&s_chirho));

            let morph_chirho = extract_attr_chirho(tag_chirho, "morph")
                .map(|m_chirho| MorphCodeChirho::new_chirho(&strip_morph_prefix_chirho(&m_chirho)));

            let lemma_chirho = extract_attr_chirho(tag_chirho, "lemma")
                .map(|l_chirho| LemmaIdChirho::new_chirho(&l_chirho));

            let clean_surface_chirho = strip_all_tags_chirho(surface_chirho)
                .trim()
                .to_string();

            if !clean_surface_chirho.is_empty() {
                tokens_chirho.push(CanonicalTokenChirho {
                    position_chirho: TokenPositionChirho(self.next_position_chirho),
                    surface_chirho: clean_surface_chirho,
                    lemma_chirho,
                    strong_chirho,
                    morph_chirho,
                    language_chirho,
                    verse_ref_chirho: verse_ref_chirho.clone(),
                    word_index_chirho,
                });
                self.next_position_chirho += 1;
                word_index_chirho += 1;
            }

            remaining_chirho = &remaining_chirho[advance_chirho..];
        }

        // If OSIS parsing found no tokens, fall back to plain
        if tokens_chirho.is_empty() {
            return self.extract_plain_chirho(text_chirho, verse_ref_chirho, language_chirho);
        }

        Ok(tokens_chirho)
    }

    /// Plain whitespace tokenization for untagged text.
    fn extract_plain_chirho(
        &mut self,
        text_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
        language_chirho: LanguageChirho,
    ) -> Result<Vec<CanonicalTokenChirho>, IngestErrorChirho> {
        let stripped_chirho = strip_all_tags_chirho(text_chirho);
        let mut tokens_chirho = Vec::new();

        for (idx_chirho, word_chirho) in stripped_chirho.split_whitespace().enumerate() {
            let cleaned_chirho = word_chirho
                .trim_matches(|c_chirho: char| c_chirho.is_ascii_punctuation())
                .to_string();

            if cleaned_chirho.is_empty() {
                continue;
            }

            tokens_chirho.push(CanonicalTokenChirho {
                position_chirho: TokenPositionChirho(self.next_position_chirho),
                surface_chirho: cleaned_chirho,
                lemma_chirho: None,
                strong_chirho: None,
                morph_chirho: None,
                language_chirho,
                verse_ref_chirho: verse_ref_chirho.clone(),
                word_index_chirho: idx_chirho as u16,
            });
            self.next_position_chirho += 1;
        }

        Ok(tokens_chirho)
    }

    /// Current position counter value.
    pub fn current_position_chirho(&self) -> u64 {
        self.next_position_chirho
    }
}

/// Extract an XML attribute value from a tag string.
fn extract_attr_chirho(tag_chirho: &str, attr_name_chirho: &str) -> Option<String> {
    let pattern_chirho = format!("{}=\"", attr_name_chirho);
    let start_chirho = tag_chirho.find(&pattern_chirho)?;
    let value_start_chirho = start_chirho + pattern_chirho.len();
    let rest_chirho = &tag_chirho[value_start_chirho..];
    let end_chirho = rest_chirho.find('"')?;
    Some(rest_chirho[..end_chirho].to_string())
}

/// Normalize a Strong's number from OSIS lemma attribute format.
/// OSIS uses "strong:H430" or "strong:G26" or just "H430".
fn normalize_strong_chirho(raw_chirho: &str) -> Option<StrongNumberChirho> {
    let cleaned_chirho = raw_chirho
        .strip_prefix("strong:")
        .unwrap_or(raw_chirho)
        .trim();

    // Handle multiple Strong's numbers (take first)
    let first_chirho = cleaned_chirho.split_whitespace().next()?;

    StrongNumberChirho::new_chirho(first_chirho).ok()
}

/// Strip morphology system prefix (e.g., "robinson:" or "oshm:").
fn strip_morph_prefix_chirho(morph_chirho: &str) -> String {
    if let Some(idx_chirho) = morph_chirho.find(':') {
        morph_chirho[idx_chirho + 1..].to_string()
    } else {
        morph_chirho.to_string()
    }
}

/// Strip all XML/HTML tags from a string, leaving only text content.
fn strip_all_tags_chirho(text_chirho: &str) -> String {
    let mut result_chirho = String::with_capacity(text_chirho.len());
    let mut in_tag_chirho = false;

    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '<' => in_tag_chirho = true,
            '>' => in_tag_chirho = false,
            _ if !in_tag_chirho => result_chirho.push(ch_chirho),
            _ => {}
        }
    }

    result_chirho
}

/// Strip non-word markup (notes, references, etc.) but preserve <w> elements.
fn strip_non_word_markup_chirho(text_chirho: &str) -> String {
    // Remove <note>...</note>, <milestone.../>, <title>...</title>
    // but keep <w ...>...</w> intact.
    // Simple approach: strip known non-word tags
    let mut result_chirho = text_chirho.to_string();

    // Remove note elements
    while let Some(start_chirho) = result_chirho.find("<note") {
        if let Some(end_chirho) = result_chirho[start_chirho..].find("</note>") {
            result_chirho.replace_range(start_chirho..start_chirho + end_chirho + 7, "");
        } else if let Some(end_chirho) = result_chirho[start_chirho..].find("/>") {
            result_chirho.replace_range(start_chirho..start_chirho + end_chirho + 2, "");
        } else {
            break;
        }
    }

    result_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_extract_plain_chirho() {
        let mut extractor_chirho = TokenExtractorChirho::new_chirho();
        let verse_ref_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();

        let tokens_chirho = extractor_chirho
            .extract_verse_chirho(
                "For God so loved the world",
                &verse_ref_chirho,
                LanguageChirho::EnglishChirho,
            )
            .unwrap();

        assert_eq!(tokens_chirho.len(), 6);
        assert_eq!(tokens_chirho[0].surface_chirho, "For");
        assert_eq!(tokens_chirho[1].surface_chirho, "God");
        assert_eq!(tokens_chirho[5].surface_chirho, "world");
        assert_eq!(tokens_chirho[0].word_index_chirho, 0);
        assert_eq!(tokens_chirho[5].word_index_chirho, 5);
    }

    #[test]
    fn test_extract_osis_tagged_chirho() {
        let mut extractor_chirho = TokenExtractorChirho::new_chirho();
        let verse_ref_chirho = VerseRefChirho::new_chirho("Gen", 1, 1).unwrap();

        let osis_chirho = r#"<w lemma="strong:H7225" morph="oshm:HNcfsa">In the beginning</w> <w lemma="strong:H430" morph="oshm:HNcmpa">God</w>"#;

        let tokens_chirho = extractor_chirho
            .extract_verse_chirho(osis_chirho, &verse_ref_chirho, LanguageChirho::HebrewChirho)
            .unwrap();

        assert_eq!(tokens_chirho.len(), 2);
        assert_eq!(tokens_chirho[0].surface_chirho, "In the beginning");
        assert!(tokens_chirho[0].strong_chirho.is_some());
        assert_eq!(
            tokens_chirho[0].strong_chirho.as_ref().unwrap().as_str_chirho(),
            "H7225"
        );
        assert!(tokens_chirho[0].morph_chirho.is_some());
        assert_eq!(tokens_chirho[1].surface_chirho, "God");
    }

    #[test]
    fn test_strip_all_tags_chirho() {
        let input_chirho = "<p>Hello <b>world</b></p>";
        assert_eq!(strip_all_tags_chirho(input_chirho), "Hello world");
    }

    #[test]
    fn test_normalize_strong_chirho() {
        assert_eq!(
            normalize_strong_chirho("strong:H430").unwrap().as_str_chirho(),
            "H430"
        );
        assert_eq!(
            normalize_strong_chirho("G26").unwrap().as_str_chirho(),
            "G26"
        );
        assert!(normalize_strong_chirho("invalid").is_none());
    }

    #[test]
    fn test_position_counter_chirho() {
        let mut extractor_chirho = TokenExtractorChirho::new_chirho();
        let ref1_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        let ref2_chirho = VerseRefChirho::new_chirho("John", 3, 17).unwrap();

        let _ = extractor_chirho.extract_verse_chirho(
            "For God",
            &ref1_chirho,
            LanguageChirho::EnglishChirho,
        );
        assert_eq!(extractor_chirho.current_position_chirho(), 2);

        let tokens2_chirho = extractor_chirho
            .extract_verse_chirho(
                "so loved",
                &ref2_chirho,
                LanguageChirho::EnglishChirho,
            )
            .unwrap();
        assert_eq!(tokens2_chirho[0].position_chirho, TokenPositionChirho(2));
        assert_eq!(extractor_chirho.current_position_chirho(), 4);
    }
}
