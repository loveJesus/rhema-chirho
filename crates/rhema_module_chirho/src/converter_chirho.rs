// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD to .rhema converter — converts existing SWORD modules into the
//! self-contained SQLite `.rhema` module format.

use std::path::Path;

use crate::error_chirho::ModuleErrorChirho;
use crate::writer_chirho::{ModuleWriterChirho, TokenEntryChirho, VerseEntryChirho};

/// All 66 canonical books (OSIS abbreviations).
const BIBLE_BOOKS_CHIRHO: &[&str] = &[
    "Gen", "Exod", "Lev", "Num", "Deut", "Josh", "Judg", "Ruth",
    "1Sam", "2Sam", "1Kgs", "2Kgs", "1Chr", "2Chr", "Ezra", "Neh",
    "Esth", "Job", "Ps", "Prov", "Eccl", "Song", "Isa", "Jer",
    "Lam", "Ezek", "Dan", "Hos", "Joel", "Amos", "Obad", "Jonah",
    "Mic", "Nah", "Hab", "Zeph", "Hag", "Zech", "Mal",
    "Matt", "Mark", "Luke", "John", "Acts", "Rom", "1Cor", "2Cor",
    "Gal", "Eph", "Phil", "Col", "1Thess", "2Thess", "1Tim", "2Tim",
    "Titus", "Phlm", "Heb", "Jas", "1Pet", "2Pet", "1John", "2John",
    "3John", "Jude", "Rev",
];

/// Converts data into `.rhema` module files.
///
/// This converter works with pre-extracted verse and token data rather
/// than directly reading SWORD binary files (to avoid a hard dependency
/// on the full rsword_chirho or rhema_ingest_chirho crates).
pub struct SwordToRhemaConverterChirho;

impl SwordToRhemaConverterChirho {
    /// Convert pre-extracted verses and tokens into a `.rhema` module file.
    ///
    /// `module_name_chirho` — name like "KJV", "SBLGNT", etc.
    /// `output_path_chirho` — where to write the `.rhema` file.
    /// `verses_chirho` — list of (verse_entry, tokens) tuples.
    pub fn convert_chirho(
        module_name_chirho: &str,
        output_path_chirho: &Path,
        verses_chirho: &[(VerseEntryChirho, Vec<TokenEntryChirho>)],
    ) -> Result<ConversionResultChirho, ModuleErrorChirho> {
        let mut writer_chirho = ModuleWriterChirho::create_chirho(output_path_chirho)?;

        writer_chirho.set_meta_chirho("module_name", module_name_chirho)?;
        writer_chirho.set_meta_chirho("format", "rhema_v1")?;

        let verse_count_chirho = writer_chirho.batch_insert_chirho(verses_chirho)?;

        let token_count_chirho: usize = verses_chirho
            .iter()
            .map(|(_, tokens_chirho)| tokens_chirho.len())
            .sum();

        Ok(ConversionResultChirho {
            module_name_chirho: module_name_chirho.to_string(),
            verse_count_chirho,
            token_count_chirho,
            output_path_chirho: output_path_chirho.display().to_string(),
        })
    }

    /// Get the list of canonical Bible books used for conversion.
    pub fn canonical_books_chirho() -> &'static [&'static str] {
        BIBLE_BOOKS_CHIRHO
    }
}

/// Result of a module conversion.
#[derive(Debug, Clone)]
pub struct ConversionResultChirho {
    pub module_name_chirho: String,
    pub verse_count_chirho: usize,
    pub token_count_chirho: usize,
    pub output_path_chirho: String,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_convert_empty_module_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let path_chirho = dir_chirho.path().join("empty.rhema");

        let result_chirho =
            SwordToRhemaConverterChirho::convert_chirho("Empty", &path_chirho, &[]).unwrap();
        assert_eq!(result_chirho.verse_count_chirho, 0);
        assert_eq!(result_chirho.token_count_chirho, 0);
        assert!(path_chirho.exists());
    }

    #[test]
    fn test_convert_with_data_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let path_chirho = dir_chirho.path().join("test.rhema");

        let verses_chirho = vec![
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

        let result_chirho =
            SwordToRhemaConverterChirho::convert_chirho("KJV", &path_chirho, &verses_chirho)
                .unwrap();
        assert_eq!(result_chirho.verse_count_chirho, 2);
        assert_eq!(result_chirho.token_count_chirho, 1);
        assert_eq!(result_chirho.module_name_chirho, "KJV");

        // Verify metadata in the file.
        let reader_chirho = crate::reader_chirho::ModuleReaderChirho::open_chirho(&path_chirho).unwrap();
        assert_eq!(reader_chirho.get_meta_chirho("module_name").unwrap(), "KJV");
        assert_eq!(reader_chirho.get_meta_chirho("format").unwrap(), "rhema_v1");
    }

    #[test]
    fn test_convert_verify_morph_parsing_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let path_chirho = dir_chirho.path().join("morph.rhema");

        let verses_chirho = vec![(
            VerseEntryChirho {
                book_chirho: "Rom".to_string(),
                chapter_chirho: 8,
                verse_chirho: 28,
                text_chirho: "we know".to_string(),
                module_chirho: "SBLGNT".to_string(),
            },
            vec![TokenEntryChirho {
                word_index_chirho: 0,
                surface_chirho: "oidamen".to_string(),
                lemma_chirho: Some("oida".to_string()),
                strong_chirho: Some("G1492".to_string()),
                morph_raw_chirho: Some("V-XAI-1P".to_string()),
                language_chirho: "Greek".to_string(),
            }],
        )];

        SwordToRhemaConverterChirho::convert_chirho("SBLGNT", &path_chirho, &verses_chirho)
            .unwrap();

        let reader_chirho = crate::reader_chirho::ModuleReaderChirho::open_chirho(&path_chirho).unwrap();
        let verse_chirho = reader_chirho.read_verse_chirho("Rom", 8, 28).unwrap().unwrap();
        let tokens_chirho = reader_chirho.read_tokens_chirho(verse_chirho.id_chirho).unwrap();
        assert_eq!(tokens_chirho.len(), 1);
        assert_eq!(tokens_chirho[0].pos_chirho.as_deref(), Some("VerbChirho"));
        assert_eq!(tokens_chirho[0].tense_chirho.as_deref(), Some("PerfectChirho"));
    }

    #[test]
    fn test_canonical_books_chirho() {
        let books_chirho = SwordToRhemaConverterChirho::canonical_books_chirho();
        assert_eq!(books_chirho.len(), 66);
        assert_eq!(books_chirho[0], "Gen");
        assert_eq!(books_chirho[65], "Rev");
    }
}
