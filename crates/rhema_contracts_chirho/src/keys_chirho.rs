// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Verse reference and key types — the canonical way to address scripture.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated verse reference (book, chapter, verse).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerseRefChirho {
    pub book_chirho: String,
    pub chapter_chirho: u16,
    pub verse_chirho: u16,
}

impl VerseRefChirho {
    pub fn new_chirho(
        book_chirho: &str,
        chapter_chirho: u16,
        verse_chirho: u16,
    ) -> Result<Self, crate::error_chirho::ContractErrorChirho> {
        if book_chirho.is_empty() {
            return Err(crate::error_chirho::ContractErrorChirho::EmptyIdentifierChirho {
                field_chirho: "book".to_string(),
            });
        }
        if chapter_chirho == 0 {
            return Err(crate::error_chirho::ContractErrorChirho::OutOfRangeChirho {
                field_chirho: "chapter".to_string(),
                value_chirho: 0,
                min_chirho: 1,
                max_chirho: 150,
            });
        }
        Ok(Self {
            book_chirho: book_chirho.to_string(),
            chapter_chirho,
            verse_chirho,
        })
    }
}

impl fmt::Display for VerseRefChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.verse_chirho == 0 {
            write!(f_chirho, "{} {}", self.book_chirho, self.chapter_chirho)
        } else {
            write!(
                f_chirho,
                "{} {}:{}",
                self.book_chirho, self.chapter_chirho, self.verse_chirho
            )
        }
    }
}

/// A passage range (from one verse to another).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PassageRangeChirho {
    pub start_chirho: VerseRefChirho,
    pub end_chirho: VerseRefChirho,
}

/// Testament classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TestamentChirho {
    OldTestamentChirho,
    NewTestamentChirho,
}

/// Search scope constraint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScopeChirho {
    EntireBibleChirho,
    TestamentChirho(TestamentChirho),
    BookChirho(String),
    PassageChirho(PassageRangeChirho),
    BooksChirho(Vec<String>),
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    // ── VerseRefChirho ──────────────────────────────────────────────

    #[test]
    fn test_verse_ref_valid_chirho() {
        let vr_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        assert_eq!(vr_chirho.book_chirho, "John");
        assert_eq!(vr_chirho.chapter_chirho, 3);
        assert_eq!(vr_chirho.verse_chirho, 16);
    }

    #[test]
    fn test_verse_ref_empty_book_rejected_chirho() {
        assert!(VerseRefChirho::new_chirho("", 1, 1).is_err());
    }

    #[test]
    fn test_verse_ref_zero_chapter_rejected_chirho() {
        assert!(VerseRefChirho::new_chirho("Genesis", 0, 1).is_err());
    }

    #[test]
    fn test_verse_ref_zero_verse_allowed_chirho() {
        // Verse 0 is allowed (represents chapter-level reference)
        let vr_chirho = VerseRefChirho::new_chirho("Genesis", 1, 0).unwrap();
        assert_eq!(vr_chirho.verse_chirho, 0);
    }

    #[test]
    fn test_verse_ref_display_with_verse_chirho() {
        let vr_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        assert_eq!(format!("{}", vr_chirho), "John 3:16");
    }

    #[test]
    fn test_verse_ref_display_chapter_only_chirho() {
        let vr_chirho = VerseRefChirho::new_chirho("Genesis", 1, 0).unwrap();
        assert_eq!(format!("{}", vr_chirho), "Genesis 1");
    }

    #[test]
    fn test_verse_ref_equality_chirho() {
        let a_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        let b_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        let c_chirho = VerseRefChirho::new_chirho("John", 3, 17).unwrap();
        assert_eq!(a_chirho, b_chirho);
        assert_ne!(a_chirho, c_chirho);
    }

    #[test]
    fn test_verse_ref_hash_chirho() {
        let mut set_chirho = std::collections::HashSet::new();
        set_chirho.insert(VerseRefChirho::new_chirho("Ps", 23, 1).unwrap());
        assert!(set_chirho.contains(&VerseRefChirho::new_chirho("Ps", 23, 1).unwrap()));
        assert!(!set_chirho.contains(&VerseRefChirho::new_chirho("Ps", 23, 2).unwrap()));
    }

    #[test]
    fn test_verse_ref_serde_roundtrip_chirho() {
        let vr_chirho = VerseRefChirho::new_chirho("I Corinthians", 13, 4).unwrap();
        let json_chirho = serde_json::to_string(&vr_chirho).unwrap();
        let parsed_chirho: VerseRefChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(vr_chirho, parsed_chirho);
    }

    #[test]
    fn test_verse_ref_multiword_book_chirho() {
        let vr_chirho = VerseRefChirho::new_chirho("Song of Solomon", 2, 4).unwrap();
        assert_eq!(format!("{}", vr_chirho), "Song of Solomon 2:4");
    }

    #[test]
    fn test_verse_ref_high_chapter_chirho() {
        // Psalms goes up to 150
        let vr_chirho = VerseRefChirho::new_chirho("Psalms", 150, 6).unwrap();
        assert_eq!(vr_chirho.chapter_chirho, 150);
    }

    // ── PassageRangeChirho ──────────────────────────────────────────

    #[test]
    fn test_passage_range_chirho() {
        let range_chirho = PassageRangeChirho {
            start_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            end_chirho: VerseRefChirho::new_chirho("John", 3, 18).unwrap(),
        };
        assert_eq!(range_chirho.start_chirho.verse_chirho, 16);
        assert_eq!(range_chirho.end_chirho.verse_chirho, 18);
    }

    #[test]
    fn test_passage_range_serde_chirho() {
        let range_chirho = PassageRangeChirho {
            start_chirho: VerseRefChirho::new_chirho("Gen", 1, 1).unwrap(),
            end_chirho: VerseRefChirho::new_chirho("Gen", 1, 5).unwrap(),
        };
        let json_chirho = serde_json::to_string(&range_chirho).unwrap();
        let parsed_chirho: PassageRangeChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(range_chirho, parsed_chirho);
    }

    // ── TestamentChirho ──────────────────────────────────────────────

    #[test]
    fn test_testament_equality_chirho() {
        assert_eq!(TestamentChirho::OldTestamentChirho, TestamentChirho::OldTestamentChirho);
        assert_ne!(TestamentChirho::OldTestamentChirho, TestamentChirho::NewTestamentChirho);
    }

    // ── ScopeChirho ──────────────────────────────────────────────────

    #[test]
    fn test_scope_book_chirho() {
        let scope_chirho = ScopeChirho::BookChirho("Romans".to_string());
        assert_eq!(scope_chirho, ScopeChirho::BookChirho("Romans".to_string()));
    }

    #[test]
    fn test_scope_books_chirho() {
        let scope_chirho = ScopeChirho::BooksChirho(vec!["Matt".to_string(), "Mark".to_string()]);
        if let ScopeChirho::BooksChirho(books_chirho) = &scope_chirho {
            assert_eq!(books_chirho.len(), 2);
        } else {
            panic!("Expected BooksChirho");
        }
    }

    #[test]
    fn test_scope_serde_chirho() {
        let scope_chirho = ScopeChirho::TestamentChirho(TestamentChirho::NewTestamentChirho);
        let json_chirho = serde_json::to_string(&scope_chirho).unwrap();
        let parsed_chirho: ScopeChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(scope_chirho, parsed_chirho);
    }
}
