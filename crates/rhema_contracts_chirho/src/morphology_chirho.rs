// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Morphology type contracts — grammatical analysis newtypes and enums.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Raw morphology code string, validated to be non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MorphCodeChirho(pub String);

impl MorphCodeChirho {
    pub fn new_chirho(code_chirho: &str) -> Self {
        Self(code_chirho.to_string())
    }

    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MorphCodeChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}", self.0)
    }
}

/// Morphology system (which tagging convention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MorphSystemChirho {
    RobinsonChirho,  // Greek NT
    OshmChirho,      // Hebrew OT
    UnknownChirho,
}

/// Part of speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartOfSpeechChirho {
    NounChirho,
    VerbChirho,
    AdjectiveChirho,
    AdverbChirho,
    PrepositionChirho,
    ConjunctionChirho,
    ArticleChirho,
    PronounChirho,
    ParticleChirho,
    InterjectionChirho,
    NumeralChirho,
    OtherChirho,
}

/// Grammatical person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersonChirho {
    FirstChirho,
    SecondChirho,
    ThirdChirho,
}

/// Grammatical number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrammaticalNumberChirho {
    SingularChirho,
    PluralChirho,
    DualChirho,
}

/// Grammatical gender.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GenderChirho {
    MasculineChirho,
    FeminineChirho,
    NeuterChirho,
    CommonChirho,
}

/// Verb tense.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TenseChirho {
    PresentChirho,
    ImperfectChirho,
    FutureChirho,
    AoristChirho,
    PerfectChirho,
    PluperfectChirho,
    // Hebrew
    QatalChirho,     // Perfect
    YiqtolChirho,    // Imperfect
    WayyiqtolChirho, // Consecutive imperfect
    WeqatalChirho,   // Consecutive perfect
}

/// Verb voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoiceChirho {
    ActiveChirho,
    MiddleChirho,
    PassiveChirho,
    MiddlePassiveChirho,
}

/// Verb mood.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoodChirho {
    IndicativeChirho,
    SubjunctiveChirho,
    OptativeChirho,
    ImperativeChirho,
    InfinitiveChirho,
    ParticipleChirho,
}

/// Noun/adjective case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaseChirho {
    NominativeChirho,
    GenitiveChirho,
    DativeChirho,
    AccusativeChirho,
    VocativeChirho,
}

/// Hebrew noun state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HebrewStateChirho {
    AbsoluteChirho,
    ConstructChirho,
    DeterminedChirho,
}

/// Hebrew verb stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HebrewStemChirho {
    QalChirho,
    NiphalChirho,
    PielChirho,
    PualChirho,
    HiphilChirho,
    HophalChirho,
    HithpaelChirho,
}

/// Fully parsed morphology with all grammatical features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedMorphologyChirho {
    pub raw_code_chirho: MorphCodeChirho,
    pub system_chirho: MorphSystemChirho,
    pub part_of_speech_chirho: PartOfSpeechChirho,
    pub person_chirho: Option<PersonChirho>,
    pub number_chirho: Option<GrammaticalNumberChirho>,
    pub gender_chirho: Option<GenderChirho>,
    pub tense_chirho: Option<TenseChirho>,
    pub voice_chirho: Option<VoiceChirho>,
    pub mood_chirho: Option<MoodChirho>,
    pub case_chirho: Option<CaseChirho>,
    pub hebrew_state_chirho: Option<HebrewStateChirho>,
    pub hebrew_stem_chirho: Option<HebrewStemChirho>,
}

