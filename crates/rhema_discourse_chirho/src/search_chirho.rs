// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Discourse search — query the discourse store and return results
//! compatible with the main search infrastructure.

use rhema_contracts_chirho::result_chirho::SearchHitChirho;

use crate::error_chirho::DiscourseResultChirho;
use crate::storage_chirho::DiscourseStoreChirho;
use crate::types_chirho::RelationshipTypeChirho;

/// Search for arcs by relationship type, returning verse references as hits.
pub fn search_by_relationship_chirho(
    store_chirho: &dyn DiscourseStoreChirho,
    relationship_type_chirho: RelationshipTypeChirho,
) -> DiscourseResultChirho<Vec<SearchHitChirho>> {
    let arc_ids_chirho = store_chirho.find_by_relationship_chirho(relationship_type_chirho)?;
    let mut hits_chirho = Vec::new();

    for arc_id_chirho in arc_ids_chirho {
        let arc_chirho = store_chirho.load_arc_chirho(arc_id_chirho)?;
        // Return the passage start as the hit verse ref.
        let verse_ref_chirho = arc_chirho.passage_chirho.start_ref_chirho;
        hits_chirho.push(SearchHitChirho {
            verse_ref_chirho,
            text_chirho: arc_chirho.title_chirho,
            highlighted_text_chirho: None,
            score_chirho: 1.0,
            matched_positions_chirho: Vec::new(),
            explain_chirho: None,
            semantic_score_chirho: None,
            hybrid_score_chirho: None,
            semantic_explain_chirho: Some(format!(
                "Contains {} relationship",
                relationship_type_chirho.as_str_chirho()
            )),
        });
    }

    Ok(hits_chirho)
}

/// Search for arcs by proposition text, returning verse references as hits.
pub fn search_by_proposition_text_chirho(
    store_chirho: &dyn DiscourseStoreChirho,
    text_query_chirho: &str,
) -> DiscourseResultChirho<Vec<SearchHitChirho>> {
    let arc_ids_chirho = store_chirho.find_by_proposition_text_chirho(text_query_chirho)?;
    let mut hits_chirho = Vec::new();

    for arc_id_chirho in arc_ids_chirho {
        let arc_chirho = store_chirho.load_arc_chirho(arc_id_chirho)?;
        let verse_ref_chirho = arc_chirho.passage_chirho.start_ref_chirho;

        // Find the matching proposition text for the highlight.
        let matching_prop_chirho = arc_chirho
            .propositions_chirho
            .iter()
            .find(|p_chirho| {
                p_chirho
                    .text_chirho
                    .to_lowercase()
                    .contains(&text_query_chirho.to_lowercase())
            });

        let highlight_chirho = matching_prop_chirho.map(|p_chirho| p_chirho.text_chirho.clone());

        hits_chirho.push(SearchHitChirho {
            verse_ref_chirho,
            text_chirho: arc_chirho.title_chirho,
            highlighted_text_chirho: highlight_chirho,
            score_chirho: 1.0,
            matched_positions_chirho: Vec::new(),
            explain_chirho: None,
            semantic_score_chirho: None,
            hybrid_score_chirho: None,
            semantic_explain_chirho: Some(format!(
                "Proposition contains: '{text_query_chirho}'"
            )),
        });
    }

    Ok(hits_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::sqlite_store_chirho::SqliteDiscourseStoreChirho;
    use crate::types_chirho::*;
    use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

    fn make_test_arc_chirho() -> ArcStructureChirho {
        ArcStructureChirho {
            id_chirho: None,
            title_chirho: "John 3:16 Arc".to_string(),
            passage_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 17).unwrap(),
                start_char_chirho: None,
                end_char_chirho: None,
            },
            propositions_chirho: vec![
                PropositionChirho {
                    id_chirho: PropositionIdChirho(1),
                    text_chirho: "God so loved the world".to_string(),
                    verse_span_chirho: VerseSpanChirho {
                        start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                        end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                        start_char_chirho: None,
                        end_char_chirho: None,
                    },
                    notes_chirho: None,
                    label_chirho: "1".to_string(),
                    phrase_structure_chirho: None,
                },
            ],
            relationships_chirho: vec![RelationshipChirho {
                source_id_chirho: PropositionIdChirho(1),
                target_id_chirho: PropositionIdChirho(1),
                relationship_type_chirho: RelationshipTypeChirho::GroundChirho,
                strength_chirho: 2,
                notes_chirho: None,
            }],
            main_proposition_id_chirho: Some(PropositionIdChirho(1)),
            tags_chirho: Vec::new(),
            brackets_chirho: Vec::new(),
        }
    }

    #[test]
    fn test_search_by_relationship_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        store_chirho.save_arc_chirho(&make_test_arc_chirho()).unwrap();

        let results_chirho =
            search_by_relationship_chirho(&store_chirho, RelationshipTypeChirho::GroundChirho)
                .unwrap();
        assert_eq!(results_chirho.len(), 1);
        assert_eq!(results_chirho[0].verse_ref_chirho.book_chirho, "John");
    }

    #[test]
    fn test_search_by_proposition_text_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        store_chirho.save_arc_chirho(&make_test_arc_chirho()).unwrap();

        let results_chirho =
            search_by_proposition_text_chirho(&store_chirho, "loved the world").unwrap();
        assert_eq!(results_chirho.len(), 1);
    }
}
