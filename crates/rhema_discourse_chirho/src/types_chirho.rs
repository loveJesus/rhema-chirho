// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Core data model for discourse analysis (arcing, bracketing, phrasing).

use serde::{Deserialize, Serialize};

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

/// Unique identifier for a proposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropositionIdChirho(pub i64);

/// Unique identifier for an arc structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArcIdChirho(pub i64);

/// A verse span — sub-verse range for precise proposition boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerseSpanChirho {
    pub start_ref_chirho: VerseRefChirho,
    pub end_ref_chirho: VerseRefChirho,
    /// Character offset within the start verse (0-based).
    pub start_char_chirho: Option<u32>,
    /// Character offset within the end verse (0-based).
    pub end_char_chirho: Option<u32>,
}

/// An atomic unit of thought (proposition) in discourse analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropositionChirho {
    pub id_chirho: PropositionIdChirho,
    /// The proposition text.
    pub text_chirho: String,
    /// Verse span this proposition covers.
    pub verse_span_chirho: VerseSpanChirho,
    /// Optional notes/annotations.
    pub notes_chirho: Option<String>,
    /// Display label (e.g. "1a", "2b").
    pub label_chirho: String,
    /// Optional phrase structure breakdown.
    pub phrase_structure_chirho: Option<PhraseStructureChirho>,
}

/// The 22 discourse relationship types for arcing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipTypeChirho {
    /// Items in a list or sequence.
    SeriesChirho,
    /// Forward movement or development.
    ProgressionChirho,
    /// Either/or alternatives.
    AlternativeChirho,
    /// Showing similarity.
    ComparisonChirho,
    /// Showing difference or opposition.
    ContrastChirho,
    /// Providing the reason/basis ("because").
    GroundChirho,
    /// Drawing a conclusion ("therefore").
    InferenceChirho,
    /// Stating the goal ("in order that").
    PurposeChirho,
    /// Stating the outcome ("so that").
    ResultChirho,
    /// Conditional statement ("if...then").
    ConditionChirho,
    /// Despite/although ("even though").
    ConcessionChirho,
    /// When something happens ("when", "after").
    TemporalChirho,
    /// How something is done.
    MannerChirho,
    /// By what means ("by", "through").
    MeansChirho,
    /// What is said or thought (indirect speech).
    ContentChirho,
    /// Clarifying or restating.
    ExplanationChirho,
    /// Action paired with its manner.
    ActionMannerChirho,
    /// Situation paired with a response.
    SituationResponseChirho,
    /// Two-sided interaction.
    BilateralChirho,
    /// Question and its answer.
    QuestionAnswerChirho,
    /// Negative statement paired with positive.
    NegativePositiveChirho,
    /// General statement followed by specifics.
    GeneralSpecificChirho,
}

impl RelationshipTypeChirho {
    /// Return all variants.
    pub fn all_chirho() -> &'static [RelationshipTypeChirho] {
        &[
            Self::SeriesChirho,
            Self::ProgressionChirho,
            Self::AlternativeChirho,
            Self::ComparisonChirho,
            Self::ContrastChirho,
            Self::GroundChirho,
            Self::InferenceChirho,
            Self::PurposeChirho,
            Self::ResultChirho,
            Self::ConditionChirho,
            Self::ConcessionChirho,
            Self::TemporalChirho,
            Self::MannerChirho,
            Self::MeansChirho,
            Self::ContentChirho,
            Self::ExplanationChirho,
            Self::ActionMannerChirho,
            Self::SituationResponseChirho,
            Self::BilateralChirho,
            Self::QuestionAnswerChirho,
            Self::NegativePositiveChirho,
            Self::GeneralSpecificChirho,
        ]
    }

    /// Convert to string representation.
    pub fn as_str_chirho(&self) -> &'static str {
        match self {
            Self::SeriesChirho => "Series",
            Self::ProgressionChirho => "Progression",
            Self::AlternativeChirho => "Alternative",
            Self::ComparisonChirho => "Comparison",
            Self::ContrastChirho => "Contrast",
            Self::GroundChirho => "Ground",
            Self::InferenceChirho => "Inference",
            Self::PurposeChirho => "Purpose",
            Self::ResultChirho => "Result",
            Self::ConditionChirho => "Condition",
            Self::ConcessionChirho => "Concession",
            Self::TemporalChirho => "Temporal",
            Self::MannerChirho => "Manner",
            Self::MeansChirho => "Means",
            Self::ContentChirho => "Content",
            Self::ExplanationChirho => "Explanation",
            Self::ActionMannerChirho => "ActionManner",
            Self::SituationResponseChirho => "SituationResponse",
            Self::BilateralChirho => "Bilateral",
            Self::QuestionAnswerChirho => "QuestionAnswer",
            Self::NegativePositiveChirho => "NegativePositive",
            Self::GeneralSpecificChirho => "GeneralSpecific",
        }
    }

    /// Parse from string.
    pub fn from_str_chirho(s_chirho: &str) -> Option<Self> {
        match s_chirho {
            "Series" => Some(Self::SeriesChirho),
            "Progression" => Some(Self::ProgressionChirho),
            "Alternative" => Some(Self::AlternativeChirho),
            "Comparison" => Some(Self::ComparisonChirho),
            "Contrast" => Some(Self::ContrastChirho),
            "Ground" => Some(Self::GroundChirho),
            "Inference" => Some(Self::InferenceChirho),
            "Purpose" => Some(Self::PurposeChirho),
            "Result" => Some(Self::ResultChirho),
            "Condition" => Some(Self::ConditionChirho),
            "Concession" => Some(Self::ConcessionChirho),
            "Temporal" => Some(Self::TemporalChirho),
            "Manner" => Some(Self::MannerChirho),
            "Means" => Some(Self::MeansChirho),
            "Content" => Some(Self::ContentChirho),
            "Explanation" => Some(Self::ExplanationChirho),
            "ActionManner" => Some(Self::ActionMannerChirho),
            "SituationResponse" => Some(Self::SituationResponseChirho),
            "Bilateral" => Some(Self::BilateralChirho),
            "QuestionAnswer" => Some(Self::QuestionAnswerChirho),
            "NegativePositive" => Some(Self::NegativePositiveChirho),
            "GeneralSpecific" => Some(Self::GeneralSpecificChirho),
            _ => None,
        }
    }
}

