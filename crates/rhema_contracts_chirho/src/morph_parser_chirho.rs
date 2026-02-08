// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Morphology code parser — converts Robinson Greek / OSHM Hebrew codes
//! into [`ParsedMorphologyChirho`] structs.

use crate::morphology_chirho::{
    CaseChirho, GenderChirho, GrammaticalNumberChirho, HebrewStemChirho, HebrewStateChirho,
    MoodChirho, MorphCodeChirho, MorphConstraintChirho, MorphSystemChirho,
    ParsedMorphologyChirho, PartOfSpeechChirho, PersonChirho, TenseChirho, VoiceChirho,
};

/// Parses morphology code strings into structured [`ParsedMorphologyChirho`].
pub struct MorphParserChirho;

impl MorphParserChirho {
    /// Parse a morphology code string, auto-detecting Robinson vs OSHM format.
    pub fn parse_chirho(code_chirho: &str) -> Result<ParsedMorphologyChirho, MorphParseErrorChirho> {
        if code_chirho.is_empty() {
            return Err(MorphParseErrorChirho::EmptyCodeChirho);
        }

        if is_oshm_chirho(code_chirho) {
            parse_oshm_chirho(code_chirho)
        } else {
            parse_robinson_chirho(code_chirho)
        }
    }

    /// Build a [`MorphConstraintChirho`] from user-friendly field names.
    pub fn constraint_from_code_chirho(
        code_chirho: &str,
    ) -> Result<MorphConstraintChirho, MorphParseErrorChirho> {
        let parsed_chirho = Self::parse_chirho(code_chirho)?;
        Ok(MorphConstraintChirho {
            part_of_speech_chirho: Some(parsed_chirho.part_of_speech_chirho),
            person_chirho: parsed_chirho.person_chirho,
            number_chirho: parsed_chirho.number_chirho,
            gender_chirho: parsed_chirho.gender_chirho,
            tense_chirho: parsed_chirho.tense_chirho,
            voice_chirho: parsed_chirho.voice_chirho,
            mood_chirho: parsed_chirho.mood_chirho,
            case_chirho: parsed_chirho.case_chirho,
        })
    }
}

/// Errors from morphology code parsing.
#[derive(Debug, Clone, thiserror::Error)]
pub enum MorphParseErrorChirho {
    #[error("Empty morphology code")]
    EmptyCodeChirho,

    #[error("Unknown part of speech: '{value_chirho}'")]
    UnknownPosChirho { value_chirho: String },

    #[error("Invalid Robinson format: '{value_chirho}'")]
    InvalidRobinsonChirho { value_chirho: String },

    #[error("Invalid OSHM format: '{value_chirho}'")]
    InvalidOshmChirho { value_chirho: String },

    #[error("Unknown morphology feature: '{value_chirho}'")]
    UnknownFeatureChirho { value_chirho: String },
}

/// Detect OSHM Hebrew format: starts with 'H' followed by uppercase POS letter.
fn is_oshm_chirho(code_chirho: &str) -> bool {
    let bytes_chirho = code_chirho.as_bytes();
    bytes_chirho.len() >= 2
        && bytes_chirho[0] == b'H'
        && bytes_chirho[1].is_ascii_uppercase()
}

