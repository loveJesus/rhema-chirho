// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Semantic domain importer — bridges the semantic-chirho dataset into
//! rhema_chirho's `DomainStoreChirho`.
//!
//! Reads from a semantic-chirho SQLite database and populates a
//! `DomainStoreChirho` with sense→domain mappings and verse→sense mappings.

use rusqlite::{Connection, OpenFlags};

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

use crate::domain_store_chirho::DomainStoreChirho;
use crate::error_chirho::ModuleErrorChirho;

/// Result of an import operation.
#[derive(Debug, Clone)]
pub struct ImportResultChirho {
    /// Number of senses imported.
    pub senses_imported_chirho: usize,
    /// Number of verse-to-sense mappings created.
    pub verse_mappings_chirho: usize,
    /// Number of senses skipped (no domain assignment).
    pub senses_skipped_chirho: usize,
}

/// Imports semantic domain data from a semantic-chirho SQLite database.
pub struct SemanticImporterChirho {
    conn_chirho: Connection,
}

impl SemanticImporterChirho {
    /// Open a semantic-chirho database for reading.
    pub fn open_chirho(path_chirho: &str) -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open_with_flags(
            path_chirho,
            OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?;
        Ok(Self { conn_chirho })
    }

    /// Import all senses and verse mappings into the given `DomainStoreChirho`.
    ///
    /// For each sense in the source DB:
    /// 1. Finds the best-fit domain assignment (preferring Louw-Nida hierarchy)
    /// 2. Inserts the sense with a cleaned domain name and the sense label as gloss
    /// 3. Maps all word occurrences of that sense to their verses
    pub fn import_to_store_chirho(
        &self,
        store_chirho: &DomainStoreChirho,
    ) -> Result<ImportResultChirho, ModuleErrorChirho> {
        let mut result_chirho = ImportResultChirho {
            senses_imported_chirho: 0,
            verse_mappings_chirho: 0,
            senses_skipped_chirho: 0,
        };

        // Step 1: Build domain_id → clean name lookup
        let domain_names_chirho = self.load_domain_names_chirho()?;

        // Step 2: Load senses with their best-fit domain
        let senses_chirho = self.load_senses_with_domains_chirho(&domain_names_chirho)?;

        // Step 3: Insert senses in a single transaction (~20K rows)
        store_chirho.begin_transaction_chirho()?;
        for sense_chirho in &senses_chirho {
            store_chirho.insert_sense_chirho(
                &sense_chirho.sense_key_chirho,
                &sense_chirho.domain_name_chirho,
                &sense_chirho.gloss_chirho,
            )?;
            result_chirho.senses_imported_chirho += 1;
        }
        store_chirho.commit_transaction_chirho()?;

        // Step 4: Load verse mappings in batched transactions (~448K rows)
        let verse_count_chirho = self.map_verses_chirho(store_chirho, &senses_chirho)?;
        result_chirho.verse_mappings_chirho = verse_count_chirho;

        result_chirho.senses_skipped_chirho =
            self.total_sense_count_chirho()?.saturating_sub(senses_chirho.len());

        Ok(result_chirho)
    }

