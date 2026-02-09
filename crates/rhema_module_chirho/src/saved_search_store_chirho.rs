// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SQLite-backed saved search store.
//!
//! Follows the same pattern as `XrefStoreChirho`:
//! - `open_chirho(path)` / `in_memory_chirho()` constructors
//! - Automatic schema migration on open

use rusqlite::{params, Connection};
use uuid::Uuid;

use rhema_contracts_chirho::saved_search_chirho::SavedSearchChirho;

use crate::error_chirho::ModuleErrorChirho;

/// SQL migration for the saved search schema.
const SAVED_SEARCH_MIGRATION_SQL_CHIRHO: &str =
    include_str!("../migrations_chirho/003_saved_search_schema_chirho.sql");

/// SQLite saved search store.
pub struct SavedSearchStoreChirho {
    conn_chirho: Connection,
}

impl SavedSearchStoreChirho {
    /// Open a saved search store from a file path.
    pub fn open_chirho(path_chirho: &str) -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open(path_chirho)?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Create an in-memory saved search store (for testing).
    pub fn in_memory_chirho() -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open_in_memory()?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Create from an existing connection (for shared databases).
    pub fn from_connection_chirho(conn_chirho: Connection) -> Result<Self, ModuleErrorChirho> {
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Apply the saved search schema migration.
    fn run_migrations_chirho(&self) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho
            .execute_batch("PRAGMA journal_mode=WAL;")?;
        self.conn_chirho
            .execute_batch(SAVED_SEARCH_MIGRATION_SQL_CHIRHO)?;
        Ok(())
    }

    /// Save a new search. Generates a UUID if `id_chirho` is empty.
    pub fn save_chirho(
        &self,
        name_chirho: &str,
        query_text_chirho: &str,
        tags_chirho: &str,
    ) -> Result<SavedSearchChirho, ModuleErrorChirho> {
        let id_chirho = Uuid::new_v4().to_string();
        self.conn_chirho.execute(
            "INSERT INTO saved_searches_chirho \
             (id_chirho, name_chirho, query_text_chirho, tags_chirho) \
             VALUES (?1, ?2, ?3, ?4)",
            params![id_chirho, name_chirho, query_text_chirho, tags_chirho],
        )?;
        self.load_chirho(&id_chirho)
    }

    /// Load a saved search by ID.
    pub fn load_chirho(&self, id_chirho: &str) -> Result<SavedSearchChirho, ModuleErrorChirho> {
        let row_chirho = self.conn_chirho.query_row(
            "SELECT id_chirho, name_chirho, query_text_chirho, \
                    created_at_chirho, last_used_at_chirho, usage_count_chirho, tags_chirho \
             FROM saved_searches_chirho WHERE id_chirho = ?1",
            params![id_chirho],
            row_to_saved_search_chirho,
        )?;
        Ok(row_chirho)
    }

    /// Load a saved search by name.
    pub fn load_by_name_chirho(
        &self,
        name_chirho: &str,
    ) -> Result<SavedSearchChirho, ModuleErrorChirho> {
        let row_chirho = self.conn_chirho.query_row(
            "SELECT id_chirho, name_chirho, query_text_chirho, \
                    created_at_chirho, last_used_at_chirho, usage_count_chirho, tags_chirho \
             FROM saved_searches_chirho WHERE name_chirho = ?1",
            params![name_chirho],
            row_to_saved_search_chirho,
        )?;
        Ok(row_chirho)
    }

    /// List all saved searches, ordered by last use.
    pub fn list_chirho(&self) -> Result<Vec<SavedSearchChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, name_chirho, query_text_chirho, \
                    created_at_chirho, last_used_at_chirho, usage_count_chirho, tags_chirho \
             FROM saved_searches_chirho ORDER BY last_used_at_chirho DESC",
        )?;
        let rows_chirho = stmt_chirho.query_map([], row_to_saved_search_chirho)?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    /// Search saved searches by tag substring.
    pub fn search_by_tag_chirho(
        &self,
        tag_chirho: &str,
    ) -> Result<Vec<SavedSearchChirho>, ModuleErrorChirho> {
        let pattern_chirho = format!("%{tag_chirho}%");
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, name_chirho, query_text_chirho, \
                    created_at_chirho, last_used_at_chirho, usage_count_chirho, tags_chirho \
             FROM saved_searches_chirho WHERE tags_chirho LIKE ?1",
        )?;
        let rows_chirho = stmt_chirho.query_map(params![pattern_chirho], row_to_saved_search_chirho)?;
        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    /// Delete a saved search by ID.
    pub fn delete_chirho(&self, id_chirho: &str) -> Result<bool, ModuleErrorChirho> {
        let affected_chirho = self.conn_chirho.execute(
            "DELETE FROM saved_searches_chirho WHERE id_chirho = ?1",
            params![id_chirho],
        )?;
        Ok(affected_chirho > 0)
    }

    /// Increment the usage count and update last_used_at for a saved search.
    pub fn increment_usage_chirho(&self, id_chirho: &str) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute(
            "UPDATE saved_searches_chirho SET \
             usage_count_chirho = usage_count_chirho + 1, \
             last_used_at_chirho = datetime('now') \
             WHERE id_chirho = ?1",
            params![id_chirho],
        )?;
        Ok(())
    }

    /// Count total saved searches.
    pub fn count_chirho(&self) -> Result<u64, ModuleErrorChirho> {
        let count_chirho: i64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM saved_searches_chirho",
            [],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho as u64)
    }
}

