// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! LLM-powered arc generation — produce complete discourse analyses from text.

use rhema_ai_chirho::llm_provider_chirho::{ChatMessageChirho, LlmProviderChirho};
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

use crate::error_chirho::{DiscourseErrorChirho, DiscourseResultChirho};
use crate::types_chirho::*;

/// A suggested improvement to an existing arc.
#[derive(Debug, Clone)]
pub struct ArcImprovementChirho {
    pub description_chirho: String,
    pub category_chirho: ImprovementCategoryChirho,
}

/// Categories of arc improvements.
#[derive(Debug, Clone)]
pub enum ImprovementCategoryChirho {
    MissingPropositionChirho,
    IncorrectRelationshipChirho,
    BetterMainPropositionChirho,
    StructuralChirho,
}

/// LLM-powered arc generator.
pub struct LlmArcGeneratorChirho<'a> {
    provider_chirho: &'a dyn LlmProviderChirho,
}

impl<'a> LlmArcGeneratorChirho<'a> {
    pub fn new_chirho(provider_chirho: &'a dyn LlmProviderChirho) -> Self {
        Self { provider_chirho }
    }

    /// Generate a complete arc structure from a passage.
    pub async fn generate_arc_chirho(
        &self,
        passage_ref_chirho: &str,
        text_chirho: &str,
    ) -> DiscourseResultChirho<ArcStructureChirho> {
        let relationship_types_chirho = RelationshipTypeChirho::all_chirho()
            .iter()
            .map(|rt_chirho| rt_chirho.as_str_chirho())
            .collect::<Vec<_>>()
            .join(", ");

        let prompt_chirho = format!(
            "Analyze the discourse structure of this biblical passage:\n\
             Passage: {passage_ref_chirho}\n\
             Text: \"{text_chirho}\"\n\n\
             Return a JSON object with:\n\
             - \"propositions\": array of objects with \"id\" (integer), \"text\" (string), \"label\" (string like \"1a\")\n\
             - \"relationships\": array of objects with \"source\" (id), \"target\" (id), \
               \"type\" (one of: {relationship_types_chirho}), \"strength\" (1-3)\n\
             - \"main_proposition\": the id of the main (thesis) proposition\n\
             Return ONLY valid JSON, no markdown."
        );

        let messages_chirho = vec![ChatMessageChirho {
            role_chirho: "user".to_string(),
            content_chirho: prompt_chirho,
        }];

        let response_chirho = self
            .provider_chirho
            .chat_completion_chirho(&messages_chirho, 1024, 0.3)
            .await
            .map_err(|e_chirho| DiscourseErrorChirho::LlmGenerationChirho {
                reason_chirho: format!("LLM call failed: {e_chirho}"),
            })?;

        parse_arc_response_chirho(passage_ref_chirho, &response_chirho.content_chirho)
    }

    /// Suggest improvements to an existing arc analysis.
    pub async fn suggest_improvements_chirho(
        &self,
        arc_chirho: &ArcStructureChirho,
        text_chirho: &str,
    ) -> DiscourseResultChirho<Vec<ArcImprovementChirho>> {
        let arc_json_chirho = serde_json::to_string_pretty(arc_chirho).map_err(|e_chirho| {
            DiscourseErrorChirho::LlmGenerationChirho {
                reason_chirho: format!("Failed to serialize arc: {e_chirho}"),
            }
        })?;

        let prompt_chirho = format!(
            "Review this discourse analysis and suggest improvements:\n\
             Text: \"{text_chirho}\"\n\
             Current analysis:\n{arc_json_chirho}\n\n\
             Return a JSON array of objects with:\n\
             - \"description\": what to improve\n\
             - \"category\": one of \"MissingProposition\", \"IncorrectRelationship\", \
               \"BetterMainProposition\", \"Structural\"\n\
             Return ONLY valid JSON, no markdown."
        );

        let messages_chirho = vec![ChatMessageChirho {
            role_chirho: "user".to_string(),
            content_chirho: prompt_chirho,
        }];

        let response_chirho = self
            .provider_chirho
            .chat_completion_chirho(&messages_chirho, 512, 0.3)
            .await
            .map_err(|e_chirho| DiscourseErrorChirho::LlmGenerationChirho {
                reason_chirho: format!("LLM call failed: {e_chirho}"),
            })?;

        parse_improvements_response_chirho(&response_chirho.content_chirho)
    }
}

