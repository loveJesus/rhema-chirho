// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Concept mapper — maps natural language concepts to biblical themes.

use crate::error_chirho::AiResultChirho;
use crate::llm_provider_chirho::{ChatMessageChirho, LlmProviderChirho};

/// A mapped biblical concept with related search terms.
#[derive(Debug, Clone)]
pub struct MappedConceptChirho {
    /// The original concept.
    pub concept_chirho: String,
    /// Related biblical themes.
    pub themes_chirho: Vec<String>,
    /// Key search terms for this concept.
    pub search_terms_chirho: Vec<String>,
    /// Related Strong's numbers.
    pub strongs_chirho: Vec<String>,
    /// Key passage references.
    pub key_passages_chirho: Vec<String>,
}

/// Maps natural language concepts to biblical themes using an LLM.
pub struct ConceptMapperChirho<'a> {
    provider_chirho: &'a dyn LlmProviderChirho,
}

impl<'a> ConceptMapperChirho<'a> {
    pub fn new_chirho(provider_chirho: &'a dyn LlmProviderChirho) -> Self {
        Self { provider_chirho }
    }

    /// Map a concept to biblical themes, terms, and references.
    pub async fn map_concept_chirho(
        &self,
        concept_chirho: &str,
    ) -> AiResultChirho<MappedConceptChirho> {
        let prompt_chirho = format!(
            "For the biblical concept: \"{concept_chirho}\"\n\
             Return a JSON object with:\n\
             - \"themes\": array of 2-4 related biblical themes\n\
             - \"search_terms\": array of 3-6 English search terms\n\
             - \"strongs\": array of key Strong's numbers (G/H prefix)\n\
             - \"key_passages\": array of 2-4 key passage references (e.g. \"John 3:16\")\n\
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

        parse_concept_response_chirho(concept_chirho, &response_chirho.content_chirho)
    }
}

/// Parse the LLM JSON response into a MappedConceptChirho.
fn parse_concept_response_chirho(
    concept_chirho: &str,
    response_chirho: &str,
) -> AiResultChirho<MappedConceptChirho> {
    let parsed_chirho: serde_json::Value = serde_json::from_str(response_chirho).map_err(
        |e_chirho| crate::error_chirho::AiErrorChirho::QueryExpansionChirho {
            reason_chirho: format!("Failed to parse concept mapping JSON: {e_chirho}"),
        },
    )?;

    let extract_strings_chirho = |key_chirho: &str| -> Vec<String> {
        parsed_chirho[key_chirho]
            .as_array()
            .map(|a_chirho| {
                a_chirho
                    .iter()
                    .filter_map(|v_chirho| v_chirho.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    };

    Ok(MappedConceptChirho {
        concept_chirho: concept_chirho.to_string(),
        themes_chirho: extract_strings_chirho("themes"),
        search_terms_chirho: extract_strings_chirho("search_terms"),
        strongs_chirho: extract_strings_chirho("strongs"),
        key_passages_chirho: extract_strings_chirho("key_passages"),
    })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::llm_provider_chirho::MockLlmProviderChirho;

    #[tokio::test]
    async fn test_map_concept_chirho() {
        let mock_response_chirho = r#"{
            "themes": ["covenant faithfulness", "divine love"],
            "search_terms": ["hesed", "lovingkindness", "mercy", "faithfulness"],
            "strongs": ["H2617"],
            "key_passages": ["Psalm 136:1", "Lamentations 3:22-23"]
        }"#;

        let provider_chirho =
            MockLlmProviderChirho::new_chirho().with_response_chirho(mock_response_chirho);
        let mapper_chirho = ConceptMapperChirho::new_chirho(&provider_chirho);
        let result_chirho = mapper_chirho
            .map_concept_chirho("God's faithfulness")
            .await
            .unwrap();

        assert_eq!(result_chirho.concept_chirho, "God's faithfulness");
        assert!(!result_chirho.themes_chirho.is_empty());
        assert!(result_chirho.search_terms_chirho.contains(&"hesed".to_string()));
        assert!(result_chirho.strongs_chirho.contains(&"H2617".to_string()));
    }

    #[test]
    fn test_parse_concept_bad_json_chirho() {
        let result_chirho = parse_concept_response_chirho("test", "bad json");
        assert!(result_chirho.is_err());
    }
}
