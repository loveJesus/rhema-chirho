// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Arc structure validation — ensures structural integrity.

use std::collections::{HashMap, HashSet};

use crate::types_chirho::ArcStructureChirho;

/// Validation result for an arc structure.
#[derive(Debug, Clone)]
pub struct ValidationResultChirho {
    pub is_valid_chirho: bool,
    pub errors_chirho: Vec<String>,
    pub warnings_chirho: Vec<String>,
}

/// Validate an arc structure for structural integrity.
pub fn validate_arc_chirho(arc_chirho: &ArcStructureChirho) -> ValidationResultChirho {
    let mut errors_chirho = Vec::new();
    let mut warnings_chirho = Vec::new();

    // Check: at least one proposition.
    if arc_chirho.propositions_chirho.is_empty() {
        errors_chirho.push("Arc has no propositions".to_string());
    }

    // Collect all proposition IDs.
    let prop_ids_chirho: HashSet<i64> = arc_chirho
        .propositions_chirho
        .iter()
        .map(|p_chirho| p_chirho.id_chirho.0)
        .collect();

    // Check: no duplicate proposition IDs.
    if prop_ids_chirho.len() != arc_chirho.propositions_chirho.len() {
        errors_chirho.push("Duplicate proposition IDs detected".to_string());
    }

    // Check: all relationship references valid propositions.
    for rel_chirho in &arc_chirho.relationships_chirho {
        if !prop_ids_chirho.contains(&rel_chirho.source_id_chirho.0) {
            errors_chirho.push(format!(
                "Relationship source {} not found in propositions",
                rel_chirho.source_id_chirho.0
            ));
        }
        if !prop_ids_chirho.contains(&rel_chirho.target_id_chirho.0) {
            errors_chirho.push(format!(
                "Relationship target {} not found in propositions",
                rel_chirho.target_id_chirho.0
            ));
        }
    }

    // Check: main proposition references a valid proposition.
    if let Some(main_chirho) = arc_chirho.main_proposition_id_chirho {
        if !prop_ids_chirho.contains(&main_chirho.0) {
            errors_chirho.push(format!(
                "Main proposition {} not found in propositions",
                main_chirho.0
            ));
        }
    } else if !arc_chirho.propositions_chirho.is_empty() {
        warnings_chirho.push("No main proposition designated".to_string());
    }

    // Check: no cycles in relationships.
    if has_cycle_chirho(&arc_chirho.relationships_chirho, &prop_ids_chirho) {
        errors_chirho.push("Cycle detected in relationships".to_string());
    }

    // Check: relationship strength in valid range.
    for rel_chirho in &arc_chirho.relationships_chirho {
        if rel_chirho.strength_chirho == 0 || rel_chirho.strength_chirho > 3 {
            warnings_chirho.push(format!(
                "Relationship strength {} is outside expected range 1-3",
                rel_chirho.strength_chirho
            ));
        }
    }

    // Check: all propositions referenced in at least one relationship.
    if arc_chirho.propositions_chirho.len() > 1 {
        let referenced_chirho: HashSet<i64> = arc_chirho
            .relationships_chirho
            .iter()
            .flat_map(|r_chirho| [r_chirho.source_id_chirho.0, r_chirho.target_id_chirho.0])
            .collect();

        for prop_chirho in &arc_chirho.propositions_chirho {
            if !referenced_chirho.contains(&prop_chirho.id_chirho.0) {
                warnings_chirho.push(format!(
                    "Proposition {} is not referenced in any relationship",
                    prop_chirho.id_chirho.0
                ));
            }
        }
    }

    ValidationResultChirho {
        is_valid_chirho: errors_chirho.is_empty(),
        errors_chirho,
        warnings_chirho,
    }
}

/// Check for cycles using DFS.
fn has_cycle_chirho(
    relationships_chirho: &[crate::types_chirho::RelationshipChirho],
    prop_ids_chirho: &HashSet<i64>,
) -> bool {
    // Build adjacency list.
    let mut adj_chirho: HashMap<i64, Vec<i64>> = HashMap::new();
    for rel_chirho in relationships_chirho {
        adj_chirho
            .entry(rel_chirho.source_id_chirho.0)
            .or_default()
            .push(rel_chirho.target_id_chirho.0);
    }

    let mut visited_chirho = HashSet::new();
    let mut in_stack_chirho = HashSet::new();

    for &id_chirho in prop_ids_chirho {
        if dfs_cycle_chirho(id_chirho, &adj_chirho, &mut visited_chirho, &mut in_stack_chirho) {
            return true;
        }
    }

    false
}