/// Parse Robinson Greek morphology code.
///
/// Format: `POS[-Tense+Voice+Mood[-Person+Number]][-Case[-Gender]]`
/// Examples: `V-AAI-3S`, `N-NSM`, `A-GSF`, `CONJ`, `PREP`
fn parse_robinson_chirho(
    code_chirho: &str,
) -> Result<ParsedMorphologyChirho, MorphParseErrorChirho> {
    let parts_chirho: Vec<&str> = code_chirho.split('-').collect();

    let pos_chirho = match parts_chirho.first() {
        Some(&"V") => PartOfSpeechChirho::VerbChirho,
        Some(&"N") => PartOfSpeechChirho::NounChirho,
        Some(&"A") => PartOfSpeechChirho::AdjectiveChirho,
        Some(&"R") => PartOfSpeechChirho::PronounChirho,
        Some(&"C") | Some(&"CONJ") => PartOfSpeechChirho::ConjunctionChirho,
        Some(&"D") | Some(&"ADV") => PartOfSpeechChirho::AdverbChirho,
        Some(&"P") | Some(&"PREP") => PartOfSpeechChirho::PrepositionChirho,
        Some(&"T") => PartOfSpeechChirho::ArticleChirho,
        Some(&"X") | Some(&"PRT") => PartOfSpeechChirho::ParticleChirho,
        Some(&"I") | Some(&"INJ") => PartOfSpeechChirho::InterjectionChirho,
        Some(val_chirho) => {
            return Err(MorphParseErrorChirho::UnknownPosChirho {
                value_chirho: val_chirho.to_string(),
            });
        }
        None => {
            return Err(MorphParseErrorChirho::EmptyCodeChirho);
        }
    };

    let mut tense_chirho = None;
    let mut voice_chirho = None;
    let mut mood_chirho = None;
    let mut person_chirho = None;
    let mut number_chirho = None;
    let mut case_chirho = None;
    let mut gender_chirho = None;

    if pos_chirho == PartOfSpeechChirho::VerbChirho && parts_chirho.len() >= 2 {
        // V-TVM or V-TVM-PN format
        let tvm_chirho = parts_chirho[1];
        let chars_chirho: Vec<char> = tvm_chirho.chars().collect();

        if !chars_chirho.is_empty() {
            tense_chirho = parse_robinson_tense_chirho(chars_chirho[0]);
        }
        if chars_chirho.len() > 1 {
            voice_chirho = parse_robinson_voice_chirho(chars_chirho[1]);
        }
        if chars_chirho.len() > 2 {
            mood_chirho = parse_robinson_mood_chirho(chars_chirho[2]);
        }

        // Person+Number in next segment (e.g., "3S", "1P")
        if parts_chirho.len() >= 3 {
            let pn_chirho = parts_chirho[2];
            let pn_chars_chirho: Vec<char> = pn_chirho.chars().collect();
            if !pn_chars_chirho.is_empty() {
                person_chirho = parse_robinson_person_chirho(pn_chars_chirho[0]);
            }
            if pn_chars_chirho.len() > 1 {
                number_chirho = parse_robinson_number_chirho(pn_chars_chirho[1]);
            }
        }

        // Participles can have case/gender in segment 3 or 4
        if mood_chirho == Some(MoodChirho::ParticipleChirho) && parts_chirho.len() >= 3 {
            let last_part_chirho = parts_chirho[parts_chirho.len() - 1];
            let last_chars_chirho: Vec<char> = last_part_chirho.chars().collect();
            // Could be CGN or NG or similar
            for ch_chirho in &last_chars_chirho {
                if case_chirho.is_none() {
                    if let Some(c_chirho) = parse_robinson_case_chirho(*ch_chirho) {
                        case_chirho = Some(c_chirho);
                        continue;
                    }
                }
                if gender_chirho.is_none() {
                    if let Some(g_chirho) = parse_robinson_gender_chirho(*ch_chirho) {
                        gender_chirho = Some(g_chirho);
                        continue;
                    }
                }
                if number_chirho.is_none() {
                    number_chirho = parse_robinson_number_chirho(*ch_chirho);
                }
            }
        }
    } else if parts_chirho.len() >= 2 {
        // Non-verb: N-CSG, A-GSF, R-NSM, T-GSN etc.
        let features_chirho = parts_chirho[1];
        let chars_chirho: Vec<char> = features_chirho.chars().collect();

        if !chars_chirho.is_empty() {
            case_chirho = parse_robinson_case_chirho(chars_chirho[0]);
        }
        if chars_chirho.len() > 1 {
            number_chirho = parse_robinson_number_chirho(chars_chirho[1]);
        }
        if chars_chirho.len() > 2 {
            gender_chirho = parse_robinson_gender_chirho(chars_chirho[2]);
        }
    }

    Ok(ParsedMorphologyChirho {
        raw_code_chirho: MorphCodeChirho::new_chirho(code_chirho),
        system_chirho: MorphSystemChirho::RobinsonChirho,
        part_of_speech_chirho: pos_chirho,
        person_chirho,
        number_chirho,
        gender_chirho,
        tense_chirho,
        voice_chirho,
        mood_chirho,
        case_chirho,
        hebrew_state_chirho: None,
        hebrew_stem_chirho: None,
    })
}

