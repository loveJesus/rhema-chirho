// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Proposition detection — automatically split passage text into propositions.

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

use crate::types_chirho::{PropositionChirho, PropositionIdChirho, VerseSpanChirho};

/// Heuristic proposition detector — splits text on punctuation and conjunctions.
pub struct HeuristicPropositionDetectorChirho;

impl HeuristicPropositionDetectorChirho {
    /// Detect propositions in a passage text.
    ///
    /// Splits on clause boundaries: semicolons, colons, "that", "because",
    /// "for" (at start of clause), "so that", relative pronouns, etc.
    pub fn detect_chirho(
        text_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Vec<PropositionChirho> {
        let clauses_chirho = split_into_clauses_chirho(text_chirho);
        let mut propositions_chirho = Vec::new();

        for (i_chirho, clause_chirho) in clauses_chirho.iter().enumerate() {
            let trimmed_chirho = clause_chirho.trim();
            if trimmed_chirho.is_empty() {
                continue;
            }

            let label_chirho = format!("{}", i_chirho + 1);

            propositions_chirho.push(PropositionChirho {
                id_chirho: PropositionIdChirho((i_chirho + 1) as i64),
                text_chirho: trimmed_chirho.to_string(),
                verse_span_chirho: VerseSpanChirho {
                    start_ref_chirho: verse_ref_chirho.clone(),
                    end_ref_chirho: verse_ref_chirho.clone(),
                    start_char_chirho: None,
                    end_char_chirho: None,
                },
                notes_chirho: None,
                label_chirho,
                phrase_structure_chirho: None,
            });
        }

        propositions_chirho
    }
}

/// Split text into clauses based on punctuation and discourse markers.
fn split_into_clauses_chirho(text_chirho: &str) -> Vec<String> {
    let mut clauses_chirho = Vec::new();
    let mut current_chirho = String::new();

    // Split markers — these indicate clause boundaries.
    let split_markers_chirho = ["; ", ": ", ", that ", ", so that ", ", because "];

    let mut remaining_chirho = text_chirho;

    while !remaining_chirho.is_empty() {
        let mut found_chirho = false;

        for marker_chirho in &split_markers_chirho {
            if let Some(pos_chirho) = remaining_chirho.find(marker_chirho) {
                // Include text before the marker.
                current_chirho.push_str(&remaining_chirho[..pos_chirho]);
                if !current_chirho.trim().is_empty() {
                    clauses_chirho.push(current_chirho.trim().to_string());
                }
                current_chirho = String::new();

                // Skip past the marker (but keep the conjunction as part of next clause).
                let skip_chirho = if marker_chirho.starts_with(", ") {
                    // Keep the conjunction word with the next clause.
                    pos_chirho + 2 // Skip ", "
                } else {
                    pos_chirho + marker_chirho.len()
                };
                remaining_chirho = &remaining_chirho[skip_chirho..];
                found_chirho = true;
                break;
            }
        }

        if !found_chirho {
            current_chirho.push_str(remaining_chirho);
            remaining_chirho = "";
        }
    }

    if !current_chirho.trim().is_empty() {
        clauses_chirho.push(current_chirho.trim().to_string());
    }

    clauses_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_detect_single_clause_chirho() {
        let ref_chirho = VerseRefChirho::new_chirho("John", 1, 1).unwrap();
        let props_chirho =
            HeuristicPropositionDetectorChirho::detect_chirho("In the beginning was the Word", &ref_chirho);
        assert_eq!(props_chirho.len(), 1);
        assert_eq!(props_chirho[0].text_chirho, "In the beginning was the Word");
    }

    #[test]
    fn test_detect_semicolon_split_chirho() {
        let ref_chirho = VerseRefChirho::new_chirho("John", 1, 1).unwrap();
        let text_chirho = "In the beginning was the Word; and the Word was with God";
        let props_chirho =
            HeuristicPropositionDetectorChirho::detect_chirho(text_chirho, &ref_chirho);
        assert_eq!(props_chirho.len(), 2);
    }

    #[test]
    fn test_detect_because_split_chirho() {
        let ref_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        let text_chirho = "whoever believes in him should not perish, because God loved the world";
        let props_chirho =
            HeuristicPropositionDetectorChirho::detect_chirho(text_chirho, &ref_chirho);
        assert!(props_chirho.len() >= 2);
    }

    #[test]
    fn test_detect_empty_text_chirho() {
        let ref_chirho = VerseRefChirho::new_chirho("Gen", 1, 1).unwrap();
        let props_chirho = HeuristicPropositionDetectorChirho::detect_chirho("", &ref_chirho);
        assert!(props_chirho.is_empty());
    }

    #[test]
    fn test_detect_labels_sequential_chirho() {
        let ref_chirho = VerseRefChirho::new_chirho("Rom", 8, 28).unwrap();
        let text_chirho = "All things work together for good; for those who love God; for those who are called";
        let props_chirho =
            HeuristicPropositionDetectorChirho::detect_chirho(text_chirho, &ref_chirho);
        assert_eq!(props_chirho[0].label_chirho, "1");
        assert_eq!(props_chirho[1].label_chirho, "2");
    }
}
