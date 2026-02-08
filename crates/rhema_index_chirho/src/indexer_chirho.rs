// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module indexer — builds Tantivy indexes from SWORD module content.

use std::path::{Path, PathBuf};
use std::time::Instant;

use tantivy::{Index, IndexWriter, TantivyDocument};

use rhema_contracts_chirho::morph_parser_chirho::MorphParserChirho;
use rhema_ingest_chirho::sword_adapter_chirho::SwordAdapterChirho;
use rhema_ingest_chirho::token_extractor_chirho::TokenExtractorChirho;

use crate::error_chirho::IndexErrorChirho;
use crate::manifest_chirho::IndexManifestChirho;
use crate::schema_chirho::RhemaSchemaChirho;

/// Default writer heap size: 50 MB.
const WRITER_HEAP_CHIRHO: usize = 50_000_000;

/// All 66 books of the Bible in canonical order (OSIS abbreviations).
const BIBLE_BOOKS_CHIRHO: &[&str] = &[
    // Old Testament (39)
    "Gen", "Exod", "Lev", "Num", "Deut", "Josh", "Judg", "Ruth",
    "1Sam", "2Sam", "1Kgs", "2Kgs", "1Chr", "2Chr", "Ezra", "Neh",
    "Esth", "Job", "Ps", "Prov", "Eccl", "Song", "Isa", "Jer",
    "Lam", "Ezek", "Dan", "Hos", "Joel", "Amos", "Obad", "Jonah",
    "Mic", "Nah", "Hab", "Zeph", "Hag", "Zech", "Mal",
    // New Testament (27)
    "Matt", "Mark", "Luke", "John", "Acts", "Rom", "1Cor", "2Cor",
    "Gal", "Eph", "Phil", "Col", "1Thess", "2Thess", "1Tim", "2Tim",
    "Titus", "Phlm", "Heb", "Jas", "1Pet", "2Pet", "1John", "2John",
    "3John", "Jude", "Rev",
];

/// Progress callback: (books_done, total_books, verses_indexed).
pub type ProgressCallbackChirho = Box<dyn Fn(usize, usize, usize) + Send>;

/// Builds a Tantivy index for a SWORD module.
pub struct ModuleIndexerChirho {
    /// Base directory for storing indexes.
    index_base_chirho: PathBuf,
    /// Rhema schema with structured fields.
    schema_chirho: RhemaSchemaChirho,
}

impl ModuleIndexerChirho {
    /// Create an indexer that stores indexes under the given base directory.
    pub fn new_chirho(index_base_chirho: &Path) -> Self {
        Self {
            index_base_chirho: index_base_chirho.to_path_buf(),
            schema_chirho: RhemaSchemaChirho::build_chirho(),
        }
    }

    /// Path where the index for a given module would be stored.
    pub fn index_path_chirho(&self, module_name_chirho: &str) -> PathBuf {
        self.index_base_chirho.join(module_name_chirho)
    }

    /// Check if an index exists for a module.
    pub fn has_index_chirho(&self, module_name_chirho: &str) -> bool {
        let path_chirho = self.index_path_chirho(module_name_chirho);
        path_chirho.is_dir()
    }

