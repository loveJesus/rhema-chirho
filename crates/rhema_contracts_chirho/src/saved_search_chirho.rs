// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Saved search types — persisted queries for quick re-use.

use serde::{Deserialize, Serialize};

/// A saved search query that can be persisted and re-used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchChirho {
    /// Unique identifier for this saved search.
    pub id_chirho: String,
    /// Human-readable name for the search.
    pub name_chirho: String,
    /// The query text (parseable by QueryParserChirho).
    pub query_text_chirho: String,
    /// Timestamp when first saved (ISO 8601).
    pub created_at_chirho: String,
    /// Timestamp of last use (ISO 8601).
    pub last_used_at_chirho: String,
    /// Number of times this search has been executed.
    pub usage_count_chirho: u64,
    /// Comma-separated tags for categorization.
    pub tags_chirho: String,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_saved_search_serde_roundtrip_chirho() {
        let search_chirho = SavedSearchChirho {
            id_chirho: "abc123".to_string(),
            name_chirho: "Love verses".to_string(),
            query_text_chirho: "love AND world".to_string(),
            created_at_chirho: "2025-01-01T00:00:00Z".to_string(),
            last_used_at_chirho: "2025-06-01T12:00:00Z".to_string(),
            usage_count_chirho: 5,
            tags_chirho: "love,world,johannine".to_string(),
        };

        let json_chirho = serde_json::to_string(&search_chirho).unwrap();
        let parsed_chirho: SavedSearchChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.id_chirho, "abc123");
        assert_eq!(parsed_chirho.name_chirho, "Love verses");
        assert_eq!(parsed_chirho.query_text_chirho, "love AND world");
        assert_eq!(parsed_chirho.usage_count_chirho, 5);
        assert_eq!(parsed_chirho.tags_chirho, "love,world,johannine");
    }
}
