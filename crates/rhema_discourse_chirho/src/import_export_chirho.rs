// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! JSON import/export for discourse analyses — full round-trip serialization.

use serde::{Deserialize, Serialize};

use crate::error_chirho::{DiscourseErrorChirho, DiscourseResultChirho};
use crate::types_chirho::ArcStructureChirho;

/// Version string for the export format.
const FORMAT_VERSION_CHIRHO: &str = "rhema_discourse_chirho/1.0";

/// Wrapper for JSON export with version metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportEnvelopeChirho {
    pub version_chirho: String,
    pub arc_chirho: ArcStructureChirho,
}

/// Export an arc structure to JSON string.
pub fn export_to_json_chirho(
    arc_chirho: &ArcStructureChirho,
) -> DiscourseResultChirho<String> {
    let envelope_chirho = ExportEnvelopeChirho {
        version_chirho: FORMAT_VERSION_CHIRHO.to_string(),
        arc_chirho: arc_chirho.clone(),
    };

    serde_json::to_string_pretty(&envelope_chirho).map_err(|e_chirho| {
        DiscourseErrorChirho::ImportExportChirho {
            reason_chirho: format!("Failed to export: {e_chirho}"),
        }
    })
}

/// Import an arc structure from a JSON string.
pub fn import_from_json_chirho(
    json_chirho: &str,
) -> DiscourseResultChirho<ArcStructureChirho> {
    let envelope_chirho: ExportEnvelopeChirho =
        serde_json::from_str(json_chirho).map_err(|e_chirho| {
            DiscourseErrorChirho::ImportExportChirho {
                reason_chirho: format!("Failed to import: {e_chirho}"),
            }
        })?;

    if !envelope_chirho.version_chirho.starts_with("rhema_discourse_chirho/") {
        return Err(DiscourseErrorChirho::ImportExportChirho {
            reason_chirho: format!(
                "Unknown format version: {}",
                envelope_chirho.version_chirho
            ),
        });
    }

    Ok(envelope_chirho.arc_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::types_chirho::*;
    use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

    fn make_test_arc_chirho() -> ArcStructureChirho {
        ArcStructureChirho {
            id_chirho: None,
            title_chirho: "Test Arc".to_string(),
            passage_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 18).unwrap(),
                start_char_chirho: None,
                end_char_chirho: None,
            },
            propositions_chirho: vec![PropositionChirho {
                id_chirho: PropositionIdChirho(1),
                text_chirho: "God so loved".to_string(),
                verse_span_chirho: VerseSpanChirho {
                    start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                    end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                    start_char_chirho: None,
                    end_char_chirho: None,
                },
                notes_chirho: None,
                label_chirho: "1a".to_string(),
                phrase_structure_chirho: None,
            }],
            relationships_chirho: Vec::new(),
            main_proposition_id_chirho: Some(PropositionIdChirho(1)),
            tags_chirho: vec!["love".to_string()],
            brackets_chirho: Vec::new(),
        }
    }

    #[test]
    fn test_roundtrip_chirho() {
        let arc_chirho = make_test_arc_chirho();
        let json_chirho = export_to_json_chirho(&arc_chirho).unwrap();
        let imported_chirho = import_from_json_chirho(&json_chirho).unwrap();

        assert_eq!(imported_chirho.title_chirho, "Test Arc");
        assert_eq!(imported_chirho.propositions_chirho.len(), 1);
        assert_eq!(imported_chirho.tags_chirho, vec!["love"]);
    }

    #[test]
    fn test_export_contains_version_chirho() {
        let arc_chirho = make_test_arc_chirho();
        let json_chirho = export_to_json_chirho(&arc_chirho).unwrap();
        assert!(json_chirho.contains("rhema_discourse_chirho/1.0"));
    }

    #[test]
    fn test_import_bad_version_chirho() {
        let bad_json_chirho = r#"{"version_chirho":"unknown/1.0","arc_chirho":{}}"#;
        // This should fail due to bad version or missing fields.
        assert!(import_from_json_chirho(bad_json_chirho).is_err());
    }

    #[test]
    fn test_import_invalid_json_chirho() {
        assert!(import_from_json_chirho("not json").is_err());
    }
}
