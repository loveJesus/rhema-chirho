// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SQLite-backed cross-reference graph store.
//!
//! Follows the discourse store pattern (`SqliteDiscourseStoreChirho`):
//! - `open_chirho(path)` / `in_memory_chirho()` constructors
//! - Automatic schema migration on open
//! - BFS traversal with depth limit for graph expansion

use std::collections::{HashSet, VecDeque};

use rusqlite::{params, Connection};

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_contracts_chirho::xref_chirho::{
    CrossRefEntryChirho, CrossRefGraphChirho, XRefTypeChirho,
};

use crate::error_chirho::ModuleErrorChirho;

/// SQL migration for the cross-reference schema.
const XREF_MIGRATION_SQL_CHIRHO: &str =
    include_str!("../migrations_chirho/002_xref_schema_chirho.sql");

/// SQLite cross-reference graph store.
pub struct XrefStoreChirho {
    conn_chirho: Connection,
}

impl XrefStoreChirho {
    /// Open a cross-reference store from a file path.
    pub fn open_chirho(path_chirho: &str) -> Result<Self, ModuleErrorChirho> {
        let conn_chirho = Connection::open(path_chirho)?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Create an in-memory cross-reference store (for testing).
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

    /// Apply the cross-reference schema migration.
    fn run_migrations_chirho(&self) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho
            .execute_batch("PRAGMA journal_mode=WAL;")?;
        self.conn_chirho
            .execute_batch("PRAGMA foreign_keys=ON;")?;
        self.conn_chirho
            .execute_batch(XREF_MIGRATION_SQL_CHIRHO)?;
        Ok(())
    }

    /// Get a reference to the underlying connection.
    pub fn connection_chirho(&self) -> &Connection {
        &self.conn_chirho
    }

    /// Query neighbours in both directions for a given verse (used by BFS).
    fn neighbours_chirho(
        &self,
        verse_chirho: &VerseRefChirho,
    ) -> Result<Vec<VerseRefChirho>, ModuleErrorChirho> {
        let mut results_chirho = Vec::new();

        // Outgoing edges: source → target
        let mut stmt_out_chirho = self.conn_chirho.prepare_cached(
            "SELECT target_book_chirho, target_chapter_chirho, target_verse_chirho \
             FROM cross_refs_chirho \
             WHERE source_book_chirho = ?1 AND source_chapter_chirho = ?2 AND source_verse_chirho = ?3",
        )?;

        let out_rows_chirho = stmt_out_chirho.query_map(
            params![
                verse_chirho.book_chirho,
                verse_chirho.chapter_chirho,
                verse_chirho.verse_chirho
            ],
            |row_chirho| {
                Ok(VerseRefChirho {
                    book_chirho: row_chirho.get(0)?,
                    chapter_chirho: row_chirho.get(1)?,
                    verse_chirho: row_chirho.get(2)?,
                })
            },
        )?;

        for row_chirho in out_rows_chirho {
            results_chirho.push(row_chirho?);
        }

        // Incoming edges: target → source (bidirectional traversal)
        let mut stmt_in_chirho = self.conn_chirho.prepare_cached(
            "SELECT source_book_chirho, source_chapter_chirho, source_verse_chirho \
             FROM cross_refs_chirho \
             WHERE target_book_chirho = ?1 AND target_chapter_chirho = ?2 AND target_verse_chirho = ?3",
        )?;

        let in_rows_chirho = stmt_in_chirho.query_map(
            params![
                verse_chirho.book_chirho,
                verse_chirho.chapter_chirho,
                verse_chirho.verse_chirho
            ],
            |row_chirho| {
                Ok(VerseRefChirho {
                    book_chirho: row_chirho.get(0)?,
                    chapter_chirho: row_chirho.get(1)?,
                    verse_chirho: row_chirho.get(2)?,
                })
            },
        )?;

        for row_chirho in in_rows_chirho {
            results_chirho.push(row_chirho?);
        }

        Ok(results_chirho)
    }
}

