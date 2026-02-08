// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Relationship suggestion — rule-based and LLM-powered.

use crate::types_chirho::{PropositionChirho, PropositionIdChirho, RelationshipChirho, RelationshipTypeChirho};

/// A suggested relationship with confidence score.
#[derive(Debug, Clone)]
pub struct SuggestionChirho {
    pub source_id_chirho: PropositionIdChirho,
    pub target_id_chirho: PropositionIdChirho,
    pub relationship_type_chirho: RelationshipTypeChirho,
    pub confidence_chirho: f32,
    pub reason_chirho: String,
}

/// Rule-based relationship suggester — uses discourse markers to infer types.
pub struct RuleBasedSuggesterChirho;

impl RuleBasedSuggesterChirho {
    /// Suggest relationships between propositions based on discourse markers.
    pub fn suggest_chirho(
        propositions_chirho: &[PropositionChirho],
    ) -> Vec<SuggestionChirho> {
        let mut suggestions_chirho = Vec::new();

        for i_chirho in 1..propositions_chirho.len() {
            let prev_chirho = &propositions_chirho[i_chirho - 1];
            let curr_chirho = &propositions_chirho[i_chirho];
            let lower_chirho = curr_chirho.text_chirho.to_lowercase();

            if let Some((rel_type_chirho, confidence_chirho, reason_chirho)) =
                detect_marker_chirho(&lower_chirho)
            {
                suggestions_chirho.push(SuggestionChirho {
                    source_id_chirho: prev_chirho.id_chirho,
                    target_id_chirho: curr_chirho.id_chirho,
                    relationship_type_chirho: rel_type_chirho,
                    confidence_chirho,
                    reason_chirho,
                });
            } else {
                // Default: adjacent propositions are in Series.
                suggestions_chirho.push(SuggestionChirho {
                    source_id_chirho: prev_chirho.id_chirho,
                    target_id_chirho: curr_chirho.id_chirho,
                    relationship_type_chirho: RelationshipTypeChirho::SeriesChirho,
                    confidence_chirho: 0.3,
                    reason_chirho: "Adjacent propositions (default Series)".to_string(),
                });
            }
        }

        suggestions_chirho
    }

    /// Convert suggestions to relationships (accepting those above a threshold).
    pub fn accept_above_threshold_chirho(
        suggestions_chirho: &[SuggestionChirho],
        threshold_chirho: f32,
    ) -> Vec<RelationshipChirho> {
        suggestions_chirho
            .iter()
            .filter(|s_chirho| s_chirho.confidence_chirho >= threshold_chirho)
            .map(|s_chirho| RelationshipChirho {
                source_id_chirho: s_chirho.source_id_chirho,
                target_id_chirho: s_chirho.target_id_chirho,
                relationship_type_chirho: s_chirho.relationship_type_chirho,
                strength_chirho: confidence_to_strength_chirho(s_chirho.confidence_chirho),
                notes_chirho: Some(s_chirho.reason_chirho.clone()),
            })
            .collect()
    }
}

/// Detect a discourse marker and return the relationship type with confidence.
fn detect_marker_chirho(text_chirho: &str) -> Option<(RelationshipTypeChirho, f32, String)> {
    // Check for common discourse markers at the start of the proposition.
    let markers_chirho: &[(&[&str], RelationshipTypeChirho, f32)] = &[
        (&["because", "for", "since"], RelationshipTypeChirho::GroundChirho, 0.8),
        (&["therefore", "so", "thus", "hence"], RelationshipTypeChirho::InferenceChirho, 0.8),
        (&["in order that", "so that", "that"], RelationshipTypeChirho::PurposeChirho, 0.7),
        (&["but", "however", "yet", "nevertheless"], RelationshipTypeChirho::ContrastChirho, 0.8),
        (&["although", "even though", "though"], RelationshipTypeChirho::ConcessionChirho, 0.8),
        (&["when", "while", "after", "before", "until"], RelationshipTypeChirho::TemporalChirho, 0.7),
        (&["by", "through", "by means of"], RelationshipTypeChirho::MeansChirho, 0.7),
        (&["like", "as", "just as"], RelationshipTypeChirho::ComparisonChirho, 0.7),
        (&["if"], RelationshipTypeChirho::ConditionChirho, 0.8),
        (&["or", "either"], RelationshipTypeChirho::AlternativeChirho, 0.7),
        (&["that is", "namely", "in other words"], RelationshipTypeChirho::ExplanationChirho, 0.8),
        (&["not", "no"], RelationshipTypeChirho::NegativePositiveChirho, 0.5),
    ];

    for (words_chirho, rel_type_chirho, confidence_chirho) in markers_chirho {
        for word_chirho in *words_chirho {
            if text_chirho.starts_with(word_chirho) {
                return Some((
                    *rel_type_chirho,
                    *confidence_chirho,
                    format!("Discourse marker: '{word_chirho}'"),
                ));
            }
        }
    }

    None
}