/// Parse OSHM Hebrew morphology code.
///
/// Format: `H` + POS char + positional features.
/// Examples: `HNcmsa` (Noun, common, masculine, singular, absolute),
///           `HVqp3ms` (Verb, Qal, perfect, 3rd, masculine, singular)
fn parse_oshm_chirho(
    code_chirho: &str,
) -> Result<ParsedMorphologyChirho, MorphParseErrorChirho> {
    let chars_chirho: Vec<char> = code_chirho.chars().collect();
    if chars_chirho.len() < 2 {
        return Err(MorphParseErrorChirho::InvalidOshmChirho {
            value_chirho: code_chirho.to_string(),
        });
    }

    // chars_chirho[0] == 'H', chars_chirho[1] == POS
    let pos_chirho = match chars_chirho[1] {
        'N' => PartOfSpeechChirho::NounChirho,
        'V' => PartOfSpeechChirho::VerbChirho,
        'A' => PartOfSpeechChirho::AdjectiveChirho,
        'R' => PartOfSpeechChirho::PronounChirho,
        'C' => PartOfSpeechChirho::ConjunctionChirho,
        'D' => PartOfSpeechChirho::AdverbChirho,
        'P' => PartOfSpeechChirho::PrepositionChirho,
        'T' => PartOfSpeechChirho::ArticleChirho,
        'S' => PartOfSpeechChirho::ParticleChirho,
        'I' => PartOfSpeechChirho::InterjectionChirho,
        other_chirho => {
            return Err(MorphParseErrorChirho::UnknownPosChirho {
                value_chirho: other_chirho.to_string(),
            });
        }
    };

    let mut tense_chirho = None;
    let voice_chirho = None;
    let mut person_chirho = None;
    let mut number_chirho = None;
    let mut gender_chirho = None;
    let mut hebrew_state_chirho = None;
    let mut hebrew_stem_chirho = None;

    if pos_chirho == PartOfSpeechChirho::VerbChirho {
        // HV + stem + tense + person + gender + number
        if chars_chirho.len() > 2 {
            hebrew_stem_chirho = parse_oshm_stem_chirho(chars_chirho[2]);
        }
        if chars_chirho.len() > 3 {
            tense_chirho = parse_oshm_tense_chirho(chars_chirho[3]);
        }
        if chars_chirho.len() > 4 {
            person_chirho = parse_oshm_person_chirho(chars_chirho[4]);
        }
        if chars_chirho.len() > 5 {
            gender_chirho = parse_oshm_gender_chirho(chars_chirho[5]);
        }
        if chars_chirho.len() > 6 {
            number_chirho = parse_oshm_number_chirho(chars_chirho[6]);
        }
    } else {
        // HN + type + gender + number + state
        // type is at index 2 (c=common, p=proper — informational, skip)
        if chars_chirho.len() > 3 {
            gender_chirho = parse_oshm_gender_chirho(chars_chirho[3]);
        }
        if chars_chirho.len() > 4 {
            number_chirho = parse_oshm_number_chirho(chars_chirho[4]);
        }
        if chars_chirho.len() > 5 {
            hebrew_state_chirho = parse_oshm_state_chirho(chars_chirho[5]);
        }
    }

    Ok(ParsedMorphologyChirho {
        raw_code_chirho: MorphCodeChirho::new_chirho(code_chirho),
        system_chirho: MorphSystemChirho::OshmChirho,
        part_of_speech_chirho: pos_chirho,
        person_chirho,
        number_chirho,
        gender_chirho,
        tense_chirho,
        voice_chirho,
        mood_chirho: None,
        case_chirho: None,
        hebrew_state_chirho,
        hebrew_stem_chirho,
    })
}

// ── Robinson helper parsers ──────────────────────────────────────

fn parse_robinson_tense_chirho(ch_chirho: char) -> Option<TenseChirho> {
    match ch_chirho {
        'P' => Some(TenseChirho::PresentChirho),
        'I' => Some(TenseChirho::ImperfectChirho),
        'F' => Some(TenseChirho::FutureChirho),
        'A' => Some(TenseChirho::AoristChirho),
        'X' => Some(TenseChirho::PerfectChirho),
        'Y' => Some(TenseChirho::PluperfectChirho),
        _ => None,
    }
}

