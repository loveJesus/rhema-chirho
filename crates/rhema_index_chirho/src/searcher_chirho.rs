// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Index searcher — queries a Tantivy index with the Rhema schema.

use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::{
    BooleanQuery, FuzzyTermQuery, Occur, PhraseQuery, QueryParser, RegexQuery,
    TermQuery,
};
use tantivy::schema::Value;
use tantivy::{Index, Term};

use crate::error_chirho::IndexErrorChirho;
use crate::schema_chirho::RhemaSchemaChirho;

/// A single search hit from the index.
#[derive(Debug, Clone)]
pub struct IndexHitChirho {
    /// Verse reference key — e.g., "John 3:16".
    pub key_chirho: String,
    /// Book name — e.g., "John".
    pub book_chirho: String,
    /// Chapter number.
    pub chapter_chirho: u64,
    /// Verse number.
    pub verse_chirho: u64,
    /// Full text of the verse.
    pub text_chirho: String,
    /// Module name.
    pub module_chirho: String,
    /// Tantivy relevance score.
    pub score_chirho: f32,
}

/// Searches a Tantivy index created by [`ModuleIndexerChirho`].
pub struct IndexSearcherChirho {
    /// The Tantivy index handle.
    index_chirho: Index,
    /// Schema with field handles.
    schema_chirho: RhemaSchemaChirho,
}

impl IndexSearcherChirho {
    /// Open an existing index from disk.
    pub fn open_chirho(
        index_dir_chirho: &Path,
    ) -> Result<Self, IndexErrorChirho> {
        let schema_chirho = RhemaSchemaChirho::build_chirho();
        let index_chirho = Index::open_in_dir(index_dir_chirho)?;
        Ok(Self {
            index_chirho,
            schema_chirho,
        })
    }

