// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Data Transfer Objects — stable, serializable types for the integration API.
//!
//! These DTOs are the contract between the engine and external consumers.
//! They are designed to be simple, serializable, and stable across versions.

use serde::{Deserialize, Serialize};

/// Information about an available module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfoChirho {
    pub name_chirho: String,
    pub description_chirho: String,
    pub module_type_chirho: String,
    pub language_chirho: String,
    pub versification_chirho: String,
}

/// A single verse with its reference and text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerseTextChirho {
    pub book_chirho: String,
    pub chapter_chirho: u16,
    pub verse_chirho: u16,
    pub text_chirho: String,
}

/// A chapter's worth of verses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterTextChirho {
    pub module_chirho: String,
    pub book_chirho: String,
    pub chapter_chirho: u16,
    pub verses_chirho: Vec<VerseTextChirho>,
}

/// A search result entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHitDtoChirho {
    pub book_chirho: String,
    pub chapter_chirho: u16,
    pub verse_chirho: u16,
    pub text_chirho: String,
    pub score_chirho: f32,
}

/// Complete search response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponseChirho {
    pub query_chirho: String,
    pub module_chirho: String,
    pub total_hits_chirho: u64,
    pub hits_chirho: Vec<SearchHitDtoChirho>,
    pub execution_time_ms_chirho: f64,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_module_info_serde_chirho() {
        let info_chirho = ModuleInfoChirho {
            name_chirho: "KJV".to_string(),
            description_chirho: "King James Version".to_string(),
            module_type_chirho: "Bible".to_string(),
            language_chirho: "en".to_string(),
            versification_chirho: "KJV".to_string(),
        };
        let json_chirho = serde_json::to_string(&info_chirho).unwrap();
        let parsed_chirho: ModuleInfoChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.name_chirho, "KJV");
        assert_eq!(parsed_chirho.language_chirho, "en");
    }

    #[test]
    fn test_verse_text_serde_chirho() {
        let verse_chirho = VerseTextChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 3,
            verse_chirho: 16,
            text_chirho: "For God so loved the world".to_string(),
        };
        let json_chirho = serde_json::to_string(&verse_chirho).unwrap();
        let parsed_chirho: VerseTextChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.book_chirho, "John");
        assert_eq!(parsed_chirho.chapter_chirho, 3);
        assert_eq!(parsed_chirho.verse_chirho, 16);
    }

    #[test]
    fn test_chapter_text_serde_chirho() {
        let chapter_chirho = ChapterTextChirho {
            module_chirho: "KJV".to_string(),
            book_chirho: "Genesis".to_string(),
            chapter_chirho: 1,
            verses_chirho: vec![
                VerseTextChirho {
                    book_chirho: "Genesis".to_string(),
                    chapter_chirho: 1,
                    verse_chirho: 1,
                    text_chirho: "In the beginning God created the heaven and the earth.".to_string(),
                },
                VerseTextChirho {
                    book_chirho: "Genesis".to_string(),
                    chapter_chirho: 1,
                    verse_chirho: 2,
                    text_chirho: "And the earth was without form, and void.".to_string(),
                },
            ],
        };
        let json_chirho = serde_json::to_string(&chapter_chirho).unwrap();
        let parsed_chirho: ChapterTextChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.module_chirho, "KJV");
        assert_eq!(parsed_chirho.verses_chirho.len(), 2);
    }

    #[test]
    fn test_search_hit_dto_serde_chirho() {
        let hit_chirho = SearchHitDtoChirho {
            book_chirho: "Romans".to_string(),
            chapter_chirho: 3,
            verse_chirho: 23,
            text_chirho: "For all have sinned".to_string(),
            score_chirho: 0.95,
        };
        let json_chirho = serde_json::to_string(&hit_chirho).unwrap();
        let parsed_chirho: SearchHitDtoChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.score_chirho, 0.95);
    }

    #[test]
    fn test_search_response_serde_chirho() {
        let response_chirho = SearchResponseChirho {
            query_chirho: "love AND mercy".to_string(),
            module_chirho: "KJV".to_string(),
            total_hits_chirho: 42,
            hits_chirho: vec![SearchHitDtoChirho {
                book_chirho: "Psalms".to_string(),
                chapter_chirho: 23,
                verse_chirho: 6,
                text_chirho: "mercy shall follow me".to_string(),
                score_chirho: 0.88,
            }],
            execution_time_ms_chirho: 12.5,
        };
        let json_chirho = serde_json::to_string(&response_chirho).unwrap();
        let parsed_chirho: SearchResponseChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.total_hits_chirho, 42);
        assert_eq!(parsed_chirho.hits_chirho.len(), 1);
    }

    #[test]
    fn test_search_response_empty_chirho() {
        let response_chirho = SearchResponseChirho {
            query_chirho: "xyzzy".to_string(),
            module_chirho: "KJV".to_string(),
            total_hits_chirho: 0,
            hits_chirho: Vec::new(),
            execution_time_ms_chirho: 0.1,
        };
        assert!(response_chirho.hits_chirho.is_empty());
    }

    #[test]
    fn test_module_info_clone_chirho() {
        let info_chirho = ModuleInfoChirho {
            name_chirho: "ESV".to_string(),
            description_chirho: "English Standard Version".to_string(),
            module_type_chirho: "Bible".to_string(),
            language_chirho: "en".to_string(),
            versification_chirho: "KJV".to_string(),
        };
        let cloned_chirho = info_chirho.clone();
        assert_eq!(info_chirho.name_chirho, cloned_chirho.name_chirho);
    }
}
