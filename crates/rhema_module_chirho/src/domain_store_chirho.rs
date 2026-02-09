// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SQLite-backed semantic domain store.

use rusqlite::{params, Connection};

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

use crate::error_chirho::ModuleErrorChirho;

/// SQL migration for the domain schema.
const DOMAIN_MIGRATION_SQL_CHIRHO: &str =
    include_str!("../migrations_chirho/004_domain_schema_chirho.sql");

/// SQLite semantic domain store.
pub struct DomainStoreChirho {
    conn_chirho: Connection,
}

impl DomainStoreChirho {
    /// Open a domain store from a file path.
    pub fn open_chirho(path_chirho: &str) -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open(path_chirho)?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Create an in-memory domain store (for testing).
    pub fn in_memory_chirho() -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open_in_memory()?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    fn run_migrations_chirho(&self) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho
            .execute_batch("PRAGMA journal_mode=WAL;")?;
        self.conn_chirho
            .execute_batch(DOMAIN_MIGRATION_SQL_CHIRHO)?;
        Ok(())
    }

    /// Insert a sense into the store.
    pub fn insert_sense_chirho(
        &self,
        sense_id_chirho: &str,
        domain_chirho: &str,
        gloss_chirho: &str,
    ) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute(
            "INSERT OR IGNORE INTO senses_chirho (sense_id_chirho, domain_chirho, gloss_chirho) \
             VALUES (?1, ?2, ?3)",
            params![sense_id_chirho, domain_chirho, gloss_chirho],
        )?;
        Ok(())
    }

    /// Map a verse to a sense.
    pub fn map_verse_to_sense_chirho(
        &self,
        sense_id_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute(
            "INSERT INTO sense_verses_chirho \
             (sense_id_chirho, book_chirho, chapter_chirho, verse_chirho) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                sense_id_chirho,
                verse_ref_chirho.book_chirho,
                verse_ref_chirho.chapter_chirho,
                verse_ref_chirho.verse_chirho
            ],
        )?;
        Ok(())
    }

    /// Begin a transaction for bulk operations.
    pub fn begin_transaction_chirho(&self) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute_batch("BEGIN TRANSACTION")?;
        Ok(())
    }

    /// Commit the current transaction.
    pub fn commit_transaction_chirho(&self) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute_batch("COMMIT")?;
        Ok(())
    }

    /// Get all verse references for a given semantic domain.
    pub fn verses_by_domain_chirho(
        &self,
        domain_chirho: &str,
    ) -> Result<Vec<VerseRefChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT sv.book_chirho, sv.chapter_chirho, sv.verse_chirho \
             FROM sense_verses_chirho sv \
             JOIN senses_chirho s ON sv.sense_id_chirho = s.sense_id_chirho \
             WHERE s.domain_chirho = ?1",
        )?;
        let rows_chirho = stmt_chirho.query_map(params![domain_chirho], |row_chirho| {
            Ok(VerseRefChirho {
                book_chirho: row_chirho.get(0)?,
                chapter_chirho: row_chirho.get(1)?,
                verse_chirho: row_chirho.get(2)?,
            })
        })?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    /// Get all verse references for a given sense ID.
    pub fn verses_by_sense_chirho(
        &self,
        sense_id_chirho: &str,
    ) -> Result<Vec<VerseRefChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT book_chirho, chapter_chirho, verse_chirho \
             FROM sense_verses_chirho WHERE sense_id_chirho = ?1",
        )?;
        let rows_chirho = stmt_chirho.query_map(params![sense_id_chirho], |row_chirho| {
            Ok(VerseRefChirho {
                book_chirho: row_chirho.get(0)?,
                chapter_chirho: row_chirho.get(1)?,
                verse_chirho: row_chirho.get(2)?,
            })
        })?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    /// Get all domains associated with a verse.
    pub fn domains_for_verse_chirho(
        &self,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<Vec<String>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT DISTINCT s.domain_chirho \
             FROM senses_chirho s \
             JOIN sense_verses_chirho sv ON s.sense_id_chirho = sv.sense_id_chirho \
             WHERE sv.book_chirho = ?1 AND sv.chapter_chirho = ?2 AND sv.verse_chirho = ?3",
        )?;
        let rows_chirho = stmt_chirho.query_map(
            params![
                verse_ref_chirho.book_chirho,
                verse_ref_chirho.chapter_chirho,
                verse_ref_chirho.verse_chirho
            ],
            |row_chirho| row_chirho.get(0),
        )?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn ref_chirho(book_chirho: &str, ch_chirho: u16, v_chirho: u16) -> VerseRefChirho {
        VerseRefChirho {
            book_chirho: book_chirho.to_string(),
            chapter_chirho: ch_chirho,
            verse_chirho: v_chirho,
        }
    }

    #[test]
    fn test_insert_sense_and_map_chirho() {
        let store_chirho = DomainStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_sense_chirho("love.01", "love", "divine love")
            .unwrap();
        store_chirho
            .map_verse_to_sense_chirho("love.01", &ref_chirho("John", 3, 16))
            .unwrap();

        let verses_chirho = store_chirho.verses_by_sense_chirho("love.01").unwrap();
        assert_eq!(verses_chirho.len(), 1);
        assert_eq!(verses_chirho[0].book_chirho, "John");
    }

    #[test]
    fn test_verses_by_domain_chirho() {
        let store_chirho = DomainStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_sense_chirho("love.01", "love", "divine love")
            .unwrap();
        store_chirho
            .insert_sense_chirho("love.02", "love", "brotherly love")
            .unwrap();
        store_chirho
            .map_verse_to_sense_chirho("love.01", &ref_chirho("John", 3, 16))
            .unwrap();
        store_chirho
            .map_verse_to_sense_chirho("love.02", &ref_chirho("I John", 4, 7))
            .unwrap();

        let verses_chirho = store_chirho.verses_by_domain_chirho("love").unwrap();
        assert_eq!(verses_chirho.len(), 2);
    }

    #[test]
    fn test_domains_for_verse_chirho() {
        let store_chirho = DomainStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_sense_chirho("love.01", "love", "")
            .unwrap();
        store_chirho
            .insert_sense_chirho("faith.01", "faith", "")
            .unwrap();
        let verse_chirho = ref_chirho("John", 3, 16);
        store_chirho
            .map_verse_to_sense_chirho("love.01", &verse_chirho)
            .unwrap();
        store_chirho
            .map_verse_to_sense_chirho("faith.01", &verse_chirho)
            .unwrap();

        let domains_chirho = store_chirho.domains_for_verse_chirho(&verse_chirho).unwrap();
        assert_eq!(domains_chirho.len(), 2);
        assert!(domains_chirho.contains(&"love".to_string()));
        assert!(domains_chirho.contains(&"faith".to_string()));
    }

    #[test]
    fn test_empty_domain_lookup_chirho() {
        let store_chirho = DomainStoreChirho::in_memory_chirho().unwrap();
        let verses_chirho = store_chirho.verses_by_domain_chirho("nonexistent").unwrap();
        assert!(verses_chirho.is_empty());
    }
}