    /// Full-text search using Tantivy's query parser.
    /// Supports boolean operators (AND, OR, NOT), phrases, and wildcards.
    pub fn search_text_chirho(
        &self,
        query_text_chirho: &str,
        max_results_chirho: usize,
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let reader_chirho = self.index_chirho.reader()?;
        let searcher_chirho = reader_chirho.searcher();

        let query_parser_chirho = QueryParser::for_index(
            &self.index_chirho,
            vec![self.schema_chirho.text_field_chirho],
        );
        let query_chirho = query_parser_chirho.parse_query(query_text_chirho)?;

        let top_docs_chirho =
            searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))?;

        self.collect_hits_chirho(&searcher_chirho, &top_docs_chirho)
    }

    /// Search for a specific Strong's number (e.g., "H430" or "G26").
    pub fn search_strongs_chirho(
        &self,
        strong_number_chirho: &str,
        max_results_chirho: usize,
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let reader_chirho = self.index_chirho.reader()?;
        let searcher_chirho = reader_chirho.searcher();

        let term_chirho = Term::from_field_text(
            self.schema_chirho.strongs_field_chirho,
            &strong_number_chirho.to_lowercase(),
        );
        let query_chirho = TermQuery::new(
            term_chirho,
            tantivy::schema::IndexRecordOption::Basic,
        );

        let top_docs_chirho =
            searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))?;

        self.collect_hits_chirho(&searcher_chirho, &top_docs_chirho)
    }

    /// Phrase search with optional proximity (slop).
    pub fn search_phrase_chirho(
        &self,
        terms_chirho: &[&str],
        slop_chirho: u32,
        max_results_chirho: usize,
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let reader_chirho = self.index_chirho.reader()?;
        let searcher_chirho = reader_chirho.searcher();

        let tantivy_terms_chirho: Vec<Term> = terms_chirho
            .iter()
            .map(|t_chirho| {
                Term::from_field_text(
                    self.schema_chirho.text_field_chirho,
                    &t_chirho.to_lowercase(),
                )
            })
            .collect();

        let query_chirho = PhraseQuery::new_with_offset_and_slop(
            tantivy_terms_chirho
                .into_iter()
                .enumerate()
                .map(|(i_chirho, t_chirho)| (i_chirho, t_chirho))
                .collect(),
            slop_chirho,
        );

        let top_docs_chirho =
            searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))?;

        self.collect_hits_chirho(&searcher_chirho, &top_docs_chirho)
    }

    /// Fuzzy search for typo-tolerant matching.
    pub fn search_fuzzy_chirho(
        &self,
        term_chirho: &str,
        distance_chirho: u8,
        max_results_chirho: usize,
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let reader_chirho = self.index_chirho.reader()?;
        let searcher_chirho = reader_chirho.searcher();

        let tantivy_term_chirho = Term::from_field_text(
            self.schema_chirho.text_field_chirho,
            &term_chirho.to_lowercase(),
        );
        let query_chirho =
            FuzzyTermQuery::new(tantivy_term_chirho, distance_chirho, true);

        let top_docs_chirho =
            searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))?;

        self.collect_hits_chirho(&searcher_chirho, &top_docs_chirho)
    }

    /// Regex search against verse text.
    pub fn search_regex_chirho(
        &self,
        pattern_chirho: &str,
        max_results_chirho: usize,
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let reader_chirho = self.index_chirho.reader()?;
        let searcher_chirho = reader_chirho.searcher();

        let query_chirho =
            RegexQuery::from_pattern(pattern_chirho, self.schema_chirho.text_field_chirho)?;

        let top_docs_chirho =
            searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))?;

        self.collect_hits_chirho(&searcher_chirho, &top_docs_chirho)
    }

    /// Filter search results to a specific book.
    pub fn search_in_book_chirho(
        &self,
        query_text_chirho: &str,
        book_chirho: &str,
        max_results_chirho: usize,
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let reader_chirho = self.index_chirho.reader()?;
        let searcher_chirho = reader_chirho.searcher();

        // Text query
        let query_parser_chirho = QueryParser::for_index(
            &self.index_chirho,
            vec![self.schema_chirho.text_field_chirho],
        );
        let text_query_chirho = query_parser_chirho.parse_query(query_text_chirho)?;

        // Book filter
        let book_term_chirho =
            Term::from_field_text(self.schema_chirho.book_field_chirho, book_chirho);
        let book_query_chirho = TermQuery::new(
            book_term_chirho,
            tantivy::schema::IndexRecordOption::Basic,
        );

        // Combine: text AND book
        let combined_chirho = BooleanQuery::new(vec![
            (Occur::Must, Box::new(text_query_chirho)),
            (Occur::Must, Box::new(book_query_chirho)),
        ]);

        let top_docs_chirho =
            searcher_chirho.search(&combined_chirho, &TopDocs::with_limit(max_results_chirho))?;

        self.collect_hits_chirho(&searcher_chirho, &top_docs_chirho)
    }

    /// Collect Tantivy search results into `IndexHitChirho` structs.
    fn collect_hits_chirho(
        &self,
        searcher_chirho: &tantivy::Searcher,
        top_docs_chirho: &[(f32, tantivy::DocAddress)],
    ) -> Result<Vec<IndexHitChirho>, IndexErrorChirho> {
        let mut hits_chirho = Vec::with_capacity(top_docs_chirho.len());

        for &(score_chirho, doc_addr_chirho) in top_docs_chirho {
            let doc_chirho: TantivyDocument = searcher_chirho.doc(doc_addr_chirho)?;

            let key_chirho = doc_chirho
                .get_first(self.schema_chirho.key_field_chirho)
                .and_then(|v_chirho| v_chirho.as_str())
                .unwrap_or("")
                .to_string();

            let book_chirho = doc_chirho
                .get_first(self.schema_chirho.book_field_chirho)
                .and_then(|v_chirho| v_chirho.as_str())
                .unwrap_or("")
                .to_string();

            let chapter_chirho = doc_chirho
                .get_first(self.schema_chirho.chapter_field_chirho)
                .and_then(|v_chirho| v_chirho.as_u64())
                .unwrap_or(0);

            let verse_chirho = doc_chirho
                .get_first(self.schema_chirho.verse_field_chirho)
                .and_then(|v_chirho| v_chirho.as_u64())
                .unwrap_or(0);

            let text_chirho = doc_chirho
                .get_first(self.schema_chirho.text_field_chirho)
                .and_then(|v_chirho| v_chirho.as_str())
                .unwrap_or("")
                .to_string();

            let module_chirho = doc_chirho
                .get_first(self.schema_chirho.module_field_chirho)
                .and_then(|v_chirho| v_chirho.as_str())
                .unwrap_or("")
                .to_string();

            hits_chirho.push(IndexHitChirho {
                key_chirho,
                book_chirho,
                chapter_chirho,
                verse_chirho,
                text_chirho,
                module_chirho,
                score_chirho,
            });
        }

        Ok(hits_chirho)
    }
}

