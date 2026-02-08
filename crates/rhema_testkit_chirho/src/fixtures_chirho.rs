// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Test fixtures — well-known verse references, expected texts, and query/result pairs.

use serde::{Deserialize, Serialize};

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

/// A verse fixture with reference and expected text snippet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerseFixtureChirho {
    /// Human-readable description of this fixture.
    pub label_chirho: String,
    /// The verse reference.
    pub verse_ref_chirho: VerseRefChirho,
    /// Expected text substring (must appear in the rendered verse).
    pub expected_substring_chirho: String,
    /// Module this fixture applies to (e.g., "KJV").
    pub module_chirho: String,
}

/// A query fixture with expected result properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFixtureChirho {
    /// Human-readable description.
    pub label_chirho: String,
    /// The query string.
    pub query_text_chirho: String,
    /// Module to run against.
    pub module_chirho: String,
    /// Minimum expected hit count.
    pub min_hits_chirho: u64,
    /// Maximum expected hit count (None = no upper bound).
    pub max_hits_chirho: Option<u64>,
    /// Verse references that MUST appear in results.
    pub must_contain_chirho: Vec<VerseRefChirho>,
}

/// Generate the standard set of verse fixtures for the KJV module.
pub fn kjv_verse_fixtures_chirho() -> Vec<VerseFixtureChirho> {
    vec![
        VerseFixtureChirho {
            label_chirho: "John 3:16 — most quoted verse".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            expected_substring_chirho: "God so loved the world".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Genesis 1:1 — first verse of the Bible".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Genesis", 1, 1).unwrap(),
            expected_substring_chirho: "In the beginning God created".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Psalm 23:1 — the Lord is my shepherd".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Psalms", 23, 1).unwrap(),
            expected_substring_chirho: "The LORD is my shepherd".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Romans 3:23 — all have sinned".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Romans", 3, 23).unwrap(),
            expected_substring_chirho: "all have sinned".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Romans 6:23 — wages of sin".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Romans", 6, 23).unwrap(),
            expected_substring_chirho: "wages of sin".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Ephesians 2:8 — by grace through faith".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Ephesians", 2, 8).unwrap(),
            expected_substring_chirho: "by grace".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Proverbs 3:5 — trust in the Lord".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Proverbs", 3, 5).unwrap(),
            expected_substring_chirho: "Trust in the LORD".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Isaiah 53:5 — by his stripes".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Isaiah", 53, 5).unwrap(),
            expected_substring_chirho: "with his stripes".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Matthew 28:19 — great commission".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Matthew", 28, 19).unwrap(),
            expected_substring_chirho: "teach all nations".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Revelation 22:21 — last verse of the Bible".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Revelation of John", 22, 21).unwrap(),
            expected_substring_chirho: "grace of our Lord Jesus Christ".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Philippians 4:13 — I can do all things".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Philippians", 4, 13).unwrap(),
            expected_substring_chirho: "I can do all things".to_string(),
            module_chirho: "KJV".to_string(),
        },
        VerseFixtureChirho {
            label_chirho: "Jeremiah 29:11 — plans to prosper".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("Jeremiah", 29, 11).unwrap(),
            expected_substring_chirho: "thoughts of peace".to_string(),
            module_chirho: "KJV".to_string(),
        },
    ]
}

/// Generate the standard set of query fixtures for the KJV module.
pub fn kjv_query_fixtures_chirho() -> Vec<QueryFixtureChirho> {
    vec![
        QueryFixtureChirho {
            label_chirho: "Simple term: love".to_string(),
            query_text_chirho: "love".to_string(),
            module_chirho: "KJV".to_string(),
            min_hits_chirho: 200,
            max_hits_chirho: None,
            must_contain_chirho: vec![
                VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            ],
        },
        QueryFixtureChirho {
            label_chirho: "Phrase: God so loved".to_string(),
            query_text_chirho: "\"God so loved\"".to_string(),
            module_chirho: "KJV".to_string(),
            min_hits_chirho: 1,
            max_hits_chirho: Some(5),
            must_contain_chirho: vec![
                VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            ],
        },
        QueryFixtureChirho {
            label_chirho: "Boolean: grace AND mercy".to_string(),
            query_text_chirho: "grace AND mercy".to_string(),
            module_chirho: "KJV".to_string(),
            min_hits_chirho: 1,
            max_hits_chirho: None,
            must_contain_chirho: vec![],
        },
        QueryFixtureChirho {
            label_chirho: "Scoped: [Romans] justified".to_string(),
            query_text_chirho: "[Romans] justified".to_string(),
            module_chirho: "KJV".to_string(),
            min_hits_chirho: 1,
            max_hits_chirho: None,
            must_contain_chirho: vec![],
        },
        QueryFixtureChirho {
            label_chirho: "Strong's: G26 (agape)".to_string(),
            query_text_chirho: "strong:G26".to_string(),
            module_chirho: "KJV".to_string(),
            min_hits_chirho: 50,
            max_hits_chirho: None,
            must_contain_chirho: vec![],
        },
    ]
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_kjv_verse_fixtures_non_empty_chirho() {
        let fixtures_chirho = kjv_verse_fixtures_chirho();
        assert!(!fixtures_chirho.is_empty());
        assert!(fixtures_chirho.len() >= 10);
    }

    #[test]
    fn test_kjv_query_fixtures_non_empty_chirho() {
        let fixtures_chirho = kjv_query_fixtures_chirho();
        assert!(!fixtures_chirho.is_empty());
        assert!(fixtures_chirho.len() >= 5);
    }

    #[test]
    fn test_verse_fixture_serialization_chirho() {
        let fixtures_chirho = kjv_verse_fixtures_chirho();
        let json_chirho = serde_json::to_string_pretty(&fixtures_chirho).unwrap();
        assert!(json_chirho.contains("John 3:16"));

        // Roundtrip
        let deserialized_chirho: Vec<VerseFixtureChirho> =
            serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(deserialized_chirho.len(), fixtures_chirho.len());
    }

    #[test]
    fn test_query_fixture_serialization_chirho() {
        let fixtures_chirho = kjv_query_fixtures_chirho();
        let json_chirho = serde_json::to_string_pretty(&fixtures_chirho).unwrap();
        assert!(json_chirho.contains("love"));

        let deserialized_chirho: Vec<QueryFixtureChirho> =
            serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(deserialized_chirho.len(), fixtures_chirho.len());
    }
}