    /// Load domain ID → clean name mapping from the source database.
    fn load_domain_names_chirho(
        &self,
    ) -> Result<std::collections::HashMap<i64, (String, String)>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT domain_id_chirho, name_chirho, hierarchy_chirho FROM domain_chirho",
        )?;
        let rows_chirho = stmt_chirho.query_map([], |row_chirho| {
            let id_chirho: i64 = row_chirho.get(0)?;
            let name_chirho: String = row_chirho.get(1)?;
            let hierarchy_chirho: String = row_chirho.get(2)?;
            Ok((id_chirho, name_chirho, hierarchy_chirho))
        })?;

        let mut map_chirho = std::collections::HashMap::new();
        for row_chirho in rows_chirho {
            let (id_chirho, name_chirho, hierarchy_chirho) = row_chirho?;
            let clean_name_chirho = clean_domain_name_chirho(&name_chirho);
            map_chirho.insert(id_chirho, (clean_name_chirho, hierarchy_chirho));
        }
        Ok(map_chirho)
    }

    /// Load senses with their best-fit domain assignment.
    fn load_senses_with_domains_chirho(
        &self,
        domain_names_chirho: &std::collections::HashMap<i64, (String, String)>,
    ) -> Result<Vec<SenseEntryChirho>, ModuleErrorChirho> {
        // Get all senses
        let mut sense_stmt_chirho = self.conn_chirho.prepare(
            "SELECT sense_id_chirho, lemma_id_chirho, label_chirho FROM sense_chirho",
        )?;
        let sense_rows_chirho = sense_stmt_chirho.query_map([], |row_chirho| {
            let sense_id_chirho: i64 = row_chirho.get(0)?;
            let lemma_id_chirho: String = row_chirho.get(1)?;
            let label_chirho: Option<String> = row_chirho.get(2)?;
            Ok((sense_id_chirho, lemma_id_chirho, label_chirho))
        })?;

        // Get all domain assignments, grouped by sense
        let mut assign_stmt_chirho = self.conn_chirho.prepare(
            "SELECT sense_id_chirho, domain_id_chirho, fit_score_chirho \
             FROM domain_assignment_chirho \
             ORDER BY sense_id_chirho, fit_score_chirho DESC",
        )?;
        let assign_rows_chirho = assign_stmt_chirho.query_map([], |row_chirho| {
            let sense_id_chirho: i64 = row_chirho.get(0)?;
            let domain_id_chirho: i64 = row_chirho.get(1)?;
            let fit_score_chirho: f64 = row_chirho.get(2)?;
            Ok((sense_id_chirho, domain_id_chirho, fit_score_chirho))
        })?;

        // Build sense → best domain mapping (prefer louw_nida, then highest fit_score)
        let mut best_domain_chirho: std::collections::HashMap<i64, i64> =
            std::collections::HashMap::new();
        let mut best_score_chirho: std::collections::HashMap<i64, (f64, bool)> =
            std::collections::HashMap::new();

        for row_chirho in assign_rows_chirho {
            let (sense_id_chirho, domain_id_chirho, fit_score_chirho) = row_chirho?;
            let is_ln_chirho = domain_names_chirho
                .get(&domain_id_chirho)
                .is_some_and(|(_, h_chirho)| h_chirho == "louw_nida");

            let should_replace_chirho =
                if let Some((prev_score_chirho, prev_is_ln_chirho)) =
                    best_score_chirho.get(&sense_id_chirho)
                {
                    // Prefer louw_nida over non-louw_nida, then higher score
                    (is_ln_chirho && !prev_is_ln_chirho)
                        || (is_ln_chirho == *prev_is_ln_chirho
                            && fit_score_chirho > *prev_score_chirho)
                } else {
                    true
                };

            if should_replace_chirho {
                best_domain_chirho.insert(sense_id_chirho, domain_id_chirho);
                best_score_chirho.insert(sense_id_chirho, (fit_score_chirho, is_ln_chirho));
            }
        }

        // Build final sense entries
        let mut senses_chirho = Vec::new();
        for row_chirho in sense_rows_chirho {
            let (sense_id_chirho, lemma_id_chirho, label_chirho) = row_chirho?;

            if let Some(domain_id_chirho) = best_domain_chirho.get(&sense_id_chirho)
                && let Some((domain_name_chirho, _)) = domain_names_chirho.get(domain_id_chirho)
            {
                let sense_key_chirho =
                    format!("{}:{}", lemma_id_chirho, sense_id_chirho);
                let gloss_chirho =
                    label_chirho.unwrap_or_default();

                senses_chirho.push(SenseEntryChirho {
                    sense_id_chirho,
                    sense_key_chirho,
                    domain_name_chirho: domain_name_chirho.clone(),
                    gloss_chirho,
                });
            }
        }

        Ok(senses_chirho)
    }

    /// Map word occurrences to their verses in the domain store.
    ///
    /// Uses batched transactions (every 50 000 rows) for performance with ~448K rows.
    fn map_verses_chirho(
        &self,
        store_chirho: &DomainStoreChirho,
        senses_chirho: &[SenseEntryChirho],
    ) -> Result<usize, ModuleErrorChirho> {
        const BATCH_SIZE_CHIRHO: usize = 50_000;

        // Build sense_id → sense_key lookup
        let sense_key_map_chirho: std::collections::HashMap<i64, &str> = senses_chirho
            .iter()
            .map(|s_chirho| (s_chirho.sense_id_chirho, s_chirho.sense_key_chirho.as_str()))
            .collect();

        // Query: for each verse, collect the distinct senses present
        // We GROUP BY to avoid mapping the same sense to the same verse multiple times
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT DISTINCT sa.sense_id_chirho, w.book_chirho, w.chapter_chirho, w.verse_chirho \
             FROM sense_assignment_chirho sa \
             JOIN word_chirho w ON sa.occurrence_id_chirho = w.id_chirho",
        )?;

        let rows_chirho = stmt_chirho.query_map([], |row_chirho| {
            let sense_id_chirho: i64 = row_chirho.get(0)?;
            let book_num_chirho: i64 = row_chirho.get(1)?;
            let chapter_chirho: i64 = row_chirho.get(2)?;
            let verse_chirho: i64 = row_chirho.get(3)?;
            Ok((sense_id_chirho, book_num_chirho, chapter_chirho, verse_chirho))
        })?;

        let mut count_chirho = 0usize;
        let mut batch_count_chirho = 0usize;
        let mut seen_chirho: std::collections::HashSet<(i64, i64, i64, i64)> =
            std::collections::HashSet::new();

        store_chirho.begin_transaction_chirho()?;

        for row_chirho in rows_chirho {
            let (sense_id_chirho, book_num_chirho, chapter_chirho, verse_chirho) = row_chirho?;

            // Deduplicate (same sense + same verse)
            let key_chirho = (sense_id_chirho, book_num_chirho, chapter_chirho, verse_chirho);
            if !seen_chirho.insert(key_chirho) {
                continue;
            }

            if let Some(sense_key_chirho) = sense_key_map_chirho.get(&sense_id_chirho)
                && let Some(book_name_chirho) = book_number_to_name_chirho(book_num_chirho)
            {
                let verse_ref_chirho = VerseRefChirho {
                    book_chirho: book_name_chirho.to_string(),
                    chapter_chirho: chapter_chirho as u16,
                    verse_chirho: verse_chirho as u16,
                };
                store_chirho
                    .map_verse_to_sense_chirho(sense_key_chirho, &verse_ref_chirho)?;
                count_chirho += 1;
                batch_count_chirho += 1;

                if batch_count_chirho >= BATCH_SIZE_CHIRHO {
                    store_chirho.commit_transaction_chirho()?;
                    store_chirho.begin_transaction_chirho()?;
                    batch_count_chirho = 0;
                }
            }
        }

        store_chirho.commit_transaction_chirho()?;

        Ok(count_chirho)
    }

    /// Get total sense count from the source database.
    fn total_sense_count_chirho(&self) -> Result<usize, ModuleErrorChirho> {
        let count_chirho: i64 = self
            .conn_chirho
            .query_row("SELECT COUNT(*) FROM sense_chirho", [], |r_chirho| {
                r_chirho.get(0)
            })?;
        Ok(count_chirho as usize)
    }
}