fn parse_robinson_voice_chirho(ch_chirho: char) -> Option<VoiceChirho> {
    match ch_chirho {
        'A' => Some(VoiceChirho::ActiveChirho),
        'M' => Some(VoiceChirho::MiddleChirho),
        'P' => Some(VoiceChirho::PassiveChirho),
        'E' => Some(VoiceChirho::MiddlePassiveChirho),
        _ => None,
    }
}

fn parse_robinson_mood_chirho(ch_chirho: char) -> Option<MoodChirho> {
    match ch_chirho {
        'I' => Some(MoodChirho::IndicativeChirho),
        'S' => Some(MoodChirho::SubjunctiveChirho),
        'O' => Some(MoodChirho::OptativeChirho),
        'M' => Some(MoodChirho::ImperativeChirho),
        'N' => Some(MoodChirho::InfinitiveChirho),
        'P' => Some(MoodChirho::ParticipleChirho),
        _ => None,
    }
}

fn parse_robinson_person_chirho(ch_chirho: char) -> Option<PersonChirho> {
    match ch_chirho {
        '1' => Some(PersonChirho::FirstChirho),
        '2' => Some(PersonChirho::SecondChirho),
        '3' => Some(PersonChirho::ThirdChirho),
        _ => None,
    }
}

fn parse_robinson_number_chirho(ch_chirho: char) -> Option<GrammaticalNumberChirho> {
    match ch_chirho {
        'S' => Some(GrammaticalNumberChirho::SingularChirho),
        'P' => Some(GrammaticalNumberChirho::PluralChirho),
        _ => None,
    }
}

fn parse_robinson_case_chirho(ch_chirho: char) -> Option<CaseChirho> {
    match ch_chirho {
        'N' => Some(CaseChirho::NominativeChirho),
        'G' => Some(CaseChirho::GenitiveChirho),
        'D' => Some(CaseChirho::DativeChirho),
        'A' => Some(CaseChirho::AccusativeChirho),
        'V' => Some(CaseChirho::VocativeChirho),
        _ => None,
    }
}

fn parse_robinson_gender_chirho(ch_chirho: char) -> Option<GenderChirho> {
    match ch_chirho {
        'M' => Some(GenderChirho::MasculineChirho),
        'F' => Some(GenderChirho::FeminineChirho),
        'N' => Some(GenderChirho::NeuterChirho),
        _ => None,
    }
}

// ── OSHM helper parsers ──────────────────────────────────────────

fn parse_oshm_stem_chirho(ch_chirho: char) -> Option<HebrewStemChirho> {
    match ch_chirho {
        'q' => Some(HebrewStemChirho::QalChirho),
        'N' => Some(HebrewStemChirho::NiphalChirho),
        'p' => Some(HebrewStemChirho::PielChirho),
        'P' => Some(HebrewStemChirho::PualChirho),
        'h' => Some(HebrewStemChirho::HiphilChirho),
        'H' => Some(HebrewStemChirho::HophalChirho),
        't' => Some(HebrewStemChirho::HithpaelChirho),
        _ => None,
    }
}

fn parse_oshm_tense_chirho(ch_chirho: char) -> Option<TenseChirho> {
    match ch_chirho {
        'p' => Some(TenseChirho::QatalChirho),
        'i' => Some(TenseChirho::YiqtolChirho),
        'w' => Some(TenseChirho::WayyiqtolChirho),
        'c' => Some(TenseChirho::WeqatalChirho),
        _ => None,
    }
}

fn parse_oshm_person_chirho(ch_chirho: char) -> Option<PersonChirho> {
    match ch_chirho {
        '1' => Some(PersonChirho::FirstChirho),
        '2' => Some(PersonChirho::SecondChirho),
        '3' => Some(PersonChirho::ThirdChirho),
        _ => None,
    }
}

fn parse_oshm_gender_chirho(ch_chirho: char) -> Option<GenderChirho> {
    match ch_chirho {
        'm' => Some(GenderChirho::MasculineChirho),
        'f' => Some(GenderChirho::FeminineChirho),
        'c' => Some(GenderChirho::CommonChirho),
        _ => None,
    }
}

fn parse_oshm_number_chirho(ch_chirho: char) -> Option<GrammaticalNumberChirho> {
    match ch_chirho {
        's' => Some(GrammaticalNumberChirho::SingularChirho),
        'p' => Some(GrammaticalNumberChirho::PluralChirho),
        'd' => Some(GrammaticalNumberChirho::DualChirho),
        _ => None,
    }
}