/// Constraint for morphology search queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphConstraintChirho {
    pub part_of_speech_chirho: Option<PartOfSpeechChirho>,
    pub person_chirho: Option<PersonChirho>,
    pub number_chirho: Option<GrammaticalNumberChirho>,
    pub gender_chirho: Option<GenderChirho>,
    pub tense_chirho: Option<TenseChirho>,
    pub voice_chirho: Option<VoiceChirho>,
    pub mood_chirho: Option<MoodChirho>,
    pub case_chirho: Option<CaseChirho>,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_morph_code_creation_chirho() {
        let code_chirho = MorphCodeChirho::new_chirho("V-AAI-3S");
        assert_eq!(code_chirho.as_str_chirho(), "V-AAI-3S");
    }

    #[test]
    fn test_morph_code_display_chirho() {
        let code_chirho = MorphCodeChirho::new_chirho("HNcmsa");
        assert_eq!(format!("{}", code_chirho), "HNcmsa");
    }

    #[test]
    fn test_morph_code_equality_chirho() {
        let a_chirho = MorphCodeChirho::new_chirho("V-AAI-3S");
        let b_chirho = MorphCodeChirho::new_chirho("V-AAI-3S");
        let c_chirho = MorphCodeChirho::new_chirho("N-NSM");
        assert_eq!(a_chirho, b_chirho);
        assert_ne!(a_chirho, c_chirho);
    }

    #[test]
    fn test_morph_code_serde_chirho() {
        let code_chirho = MorphCodeChirho::new_chirho("V-PAI-1S");
        let json_chirho = serde_json::to_string(&code_chirho).unwrap();
        let parsed_chirho: MorphCodeChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(code_chirho, parsed_chirho);
    }

    #[test]
    fn test_morph_system_variants_chirho() {
        assert_ne!(MorphSystemChirho::RobinsonChirho, MorphSystemChirho::OshmChirho);
        assert_ne!(MorphSystemChirho::RobinsonChirho, MorphSystemChirho::UnknownChirho);
    }

    #[test]
    fn test_part_of_speech_variants_chirho() {
        // Ensure all PartOfSpeech variants can be serialized
        let parts_chirho = vec![
            PartOfSpeechChirho::NounChirho,
            PartOfSpeechChirho::VerbChirho,
            PartOfSpeechChirho::AdjectiveChirho,
            PartOfSpeechChirho::AdverbChirho,
            PartOfSpeechChirho::PrepositionChirho,
            PartOfSpeechChirho::ConjunctionChirho,
            PartOfSpeechChirho::ArticleChirho,
            PartOfSpeechChirho::PronounChirho,
            PartOfSpeechChirho::ParticleChirho,
            PartOfSpeechChirho::InterjectionChirho,
            PartOfSpeechChirho::NumeralChirho,
            PartOfSpeechChirho::OtherChirho,
        ];
        for pos_chirho in &parts_chirho {
            let json_chirho = serde_json::to_string(pos_chirho).unwrap();
            let parsed_chirho: PartOfSpeechChirho = serde_json::from_str(&json_chirho).unwrap();
            assert_eq!(*pos_chirho, parsed_chirho);
        }
    }

    #[test]
    fn test_greek_tenses_chirho() {
        let tenses_chirho = vec![
            TenseChirho::PresentChirho,
            TenseChirho::ImperfectChirho,
            TenseChirho::FutureChirho,
            TenseChirho::AoristChirho,
            TenseChirho::PerfectChirho,
            TenseChirho::PluperfectChirho,
        ];
        assert_eq!(tenses_chirho.len(), 6);
    }

    #[test]
    fn test_hebrew_tenses_chirho() {
        let tenses_chirho = vec![
            TenseChirho::QatalChirho,
            TenseChirho::YiqtolChirho,
            TenseChirho::WayyiqtolChirho,
            TenseChirho::WeqatalChirho,
        ];
        assert_eq!(tenses_chirho.len(), 4);
    }

    #[test]
    fn test_hebrew_stems_chirho() {
        let stems_chirho = vec![
            HebrewStemChirho::QalChirho,
            HebrewStemChirho::NiphalChirho,
            HebrewStemChirho::PielChirho,
            HebrewStemChirho::PualChirho,
            HebrewStemChirho::HiphilChirho,
            HebrewStemChirho::HophalChirho,
            HebrewStemChirho::HithpaelChirho,
        ];
        assert_eq!(stems_chirho.len(), 7);
    }

    #[test]
    fn test_parsed_morphology_serde_chirho() {
        let parsed_chirho = ParsedMorphologyChirho {
            raw_code_chirho: MorphCodeChirho::new_chirho("V-AAI-3S"),
            system_chirho: MorphSystemChirho::RobinsonChirho,
            part_of_speech_chirho: PartOfSpeechChirho::VerbChirho,
            person_chirho: Some(PersonChirho::ThirdChirho),
            number_chirho: Some(GrammaticalNumberChirho::SingularChirho),
            gender_chirho: None,
            tense_chirho: Some(TenseChirho::AoristChirho),
            voice_chirho: Some(VoiceChirho::ActiveChirho),
            mood_chirho: Some(MoodChirho::IndicativeChirho),
            case_chirho: None,
            hebrew_state_chirho: None,
            hebrew_stem_chirho: None,
        };
        let json_chirho = serde_json::to_string(&parsed_chirho).unwrap();
        let roundtrip_chirho: ParsedMorphologyChirho =
            serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(roundtrip_chirho.part_of_speech_chirho, PartOfSpeechChirho::VerbChirho);
        assert_eq!(roundtrip_chirho.tense_chirho, Some(TenseChirho::AoristChirho));
    }

    #[test]
    fn test_morph_constraint_chirho() {
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::NounChirho),
            person_chirho: None,
            number_chirho: Some(GrammaticalNumberChirho::PluralChirho),
            gender_chirho: Some(GenderChirho::MasculineChirho),
            tense_chirho: None,
            voice_chirho: None,
            mood_chirho: None,
            case_chirho: Some(CaseChirho::GenitiveChirho),
        };
        let json_chirho = serde_json::to_string(&constraint_chirho).unwrap();
        let parsed_chirho: MorphConstraintChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::GenitiveChirho));
    }
}
