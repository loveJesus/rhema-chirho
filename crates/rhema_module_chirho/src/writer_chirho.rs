// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module writer — builds `.rhema` SQLite modules from verse and token data.

use std::path::Path;

use rusqlite::{params, Connection};

use rhema_contracts_chirho::morph_parser_chirho::MorphParserChirho;
use rhema_contracts_chirho::morphology_chirho::ParsedMorphologyChirho;

use crate::error_chirho::ModuleErrorChirho;
use crate::schema_chirho::apply_schema_chirho;

/// A verse to be written into a module.
#[derive(Debug, Clone)]
pub struct VerseEntryChirho {
    pub book_chirho: String,
    pub chapter_chirho: u32,
    pub verse_chirho: u32,
    pub text_chirho: String,
    pub module_chirho: String,
}

/// A word-level token to be written into a module.
#[derive(Debug, Clone)]
pub struct TokenEntryChirho {
    pub word_index_chirho: u32,
    pub surface_chirho: String,
    pub lemma_chirho: Option<String>,
    pub strong_chirho: Option<String>,
    pub morph_raw_chirho: Option<String>,
    pub language_chirho: String,
}

/// Builds a `.rhema` SQLite module file.
pub struct ModuleWriterChirho {
    conn_chirho: Connection,
}

impl ModuleWriterChirho {
    /// Create a new module file at the given path.
    pub fn create_chirho(path_chirho: &Path) -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open(path_chirho)?;
        apply_schema_chirho(&conn_chirho)?;
        Ok(Self { conn_chirho })
    }

    /// Create a module in memory (for testing).
    pub fn create_in_memory_chirho() -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open_in_memory()?;
        apply_schema_chirho(&conn_chirho)?;
        Ok(Self { conn_chirho })
    }

    /// Set a metadata key-value pair.
    pub fn set_meta_chirho(
        &self,
        key_chirho: &str,
        value_chirho: &str,
    ) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute(
            "INSERT OR REPLACE INTO module_meta_chirho (key_chirho, value_chirho) VALUES (?1, ?2)",
            params![key_chirho, value_chirho],
        )?;
        Ok(())
    }

    /// Insert a verse and return its row ID.
    pub fn insert_verse_chirho(
        &self,
        verse_chirho: &VerseEntryChirho,
    ) -> Result<i64, ModuleErrorChirho> {
        self.conn_chirho.execute(
            "INSERT INTO verses_chirho (book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                verse_chirho.book_chirho,
                verse_chirho.chapter_chirho,
                verse_chirho.verse_chirho,
                verse_chirho.text_chirho,
                verse_chirho.module_chirho,
            ],
        )?;
        Ok(self.conn_chirho.last_insert_rowid())
    }

    /// Insert a token for a given verse, auto-parsing the morph code.
    pub fn insert_token_chirho(
        &self,
        verse_id_chirho: i64,
        token_chirho: &TokenEntryChirho,
    ) -> Result<(), ModuleErrorChirho> {
        // Parse morph code if present.
        let parsed_chirho: Option<ParsedMorphologyChirho> = token_chirho
            .morph_raw_chirho
            .as_ref()
            .and_then(|code_chirho| MorphParserChirho::parse_chirho(code_chirho).ok());

        let pos_chirho = parsed_chirho.as_ref().map(|p_chirho| format!("{:?}", p_chirho.part_of_speech_chirho));
        let tense_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.tense_chirho.as_ref().map(|t_chirho| format!("{t_chirho:?}")));
        let voice_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.voice_chirho.as_ref().map(|v_chirho| format!("{v_chirho:?}")));
        let mood_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.mood_chirho.as_ref().map(|m_chirho| format!("{m_chirho:?}")));
        let case_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.case_chirho.as_ref().map(|c_chirho| format!("{c_chirho:?}")));
        let number_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.number_chirho.as_ref().map(|n_chirho| format!("{n_chirho:?}")));
        let gender_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.gender_chirho.as_ref().map(|g_chirho| format!("{g_chirho:?}")));
        let person_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.person_chirho.as_ref().map(|p2_chirho| format!("{p2_chirho:?}")));
        let hebrew_stem_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.hebrew_stem_chirho.as_ref().map(|s_chirho| format!("{s_chirho:?}")));
        let hebrew_state_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.hebrew_state_chirho.as_ref().map(|s_chirho| format!("{s_chirho:?}")));

        self.conn_chirho.execute(
            "INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
             lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, tense_chirho, voice_chirho, \
             mood_chirho, case_chirho, number_chirho, gender_chirho, person_chirho, \
             hebrew_stem_chirho, hebrew_state_chirho, language_chirho) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                verse_id_chirho,
                token_chirho.word_index_chirho,
                token_chirho.surface_chirho,
                token_chirho.lemma_chirho,
                token_chirho.strong_chirho,
                token_chirho.morph_raw_chirho,
                pos_chirho,
                tense_chirho,
                voice_chirho,
                mood_chirho,
                case_chirho,
                number_chirho,
                gender_chirho,
                person_chirho,
                hebrew_stem_chirho,
                hebrew_state_chirho,
                token_chirho.language_chirho,
            ],
        )?;
        Ok(())
    }

    /// Batch insert verses and their tokens within a transaction.
    pub fn batch_insert_chirho(
        &mut self,
        verses_chirho: &[(VerseEntryChirho, Vec<TokenEntryChirho>)],
    ) -> Result<usize, ModuleErrorChirho> {
        let tx_chirho = self.conn_chirho.transaction()?;
        let mut count_chirho = 0usize;

        for (verse_chirho, tokens_chirho) in verses_chirho {
            tx_chirho.execute(
                "INSERT INTO verses_chirho (book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho) \
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    verse_chirho.book_chirho,
                    verse_chirho.chapter_chirho,
                    verse_chirho.verse_chirho,
                    verse_chirho.text_chirho,
                    verse_chirho.module_chirho,
                ],
            )?;
            let verse_id_chirho = tx_chirho.last_insert_rowid();

            for tok_chirho in tokens_chirho {
                let parsed_chirho: Option<ParsedMorphologyChirho> = tok_chirho
                    .morph_raw_chirho
                    .as_ref()
                    .and_then(|code_chirho| MorphParserChirho::parse_chirho(code_chirho).ok());

                let pos_chirho = parsed_chirho.as_ref().map(|p_chirho| format!("{:?}", p_chirho.part_of_speech_chirho));
                let tense_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.tense_chirho.as_ref().map(|t_chirho| format!("{t_chirho:?}")));
                let voice_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.voice_chirho.as_ref().map(|v_chirho| format!("{v_chirho:?}")));
                let mood_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.mood_chirho.as_ref().map(|m_chirho| format!("{m_chirho:?}")));
                let case_val_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.case_chirho.as_ref().map(|c_chirho| format!("{c_chirho:?}")));
                let number_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.number_chirho.as_ref().map(|n_chirho| format!("{n_chirho:?}")));
                let gender_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.gender_chirho.as_ref().map(|g_chirho| format!("{g_chirho:?}")));
                let person_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.person_chirho.as_ref().map(|p2_chirho| format!("{p2_chirho:?}")));
                let hebrew_stem_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.hebrew_stem_chirho.as_ref().map(|s_chirho| format!("{s_chirho:?}")));
                let hebrew_state_chirho = parsed_chirho.as_ref().and_then(|p_chirho| p_chirho.hebrew_state_chirho.as_ref().map(|s_chirho| format!("{s_chirho:?}")));

                tx_chirho.execute(
                    "INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
                     lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, tense_chirho, voice_chirho, \
                     mood_chirho, case_chirho, number_chirho, gender_chirho, person_chirho, \
                     hebrew_stem_chirho, hebrew_state_chirho, language_chirho) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
                    params![
                        verse_id_chirho,
                        tok_chirho.word_index_chirho,
                        tok_chirho.surface_chirho,
                        tok_chirho.lemma_chirho,
                        tok_chirho.strong_chirho,
                        tok_chirho.morph_raw_chirho,
                        pos_chirho,
                        tense_chirho,
                        voice_chirho,
                        mood_chirho,
                        case_val_chirho,
                        number_chirho,
                        gender_chirho,
                        person_chirho,
                        hebrew_stem_chirho,
                        hebrew_state_chirho,
                        tok_chirho.language_chirho,
                    ],
                )?;
            }
            count_chirho += 1;
        }

        tx_chirho.commit()?;
        Ok(count_chirho)
    }

    /// Get the underlying connection (for testing).
    pub fn connection_chirho(&self) -> &Connection {
        &self.conn_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn test_verse_chirho() -> VerseEntryChirho {
        VerseEntryChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 3,
            verse_chirho: 16,
            text_chirho: "For God so loved the world".to_string(),
            module_chirho: "KJV".to_string(),
        }
    }

    fn test_token_chirho() -> TokenEntryChirho {
        TokenEntryChirho {
            word_index_chirho: 0,
            surface_chirho: "loved".to_string(),
            lemma_chirho: Some("agapao".to_string()),
            strong_chirho: Some("G25".to_string()),
            morph_raw_chirho: Some("V-AAI-3S".to_string()),
            language_chirho: "Greek".to_string(),
        }
    }

    #[test]
    fn test_create_in_memory_chirho() {
        let writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        assert!(writer_chirho.connection_chirho().is_autocommit());
    }

    #[test]
    fn test_set_meta_chirho() {
        let writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        writer_chirho.set_meta_chirho("module_name", "KJV").unwrap();

        let val_chirho: String = writer_chirho
            .connection_chirho()
            .query_row(
                "SELECT value_chirho FROM module_meta_chirho WHERE key_chirho = ?1",
                params!["module_name"],
                |row_chirho| row_chirho.get(0),
            )
            .unwrap();
        assert_eq!(val_chirho, "KJV");
    }

    #[test]
    fn test_insert_verse_chirho() {
        let writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        let id_chirho = writer_chirho.insert_verse_chirho(&test_verse_chirho()).unwrap();
        assert!(id_chirho > 0);
    }

    #[test]
    fn test_insert_token_chirho() {
        let writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        let verse_id_chirho = writer_chirho.insert_verse_chirho(&test_verse_chirho()).unwrap();
        writer_chirho
            .insert_token_chirho(verse_id_chirho, &test_token_chirho())
            .unwrap();

        // Verify parsed morph columns.
        let pos_chirho: String = writer_chirho
            .connection_chirho()
            .query_row(
                "SELECT pos_chirho FROM tokens_chirho WHERE verse_id_chirho = ?1",
                params![verse_id_chirho],
                |row_chirho| row_chirho.get(0),
            )
            .unwrap();
        assert_eq!(pos_chirho, "VerbChirho");
    }

    #[test]
    fn test_insert_token_no_morph_chirho() {
        let writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        let verse_id_chirho = writer_chirho.insert_verse_chirho(&test_verse_chirho()).unwrap();

        let token_chirho = TokenEntryChirho {
            word_index_chirho: 0,
            surface_chirho: "For".to_string(),
            lemma_chirho: None,
            strong_chirho: None,
            morph_raw_chirho: None,
            language_chirho: "English".to_string(),
        };
        writer_chirho
            .insert_token_chirho(verse_id_chirho, &token_chirho)
            .unwrap();

        let pos_chirho: Option<String> = writer_chirho
            .connection_chirho()
            .query_row(
                "SELECT pos_chirho FROM tokens_chirho WHERE verse_id_chirho = ?1",
                params![verse_id_chirho],
                |row_chirho| row_chirho.get(0),
            )
            .unwrap();
        assert!(pos_chirho.is_none());
    }

    #[test]
    fn test_batch_insert_chirho() {
        let mut writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        let entries_chirho = vec![
            (
                test_verse_chirho(),
                vec![test_token_chirho()],
            ),
            (
                VerseEntryChirho {
                    book_chirho: "John".to_string(),
                    chapter_chirho: 3,
                    verse_chirho: 17,
                    text_chirho: "For God sent not his Son".to_string(),
                    module_chirho: "KJV".to_string(),
                },
                vec![],
            ),
        ];

        let count_chirho = writer_chirho.batch_insert_chirho(&entries_chirho).unwrap();
        assert_eq!(count_chirho, 2);

        let verse_count_chirho: i64 = writer_chirho
            .connection_chirho()
            .query_row("SELECT COUNT(*) FROM verses_chirho", [], |row_chirho| {
                row_chirho.get(0)
            })
            .unwrap();
        assert_eq!(verse_count_chirho, 2);
    }

    #[test]
    fn test_create_file_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let path_chirho = dir_chirho.path().join("test.rhema");
        let writer_chirho = ModuleWriterChirho::create_chirho(&path_chirho).unwrap();
        writer_chirho.insert_verse_chirho(&test_verse_chirho()).unwrap();
        assert!(path_chirho.exists());
    }

    #[test]
    fn test_morph_parsing_in_token_chirho() {
        let writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        let verse_id_chirho = writer_chirho.insert_verse_chirho(&test_verse_chirho()).unwrap();

        let token_chirho = TokenEntryChirho {
            word_index_chirho: 0,
            surface_chirho: "loved".to_string(),
            lemma_chirho: Some("agapao".to_string()),
            strong_chirho: Some("G25".to_string()),
            morph_raw_chirho: Some("V-AAI-3S".to_string()),
            language_chirho: "Greek".to_string(),
        };
        writer_chirho
            .insert_token_chirho(verse_id_chirho, &token_chirho)
            .unwrap();

        let (tense_chirho, voice_chirho, mood_chirho): (String, String, String) = writer_chirho
            .connection_chirho()
            .query_row(
                "SELECT tense_chirho, voice_chirho, mood_chirho FROM tokens_chirho WHERE verse_id_chirho = ?1",
                params![verse_id_chirho],
                |row_chirho| Ok((row_chirho.get(0)?, row_chirho.get(1)?, row_chirho.get(2)?)),
            )
            .unwrap();
        assert_eq!(tense_chirho, "AoristChirho");
        assert_eq!(voice_chirho, "ActiveChirho");
        assert_eq!(mood_chirho, "IndicativeChirho");
    }
}