    /// Build an index for the given module using the SWORD adapter.
    ///
    /// Returns the number of documents (verses) indexed.
    pub fn build_index_chirho(
        &self,
        adapter_chirho: &SwordAdapterChirho,
        module_name_chirho: &str,
        progress_chirho: Option<ProgressCallbackChirho>,
    ) -> Result<usize, IndexErrorChirho> {
        let start_chirho = Instant::now();
        let path_chirho = self.index_path_chirho(module_name_chirho);

        // Create index directory.
        std::fs::create_dir_all(&path_chirho)?;

        // Create Tantivy index with our schema.
        let index_chirho =
            Index::create_in_dir(&path_chirho, self.schema_chirho.schema_chirho.clone())?;
        let mut writer_chirho: IndexWriter = index_chirho.writer(WRITER_HEAP_CHIRHO)?;

        let mut extractor_chirho = TokenExtractorChirho::new_chirho();
        let total_books_chirho = BIBLE_BOOKS_CHIRHO.len();
        let mut verse_count_chirho: usize = 0;

        for (book_idx_chirho, &book_chirho) in BIBLE_BOOKS_CHIRHO.iter().enumerate() {
            // Try chapters 1..=150 (Psalms has 150).
            for chapter_chirho in 1..=150_u16 {
                let verses_chirho = match adapter_chirho.read_chapter_chirho(
                    module_name_chirho,
                    book_chirho,
                    chapter_chirho,
                ) {
                    Ok(v_chirho) => v_chirho,
                    Err(_) => break, // No more chapters for this book.
                };

                if verses_chirho.is_empty() {
                    break;
                }

                // Extract Strong's numbers for each verse.
                let tokens_chirho = adapter_chirho
                    .ingest_chapter_chirho(
                        module_name_chirho,
                        book_chirho,
                        chapter_chirho,
                        &mut extractor_chirho,
                    )
                    .ok();

                for (verse_ref_chirho, text_chirho) in &verses_chirho {
                    if text_chirho.is_empty() {
                        continue;
                    }

                    // Collect Strong's numbers for this verse.
                    let strongs_text_chirho = tokens_chirho
                        .as_ref()
                        .map(|tokens_chirho| {
                            tokens_chirho
                                .iter()
                                .filter(|t_chirho| {
                                    t_chirho.verse_ref_chirho == *verse_ref_chirho
                                })
                                .filter_map(|t_chirho| t_chirho.strong_chirho.as_ref())
                                .map(|s_chirho| s_chirho.as_str_chirho())
                                .collect::<Vec<&str>>()
                                .join(" ")
                        })
                        .unwrap_or_default();

                    // Strip HTML/XML tags for plain text indexing.
                    let plain_text_chirho = strip_tags_chirho(&text_chirho);

                    // Build Tantivy document.
                    let key_chirho = format!(
                        "{} {}:{}",
                        verse_ref_chirho.book_chirho,
                        verse_ref_chirho.chapter_chirho,
                        verse_ref_chirho.verse_chirho,
                    );

                    let mut doc_chirho = TantivyDocument::default();
                    doc_chirho.add_text(self.schema_chirho.key_field_chirho, &key_chirho);
                    doc_chirho
                        .add_text(self.schema_chirho.book_field_chirho, &verse_ref_chirho.book_chirho);
                    doc_chirho.add_u64(
                        self.schema_chirho.chapter_field_chirho,
                        u64::from(verse_ref_chirho.chapter_chirho),
                    );
                    doc_chirho.add_u64(
                        self.schema_chirho.verse_field_chirho,
                        u64::from(verse_ref_chirho.verse_chirho),
                    );
                    doc_chirho.add_text(self.schema_chirho.text_field_chirho, &plain_text_chirho);
                    doc_chirho
                        .add_text(self.schema_chirho.module_field_chirho, module_name_chirho);

                    if !strongs_text_chirho.is_empty() {
                        doc_chirho
                            .add_text(self.schema_chirho.strongs_field_chirho, &strongs_text_chirho);
                    }

                    // Phase 3: Add morphology facets from tokens.
                    if let Some(ref verse_tokens_chirho) = tokens_chirho {
                        let mut morph_codes_chirho = Vec::new();
                        let mut lemma_strings_chirho = Vec::new();

                        for tok_chirho in verse_tokens_chirho
                            .iter()
                            .filter(|t_chirho| t_chirho.verse_ref_chirho == *verse_ref_chirho)
                        {
                            // Collect lemma
                            if let Some(ref lemma_chirho) = tok_chirho.lemma_chirho {
                                lemma_strings_chirho.push(lemma_chirho.as_str_chirho().to_string());
                            }

                            // Parse morph code and add facets
                            if let Some(ref morph_chirho) = tok_chirho.morph_chirho {
                                let code_str_chirho = morph_chirho.as_str_chirho();
                                morph_codes_chirho.push(code_str_chirho.to_string());

                                if let Ok(parsed_chirho) = MorphParserChirho::parse_chirho(code_str_chirho) {
                                    doc_chirho.add_text(
                                        self.schema_chirho.pos_field_chirho,
                                        &format!("{:?}", parsed_chirho.part_of_speech_chirho),
                                    );
                                    if let Some(ref t_chirho) = parsed_chirho.tense_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.tense_field_chirho,
                                            &format!("{t_chirho:?}"),
                                        );
                                    }
                                    if let Some(ref v_chirho) = parsed_chirho.voice_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.voice_field_chirho,
                                            &format!("{v_chirho:?}"),
                                        );
                                    }
                                    if let Some(ref m_chirho) = parsed_chirho.mood_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.mood_field_chirho,
                                            &format!("{m_chirho:?}"),
                                        );
                                    }
                                    if let Some(ref c_chirho) = parsed_chirho.case_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.case_field_chirho,
                                            &format!("{c_chirho:?}"),
                                        );
                                    }
                                    if let Some(ref n_chirho) = parsed_chirho.number_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.number_field_chirho,
                                            &format!("{n_chirho:?}"),
                                        );
                                    }
                                    if let Some(ref g_chirho) = parsed_chirho.gender_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.gender_field_chirho,
                                            &format!("{g_chirho:?}"),
                                        );
                                    }
                                    if let Some(ref p_chirho) = parsed_chirho.person_chirho {
                                        doc_chirho.add_text(
                                            self.schema_chirho.person_field_chirho,
                                            &format!("{p_chirho:?}"),
                                        );
                                    }
                                }
                            }
                        }

                        if !morph_codes_chirho.is_empty() {
                            doc_chirho.add_text(
                                self.schema_chirho.morph_field_chirho,
                                &morph_codes_chirho.join(" "),
                            );
                        }
                        if !lemma_strings_chirho.is_empty() {
                            doc_chirho.add_text(
                                self.schema_chirho.lemma_field_chirho,
                                &lemma_strings_chirho.join(" "),
                            );
                        }
                    }

                    writer_chirho.add_document(doc_chirho)?;
                    verse_count_chirho += 1;
                }
            }

            // Report progress.
            if let Some(ref cb_chirho) = progress_chirho {
                cb_chirho(book_idx_chirho + 1, total_books_chirho, verse_count_chirho);
            }
        }

        // Commit the index.
        writer_chirho.commit()?;

        let elapsed_chirho = start_chirho.elapsed();
        log::info!(
            "Indexed {} verses for module '{}' in {:.2}s",
            verse_count_chirho,
            module_name_chirho,
            elapsed_chirho.as_secs_f64(),
        );

        // Write manifest.
        let manifest_chirho = IndexManifestChirho::new_chirho(
            module_name_chirho,
            verse_count_chirho,
        );
        manifest_chirho.write_chirho(&path_chirho)?;

        Ok(verse_count_chirho)
    }

    /// Delete an existing index for a module.
    pub fn delete_index_chirho(
        &self,
        module_name_chirho: &str,
    ) -> Result<(), IndexErrorChirho> {
        let path_chirho = self.index_path_chirho(module_name_chirho);
        if path_chirho.is_dir() {
            std::fs::remove_dir_all(&path_chirho)?;
            log::info!("Deleted index for module '{}'", module_name_chirho);
        }
        Ok(())
    }

    /// Get the schema used by this indexer.
    pub fn schema_chirho(&self) -> &RhemaSchemaChirho {
        &self.schema_chirho
    }
}

/// Strip HTML/XML tags for plain-text indexing.
fn strip_tags_chirho(text_chirho: &str) -> String {
    let mut result_chirho = String::with_capacity(text_chirho.len());
    let mut in_tag_chirho = false;
    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '<' => in_tag_chirho = true,
            '>' => in_tag_chirho = false,
            _ if !in_tag_chirho => result_chirho.push(ch_chirho),
            _ => {}
        }
    }
    result_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_strip_tags_chirho() {
        assert_eq!(
            strip_tags_chirho("<w lemma=\"strong:H430\">God</w> created"),
            "God created"
        );
        assert_eq!(strip_tags_chirho("plain text"), "plain text");
        assert_eq!(strip_tags_chirho("<p>Hello</p>"), "Hello");
    }

    #[test]
    fn test_index_path_chirho() {
        let indexer_chirho = ModuleIndexerChirho::new_chirho(Path::new("/tmp/indexes"));
        assert_eq!(
            indexer_chirho.index_path_chirho("KJV"),
            PathBuf::from("/tmp/indexes/KJV")
        );
    }
}
