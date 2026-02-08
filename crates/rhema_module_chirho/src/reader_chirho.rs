// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module reader — queries a `.rhema` SQLite module for verses and tokens.

use std::path::Path;

use rusqlite::{params, Connection};

use crate::error_chirho::ModuleErrorChirho;

/// A verse record read from a module.
#[derive(Debug, Clone)]
pub struct VerseRowChirho {
    pub id_chirho: i64,
    pub book_chirho: String,
    pub chapter_chirho: u32,
    pub verse_chirho: u32,
    pub text_chirho: String,
    pub module_chirho: String,
}

/// A token record read from a module.
#[derive(Debug, Clone)]
pub struct TokenRowChirho {
    pub id_chirho: i64,
    pub verse_id_chirho: i64,
    pub word_index_chirho: u32,
    pub surface_chirho: String,
    pub lemma_chirho: Option<String>,
    pub strong_chirho: Option<String>,
    pub morph_raw_chirho: Option<String>,
    pub pos_chirho: Option<String>,
    pub tense_chirho: Option<String>,
    pub voice_chirho: Option<String>,
    pub mood_chirho: Option<String>,
    pub case_chirho: Option<String>,
    pub number_chirho: Option<String>,
    pub gender_chirho: Option<String>,
    pub person_chirho: Option<String>,
    pub hebrew_stem_chirho: Option<String>,
    pub hebrew_state_chirho: Option<String>,
    pub language_chirho: String,
}

/// Reads data from a `.rhema` SQLite module.
pub struct ModuleReaderChirho {
    conn_chirho: Connection,
}

impl ModuleReaderChirho {
    /// Open an existing module file.
    pub fn open_chirho(path_chirho: &Path) -> Result<Self, ModuleErrorChirho> {
        if !path_chirho.exists() {
            return Err(ModuleErrorChirho::NotFoundChirho {
                path_chirho: path_chirho.display().to_string(),
            });
        }
        let conn_chirho = Connection::open(path_chirho)?;
        Ok(Self { conn_chirho })
    }

    /// Open from an existing connection (for testing).
    pub fn from_connection_chirho(conn_chirho: Connection) -> Self {
        Self { conn_chirho }
    }

    /// Get a metadata value by key.
    pub fn get_meta_chirho(&self, key_chirho: &str) -> Result<String, ModuleErrorChirho> {
        self.conn_chirho
            .query_row(
                "SELECT value_chirho FROM module_meta_chirho WHERE key_chirho = ?1",
                params![key_chirho],
                |row_chirho| row_chirho.get(0),
            )
            .map_err(|_| ModuleErrorChirho::MissingMetaChirho {
                key_chirho: key_chirho.to_string(),
            })
    }