impl CrossRefGraphChirho for XrefStoreChirho {
    type ErrorChirho = ModuleErrorChirho;

    fn insert_xref_chirho(
        &self,
        entry_chirho: &CrossRefEntryChirho,
    ) -> Result<(), ModuleErrorChirho> {
        self.conn_chirho.execute(
            "INSERT OR IGNORE INTO cross_refs_chirho \
             (source_book_chirho, source_chapter_chirho, source_verse_chirho, \
              target_book_chirho, target_chapter_chirho, target_verse_chirho, \
              xref_type_chirho, confidence_chirho, note_chirho, source_dataset_chirho) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                entry_chirho.source_chirho.book_chirho,
                entry_chirho.source_chirho.chapter_chirho,
                entry_chirho.source_chirho.verse_chirho,
                entry_chirho.target_chirho.book_chirho,
                entry_chirho.target_chirho.chapter_chirho,
                entry_chirho.target_chirho.verse_chirho,
                entry_chirho.xref_type_chirho.to_string(),
                entry_chirho.confidence_chirho,
                entry_chirho.note_chirho,
                entry_chirho.source_dataset_chirho,
            ],
        )?;
        Ok(())
    }

    fn insert_xrefs_batch_chirho(
        &self,
        entries_chirho: &[CrossRefEntryChirho],
    ) -> Result<usize, ModuleErrorChirho> {
        let tx_chirho = self.conn_chirho.unchecked_transaction()?;
        let mut count_chirho = 0usize;

        {
            let mut stmt_chirho = tx_chirho.prepare_cached(
                "INSERT OR IGNORE INTO cross_refs_chirho \
                 (source_book_chirho, source_chapter_chirho, source_verse_chirho, \
                  target_book_chirho, target_chapter_chirho, target_verse_chirho, \
                  xref_type_chirho, confidence_chirho, note_chirho, source_dataset_chirho) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            )?;

            for entry_chirho in entries_chirho {
                let rows_chirho = stmt_chirho.execute(params![
                    entry_chirho.source_chirho.book_chirho,
                    entry_chirho.source_chirho.chapter_chirho,
                    entry_chirho.source_chirho.verse_chirho,
                    entry_chirho.target_chirho.book_chirho,
                    entry_chirho.target_chirho.chapter_chirho,
                    entry_chirho.target_chirho.verse_chirho,
                    entry_chirho.xref_type_chirho.to_string(),
                    entry_chirho.confidence_chirho,
                    entry_chirho.note_chirho,
                    entry_chirho.source_dataset_chirho,
                ])?;
                count_chirho += rows_chirho;
            }
        }

        tx_chirho.commit()?;
        Ok(count_chirho)
    }

    fn xrefs_from_chirho(
        &self,
        source_chirho: &VerseRefChirho,
    ) -> Result<Vec<CrossRefEntryChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare_cached(
            "SELECT source_book_chirho, source_chapter_chirho, source_verse_chirho, \
                    target_book_chirho, target_chapter_chirho, target_verse_chirho, \
                    xref_type_chirho, confidence_chirho, note_chirho, source_dataset_chirho \
             FROM cross_refs_chirho \
             WHERE source_book_chirho = ?1 AND source_chapter_chirho = ?2 AND source_verse_chirho = ?3",
        )?;

        let rows_chirho = stmt_chirho.query_map(
            params![
                source_chirho.book_chirho,
                source_chirho.chapter_chirho,
                source_chirho.verse_chirho
            ],
            row_to_entry_chirho,
        )?;

        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    fn xrefs_to_chirho(
        &self,
        target_chirho: &VerseRefChirho,
    ) -> Result<Vec<CrossRefEntryChirho>, ModuleErrorChirho> {
        let mut stmt_chirho = self.conn_chirho.prepare_cached(
            "SELECT source_book_chirho, source_chapter_chirho, source_verse_chirho, \
                    target_book_chirho, target_chapter_chirho, target_verse_chirho, \
                    xref_type_chirho, confidence_chirho, note_chirho, source_dataset_chirho \
             FROM cross_refs_chirho \
             WHERE target_book_chirho = ?1 AND target_chapter_chirho = ?2 AND target_verse_chirho = ?3",
        )?;

        let rows_chirho = stmt_chirho.query_map(
            params![
                target_chirho.book_chirho,
                target_chirho.chapter_chirho,
                target_chirho.verse_chirho
            ],
            row_to_entry_chirho,
        )?;

        let mut results_chirho = Vec::new();
        for row_chirho in rows_chirho {
            results_chirho.push(row_chirho?);
        }
        Ok(results_chirho)
    }

    fn bfs_expand_chirho(
        &self,
        seed_chirho: &VerseRefChirho,
        depth_chirho: u32,
        include_seed_chirho: bool,
    ) -> Result<Vec<VerseRefChirho>, ModuleErrorChirho> {
        let mut visited_chirho: HashSet<VerseRefChirho> = HashSet::new();
        let mut frontier_chirho: VecDeque<(VerseRefChirho, u32)> = VecDeque::new();
        let mut result_chirho: Vec<VerseRefChirho> = Vec::new();

        visited_chirho.insert(seed_chirho.clone());
        frontier_chirho.push_back((seed_chirho.clone(), 0));

        while let Some((current_chirho, current_depth_chirho)) = frontier_chirho.pop_front() {
            if current_depth_chirho > 0 || include_seed_chirho {
                result_chirho.push(current_chirho.clone());
            }

            if current_depth_chirho >= depth_chirho {
                continue;
            }

            let neighbours_chirho = self.neighbours_chirho(&current_chirho)?;
            for neighbour_chirho in neighbours_chirho {
                if visited_chirho.insert(neighbour_chirho.clone()) {
                    frontier_chirho
                        .push_back((neighbour_chirho, current_depth_chirho + 1));
                }
            }
        }

        Ok(result_chirho)
    }

    fn count_xrefs_chirho(&self) -> Result<u64, ModuleErrorChirho> {
        let count_chirho: i64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM cross_refs_chirho",
            [],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho as u64)
    }
}