fn dfs_cycle_chirho(
    node_chirho: i64,
    adj_chirho: &HashMap<i64, Vec<i64>>,
    visited_chirho: &mut HashSet<i64>,
    in_stack_chirho: &mut HashSet<i64>,
) -> bool {
    if in_stack_chirho.contains(&node_chirho) {
        return true;
    }
    if visited_chirho.contains(&node_chirho) {
        return false;
    }

    visited_chirho.insert(node_chirho);
    in_stack_chirho.insert(node_chirho);

    if let Some(neighbors_chirho) = adj_chirho.get(&node_chirho) {
        for &next_chirho in neighbors_chirho {
            if dfs_cycle_chirho(next_chirho, adj_chirho, visited_chirho, in_stack_chirho) {
                return true;
            }
        }
    }

    in_stack_chirho.remove(&node_chirho);
    false
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::types_chirho::*;
    use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

    fn make_valid_arc_chirho() -> ArcStructureChirho {
        let span_chirho = VerseSpanChirho {
            start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            start_char_chirho: None,
            end_char_chirho: None,
        };

        ArcStructureChirho {
            id_chirho: None,
            title_chirho: "Test".to_string(),
            passage_chirho: span_chirho.clone(),
            propositions_chirho: vec![
                PropositionChirho {
                    id_chirho: PropositionIdChirho(1),
                    text_chirho: "A".to_string(),
                    verse_span_chirho: span_chirho.clone(),
                    notes_chirho: None,
                    label_chirho: "1".to_string(),
                    phrase_structure_chirho: None,
                },
                PropositionChirho {
                    id_chirho: PropositionIdChirho(2),
                    text_chirho: "B".to_string(),
                    verse_span_chirho: span_chirho,
                    notes_chirho: None,
                    label_chirho: "2".to_string(),
                    phrase_structure_chirho: None,
                },
            ],
            relationships_chirho: vec![RelationshipChirho {
                source_id_chirho: PropositionIdChirho(1),
                target_id_chirho: PropositionIdChirho(2),
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
    fn test_valid_arc_chirho() {
        let result_chirho = validate_arc_chirho(&make_valid_arc_chirho());
        assert!(result_chirho.is_valid_chirho);
        assert!(result_chirho.errors_chirho.is_empty());
    }

    #[test]
    fn test_empty_propositions_chirho() {
        let mut arc_chirho = make_valid_arc_chirho();
        arc_chirho.propositions_chirho.clear();
        arc_chirho.relationships_chirho.clear();

        let result_chirho = validate_arc_chirho(&arc_chirho);
        assert!(!result_chirho.is_valid_chirho);
    }

    #[test]
    fn test_invalid_relationship_reference_chirho() {
        let mut arc_chirho = make_valid_arc_chirho();
        arc_chirho.relationships_chirho[0].target_id_chirho = PropositionIdChirho(99);

        let result_chirho = validate_arc_chirho(&arc_chirho);
        assert!(!result_chirho.is_valid_chirho);
    }

    #[test]
    fn test_cycle_detection_chirho() {
        let mut arc_chirho = make_valid_arc_chirho();
        // Add a cycle: 1 → 2 and 2 → 1
        arc_chirho.relationships_chirho.push(RelationshipChirho {
            source_id_chirho: PropositionIdChirho(2),
            target_id_chirho: PropositionIdChirho(1),
            relationship_type_chirho: RelationshipTypeChirho::SeriesChirho,
            strength_chirho: 1,
            notes_chirho: None,
        });

        let result_chirho = validate_arc_chirho(&arc_chirho);
        assert!(!result_chirho.is_valid_chirho);
        assert!(result_chirho.errors_chirho.iter().any(|e_chirho| e_chirho.contains("Cycle")));
    }

    #[test]
    fn test_warning_no_main_proposition_chirho() {
        let mut arc_chirho = make_valid_arc_chirho();
        arc_chirho.main_proposition_id_chirho = None;

        let result_chirho = validate_arc_chirho(&arc_chirho);
        assert!(result_chirho.is_valid_chirho);
        assert!(!result_chirho.warnings_chirho.is_empty());
    }
}