/// Convert a rusqlite row to a SavedSearchChirho.
fn row_to_saved_search_chirho(
    row_chirho: &rusqlite::Row<'_>,
) -> rusqlite::Result<SavedSearchChirho> {
    Ok(SavedSearchChirho {
        id_chirho: row_chirho.get(0)?,
        name_chirho: row_chirho.get(1)?,
        query_text_chirho: row_chirho.get(2)?,
        created_at_chirho: row_chirho.get(3)?,
        last_used_at_chirho: row_chirho.get(4)?,
        usage_count_chirho: row_chirho.get(5)?,
        tags_chirho: row_chirho.get(6)?,
    })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_save_and_load_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        let saved_chirho = store_chirho
            .save_chirho("Love search", "love AND world", "love,johannine")
            .unwrap();
        assert_eq!(saved_chirho.name_chirho, "Love search");
        assert_eq!(saved_chirho.query_text_chirho, "love AND world");

        let loaded_chirho = store_chirho.load_chirho(&saved_chirho.id_chirho).unwrap();
        assert_eq!(loaded_chirho.name_chirho, "Love search");
    }

    #[test]
    fn test_load_by_name_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .save_chirho("Grace search", "grace mercy", "pauline")
            .unwrap();
        let loaded_chirho = store_chirho.load_by_name_chirho("Grace search").unwrap();
        assert_eq!(loaded_chirho.query_text_chirho, "grace mercy");
    }

    #[test]
    fn test_list_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        store_chirho.save_chirho("S1", "love", "").unwrap();
        store_chirho.save_chirho("S2", "grace", "").unwrap();
        store_chirho.save_chirho("S3", "faith", "").unwrap();

        let all_chirho = store_chirho.list_chirho().unwrap();
        assert_eq!(all_chirho.len(), 3);
    }

    #[test]
    fn test_search_by_tag_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .save_chirho("S1", "love", "johannine,love")
            .unwrap();
        store_chirho
            .save_chirho("S2", "grace", "pauline,grace")
            .unwrap();
        store_chirho
            .save_chirho("S3", "faith", "pauline,faith")
            .unwrap();

        let pauline_chirho = store_chirho.search_by_tag_chirho("pauline").unwrap();
        assert_eq!(pauline_chirho.len(), 2);
    }

    #[test]
    fn test_delete_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        let saved_chirho = store_chirho.save_chirho("Delete me", "love", "").unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 1);

        let deleted_chirho = store_chirho.delete_chirho(&saved_chirho.id_chirho).unwrap();
        assert!(deleted_chirho);
        assert_eq!(store_chirho.count_chirho().unwrap(), 0);
    }

    #[test]
    fn test_increment_usage_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        let saved_chirho = store_chirho.save_chirho("Usage test", "love", "").unwrap();
        assert_eq!(saved_chirho.usage_count_chirho, 0);

        store_chirho
            .increment_usage_chirho(&saved_chirho.id_chirho)
            .unwrap();
        store_chirho
            .increment_usage_chirho(&saved_chirho.id_chirho)
            .unwrap();

        let loaded_chirho = store_chirho.load_chirho(&saved_chirho.id_chirho).unwrap();
        assert_eq!(loaded_chirho.usage_count_chirho, 2);
    }

    #[test]
    fn test_count_chirho() {
        let store_chirho = SavedSearchStoreChirho::in_memory_chirho().unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 0);
        store_chirho.save_chirho("S1", "love", "").unwrap();
        store_chirho.save_chirho("S2", "grace", "").unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 2);
    }
}