/// Parse LLM response into an ArcStructureChirho.
fn parse_arc_response_chirho(
    passage_ref_chirho: &str,
    response_chirho: &str,
) -> DiscourseResultChirho<ArcStructureChirho> {
    let parsed_chirho: serde_json::Value =
        serde_json::from_str(response_chirho).map_err(|e_chirho| {
            DiscourseErrorChirho::LlmGenerationChirho {
                reason_chirho: format!("Failed to parse LLM response: {e_chirho}"),
            }
        })?;

    // Parse verse ref from passage_ref string (simplified).
    let default_ref_chirho =
        VerseRefChirho::new_chirho("Unknown", 1, 1).unwrap();
    let verse_span_chirho = VerseSpanChirho {
        start_ref_chirho: default_ref_chirho.clone(),
        end_ref_chirho: default_ref_chirho.clone(),
        start_char_chirho: None,
        end_char_chirho: None,
    };

    // Parse propositions.
    let propositions_chirho = parsed_chirho["propositions"]
        .as_array()
        .map(|arr_chirho| {
            arr_chirho
                .iter()
                .filter_map(|p_chirho| {
                    Some(PropositionChirho {
                        id_chirho: PropositionIdChirho(p_chirho["id"].as_i64()?),
                        text_chirho: p_chirho["text"].as_str()?.to_string(),
                        verse_span_chirho: verse_span_chirho.clone(),
                        notes_chirho: None,
                        label_chirho: p_chirho["label"]
                            .as_str()
                            .unwrap_or("?")
                            .to_string(),
                        phrase_structure_chirho: None,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    // Parse relationships.
    let relationships_chirho = parsed_chirho["relationships"]
        .as_array()
        .map(|arr_chirho| {
            arr_chirho
                .iter()
                .filter_map(|r_chirho| {
                    let type_str_chirho = r_chirho["type"].as_str()?;
                    Some(RelationshipChirho {
                        source_id_chirho: PropositionIdChirho(r_chirho["source"].as_i64()?),
                        target_id_chirho: PropositionIdChirho(r_chirho["target"].as_i64()?),
                        relationship_type_chirho:
                            RelationshipTypeChirho::from_str_chirho(type_str_chirho)
                                .unwrap_or(RelationshipTypeChirho::SeriesChirho),
                        strength_chirho: r_chirho["strength"].as_u64().unwrap_or(2) as u8,
                        notes_chirho: None,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let main_prop_chirho = parsed_chirho["main_proposition"]
        .as_i64()
        .map(PropositionIdChirho);

    Ok(ArcStructureChirho {
        id_chirho: None,
        title_chirho: format!("{passage_ref_chirho} Analysis"),
        passage_chirho: verse_span_chirho,
        propositions_chirho,
        relationships_chirho,
        main_proposition_id_chirho: main_prop_chirho,
        tags_chirho: vec!["ai-generated".to_string()],
        brackets_chirho: Vec::new(),
    })
}

/// Parse LLM response into improvement suggestions.
fn parse_improvements_response_chirho(
    response_chirho: &str,
) -> DiscourseResultChirho<Vec<ArcImprovementChirho>> {
    let parsed_chirho: Vec<serde_json::Value> =
        serde_json::from_str(response_chirho).map_err(|e_chirho| {
            DiscourseErrorChirho::LlmGenerationChirho {
                reason_chirho: format!("Failed to parse improvements response: {e_chirho}"),
            }
        })?;

    let improvements_chirho = parsed_chirho
        .iter()
        .filter_map(|item_chirho| {
            let desc_chirho = item_chirho["description"].as_str()?.to_string();
            let cat_str_chirho = item_chirho["category"].as_str().unwrap_or("Structural");
            let category_chirho = match cat_str_chirho {
                "MissingProposition" => ImprovementCategoryChirho::MissingPropositionChirho,
                "IncorrectRelationship" => ImprovementCategoryChirho::IncorrectRelationshipChirho,
                "BetterMainProposition" => ImprovementCategoryChirho::BetterMainPropositionChirho,
                _ => ImprovementCategoryChirho::StructuralChirho,
            };
            Some(ArcImprovementChirho {
                description_chirho: desc_chirho,
                category_chirho,
            })
        })
        .collect();

    Ok(improvements_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhema_ai_chirho::llm_provider_chirho::MockLlmProviderChirho;

    #[tokio::test]
    async fn test_generate_arc_chirho() {
        let mock_response_chirho = r#"{
            "propositions": [
                {"id": 1, "text": "God so loved the world", "label": "1a"},
                {"id": 2, "text": "that He gave His only Son", "label": "1b"}
            ],
            "relationships": [
                {"source": 1, "target": 2, "type": "Result", "strength": 3}
            ],
            "main_proposition": 1
        }"#;

        let provider_chirho =
            MockLlmProviderChirho::new_chirho().with_response_chirho(mock_response_chirho);
        let generator_chirho = LlmArcGeneratorChirho::new_chirho(&provider_chirho);

        let arc_chirho = generator_chirho
            .generate_arc_chirho("John 3:16", "For God so loved the world that He gave His only Son")
            .await
            .unwrap();

        assert_eq!(arc_chirho.propositions_chirho.len(), 2);
        assert_eq!(arc_chirho.relationships_chirho.len(), 1);
        assert_eq!(
            arc_chirho.relationships_chirho[0].relationship_type_chirho,
            RelationshipTypeChirho::ResultChirho
        );
    }

    #[tokio::test]
    async fn test_suggest_improvements_chirho() {
        let mock_response_chirho = r#"[
            {"description": "Consider splitting proposition 1a", "category": "MissingProposition"},
            {"description": "Ground relationship may be more accurate", "category": "IncorrectRelationship"}
        ]"#;

        let provider_chirho =
            MockLlmProviderChirho::new_chirho().with_response_chirho(mock_response_chirho);
        let generator_chirho = LlmArcGeneratorChirho::new_chirho(&provider_chirho);

        let arc_chirho = ArcStructureChirho {
            id_chirho: None,
            title_chirho: "Test".to_string(),
            passage_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                start_char_chirho: None,
                end_char_chirho: None,
            },
            propositions_chirho: Vec::new(),
            relationships_chirho: Vec::new(),
            main_proposition_id_chirho: None,
            tags_chirho: Vec::new(),
            brackets_chirho: Vec::new(),
        };

        let improvements_chirho = generator_chirho
            .suggest_improvements_chirho(&arc_chirho, "test text")
            .await
            .unwrap();

        assert_eq!(improvements_chirho.len(), 2);
    }
}
