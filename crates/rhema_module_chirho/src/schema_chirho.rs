// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Schema migration — creates the SQLite tables for `.rhema` module files.

use rusqlite::Connection;

use crate::error_chirho::ModuleErrorChirho;

/// Current schema version.
pub const SCHEMA_VERSION_CHIRHO: u32 = 1;

/// SQL migration script (embedded at compile time).
const MIGRATION_SQL_CHIRHO: &str =
    include_str!("../migrations_chirho/001_module_schema_chirho.sql");

/// Apply the schema migration to a SQLite connection.
pub fn apply_schema_chirho(conn_chirho: &Connection) -> Result<(), ModuleErrorChirho> {
    conn_chirho.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn_chirho.execute_batch("PRAGMA foreign_keys=ON;")?;
    conn_chirho.execute_batch(MIGRATION_SQL_CHIRHO)?;

    // Store schema version in metadata.
    conn_chirho.execute(
        "INSERT OR REPLACE INTO module_meta_chirho (key_chirho, value_chirho) VALUES (?1, ?2)",
        rusqlite::params!["schema_version", SCHEMA_VERSION_CHIRHO.to_string()],
    )?;

    Ok(())
}

/// Check the schema version stored in the database.
pub fn check_schema_version_chirho(
    conn_chirho: &Connection,
) -> Result<u32, ModuleErrorChirho> {
    let version_str_chirho: String = conn_chirho
        .query_row(
            "SELECT value_chirho FROM module_meta_chirho WHERE key_chirho = 'schema_version'",
            [],
            |row_chirho| row_chirho.get(0),
        )
        .map_err(|_| ModuleErrorChirho::MissingMetaChirho {
            key_chirho: "schema_version".to_string(),
        })?;

    version_str_chirho
        .parse::<u32>()
        .map_err(|_| ModuleErrorChirho::InvalidDataChirho {
            reason_chirho: format!("Invalid schema version: '{version_str_chirho}'"),
        })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_apply_schema_chirho() {
        let conn_chirho = Connection::open_in_memory().unwrap();
        apply_schema_chirho(&conn_chirho).unwrap();

        let version_chirho = check_schema_version_chirho(&conn_chirho).unwrap();
        assert_eq!(version_chirho, SCHEMA_VERSION_CHIRHO);
    }

    #[test]
    fn test_schema_idempotent_chirho() {
        let conn_chirho = Connection::open_in_memory().unwrap();
        apply_schema_chirho(&conn_chirho).unwrap();
        apply_schema_chirho(&conn_chirho).unwrap();

        let version_chirho = check_schema_version_chirho(&conn_chirho).unwrap();
        assert_eq!(version_chirho, SCHEMA_VERSION_CHIRHO);
    }

    #[test]
    fn test_tables_exist_chirho() {
        let conn_chirho = Connection::open_in_memory().unwrap();
        apply_schema_chirho(&conn_chirho).unwrap();

        // Check all tables exist.
        let tables_chirho: Vec<String> = conn_chirho
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row_chirho| row_chirho.get(0))
            .unwrap()
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        assert!(tables_chirho.contains(&"module_meta_chirho".to_string()));
        assert!(tables_chirho.contains(&"verses_chirho".to_string()));
        assert!(tables_chirho.contains(&"tokens_chirho".to_string()));
    }

    #[test]
    fn test_indexes_exist_chirho() {
        let conn_chirho = Connection::open_in_memory().unwrap();
        apply_schema_chirho(&conn_chirho).unwrap();

        let indexes_chirho: Vec<String> = conn_chirho
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE '%chirho%' ORDER BY name")
            .unwrap()
            .query_map([], |row_chirho| row_chirho.get(0))
            .unwrap()
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        assert!(indexes_chirho.contains(&"idx_tokens_verse_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_tokens_pos_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_tokens_lemma_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_tokens_strong_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_tokens_tense_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_verses_book_chirho".to_string()));
    }
}