/// Convert a rusqlite row into a CrossRefEntryChirho.
fn row_to_entry_chirho(
    row_chirho: &rusqlite::Row<'_>,
) -> rusqlite::Result<CrossRefEntryChirho> {
    let type_str_chirho: String = row_chirho.get(6)?;
    let xref_type_chirho = XRefTypeChirho::from_str_chirho(&type_str_chirho)
        .unwrap_or(XRefTypeChirho::DirectChirho);

    Ok(CrossRefEntryChirho {
        source_chirho: VerseRefChirho {
            book_chirho: row_chirho.get(0)?,
            chapter_chirho: row_chirho.get(1)?,
            verse_chirho: row_chirho.get(2)?,
        },
        target_chirho: VerseRefChirho {
            book_chirho: row_chirho.get(3)?,
            chapter_chirho: row_chirho.get(4)?,
            verse_chirho: row_chirho.get(5)?,
        },
        xref_type_chirho,
        confidence_chirho: row_chirho.get(7)?,
        note_chirho: row_chirho.get(8)?,
        source_dataset_chirho: row_chirho.get(9)?,
    })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn make_ref_chirho(book_chirho: &str, ch_chirho: u16, v_chirho: u16) -> VerseRefChirho {
        VerseRefChirho {
            book_chirho: book_chirho.to_string(),
            chapter_chirho: ch_chirho,
            verse_chirho: v_chirho,
        }
    }

    #[test]
    fn test_insert_single_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            make_ref_chirho("John", 3, 16),
            make_ref_chirho("Romans", 5, 8),
            XRefTypeChirho::DirectChirho,
        );
        store_chirho.insert_xref_chirho(&entry_chirho).unwrap();
        assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 1);
    }

    #[test]
    fn test_insert_batch_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let entries_chirho = vec![
            CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("Romans", 5, 8),
                XRefTypeChirho::DirectChirho,
            ),
            CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("I John", 4, 9),
                XRefTypeChirho::DirectChirho,
            ),
            CrossRefEntryChirho::new_chirho(
                make_ref_chirho("Gen", 1, 1),
                make_ref_chirho("John", 1, 1),
                XRefTypeChirho::AllusionChirho,
            ),
        ];
        let count_chirho = store_chirho
            .insert_xrefs_batch_chirho(&entries_chirho)
            .unwrap();
        assert_eq!(count_chirho, 3);
        assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 3);
    }

    #[test]
    fn test_dedup_on_insert_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            make_ref_chirho("John", 3, 16),
            make_ref_chirho("Romans", 5, 8),
            XRefTypeChirho::DirectChirho,
        );
        store_chirho.insert_xref_chirho(&entry_chirho).unwrap();
        store_chirho.insert_xref_chirho(&entry_chirho).unwrap();
        assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 1);
    }

    #[test]
    fn test_xrefs_from_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let entries_chirho = vec![
            CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("Romans", 5, 8),
                XRefTypeChirho::DirectChirho,
            ),
            CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("I John", 4, 9),
                XRefTypeChirho::DirectChirho,
            ),
            CrossRefEntryChirho::new_chirho(
                make_ref_chirho("Gen", 1, 1),
                make_ref_chirho("John", 1, 1),
                XRefTypeChirho::AllusionChirho,
            ),
        ];
        store_chirho
            .insert_xrefs_batch_chirho(&entries_chirho)
            .unwrap();

        let from_john_chirho = store_chirho
            .xrefs_from_chirho(&make_ref_chirho("John", 3, 16))
            .unwrap();
        assert_eq!(from_john_chirho.len(), 2);
    }

    #[test]
    fn test_xrefs_to_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_xref_chirho(&CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("Romans", 5, 8),
                XRefTypeChirho::DirectChirho,
            ))
            .unwrap();
        store_chirho
            .insert_xref_chirho(&CrossRefEntryChirho::new_chirho(
                make_ref_chirho("Eph", 2, 4),
                make_ref_chirho("Romans", 5, 8),
                XRefTypeChirho::DirectChirho,
            ))
            .unwrap();

        let to_romans_chirho = store_chirho
            .xrefs_to_chirho(&make_ref_chirho("Romans", 5, 8))
            .unwrap();
        assert_eq!(to_romans_chirho.len(), 2);
    }

    #[test]
    fn test_bfs_depth_0_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_xref_chirho(&CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("Romans", 5, 8),
                XRefTypeChirho::DirectChirho,
            ))
            .unwrap();

        // Depth 0, include seed: just the seed
        let result_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("John", 3, 16), 0, true)
            .unwrap();
        assert_eq!(result_chirho.len(), 1);
        assert_eq!(result_chirho[0], make_ref_chirho("John", 3, 16));

        // Depth 0, no seed: empty
        let result2_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("John", 3, 16), 0, false)
            .unwrap();
        assert!(result2_chirho.is_empty());
    }

    #[test]
    fn test_bfs_depth_1_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_xrefs_batch_chirho(&[
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("John", 3, 16),
                    make_ref_chirho("Romans", 5, 8),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("John", 3, 16),
                    make_ref_chirho("I John", 4, 9),
                    XRefTypeChirho::DirectChirho,
                ),
            ])
            .unwrap();

        let result_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("John", 3, 16), 1, false)
            .unwrap();
        assert_eq!(result_chirho.len(), 2);
    }

    #[test]
    fn test_bfs_depth_2_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        // A → B → C
        store_chirho
            .insert_xrefs_batch_chirho(&[
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("Gen", 1, 1),
                    make_ref_chirho("John", 1, 1),
                    XRefTypeChirho::AllusionChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("John", 1, 1),
                    make_ref_chirho("Col", 1, 16),
                    XRefTypeChirho::DirectChirho,
                ),
            ])
            .unwrap();

        let result_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("Gen", 1, 1), 2, false)
            .unwrap();
        assert_eq!(result_chirho.len(), 2);
        assert!(result_chirho.contains(&make_ref_chirho("John", 1, 1)));
        assert!(result_chirho.contains(&make_ref_chirho("Col", 1, 16)));
    }

    #[test]
    fn test_bfs_cycle_handling_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        // A → B, B → A (cycle)
        store_chirho
            .insert_xrefs_batch_chirho(&[
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("John", 3, 16),
                    make_ref_chirho("Romans", 5, 8),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("Romans", 5, 8),
                    make_ref_chirho("John", 3, 16),
                    XRefTypeChirho::DirectChirho,
                ),
            ])
            .unwrap();

        // Should not infinite loop
        let result_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("John", 3, 16), 10, false)
            .unwrap();
        assert_eq!(result_chirho.len(), 1); // Only Romans 5:8
    }

    #[test]
    fn test_bfs_bidirectional_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        // Only one directed edge: A → B
        store_chirho
            .insert_xref_chirho(&CrossRefEntryChirho::new_chirho(
                make_ref_chirho("John", 3, 16),
                make_ref_chirho("Romans", 5, 8),
                XRefTypeChirho::DirectChirho,
            ))
            .unwrap();

        // Starting from B (target), BFS should follow incoming edge back to A
        let result_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("Romans", 5, 8), 1, false)
            .unwrap();
        assert_eq!(result_chirho.len(), 1);
        assert_eq!(result_chirho[0], make_ref_chirho("John", 3, 16));
    }

    #[test]
    fn test_bfs_linear_chain_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        // A → B → C → D
        store_chirho
            .insert_xrefs_batch_chirho(&[
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("Gen", 1, 1),
                    make_ref_chirho("Gen", 1, 2),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("Gen", 1, 2),
                    make_ref_chirho("Gen", 1, 3),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("Gen", 1, 3),
                    make_ref_chirho("Gen", 1, 4),
                    XRefTypeChirho::DirectChirho,
                ),
            ])
            .unwrap();

        // Depth 1 from A: only B
        let d1_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("Gen", 1, 1), 1, false)
            .unwrap();
        assert_eq!(d1_chirho.len(), 1);

        // Depth 2 from A: B + C
        let d2_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("Gen", 1, 1), 2, false)
            .unwrap();
        assert_eq!(d2_chirho.len(), 2);

        // Depth 3 from A: B + C + D
        let d3_chirho = store_chirho
            .bfs_expand_chirho(&make_ref_chirho("Gen", 1, 1), 3, false)
            .unwrap();
        assert_eq!(d3_chirho.len(), 3);
    }

    #[test]
    fn test_bfs_star_topology_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        // Hub → A, Hub → B, Hub → C, Hub → D
        let hub_chirho = make_ref_chirho("John", 3, 16);
        store_chirho
            .insert_xrefs_batch_chirho(&[
                CrossRefEntryChirho::new_chirho(
                    hub_chirho.clone(),
                    make_ref_chirho("Romans", 5, 8),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    hub_chirho.clone(),
                    make_ref_chirho("I John", 4, 9),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    hub_chirho.clone(),
                    make_ref_chirho("Eph", 2, 4),
                    XRefTypeChirho::DirectChirho,
                ),
                CrossRefEntryChirho::new_chirho(
                    hub_chirho.clone(),
                    make_ref_chirho("Gal", 2, 20),
                    XRefTypeChirho::DirectChirho,
                ),
            ])
            .unwrap();

        let result_chirho = store_chirho
            .bfs_expand_chirho(&hub_chirho, 1, false)
            .unwrap();
        assert_eq!(result_chirho.len(), 4);
    }

    #[test]
    fn test_file_round_trip_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let path_chirho = dir_chirho
            .path()
            .join("test_xref_chirho.db")
            .to_string_lossy()
            .to_string();

        // Write
        {
            let store_chirho = XrefStoreChirho::open_chirho(&path_chirho).unwrap();
            store_chirho
                .insert_xref_chirho(&CrossRefEntryChirho::new_chirho(
                    make_ref_chirho("John", 3, 16),
                    make_ref_chirho("Romans", 5, 8),
                    XRefTypeChirho::DirectChirho,
                ))
                .unwrap();
            assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 1);
        }

        // Read back
        {
            let store_chirho = XrefStoreChirho::open_chirho(&path_chirho).unwrap();
            assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 1);
            let refs_chirho = store_chirho
                .xrefs_from_chirho(&make_ref_chirho("John", 3, 16))
                .unwrap();
            assert_eq!(refs_chirho.len(), 1);
            assert_eq!(refs_chirho[0].target_chirho, make_ref_chirho("Romans", 5, 8));
        }
    }

    #[test]
    fn test_xref_type_preserved_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            make_ref_chirho("Matt", 1, 23),
            make_ref_chirho("Isa", 7, 14),
            XRefTypeChirho::ProphecyFulfillmentChirho,
        )
        .with_confidence_chirho(5)
        .with_note_chirho("Virgin birth")
        .with_dataset_chirho("osis");

        store_chirho.insert_xref_chirho(&entry_chirho).unwrap();

        let from_chirho = store_chirho
            .xrefs_from_chirho(&make_ref_chirho("Matt", 1, 23))
            .unwrap();
        assert_eq!(from_chirho.len(), 1);
        assert_eq!(
            from_chirho[0].xref_type_chirho,
            XRefTypeChirho::ProphecyFulfillmentChirho
        );
        assert_eq!(from_chirho[0].confidence_chirho, 5);
        assert_eq!(from_chirho[0].note_chirho.as_deref(), Some("Virgin birth"));
        assert_eq!(from_chirho[0].source_dataset_chirho, "osis");
    }

    #[test]
    fn test_schema_tables_exist_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let tables_chirho: Vec<String> = store_chirho
            .connection_chirho()
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row_chirho| row_chirho.get(0))
            .unwrap()
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();
        assert!(tables_chirho.contains(&"cross_refs_chirho".to_string()));
    }

    #[test]
    fn test_schema_indexes_exist_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let indexes_chirho: Vec<String> = store_chirho
            .connection_chirho()
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE '%xref%' ORDER BY name")
            .unwrap()
            .query_map([], |row_chirho| row_chirho.get(0))
            .unwrap()
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();
        assert!(indexes_chirho.contains(&"idx_xref_source_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_xref_target_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_xref_type_chirho".to_string()));
        assert!(indexes_chirho.contains(&"idx_xref_dataset_chirho".to_string()));
    }

    #[test]
    fn test_empty_store_count_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 0);
    }

    #[test]
    fn test_xrefs_from_empty_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let refs_chirho = store_chirho
            .xrefs_from_chirho(&make_ref_chirho("John", 3, 16))
            .unwrap();
        assert!(refs_chirho.is_empty());
    }

    #[test]
    fn test_different_datasets_not_deduped_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let e1_chirho = CrossRefEntryChirho::new_chirho(
            make_ref_chirho("John", 3, 16),
            make_ref_chirho("Romans", 5, 8),
            XRefTypeChirho::DirectChirho,
        )
        .with_dataset_chirho("osis");
        let e2_chirho = CrossRefEntryChirho::new_chirho(
            make_ref_chirho("John", 3, 16),
            make_ref_chirho("Romans", 5, 8),
            XRefTypeChirho::DirectChirho,
        )
        .with_dataset_chirho("tsk");
        store_chirho
            .insert_xrefs_batch_chirho(&[e1_chirho, e2_chirho])
            .unwrap();
        assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), 2);
    }
}