/// Convert confidence (0.0-1.0) to strength (1-3).
fn confidence_to_strength_chirho(confidence_chirho: f32) -> u8 {
    if confidence_chirho >= 0.7 {
        3
    } else if confidence_chirho >= 0.4 {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::types_chirho::VerseSpanChirho;
    use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

    fn make_prop_chirho(id_chirho: i64, text_chirho: &str) -> PropositionChirho {
        PropositionChirho {
            id_chirho: PropositionIdChirho(id_chirho),
            text_chirho: text_chirho.to_string(),
            verse_span_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                start_char_chirho: None,
                end_char_chirho: None,
            },
            notes_chirho: None,
            label_chirho: format!("{id_chirho}"),
            phrase_structure_chirho: None,
        }
    }

    #[test]
    fn test_suggest_ground_chirho() {
        let props_chirho = vec![
            make_prop_chirho(1, "God gave His Son"),
            make_prop_chirho(2, "because He loved the world"),
        ];
        let suggestions_chirho = RuleBasedSuggesterChirho::suggest_chirho(&props_chirho);
        assert_eq!(suggestions_chirho.len(), 1);
        assert_eq!(
            suggestions_chirho[0].relationship_type_chirho,
            RelationshipTypeChirho::GroundChirho
        );
    }

    #[test]
    fn test_suggest_contrast_chirho() {
        let props_chirho = vec![
            make_prop_chirho(1, "The flesh is weak"),
            make_prop_chirho(2, "but the spirit is willing"),
        ];
        let suggestions_chirho = RuleBasedSuggesterChirho::suggest_chirho(&props_chirho);
        assert_eq!(
            suggestions_chirho[0].relationship_type_chirho,
            RelationshipTypeChirho::ContrastChirho
        );
    }

    #[test]
    fn test_suggest_inference_chirho() {
        let props_chirho = vec![
            make_prop_chirho(1, "All have sinned"),
            make_prop_chirho(2, "therefore all need grace"),
        ];
        let suggestions_chirho = RuleBasedSuggesterChirho::suggest_chirho(&props_chirho);
        assert_eq!(
            suggestions_chirho[0].relationship_type_chirho,
            RelationshipTypeChirho::InferenceChirho
        );
    }

    #[test]
    fn test_suggest_default_series_chirho() {
        let props_chirho = vec![
            make_prop_chirho(1, "He came"),
            make_prop_chirho(2, "He saw"),
        ];
        let suggestions_chirho = RuleBasedSuggesterChirho::suggest_chirho(&props_chirho);
        assert_eq!(
            suggestions_chirho[0].relationship_type_chirho,
            RelationshipTypeChirho::SeriesChirho
        );
    }

    #[test]
    fn test_accept_above_threshold_chirho() {
        let props_chirho = vec![
            make_prop_chirho(1, "God loved"),
            make_prop_chirho(2, "because He is love"),
            make_prop_chirho(3, "He acts"),
        ];
        let suggestions_chirho = RuleBasedSuggesterChirho::suggest_chirho(&props_chirho);
        let accepted_chirho =
            RuleBasedSuggesterChirho::accept_above_threshold_chirho(&suggestions_chirho, 0.5);
        // Only the "because" suggestion should pass threshold (0.8 >= 0.5)
        // The Series default (0.3) should be filtered out
        assert_eq!(accepted_chirho.len(), 1);
    }
}
