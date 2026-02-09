// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SQLite-backed syntax/clause store.

use rusqlite::{params, Connection};

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

use crate::error_chirho::ModuleErrorChirho;

/// SQL migration for the syntax schema.
const SYNTAX_MIGRATION_SQL_CHIRHO: &str =
    include_str!("../migrations_chirho/005_syntax_schema_chirho.sql");

/// A clause record from the syntax store.
#[derive(Debug, Clone)]
pub struct ClauseRowChirho {
    /// Clause ID.
    pub clause_id_chirho: i64,
    /// Parent clause ID (None for root clauses).
    pub parent_id_chirho: Option<i64>,
    /// Clause type (e.g., "independent", "relative", "conditional").
    pub clause_type_chirho: String,
    /// Verse reference where this clause appears.
    pub verse_ref_chirho: VerseRefChirho,
}

/// SQLite syntax/clause store.
pub struct SyntaxStoreChirho {
    conn_chirho: Connection,
}

impl SyntaxStoreChirho {
    /// Open a syntax store from a file path.
    pub fn open_chirho(path_chirho: &str) -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open(path_chirho)?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Create an in-memory syntax store (for testing).
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
            .execute_batch(SYNTAX_MIGRATION_SQL_CHIRHO)?;
        Ok(())
    }

    /// Insert a clause into the store.
    pub fn insert_clause_chirho(
        &self,
        parent_id_chirho: Option<i64>,
        clause_type_chirho: &str,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<i64, ModuleErrorChirho> {
        self.conn_chirho.execute(
            "INSERT INTO clauses_chirho \
             (parent_id_chirho, clause_type_chirho, book_chirho, chapter_chirho, verse_chirho) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                parent_id_chirho,
                clause_type_chirho,
                verse_ref_chirho.book_chirho,
                verse_ref_chirho.chapter_chirho,
                verse_ref_chirho.verse_chirho
            ],
        )?;
        Ok(self.conn_chirho.last_insert_rowid())
    }

    /// Get all clauses of a specific type.
    pub fn clauses_by_type_chirho(
        &self,
        clause_type_chirho: &str,
    ) -> Result<Vec<ClauseRowChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT clause_id_chirho, parent_id_chirho, clause_type_chirho, \
                    book_chirho, chapter_chirho, verse_chirho \
             FROM clauses_chirho WHERE clause_type_chirho = ?1",
        )?;
        let rows_chirho = stmt_chirho.query_map(params![clause_type_chirho], row_to_clause_chirho)?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    /// Get all clauses for a specific verse.
    pub fn clauses_for_verse_chirho(
        &self,
        verse_ref_chirho: &VerseRefChirho,
    ) -> Result<Vec<ClauseRowChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT clause_id_chirho, parent_id_chirho, clause_type_chirho, \
                    book_chirho, chapter_chirho, verse_chirho \
             FROM clauses_chirho \
             WHERE book_chirho = ?1 AND chapter_chirho = ?2 AND verse_chirho = ?3",
        )?;
        let rows_chirho = stmt_chirho.query_map(
            params![
                verse_ref_chirho.book_chirho,
                verse_ref_chirho.chapter_chirho,
                verse_ref_chirho.verse_chirho
            ],
            row_to_clause_chirho,
        )?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }
}

/// Convert a rusqlite row to ClauseRowChirho.
fn row_to_clause_chirho(row_chirho: &rusqlite::Row<'_>) -> rusqlite::Result<ClauseRowChirho> {
    Ok(ClauseRowChirho {
        clause_id_chirho: row_chirho.get(0)?,
        parent_id_chirho: row_chirho.get(1)?,
        clause_type_chirho: row_chirho.get(2)?,
        verse_ref_chirho: VerseRefChirho {
            book_chirho: row_chirho.get(3)?,
            chapter_chirho: row_chirho.get(4)?,
            verse_chirho: row_chirho.get(5)?,
        },
    })
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
    fn test_insert_clause_chirho() {
        let store_chirho = SyntaxStoreChirho::in_memory_chirho().unwrap();
        let id_chirho = store_chirho
            .insert_clause_chirho(None, "independent", &ref_chirho("John", 3, 16))
            .unwrap();
        assert!(id_chirho > 0);
    }

    #[test]
    fn test_clauses_by_type_chirho() {
        let store_chirho = SyntaxStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_clause_chirho(None, "independent", &ref_chirho("John", 3, 16))
            .unwrap();
        store_chirho
            .insert_clause_chirho(None, "relative", &ref_chirho("John", 3, 16))
            .unwrap();
        store_chirho
            .insert_clause_chirho(None, "independent", &ref_chirho("Romans", 8, 28))
            .unwrap();

        let ind_chirho = store_chirho
            .clauses_by_type_chirho("independent")
            .unwrap();
        assert_eq!(ind_chirho.len(), 2);

        let rel_chirho = store_chirho
            .clauses_by_type_chirho("relative")
            .unwrap();
        assert_eq!(rel_chirho.len(), 1);
    }

    #[test]
    fn test_clauses_for_verse_chirho() {
        let store_chirho = SyntaxStoreChirho::in_memory_chirho().unwrap();
        let verse_chirho = ref_chirho("John", 3, 16);
        store_chirho
            .insert_clause_chirho(None, "independent", &verse_chirho)
            .unwrap();
        store_chirho
            .insert_clause_chirho(None, "relative", &verse_chirho)
            .unwrap();

        let clauses_chirho = store_chirho
            .clauses_for_verse_chirho(&verse_chirho)
            .unwrap();
        assert_eq!(clauses_chirho.len(), 2);
    }
}
