// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Morphology query — searches tokens in a `.rhema` module by grammatical features.

use rusqlite::{params_from_iter, Connection};

use rhema_contracts_chirho::morphology_chirho::MorphConstraintChirho;

use crate::error_chirho::ModuleErrorChirho;

/// A morphology search hit from the module.
#[derive(Debug, Clone)]
pub struct MorphHitChirho {
    pub verse_id_chirho: i64,
    pub book_chirho: String,
    pub chapter_chirho: u32,
    pub verse_num_chirho: u32,
    pub text_chirho: String,
    pub surface_chirho: String,
    pub lemma_chirho: Option<String>,
    pub strong_chirho: Option<String>,
    pub morph_raw_chirho: Option<String>,
    pub word_index_chirho: u32,
}

/// Search for tokens matching a morphology constraint.
pub fn search_morph_chirho(
    conn_chirho: &Connection,
    constraint_chirho: &MorphConstraintChirho,
    max_results_chirho: usize,
) -> Result<Vec<MorphHitChirho>, ModuleErrorChirho> {
    let mut where_clauses_chirho: Vec<String> = Vec::new();
    let mut param_values_chirho: Vec<String> = Vec::new();

    if let Some(ref pos_chirho) = constraint_chirho.part_of_speech_chirho {
        where_clauses_chirho.push("t.pos_chirho = ?".to_string());
        param_values_chirho.push(format!("{pos_chirho:?}"));
    }
    if let Some(ref tense_chirho) = constraint_chirho.tense_chirho {
        where_clauses_chirho.push("t.tense_chirho = ?".to_string());
        param_values_chirho.push(format!("{tense_chirho:?}"));
    }
    if let Some(ref voice_chirho) = constraint_chirho.voice_chirho {
        where_clauses_chirho.push("t.voice_chirho = ?".to_string());
        param_values_chirho.push(format!("{voice_chirho:?}"));
    }
    if let Some(ref mood_chirho) = constraint_chirho.mood_chirho {
        where_clauses_chirho.push("t.mood_chirho = ?".to_string());
        param_values_chirho.push(format!("{mood_chirho:?}"));
    }
    if let Some(ref case_chirho) = constraint_chirho.case_chirho {
        where_clauses_chirho.push("t.case_chirho = ?".to_string());
        param_values_chirho.push(format!("{case_chirho:?}"));
    }
    if let Some(ref number_chirho) = constraint_chirho.number_chirho {
        where_clauses_chirho.push("t.number_chirho = ?".to_string());
        param_values_chirho.push(format!("{number_chirho:?}"));
    }
    if let Some(ref gender_chirho) = constraint_chirho.gender_chirho {
        where_clauses_chirho.push("t.gender_chirho = ?".to_string());
        param_values_chirho.push(format!("{gender_chirho:?}"));
    }
    if let Some(ref person_chirho) = constraint_chirho.person_chirho {
        where_clauses_chirho.push("t.person_chirho = ?".to_string());
        param_values_chirho.push(format!("{person_chirho:?}"));
    }

    if where_clauses_chirho.is_empty() {
        return Ok(Vec::new());
    }

    let where_sql_chirho = where_clauses_chirho.join(" AND ");
    let sql_chirho = format!(
        "SELECT t.verse_id_chirho, v.book_chirho, v.chapter_chirho, v.verse_chirho, v.text_chirho, \
         t.surface_chirho, t.lemma_chirho, t.strong_chirho, t.morph_raw_chirho, t.word_index_chirho \
         FROM tokens_chirho t \
         JOIN verses_chirho v ON v.id_chirho = t.verse_id_chirho \
         WHERE {where_sql_chirho} \
         LIMIT {max_results_chirho}"
    );

    let mut stmt_chirho = conn_chirho.prepare(&sql_chirho)?;
    let rows_chirho = stmt_chirho
        .query_map(params_from_iter(param_values_chirho.iter()), |row_chirho| {
            Ok(MorphHitChirho {
                verse_id_chirho: row_chirho.get(0)?,
                book_chirho: row_chirho.get(1)?,
                chapter_chirho: row_chirho.get(2)?,
                verse_num_chirho: row_chirho.get(3)?,
                text_chirho: row_chirho.get(4)?,
                surface_chirho: row_chirho.get(5)?,
                lemma_chirho: row_chirho.get(6)?,
                strong_chirho: row_chirho.get(7)?,
                morph_raw_chirho: row_chirho.get(8)?,
                word_index_chirho: row_chirho.get(9)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhema_contracts_chirho::morphology_chirho::{
        CaseChirho, GrammaticalNumberChirho, MoodChirho, PartOfSpeechChirho, PersonChirho,
        TenseChirho, VoiceChirho,
    };
    use rusqlite::Connection;

    fn setup_morph_db_chirho() -> Connection {
        let conn_chirho = Connection::open_in_memory().unwrap();
        crate::schema_chirho::apply_schema_chirho(&conn_chirho).unwrap();

        conn_chirho
            .execute_batch(
                "INSERT INTO verses_chirho (id_chirho, book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho) \
                 VALUES (1, 'John', 3, 16, 'For God so loved the world', 'KJV');
                 INSERT INTO verses_chirho (id_chirho, book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho) \
                 VALUES (2, 'John', 3, 17, 'For God sent not his Son', 'KJV');

                 INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
                  lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, tense_chirho, \
                  voice_chirho, mood_chirho, person_chirho, number_chirho, language_chirho) \
                 VALUES (1, 0, 'loved', 'agapao', 'G25', 'V-AAI-3S', 'VerbChirho', 'AoristChirho', \
                  'ActiveChirho', 'IndicativeChirho', 'ThirdChirho', 'SingularChirho', 'Greek');

                 INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
                  lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, case_chirho, \
                  number_chirho, gender_chirho, language_chirho) \
                 VALUES (1, 1, 'world', 'kosmos', 'G2889', 'N-ASM', 'NounChirho', 'AccusativeChirho', \
                  'SingularChirho', 'MasculineChirho', 'Greek');

                 INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
                  lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, tense_chirho, \
                  voice_chirho, mood_chirho, person_chirho, number_chirho, language_chirho) \
                 VALUES (2, 0, 'sent', 'apostello', 'G649', 'V-AAI-3S', 'VerbChirho', 'AoristChirho', \
                  'ActiveChirho', 'IndicativeChirho', 'ThirdChirho', 'SingularChirho', 'Greek');

                 INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
                  lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, case_chirho, \
                  number_chirho, gender_chirho, language_chirho) \
                 VALUES (2, 1, 'Son', 'huios', 'G5207', 'N-ASM', 'NounChirho', 'AccusativeChirho', \
                  'SingularChirho', 'MasculineChirho', 'Greek');"
            )
            .unwrap();

        conn_chirho
    }

    #[test]
    fn test_search_morph_pos_verb_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::VerbChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert_eq!(hits_chirho.len(), 2);
        assert!(hits_chirho.iter().all(|h_chirho| h_chirho.surface_chirho == "loved" || h_chirho.surface_chirho == "sent"));
    }

    #[test]
    fn test_search_morph_pos_noun_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::NounChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert_eq!(hits_chirho.len(), 2);
    }

    #[test]
    fn test_search_morph_verb_aorist_active_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::VerbChirho),
            tense_chirho: Some(TenseChirho::AoristChirho),
            voice_chirho: Some(VoiceChirho::ActiveChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert_eq!(hits_chirho.len(), 2);
    }

    #[test]
    fn test_search_morph_verb_aorist_active_indicative_3s_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::VerbChirho),
            tense_chirho: Some(TenseChirho::AoristChirho),
            voice_chirho: Some(VoiceChirho::ActiveChirho),
            mood_chirho: Some(MoodChirho::IndicativeChirho),
            person_chirho: Some(PersonChirho::ThirdChirho),
            number_chirho: Some(GrammaticalNumberChirho::SingularChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert_eq!(hits_chirho.len(), 2);
    }

    #[test]
    fn test_search_morph_noun_accusative_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::NounChirho),
            case_chirho: Some(CaseChirho::AccusativeChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert_eq!(hits_chirho.len(), 2);
    }

    #[test]
    fn test_search_morph_no_match_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::AdverbChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert!(hits_chirho.is_empty());
    }

    #[test]
    fn test_search_morph_empty_constraint_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho::default();
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 100).unwrap();
        assert!(hits_chirho.is_empty());
    }

    #[test]
    fn test_search_morph_max_results_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::VerbChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 1).unwrap();
        assert_eq!(hits_chirho.len(), 1);
    }

    #[test]
    fn test_search_morph_hit_fields_chirho() {
        let conn_chirho = setup_morph_db_chirho();
        let constraint_chirho = MorphConstraintChirho {
            part_of_speech_chirho: Some(PartOfSpeechChirho::NounChirho),
            ..Default::default()
        };
        let hits_chirho = search_morph_chirho(&conn_chirho, &constraint_chirho, 1).unwrap();
        let hit_chirho = &hits_chirho[0];
        assert!(!hit_chirho.book_chirho.is_empty());
        assert!(!hit_chirho.text_chirho.is_empty());
        assert!(hit_chirho.strong_chirho.is_some());
    }
}