    /// Read a single verse by reference.
    pub fn read_verse_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u32,
        verse_chirho: u32,
    ) -> Result<Option<VerseRowChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho \
             FROM verses_chirho WHERE book_chirho = ?1 AND chapter_chirho = ?2 AND verse_chirho = ?3"
        )?;

        let result_chirho = stmt_chirho.query_row(
            params![book_chirho, chapter_chirho, verse_chirho],
            |row_chirho| {
                Ok(VerseRowChirho {
                    id_chirho: row_chirho.get(0)?,
                    book_chirho: row_chirho.get(1)?,
                    chapter_chirho: row_chirho.get(2)?,
                    verse_chirho: row_chirho.get(3)?,
                    text_chirho: row_chirho.get(4)?,
                    module_chirho: row_chirho.get(5)?,
                })
            },
        );

        match result_chirho {
            Ok(v_chirho) => Ok(Some(v_chirho)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e_chirho) => Err(ModuleErrorChirho::SqliteChirho(e_chirho)),
        }
    }

    /// Read all verses in a chapter.
    pub fn read_chapter_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u32,
    ) -> Result<Vec<VerseRowChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho \
             FROM verses_chirho WHERE book_chirho = ?1 AND chapter_chirho = ?2 \
             ORDER BY verse_chirho"
        )?;

        let rows_chirho = stmt_chirho
            .query_map(params![book_chirho, chapter_chirho], |row_chirho| {
                Ok(VerseRowChirho {
                    id_chirho: row_chirho.get(0)?,
                    book_chirho: row_chirho.get(1)?,
                    chapter_chirho: row_chirho.get(2)?,
                    verse_chirho: row_chirho.get(3)?,
                    text_chirho: row_chirho.get(4)?,
                    module_chirho: row_chirho.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows_chirho)
    }

    /// Read tokens for a given verse ID.
    pub fn read_tokens_chirho(
        &self,
        verse_id_chirho: i64,
    ) -> Result<Vec<TokenRowChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, verse_id_chirho, word_index_chirho, surface_chirho, \
             lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, tense_chirho, \
             voice_chirho, mood_chirho, case_chirho, number_chirho, gender_chirho, \
             person_chirho, hebrew_stem_chirho, hebrew_state_chirho, language_chirho \
             FROM tokens_chirho WHERE verse_id_chirho = ?1 ORDER BY word_index_chirho"
        )?;

        let rows_chirho = stmt_chirho
            .query_map(params![verse_id_chirho], |row_chirho| {
                Ok(TokenRowChirho {
                    id_chirho: row_chirho.get(0)?,
                    verse_id_chirho: row_chirho.get(1)?,
                    word_index_chirho: row_chirho.get(2)?,
                    surface_chirho: row_chirho.get(3)?,
                    lemma_chirho: row_chirho.get(4)?,
                    strong_chirho: row_chirho.get(5)?,
                    morph_raw_chirho: row_chirho.get(6)?,
                    pos_chirho: row_chirho.get(7)?,
                    tense_chirho: row_chirho.get(8)?,
                    voice_chirho: row_chirho.get(9)?,
                    mood_chirho: row_chirho.get(10)?,
                    case_chirho: row_chirho.get(11)?,
                    number_chirho: row_chirho.get(12)?,
                    gender_chirho: row_chirho.get(13)?,
                    person_chirho: row_chirho.get(14)?,
                    hebrew_stem_chirho: row_chirho.get(15)?,
                    hebrew_state_chirho: row_chirho.get(16)?,
                    language_chirho: row_chirho.get(17)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows_chirho)
    }

    /// Count total verses in the module.
    pub fn count_verses_chirho(&self) -> Result<u64, ModuleErrorChirho> {
        let count_chirho: i64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM verses_chirho",
            [],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho as u64)
    }

    /// Count total tokens in the module.
    pub fn count_tokens_chirho(&self) -> Result<u64, ModuleErrorChirho> {
        let count_chirho: i64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM tokens_chirho",
            [],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho as u64)
    }

    /// Get the underlying connection (for testing / morph_query).
    pub fn connection_chirho(&self) -> &Connection {
        &self.conn_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::writer_chirho::{ModuleWriterChirho, TokenEntryChirho, VerseEntryChirho};

    fn setup_test_db_chirho() -> Connection {
        let mut writer_chirho = ModuleWriterChirho::create_in_memory_chirho().unwrap();
        writer_chirho.set_meta_chirho("module_name", "TestMod").unwrap();

        let entries_chirho = vec![
            (
                VerseEntryChirho {
                    book_chirho: "John".to_string(),
                    chapter_chirho: 3,
                    verse_chirho: 16,
                    text_chirho: "For God so loved the world".to_string(),
                    module_chirho: "KJV".to_string(),
                },
                vec![
                    TokenEntryChirho {
                        word_index_chirho: 0,
                        surface_chirho: "loved".to_string(),
                        lemma_chirho: Some("agapao".to_string()),
                        strong_chirho: Some("G25".to_string()),
                        morph_raw_chirho: Some("V-AAI-3S".to_string()),
                        language_chirho: "Greek".to_string(),
                    },
                    TokenEntryChirho {
                        word_index_chirho: 1,
                        surface_chirho: "world".to_string(),
                        lemma_chirho: Some("kosmos".to_string()),
                        strong_chirho: Some("G2889".to_string()),
                        morph_raw_chirho: Some("N-ASM".to_string()),
                        language_chirho: "Greek".to_string(),
                    },
                ],
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

        writer_chirho.batch_insert_chirho(&entries_chirho).unwrap();

        // Transfer connection — writer drops, but we keep the in-memory db by
        // returning the same connection wrapped in a reader.
        // We actually need a fresh connection from the writer's path.
        // For in-memory, just test via the writer's connection.
        // So let's use a different approach: use a temp file.
        // For unit testing, we'll use the from_connection approach.
        // Unfortunately, rusqlite::Connection doesn't impl Clone.
        // We'll rely on the file-based test for round-trip.
        // For these tests, we'll re-open via tempfile.
        drop(writer_chirho);

        // Create a fresh DB in memory for reader tests.
        let conn_chirho = Connection::open_in_memory().unwrap();
        crate::schema_chirho::apply_schema_chirho(&conn_chirho).unwrap();
        conn_chirho.execute(
            "INSERT INTO module_meta_chirho (key_chirho, value_chirho) VALUES ('module_name', 'TestMod')",
            [],
        ).unwrap();
        conn_chirho.execute(
            "INSERT INTO verses_chirho (book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho) \
             VALUES ('John', 3, 16, 'For God so loved the world', 'KJV')",
            [],
        ).unwrap();
        conn_chirho.execute(
            "INSERT INTO verses_chirho (book_chirho, chapter_chirho, verse_chirho, text_chirho, module_chirho) \
             VALUES ('John', 3, 17, 'For God sent not his Son', 'KJV')",
            [],
        ).unwrap();
        conn_chirho.execute(
            "INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
             lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, tense_chirho, voice_chirho, \
             mood_chirho, language_chirho) \
             VALUES (1, 0, 'loved', 'agapao', 'G25', 'V-AAI-3S', 'VerbChirho', 'AoristChirho', \
             'ActiveChirho', 'IndicativeChirho', 'Greek')",
            [],
        ).unwrap();
        conn_chirho.execute(
            "INSERT INTO tokens_chirho (verse_id_chirho, word_index_chirho, surface_chirho, \
             lemma_chirho, strong_chirho, morph_raw_chirho, pos_chirho, case_chirho, number_chirho, \
             gender_chirho, language_chirho) \
             VALUES (1, 1, 'world', 'kosmos', 'G2889', 'N-ASM', 'NounChirho', 'AccusativeChirho', \
             'SingularChirho', 'MasculineChirho', 'Greek')",
            [],
        ).unwrap();

        conn_chirho
    }

    #[test]
    fn test_get_meta_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        let name_chirho = reader_chirho.get_meta_chirho("module_name").unwrap();
        assert_eq!(name_chirho, "TestMod");
    }

    #[test]
    fn test_get_meta_missing_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        assert!(reader_chirho.get_meta_chirho("nonexistent").is_err());
    }

    #[test]
    fn test_read_verse_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        let verse_chirho = reader_chirho.read_verse_chirho("John", 3, 16).unwrap().unwrap();
        assert_eq!(verse_chirho.text_chirho, "For God so loved the world");
    }

    #[test]
    fn test_read_verse_not_found_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        let verse_chirho = reader_chirho.read_verse_chirho("Genesis", 1, 1).unwrap();
        assert!(verse_chirho.is_none());
    }

    #[test]
    fn test_read_chapter_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        let verses_chirho = reader_chirho.read_chapter_chirho("John", 3).unwrap();
        assert_eq!(verses_chirho.len(), 2);
        assert_eq!(verses_chirho[0].verse_chirho, 16);
        assert_eq!(verses_chirho[1].verse_chirho, 17);
    }

    #[test]
    fn test_read_tokens_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        let tokens_chirho = reader_chirho.read_tokens_chirho(1).unwrap();
        assert_eq!(tokens_chirho.len(), 2);
        assert_eq!(tokens_chirho[0].surface_chirho, "loved");
        assert_eq!(tokens_chirho[0].pos_chirho.as_deref(), Some("VerbChirho"));
        assert_eq!(tokens_chirho[1].surface_chirho, "world");
        assert_eq!(tokens_chirho[1].pos_chirho.as_deref(), Some("NounChirho"));
    }

    #[test]
    fn test_count_verses_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        assert_eq!(reader_chirho.count_verses_chirho().unwrap(), 2);
    }

    #[test]
    fn test_count_tokens_chirho() {
        let conn_chirho = setup_test_db_chirho();
        let reader_chirho = ModuleReaderChirho::from_connection_chirho(conn_chirho);
        assert_eq!(reader_chirho.count_tokens_chirho().unwrap(), 2);
    }

    #[test]
    fn test_file_round_trip_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let path_chirho = dir_chirho.path().join("test.rhema");

        // Write
        {
            let writer_chirho = ModuleWriterChirho::create_chirho(&path_chirho).unwrap();
            writer_chirho.set_meta_chirho("module_name", "RoundTrip").unwrap();
            let verse_id_chirho = writer_chirho
                .insert_verse_chirho(&VerseEntryChirho {
                    book_chirho: "Gen".to_string(),
                    chapter_chirho: 1,
                    verse_chirho: 1,
                    text_chirho: "In the beginning".to_string(),
                    module_chirho: "KJV".to_string(),
                })
                .unwrap();
            writer_chirho
                .insert_token_chirho(
                    verse_id_chirho,
                    &TokenEntryChirho {
                        word_index_chirho: 0,
                        surface_chirho: "beginning".to_string(),
                        lemma_chirho: Some("reshith".to_string()),
                        strong_chirho: Some("H7225".to_string()),
                        morph_raw_chirho: Some("HNcfsa".to_string()),
                        language_chirho: "Hebrew".to_string(),
                    },
                )
                .unwrap();
        }

        // Read
        let reader_chirho = ModuleReaderChirho::open_chirho(&path_chirho).unwrap();
        assert_eq!(reader_chirho.get_meta_chirho("module_name").unwrap(), "RoundTrip");
        let verse_chirho = reader_chirho.read_verse_chirho("Gen", 1, 1).unwrap().unwrap();
        assert_eq!(verse_chirho.text_chirho, "In the beginning");

        let tokens_chirho = reader_chirho.read_tokens_chirho(verse_chirho.id_chirho).unwrap();
        assert_eq!(tokens_chirho.len(), 1);
        assert_eq!(tokens_chirho[0].pos_chirho.as_deref(), Some("NounChirho"));
    }
}