/// Internal sense entry for import.
struct SenseEntryChirho {
    sense_id_chirho: i64,
    sense_key_chirho: String,
    domain_name_chirho: String,
    gloss_chirho: String,
}

/// Clean a semantic-chirho domain name into a user-friendly search term.
///
/// - "LN28_Know" → "know"
/// - "LN01_Geographical_Objects_and_Features" → "geographical objects and features"
/// - "MORAL" → "moral"
/// - "MORAL_35" → "moral"
fn clean_domain_name_chirho(raw_chirho: &str) -> String {
    let stripped_chirho = if raw_chirho.starts_with("LN") {
        // Strip "LN##_" prefix
        if let Some(idx_chirho) = raw_chirho.find('_') {
            &raw_chirho[idx_chirho + 1..]
        } else {
            raw_chirho
        }
    } else {
        // Strip trailing "_##" from community domains like "MORAL_35"
        if let Some(idx_chirho) = raw_chirho.rfind('_') {
            let suffix_chirho = &raw_chirho[idx_chirho + 1..];
            if suffix_chirho.chars().all(|c_chirho| c_chirho.is_ascii_digit()) {
                &raw_chirho[..idx_chirho]
            } else {
                raw_chirho
            }
        } else {
            raw_chirho
        }
    };

    stripped_chirho
        .replace('_', " ")
        .to_lowercase()
}

/// Map a 1-based book number (1..66) to a SWORD-style book name.
fn book_number_to_name_chirho(num_chirho: i64) -> Option<&'static str> {
    BOOK_NAMES_CHIRHO.get((num_chirho - 1) as usize).copied()
}

/// Standard 66-book canon in SWORD name order.
const BOOK_NAMES_CHIRHO: &[&str] = &[
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Deuteronomy",
    "Joshua",
    "Judges",
    "Ruth",
    "I Samuel",
    "II Samuel",
    "I Kings",
    "II Kings",
    "I Chronicles",
    "II Chronicles",
    "Ezra",
    "Nehemiah",
    "Esther",
    "Job",
    "Psalms",
    "Proverbs",
    "Ecclesiastes",
    "Song of Solomon",
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Ezekiel",
    "Daniel",
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
    "Matthew",
    "Mark",
    "Luke",
    "John",
    "Acts",
    "Romans",
    "I Corinthians",
    "II Corinthians",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "I Thessalonians",
    "II Thessalonians",
    "I Timothy",
    "II Timothy",
    "Titus",
    "Philemon",
    "Hebrews",
    "James",
    "I Peter",
    "II Peter",
    "I John",
    "II John",
    "III John",
    "Jude",
    "Revelation",
];

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_clean_domain_name_ln_chirho() {
        assert_eq!(clean_domain_name_chirho("LN28_Know"), "know");
        assert_eq!(
            clean_domain_name_chirho("LN01_Geographical_Objects_and_Features"),
            "geographical objects and features"
        );
    }

    #[test]
    fn test_clean_domain_name_macro_chirho() {
        assert_eq!(clean_domain_name_chirho("MORAL"), "moral");
        assert_eq!(clean_domain_name_chirho("PERSON"), "person");
    }

    #[test]
    fn test_clean_domain_name_community_chirho() {
        assert_eq!(clean_domain_name_chirho("MORAL_35"), "moral");
        assert_eq!(clean_domain_name_chirho("PERSON_12"), "person");
    }

    #[test]
    fn test_book_number_to_name_chirho() {
        assert_eq!(book_number_to_name_chirho(1), Some("Genesis"));
        assert_eq!(book_number_to_name_chirho(43), Some("John"));
        assert_eq!(book_number_to_name_chirho(66), Some("Revelation"));
        assert_eq!(book_number_to_name_chirho(0), None);
        assert_eq!(book_number_to_name_chirho(67), None);
    }

    #[test]
    fn test_book_names_count_chirho() {
        assert_eq!(BOOK_NAMES_CHIRHO.len(), 66);
    }
}