/// A logical relationship between two propositions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipChirho {
    /// Source proposition ID.
    pub source_id_chirho: PropositionIdChirho,
    /// Target proposition ID.
    pub target_id_chirho: PropositionIdChirho,
    /// Type of relationship.
    pub relationship_type_chirho: RelationshipTypeChirho,
    /// Strength of the relationship (1=weak, 2=moderate, 3=strong).
    pub strength_chirho: u8,
    /// Optional notes.
    pub notes_chirho: Option<String>,
}

/// A complete arc structure (discourse analysis of a passage).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStructureChirho {
    /// Unique ID.
    pub id_chirho: Option<ArcIdChirho>,
    /// Title or label for this analysis.
    pub title_chirho: String,
    /// The passage being analyzed.
    pub passage_chirho: VerseSpanChirho,
    /// All propositions in this analysis.
    pub propositions_chirho: Vec<PropositionChirho>,
    /// Relationships between propositions.
    pub relationships_chirho: Vec<RelationshipChirho>,
    /// The main (thesis) proposition ID.
    pub main_proposition_id_chirho: Option<PropositionIdChirho>,
    /// Tags for categorization.
    pub tags_chirho: Vec<String>,
    /// Bracket structure (hierarchical grouping).
    pub brackets_chirho: Vec<BracketNodeChirho>,
}

/// Bracket node types for hierarchical grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketTypeChirho {
    /// Propositions at the same level.
    CoordinateChirho,
    /// One proposition supporting another.
    SubordinateChirho,
}

/// A bracket grouping node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketNodeChirho {
    /// Optional label.
    pub label_chirho: Option<String>,
    /// Bracket type.
    pub bracket_type_chirho: BracketTypeChirho,
    /// Proposition IDs in this bracket group.
    pub proposition_ids_chirho: Vec<PropositionIdChirho>,
    /// Display order.
    pub display_order_chirho: u32,
}

/// Grammatical phrase role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhraseRoleChirho {
    SubjectChirho,
    VerbChirho,
    ObjectChirho,
    ModifierChirho,
    ComplementChirho,
}

/// A phrase within a clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhraseChirho {
    pub text_chirho: String,
    pub role_chirho: PhraseRoleChirho,
}

/// Grammatical phrase structure for a proposition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhraseStructureChirho {
    /// The phrases making up this proposition.
    pub phrases_chirho: Vec<PhraseChirho>,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_relationship_type_roundtrip_chirho() {
        for rt_chirho in RelationshipTypeChirho::all_chirho() {
            let s_chirho = rt_chirho.as_str_chirho();
            let parsed_chirho = RelationshipTypeChirho::from_str_chirho(s_chirho).unwrap();
            assert_eq!(*rt_chirho, parsed_chirho);
        }
    }

    #[test]
    fn test_relationship_type_count_chirho() {
        assert_eq!(RelationshipTypeChirho::all_chirho().len(), 22);
    }

    #[test]
    fn test_relationship_type_unknown_chirho() {
        assert!(RelationshipTypeChirho::from_str_chirho("Unknown").is_none());
    }

    #[test]
    fn test_proposition_serde_chirho() {
        let prop_chirho = PropositionChirho {
            id_chirho: PropositionIdChirho(1),
            text_chirho: "For God so loved the world".to_string(),
            verse_span_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                start_char_chirho: Some(0),
                end_char_chirho: Some(25),
            },
            notes_chirho: None,
            label_chirho: "1a".to_string(),
            phrase_structure_chirho: None,
        };

        let json_chirho = serde_json::to_string(&prop_chirho).unwrap();
        let parsed_chirho: PropositionChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.text_chirho, "For God so loved the world");
    }

    #[test]
    fn test_arc_structure_serde_chirho() {
        let arc_chirho = ArcStructureChirho {
            id_chirho: Some(ArcIdChirho(1)),
            title_chirho: "John 3:16 Arc".to_string(),
            passage_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 18).unwrap(),
                start_char_chirho: None,
                end_char_chirho: None,
            },
            propositions_chirho: Vec::new(),
            relationships_chirho: Vec::new(),
            main_proposition_id_chirho: None,
            tags_chirho: vec!["gospel".to_string()],
            brackets_chirho: Vec::new(),
        };

        let json_chirho = serde_json::to_string(&arc_chirho).unwrap();
        let parsed_chirho: ArcStructureChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.title_chirho, "John 3:16 Arc");
    }

    #[test]
    fn test_phrase_structure_chirho() {
        let ps_chirho = PhraseStructureChirho {
            phrases_chirho: vec![
                PhraseChirho {
                    text_chirho: "God".to_string(),
                    role_chirho: PhraseRoleChirho::SubjectChirho,
                },
                PhraseChirho {
                    text_chirho: "loved".to_string(),
                    role_chirho: PhraseRoleChirho::VerbChirho,
                },
                PhraseChirho {
                    text_chirho: "the world".to_string(),
                    role_chirho: PhraseRoleChirho::ObjectChirho,
                },
            ],
        };

        let json_chirho = serde_json::to_string(&ps_chirho).unwrap();
        let parsed_chirho: PhraseStructureChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.phrases_chirho.len(), 3);
    }
}