fn parse_oshm_state_chirho(ch_chirho: char) -> Option<HebrewStateChirho> {
    match ch_chirho {
        'a' => Some(HebrewStateChirho::AbsoluteChirho),
        'c' => Some(HebrewStateChirho::ConstructChirho),
        'd' => Some(HebrewStateChirho::DeterminedChirho),
        _ => None,
    }
}

// ── Public helpers for query parser ──────────────────────────────

/// Parse a human-friendly POS name into a [`PartOfSpeechChirho`].
pub fn parse_pos_name_chirho(name_chirho: &str) -> Option<PartOfSpeechChirho> {
    match name_chirho.to_lowercase().as_str() {
        "verb" | "v" => Some(PartOfSpeechChirho::VerbChirho),
        "noun" | "n" => Some(PartOfSpeechChirho::NounChirho),
        "adjective" | "adj" | "a" => Some(PartOfSpeechChirho::AdjectiveChirho),
        "adverb" | "adv" | "d" => Some(PartOfSpeechChirho::AdverbChirho),
        "preposition" | "prep" | "p" => Some(PartOfSpeechChirho::PrepositionChirho),
        "conjunction" | "conj" | "c" => Some(PartOfSpeechChirho::ConjunctionChirho),
        "article" | "art" | "t" => Some(PartOfSpeechChirho::ArticleChirho),
        "pronoun" | "pron" | "r" => Some(PartOfSpeechChirho::PronounChirho),
        "particle" | "prt" | "x" => Some(PartOfSpeechChirho::ParticleChirho),
        "interjection" | "inj" | "i" => Some(PartOfSpeechChirho::InterjectionChirho),
        _ => None,
    }
}

/// Parse a human-friendly tense name into a [`TenseChirho`].
pub fn parse_tense_name_chirho(name_chirho: &str) -> Option<TenseChirho> {
    match name_chirho.to_lowercase().as_str() {
        "present" => Some(TenseChirho::PresentChirho),
        "imperfect" => Some(TenseChirho::ImperfectChirho),
        "future" => Some(TenseChirho::FutureChirho),
        "aorist" => Some(TenseChirho::AoristChirho),
        "perfect" => Some(TenseChirho::PerfectChirho),
        "pluperfect" => Some(TenseChirho::PluperfectChirho),
        _ => None,
    }
}

/// Parse a human-friendly voice name into a [`VoiceChirho`].
pub fn parse_voice_name_chirho(name_chirho: &str) -> Option<VoiceChirho> {
    match name_chirho.to_lowercase().as_str() {
        "active" => Some(VoiceChirho::ActiveChirho),
        "middle" => Some(VoiceChirho::MiddleChirho),
        "passive" => Some(VoiceChirho::PassiveChirho),
        "middle_passive" | "middlepassive" | "middle/passive" => {
            Some(VoiceChirho::MiddlePassiveChirho)
        }
        _ => None,
    }
}

/// Parse a human-friendly mood name into a [`MoodChirho`].
pub fn parse_mood_name_chirho(name_chirho: &str) -> Option<MoodChirho> {
    match name_chirho.to_lowercase().as_str() {
        "indicative" => Some(MoodChirho::IndicativeChirho),
        "subjunctive" => Some(MoodChirho::SubjunctiveChirho),
        "optative" => Some(MoodChirho::OptativeChirho),
        "imperative" => Some(MoodChirho::ImperativeChirho),
        "infinitive" => Some(MoodChirho::InfinitiveChirho),
        "participle" => Some(MoodChirho::ParticipleChirho),
        _ => None,
    }
}

/// Parse a human-friendly case name into a [`CaseChirho`].
pub fn parse_case_name_chirho(name_chirho: &str) -> Option<CaseChirho> {
    match name_chirho.to_lowercase().as_str() {
        "nominative" | "nom" => Some(CaseChirho::NominativeChirho),
        "genitive" | "gen" => Some(CaseChirho::GenitiveChirho),
        "dative" | "dat" => Some(CaseChirho::DativeChirho),
        "accusative" | "acc" => Some(CaseChirho::AccusativeChirho),
        "vocative" | "voc" => Some(CaseChirho::VocativeChirho),
        _ => None,
    }
}

