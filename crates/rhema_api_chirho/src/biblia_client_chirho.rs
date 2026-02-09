// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Biblia.com API client for accessing Bible texts via their REST API.
//!
//! API docs: <https://api.biblia.com/docs/>
//!
//! All methods are async and return deserialized DTOs.
//! Tests use mock JSON data and do not make HTTP calls.

use serde::{Deserialize, Serialize};

/// Biblia API client.
pub struct BibliaClientChirho {
    /// API key for authentication.
    pub api_key_chirho: String,
    /// Base URL (default: `https://api.biblia.com/v1`).
    pub base_url_chirho: String,
}

/// A Biblia search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliaSearchResultChirho {
    pub passage_chirho: String,
    pub text_chirho: String,
}

/// Search response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliaSearchResponseChirho {
    pub results_chirho: Vec<BibliaSearchResultChirho>,
    pub total_chirho: u64,
}

/// A passage lookup result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliaPassageChirho {
    pub text_chirho: String,
}

/// A Bible available on Biblia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliaBibleChirho {
    pub bible_chirho: String,
    pub title_chirho: String,
    pub abbreviation_chirho: String,
}

/// Available Bibles response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibliaBiblesResponseChirho {
    pub bibles_chirho: Vec<BibliaBibleChirho>,
}

impl BibliaClientChirho {
    /// Create a new client with API key and default base URL.
    pub fn new_chirho(api_key_chirho: &str) -> Self {
        Self {
            api_key_chirho: api_key_chirho.to_string(),
            base_url_chirho: "https://api.biblia.com/v1".to_string(),
        }
    }

    /// Create a new client with a custom base URL (for testing).
    pub fn with_base_url_chirho(api_key_chirho: &str, base_url_chirho: &str) -> Self {
        Self {
            api_key_chirho: api_key_chirho.to_string(),
            base_url_chirho: base_url_chirho.to_string(),
        }
    }

    /// Build the full URL for an endpoint.
    pub fn url_chirho(&self, path_chirho: &str) -> String {
        format!("{}{}", self.base_url_chirho, path_chirho)
    }

    /// Parse a search response from JSON.
    pub fn parse_search_response_chirho(
        json_chirho: &str,
    ) -> Result<BibliaSearchResponseChirho, serde_json::Error> {
        serde_json::from_str(json_chirho)
    }

    /// Parse a passage response from JSON.
    pub fn parse_passage_response_chirho(
        json_chirho: &str,
    ) -> Result<BibliaPassageChirho, serde_json::Error> {
        serde_json::from_str(json_chirho)
    }

    /// Parse available bibles response from JSON.
    pub fn parse_bibles_response_chirho(
        json_chirho: &str,
    ) -> Result<BibliaBiblesResponseChirho, serde_json::Error> {
        serde_json::from_str(json_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_client_new_chirho() {
        let client_chirho = BibliaClientChirho::new_chirho("test-key");
        assert_eq!(client_chirho.api_key_chirho, "test-key");
        assert_eq!(
            client_chirho.base_url_chirho,
            "https://api.biblia.com/v1"
        );
    }

    #[test]
    fn test_client_custom_url_chirho() {
        let client_chirho =
            BibliaClientChirho::with_base_url_chirho("key", "http://localhost:8080");
        assert_eq!(client_chirho.base_url_chirho, "http://localhost:8080");
    }

    #[test]
    fn test_url_building_chirho() {
        let client_chirho = BibliaClientChirho::new_chirho("key");
        assert_eq!(
            client_chirho.url_chirho("/bible/search"),
            "https://api.biblia.com/v1/bible/search"
        );
    }

    #[test]
    fn test_parse_search_response_chirho() {
        let json_chirho = r#"{
            "results_chirho": [
                {"passage_chirho": "John 3:16", "text_chirho": "For God so loved the world..."}
            ],
            "total_chirho": 1
        }"#;
        let response_chirho =
            BibliaClientChirho::parse_search_response_chirho(json_chirho).unwrap();
        assert_eq!(response_chirho.total_chirho, 1);
        assert_eq!(response_chirho.results_chirho.len(), 1);
        assert_eq!(response_chirho.results_chirho[0].passage_chirho, "John 3:16");
    }

    #[test]
    fn test_parse_passage_response_chirho() {
        let json_chirho = r#"{"text_chirho": "For God so loved the world..."}"#;
        let passage_chirho =
            BibliaClientChirho::parse_passage_response_chirho(json_chirho).unwrap();
        assert!(passage_chirho.text_chirho.contains("loved"));
    }

    #[test]
    fn test_parse_bibles_response_chirho() {
        let json_chirho = r#"{
            "bibles_chirho": [
                {"bible_chirho": "KJV", "title_chirho": "King James Version", "abbreviation_chirho": "KJV"},
                {"bible_chirho": "ESV", "title_chirho": "English Standard Version", "abbreviation_chirho": "ESV"}
            ]
        }"#;
        let response_chirho =
            BibliaClientChirho::parse_bibles_response_chirho(json_chirho).unwrap();
        assert_eq!(response_chirho.bibles_chirho.len(), 2);
        assert_eq!(response_chirho.bibles_chirho[0].bible_chirho, "KJV");
    }

    #[test]
    fn test_search_result_serde_roundtrip_chirho() {
        let result_chirho = BibliaSearchResultChirho {
            passage_chirho: "Romans 8:28".to_string(),
            text_chirho: "And we know...".to_string(),
        };
        let json_chirho = serde_json::to_string(&result_chirho).unwrap();
        let parsed_chirho: BibliaSearchResultChirho =
            serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.passage_chirho, "Romans 8:28");
    }

    #[test]
    fn test_bible_dto_serde_chirho() {
        let bible_chirho = BibliaBibleChirho {
            bible_chirho: "NASB".to_string(),
            title_chirho: "New American Standard Bible".to_string(),
            abbreviation_chirho: "NASB".to_string(),
        };
        let json_chirho = serde_json::to_string(&bible_chirho).unwrap();
        assert!(json_chirho.contains("NASB"));
    }
}