use tantivy::TantivyDocument;

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tantivy::{Index, IndexWriter};

    fn build_test_index_chirho(dir_chirho: &std::path::Path) -> IndexSearcherChirho {
        let schema_chirho = RhemaSchemaChirho::build_chirho();
        let index_chirho =
            Index::create_in_dir(dir_chirho, schema_chirho.schema_chirho.clone()).unwrap();
        let mut writer_chirho: IndexWriter = index_chirho.writer(15_000_000).unwrap();

        // Add test verses
        let verses_chirho = vec![
            ("John", 3u64, 16u64, "For God so loved the world that he gave his only begotten Son that whosoever believeth in him should not perish but have everlasting life", "KJV"),
            ("John", 3, 17, "For God sent not his Son into the world to condemn the world but that the world through him might be saved", "KJV"),
            ("Romans", 8, 28, "And we know that all things work together for good to them that love God to them who are the called according to his purpose", "KJV"),
            ("Genesis", 1, 1, "In the beginning God created the heaven and the earth", "KJV"),
            ("Psalms", 23, 1, "The LORD is my shepherd I shall not want", "KJV"),
        ];

        for (book_chirho, ch_chirho, v_chirho, text_chirho, module_chirho) in &verses_chirho {
            let key_chirho = format!("{} {}:{}", book_chirho, ch_chirho, v_chirho);
            let mut doc_chirho = TantivyDocument::default();
            doc_chirho.add_text(schema_chirho.key_field_chirho, &key_chirho);
            doc_chirho.add_text(schema_chirho.book_field_chirho, *book_chirho);
            doc_chirho.add_u64(schema_chirho.chapter_field_chirho, *ch_chirho);
            doc_chirho.add_u64(schema_chirho.verse_field_chirho, *v_chirho);
            doc_chirho.add_text(schema_chirho.text_field_chirho, *text_chirho);
            doc_chirho.add_text(schema_chirho.module_field_chirho, *module_chirho);
            writer_chirho.add_document(doc_chirho).unwrap();
        }
        writer_chirho.commit().unwrap();

        IndexSearcherChirho::open_chirho(dir_chirho).unwrap()
    }

    #[test]
    fn test_schema_builds_chirho() {
        let _schema_chirho = RhemaSchemaChirho::build_chirho();
    }

    #[test]
    fn test_search_text_finds_love_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        let hits_chirho = searcher_chirho.search_text_chirho("love", 10).unwrap();
        assert!(!hits_chirho.is_empty(), "Expected hits for 'love'");
        // Exact word "love" appears in Romans 8:28
        let keys_chirho: Vec<&str> = hits_chirho.iter().map(|h_chirho| h_chirho.key_chirho.as_str()).collect();
        assert!(keys_chirho.iter().any(|k_chirho| k_chirho.contains("Romans 8:28")));

        // "loved" is a different token (no stemming in default tokenizer)
        let loved_hits_chirho = searcher_chirho.search_text_chirho("loved", 10).unwrap();
        assert!(!loved_hits_chirho.is_empty(), "Expected hits for 'loved'");
        let loved_keys_chirho: Vec<&str> = loved_hits_chirho.iter().map(|h_chirho| h_chirho.key_chirho.as_str()).collect();
        assert!(loved_keys_chirho.iter().any(|k_chirho| k_chirho.contains("John 3:16")));
    }

    #[test]
    fn test_search_text_no_results_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        let hits_chirho = searcher_chirho.search_text_chirho("xyzzynonexistent", 10).unwrap();
        assert!(hits_chirho.is_empty());
    }

    #[test]
    fn test_search_phrase_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        let hits_chirho = searcher_chirho.search_phrase_chirho(&["so", "loved"], 0, 10).unwrap();
        assert!(!hits_chirho.is_empty(), "Expected hits for phrase 'so loved'");
    }

    #[test]
    fn test_search_in_book_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        let hits_chirho = searcher_chirho.search_in_book_chirho("God", "John", 10).unwrap();
        for hit_chirho in &hits_chirho {
            assert_eq!(hit_chirho.book_chirho, "John", "All results should be from John");
        }
    }

    #[test]
    fn test_search_fuzzy_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        // "lov" with distance 1 should match "love" and "loved"
        let hits_chirho = searcher_chirho.search_fuzzy_chirho("lov", 1, 10).unwrap();
        assert!(!hits_chirho.is_empty(), "Fuzzy search should find results");
    }

    #[test]
    fn test_search_result_fields_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        let hits_chirho = searcher_chirho.search_text_chirho("beginning", 10).unwrap();
        assert_eq!(hits_chirho.len(), 1);
        let hit_chirho = &hits_chirho[0];
        assert_eq!(hit_chirho.book_chirho, "Genesis");
        assert_eq!(hit_chirho.chapter_chirho, 1);
        assert_eq!(hit_chirho.verse_chirho, 1);
        assert_eq!(hit_chirho.module_chirho, "KJV");
        assert!(hit_chirho.text_chirho.contains("beginning"));
        assert!(hit_chirho.score_chirho > 0.0);
    }

    #[test]
    fn test_search_max_results_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let searcher_chirho = build_test_index_chirho(dir_chirho.path());

        // "God" appears in multiple verses
        let hits_chirho = searcher_chirho.search_text_chirho("God", 2).unwrap();
        assert!(hits_chirho.len() <= 2);
    }
}
