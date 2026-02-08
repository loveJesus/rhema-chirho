// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! LLM-powered query expansion — enrich user queries with synonyms, Strong's, concepts.

use crate::error_chirho::{AiErrorChirho, AiResultChirho};
use crate::llm_provider_chirho::{ChatMessageChirho, LlmProviderChirho};

/// Result of a query expansion.
#[derive(Debug, Clone)]
pub struct ExpandedQueryChirho {
    /// The original user query.
    pub original_chirho: String,
    /// Expanded search terms (synonyms, related words).
    pub terms_chirho: Vec<String>,
    /// Relevant Strong's numbers discovered by the LLM.
    pub strongs_chirho: Vec<String>,
    /// Thematic concepts identified.
    pub concepts_chirho: Vec<String>,
}

/// Query expander that uses an LLM to enrich search queries.
pub struct QueryExpanderChirho<'a> {
    provider_chirho: &'a dyn LlmProviderChirho,
}

impl<'a> QueryExpanderChirho<'a> {
    pub fn new_chirho(provider_chirho: &'a dyn LlmProviderChirho) -> Self {
        Self { provider_chirho }
    }

    /// Expand a query with synonyms and related biblical terms.
    pub async fn expand_terms_chirho(
        &self,
        query_chirho: &str,
    ) -> AiResultChirho<ExpandedQueryChirho> {
        let prompt_chirho = format!(
            "Given the biblical search query: \"{query_chirho}\"\n\
             Return a JSON object with:\n\
             - \"terms\": array of 3-5 synonym/related English words for Bible search\n\
             - \"strongs\": array of relevant Strong's numbers (e.g. \"G26\", \"H2617\")\n\
             - \"concepts\": array of 1-2 thematic biblical concepts\n\
             Return ONLY valid JSON, no markdown."
        );

        let messages_chirho = vec![ChatMessageChirho {
            role_chirho: "user".to_string(),
            content_chirho: prompt_chirho,
        }];

        let response_chirho = self
            .provider_chirho
            .chat_completion_chirho(&messages_chirho, 256, 0.3)
            .await?;

        parse_expansion_response_chirho(query_chirho, &response_chirho.content_chirho)
    }

    /// Expand a query focusing on Strong's number discovery.
    pub async fn expand_strongs_chirho(
        &self,
        query_chirho: &str,
    ) -> AiResultChirho<ExpandedQueryChirho> {
        let prompt_chirho = format!(
            "For the biblical concept: \"{query_chirho}\"\n\
             Return a JSON object with:\n\
             - \"strongs\": array of all relevant Strong's numbers \
               (Greek G-numbers and Hebrew H-numbers)\n\
             - \"terms\": array of the English glosses for each Strong's number\n\
             - \"concepts\": empty array\n\
             Return ONLY valid JSON, no markdown."
        );

        let messages_chirho = vec![ChatMessageChirho {
            role_chirho: "user".to_string(),
            content_chirho: prompt_chirho,
        }];

        let response_chirho = self
            .provider_chirho
            .chat_completion_chirho(&messages_chirho, 256, 0.3)
            .await?;

        parse_expansion_response_chirho(query_chirho, &response_chirho.content_chirho)
    }
}

/// Parse the LLM response JSON into an `ExpandedQueryChirho`.
fn parse_expansion_response_chirho(
    original_chirho: &str,
    response_chirho: &str,
) -> AiResultChirho<ExpandedQueryChirho> {
    let parsed_chirho: serde_json::Value = serde_json::from_str(response_chirho).map_err(|e_chirho| {
        AiErrorChirho::QueryExpansionChirho {
            reason_chirho: format!("Failed to parse LLM response as JSON: {e_chirho}"),
        }
    })?;

    let terms_chirho = parsed_chirho["terms"]
        .as_array()
        .map(|a_chirho| {
            a_chirho
                .iter()
                .filter_map(|v_chirho| v_chirho.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let strongs_chirho = parsed_chirho["strongs"]
        .as_array()
        .map(|a_chirho| {
            a_chirho
                .iter()
                .filter_map(|v_chirho| v_chirho.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let concepts_chirho = parsed_chirho["concepts"]
        .as_array()
        .map(|a_chirho| {
            a_chirho
                .iter()
                .filter_map(|v_chirho| v_chirho.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(ExpandedQueryChirho {
        original_chirho: original_chirho.to_string(),
        terms_chirho,
        strongs_chirho,
        concepts_chirho,
    })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::llm_provider_chirho::MockLlmProviderChirho;

    #[tokio::test]
    async fn test_expand_terms_chirho() {
        let mock_response_chirho = r#"{"terms":["love","charity","agape","affection"],"strongs":["G26","G5368"],"concepts":["divine love"]}"#;

        let provider_chirho =
            MockLlmProviderChirho::new_chirho().with_response_chirho(mock_response_chirho);
        let expander_chirho = QueryExpanderChirho::new_chirho(&provider_chirho);
        let result_chirho = expander_chirho.expand_terms_chirho("love").await.unwrap();

        assert_eq!(result_chirho.original_chirho, "love");
        assert!(!result_chirho.terms_chirho.is_empty());
        assert!(result_chirho.terms_chirho.contains(&"agape".to_string()));
        assert!(result_chirho.strongs_chirho.contains(&"G26".to_string()));
    }

    #[tokio::test]
    async fn test_expand_strongs_chirho() {
        let mock_response_chirho =
            r#"{"strongs":["G26","H2617"],"terms":["love","lovingkindness"],"concepts":[]}"#;

        let provider_chirho =
            MockLlmProviderChirho::new_chirho().with_response_chirho(mock_response_chirho);
        let expander_chirho = QueryExpanderChirho::new_chirho(&provider_chirho);
        let result_chirho = expander_chirho
            .expand_strongs_chirho("God's love")
            .await
            .unwrap();

        assert!(result_chirho.strongs_chirho.contains(&"H2617".to_string()));
    }

    #[test]
    fn test_parse_expansion_bad_json_chirho() {
        let result_chirho = parse_expansion_response_chirho("test", "not json");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_parse_expansion_missing_fields_chirho() {
        let result_chirho = parse_expansion_response_chirho("test", "{}").unwrap();
        assert!(result_chirho.terms_chirho.is_empty());
        assert!(result_chirho.strongs_chirho.is_empty());
    }
}