/// Parse a human-friendly number name into a [`GrammaticalNumberChirho`].
pub fn parse_number_name_chirho(name_chirho: &str) -> Option<GrammaticalNumberChirho> {
    match name_chirho.to_lowercase().as_str() {
        "singular" | "sing" | "sg" => Some(GrammaticalNumberChirho::SingularChirho),
        "plural" | "plur" | "pl" => Some(GrammaticalNumberChirho::PluralChirho),
        "dual" => Some(GrammaticalNumberChirho::DualChirho),
        _ => None,
    }
}

/// Parse a human-friendly gender name into a [`GenderChirho`].
pub fn parse_gender_name_chirho(name_chirho: &str) -> Option<GenderChirho> {
    match name_chirho.to_lowercase().as_str() {
        "masculine" | "masc" | "m" => Some(GenderChirho::MasculineChirho),
        "feminine" | "fem" | "f" => Some(GenderChirho::FeminineChirho),
        "neuter" | "neut" | "n" => Some(GenderChirho::NeuterChirho),
        "common" => Some(GenderChirho::CommonChirho),
        _ => None,
    }
}

/// Parse a human-friendly person name into a [`PersonChirho`].
pub fn parse_person_name_chirho(name_chirho: &str) -> Option<PersonChirho> {
    match name_chirho.to_lowercase().as_str() {
        "1" | "first" | "1st" => Some(PersonChirho::FirstChirho),
        "2" | "second" | "2nd" => Some(PersonChirho::SecondChirho),
        "3" | "third" | "3rd" => Some(PersonChirho::ThirdChirho),
        _ => None,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    // ── Robinson Greek tests ─────────────────────────────────────

    #[test]
    fn test_parse_robinson_verb_aai_3s_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("V-AAI-3S").unwrap();
        assert_eq!(parsed_chirho.system_chirho, MorphSystemChirho::RobinsonChirho);
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::VerbChirho);
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::AoristChirho));
        assert_eq!(parsed_chirho.voice_chirho, Some(VoiceChirho::ActiveChirho));
        assert_eq!(parsed_chirho.mood_chirho, Some(MoodChirho::IndicativeChirho));
        assert_eq!(parsed_chirho.person_chirho, Some(PersonChirho::ThirdChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::SingularChirho));
    }

    #[test]
    fn test_parse_robinson_verb_pai_1s_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("V-PAI-1S").unwrap();
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::PresentChirho));
        assert_eq!(parsed_chirho.voice_chirho, Some(VoiceChirho::ActiveChirho));
        assert_eq!(parsed_chirho.mood_chirho, Some(MoodChirho::IndicativeChirho));
        assert_eq!(parsed_chirho.person_chirho, Some(PersonChirho::FirstChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::SingularChirho));
    }

    #[test]
    fn test_parse_robinson_verb_pps_3p_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("V-PPS-3P").unwrap();
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::PresentChirho));
        assert_eq!(parsed_chirho.voice_chirho, Some(VoiceChirho::PassiveChirho));
        assert_eq!(parsed_chirho.mood_chirho, Some(MoodChirho::SubjunctiveChirho));
        assert_eq!(parsed_chirho.person_chirho, Some(PersonChirho::ThirdChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::PluralChirho));
    }

    #[test]
    fn test_parse_robinson_noun_nsm_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("N-NSM").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::NounChirho);
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::NominativeChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::SingularChirho));
        assert_eq!(parsed_chirho.gender_chirho, Some(GenderChirho::MasculineChirho));
    }

    #[test]
    fn test_parse_robinson_noun_gsf_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("N-GSF").unwrap();
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::GenitiveChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::SingularChirho));
        assert_eq!(parsed_chirho.gender_chirho, Some(GenderChirho::FeminineChirho));
    }

    #[test]
    fn test_parse_robinson_adjective_gsn_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("A-GSN").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::AdjectiveChirho);
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::GenitiveChirho));
        assert_eq!(parsed_chirho.gender_chirho, Some(GenderChirho::NeuterChirho));
    }

    #[test]
    fn test_parse_robinson_article_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("T-NSM").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::ArticleChirho);
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::NominativeChirho));
    }

    #[test]
    fn test_parse_robinson_pronoun_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("R-ASM").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::PronounChirho);
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::AccusativeChirho));
    }

    #[test]
    fn test_parse_robinson_conjunction_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("CONJ").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::ConjunctionChirho);
    }

    #[test]
    fn test_parse_robinson_preposition_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("PREP").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::PrepositionChirho);
    }

    #[test]
    fn test_parse_robinson_particle_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("PRT").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::ParticleChirho);
    }

    #[test]
    fn test_parse_robinson_verb_future_middle_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("V-FMI-2P").unwrap();
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::FutureChirho));
        assert_eq!(parsed_chirho.voice_chirho, Some(VoiceChirho::MiddleChirho));
        assert_eq!(parsed_chirho.person_chirho, Some(PersonChirho::SecondChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::PluralChirho));
    }

    #[test]
    fn test_parse_robinson_verb_perfect_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("V-XAI-3S").unwrap();
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::PerfectChirho));
    }

    #[test]
    fn test_parse_robinson_verb_pluperfect_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("V-YAI-3S").unwrap();
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::PluperfectChirho));
    }

    #[test]
    fn test_parse_robinson_vocative_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("N-VSM").unwrap();
        assert_eq!(parsed_chirho.case_chirho, Some(CaseChirho::VocativeChirho));
    }

    // ── OSHM Hebrew tests ────────────────────────────────────────

    #[test]
    fn test_parse_oshm_noun_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HNcmsa").unwrap();
        assert_eq!(parsed_chirho.system_chirho, MorphSystemChirho::OshmChirho);
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::NounChirho);
        assert_eq!(parsed_chirho.gender_chirho, Some(GenderChirho::MasculineChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::SingularChirho));
        assert_eq!(parsed_chirho.hebrew_state_chirho, Some(HebrewStateChirho::AbsoluteChirho));
    }

    #[test]
    fn test_parse_oshm_noun_construct_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HNcfpc").unwrap();
        assert_eq!(parsed_chirho.gender_chirho, Some(GenderChirho::FeminineChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::PluralChirho));
        assert_eq!(parsed_chirho.hebrew_state_chirho, Some(HebrewStateChirho::ConstructChirho));
    }

    #[test]
    fn test_parse_oshm_verb_qal_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HVqp3ms").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::VerbChirho);
        assert_eq!(parsed_chirho.hebrew_stem_chirho, Some(HebrewStemChirho::QalChirho));
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::QatalChirho));
        assert_eq!(parsed_chirho.person_chirho, Some(PersonChirho::ThirdChirho));
        assert_eq!(parsed_chirho.gender_chirho, Some(GenderChirho::MasculineChirho));
        assert_eq!(parsed_chirho.number_chirho, Some(GrammaticalNumberChirho::SingularChirho));
    }

    #[test]
    fn test_parse_oshm_verb_hiphil_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HVhi3fs").unwrap();
        assert_eq!(parsed_chirho.hebrew_stem_chirho, Some(HebrewStemChirho::HiphilChirho));
        assert_eq!(parsed_chirho.tense_chirho, Some(TenseChirho::YiqtolChirho));
    }

    #[test]
    fn test_parse_oshm_verb_niphal_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HVNp3ms").unwrap();
        assert_eq!(parsed_chirho.hebrew_stem_chirho, Some(HebrewStemChirho::NiphalChirho));
    }

    #[test]
    fn test_parse_oshm_verb_piel_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HVpp3ms").unwrap();
        assert_eq!(parsed_chirho.hebrew_stem_chirho, Some(HebrewStemChirho::PielChirho));
    }

    #[test]
    fn test_parse_oshm_conjunction_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HC").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::ConjunctionChirho);
    }

    #[test]
    fn test_parse_oshm_preposition_chirho() {
        let parsed_chirho = MorphParserChirho::parse_chirho("HP").unwrap();
        assert_eq!(parsed_chirho.part_of_speech_chirho, PartOfSpeechChirho::PrepositionChirho);
    }

    // ── Error cases ──────────────────────────────────────────────

    #[test]
    fn test_parse_empty_error_chirho() {
        assert!(matches!(
            MorphParserChirho::parse_chirho(""),
            Err(MorphParseErrorChirho::EmptyCodeChirho)
        ));
    }

    #[test]
    fn test_parse_unknown_pos_chirho() {
        assert!(MorphParserChirho::parse_chirho("Z-ABC").is_err());
    }

    // ── Constraint builder tests ─────────────────────────────────

    #[test]
    fn test_constraint_from_code_chirho() {
        let constraint_chirho =
            MorphParserChirho::constraint_from_code_chirho("V-AAI-3S").unwrap();
        assert_eq!(constraint_chirho.part_of_speech_chirho, Some(PartOfSpeechChirho::VerbChirho));
        assert_eq!(constraint_chirho.tense_chirho, Some(TenseChirho::AoristChirho));
        assert_eq!(constraint_chirho.voice_chirho, Some(VoiceChirho::ActiveChirho));
        assert_eq!(constraint_chirho.mood_chirho, Some(MoodChirho::IndicativeChirho));
    }

    // ── Name parser tests ────────────────────────────────────────

    #[test]
    fn test_parse_pos_names_chirho() {
        assert_eq!(parse_pos_name_chirho("verb"), Some(PartOfSpeechChirho::VerbChirho));
        assert_eq!(parse_pos_name_chirho("noun"), Some(PartOfSpeechChirho::NounChirho));
        assert_eq!(parse_pos_name_chirho("adj"), Some(PartOfSpeechChirho::AdjectiveChirho));
        assert_eq!(parse_pos_name_chirho("Verb"), Some(PartOfSpeechChirho::VerbChirho));
        assert_eq!(parse_pos_name_chirho("unknown"), None);
    }

    #[test]
    fn test_parse_tense_names_chirho() {
        assert_eq!(parse_tense_name_chirho("aorist"), Some(TenseChirho::AoristChirho));
        assert_eq!(parse_tense_name_chirho("present"), Some(TenseChirho::PresentChirho));
        assert_eq!(parse_tense_name_chirho("future"), Some(TenseChirho::FutureChirho));
        assert_eq!(parse_tense_name_chirho("xyz"), None);
    }

    #[test]
    fn test_parse_voice_names_chirho() {
        assert_eq!(parse_voice_name_chirho("active"), Some(VoiceChirho::ActiveChirho));
        assert_eq!(parse_voice_name_chirho("passive"), Some(VoiceChirho::PassiveChirho));
        assert_eq!(parse_voice_name_chirho("middle"), Some(VoiceChirho::MiddleChirho));
    }

    #[test]
    fn test_parse_mood_names_chirho() {
        assert_eq!(parse_mood_name_chirho("indicative"), Some(MoodChirho::IndicativeChirho));
        assert_eq!(parse_mood_name_chirho("participle"), Some(MoodChirho::ParticipleChirho));
    }

    #[test]
    fn test_parse_case_names_chirho() {
        assert_eq!(parse_case_name_chirho("genitive"), Some(CaseChirho::GenitiveChirho));
        assert_eq!(parse_case_name_chirho("nom"), Some(CaseChirho::NominativeChirho));
        assert_eq!(parse_case_name_chirho("acc"), Some(CaseChirho::AccusativeChirho));
    }

    #[test]
    fn test_parse_number_names_chirho() {
        assert_eq!(parse_number_name_chirho("singular"), Some(GrammaticalNumberChirho::SingularChirho));
        assert_eq!(parse_number_name_chirho("pl"), Some(GrammaticalNumberChirho::PluralChirho));
        assert_eq!(parse_number_name_chirho("dual"), Some(GrammaticalNumberChirho::DualChirho));
    }

    #[test]
    fn test_parse_gender_names_chirho() {
        assert_eq!(parse_gender_name_chirho("masculine"), Some(GenderChirho::MasculineChirho));
        assert_eq!(parse_gender_name_chirho("f"), Some(GenderChirho::FeminineChirho));
        assert_eq!(parse_gender_name_chirho("neuter"), Some(GenderChirho::NeuterChirho));
    }

    #[test]
    fn test_parse_person_names_chirho() {
        assert_eq!(parse_person_name_chirho("1"), Some(PersonChirho::FirstChirho));
        assert_eq!(parse_person_name_chirho("3rd"), Some(PersonChirho::ThirdChirho));
        assert_eq!(parse_person_name_chirho("second"), Some(PersonChirho::SecondChirho));
    }

    #[test]
    fn test_morph_constraint_default_chirho() {
        let constraint_chirho = MorphConstraintChirho::default();
        assert!(constraint_chirho.part_of_speech_chirho.is_none());
        assert!(constraint_chirho.tense_chirho.is_none());
    }
}
