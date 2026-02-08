// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SQLite implementation of the discourse store.

use rusqlite::{params, Connection};

use crate::error_chirho::{DiscourseErrorChirho, DiscourseResultChirho};
use crate::storage_chirho::DiscourseStoreChirho;
use crate::types_chirho::*;
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

/// SQLite-backed discourse store.
pub struct SqliteDiscourseStoreChirho {
    conn_chirho: Connection,
}

impl SqliteDiscourseStoreChirho {
    /// Open or create a discourse store at the given path.
    pub fn open_chirho(path_chirho: &str) -> DiscourseResultChirho<Self> {
        let conn_chirho = Connection::open(path_chirho)?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    /// Create an in-memory discourse store (for testing).
    pub fn in_memory_chirho() -> DiscourseResultChirho<Self> {
        let conn_chirho = Connection::open_in_memory()?;
        let store_chirho = Self { conn_chirho };
        store_chirho.run_migrations_chirho()?;
        Ok(store_chirho)
    }

    fn run_migrations_chirho(&self) -> DiscourseResultChirho<()> {
        let schema_chirho = include_str!("../migrations_chirho/001_initial_schema_chirho.sql");
        self.conn_chirho
            .execute_batch(schema_chirho)
            .map_err(|e_chirho| DiscourseErrorChirho::StorageChirho {
                reason_chirho: format!("Migration failed: {e_chirho}"),
            })?;
        Ok(())
    }

    fn save_propositions_chirho(
        &self,
        arc_id_chirho: i64,
        propositions_chirho: &[PropositionChirho],
    ) -> DiscourseResultChirho<()> {
        for prop_chirho in propositions_chirho {
            let phrase_json_chirho = prop_chirho
                .phrase_structure_chirho
                .as_ref()
                .map(|ps_chirho| serde_json::to_string(ps_chirho).unwrap_or_default());

            self.conn_chirho.execute(
                "INSERT INTO propositions_chirho
                    (id_chirho, arc_id_chirho, text_chirho, label_chirho,
                     start_book_chirho, start_chapter_chirho, start_verse_chirho,
                     end_book_chirho, end_chapter_chirho, end_verse_chirho,
                     start_char_chirho, end_char_chirho, notes_chirho, phrase_structure_chirho)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                params![
                    prop_chirho.id_chirho.0,
                    arc_id_chirho,
                    prop_chirho.text_chirho,
                    prop_chirho.label_chirho,
                    prop_chirho.verse_span_chirho.start_ref_chirho.book_chirho,
                    prop_chirho.verse_span_chirho.start_ref_chirho.chapter_chirho,
                    prop_chirho.verse_span_chirho.start_ref_chirho.verse_chirho,
                    prop_chirho.verse_span_chirho.end_ref_chirho.book_chirho,
                    prop_chirho.verse_span_chirho.end_ref_chirho.chapter_chirho,
                    prop_chirho.verse_span_chirho.end_ref_chirho.verse_chirho,
                    prop_chirho.verse_span_chirho.start_char_chirho,
                    prop_chirho.verse_span_chirho.end_char_chirho,
                    prop_chirho.notes_chirho,
                    phrase_json_chirho,
                ],
            )?;
        }
        Ok(())
    }

    fn save_relationships_chirho(
        &self,
        arc_id_chirho: i64,
        relationships_chirho: &[RelationshipChirho],
    ) -> DiscourseResultChirho<()> {
        for rel_chirho in relationships_chirho {
            self.conn_chirho.execute(
                "INSERT INTO relationships_chirho
                    (arc_id_chirho, source_id_chirho, target_id_chirho,
                     relationship_type_chirho, strength_chirho, notes_chirho)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    arc_id_chirho,
                    rel_chirho.source_id_chirho.0,
                    rel_chirho.target_id_chirho.0,
                    rel_chirho.relationship_type_chirho.as_str_chirho(),
                    rel_chirho.strength_chirho,
                    rel_chirho.notes_chirho,
                ],
            )?;
        }
        Ok(())
    }

    fn save_brackets_chirho(
        &self,
        arc_id_chirho: i64,
        brackets_chirho: &[BracketNodeChirho],
    ) -> DiscourseResultChirho<()> {
        for bracket_chirho in brackets_chirho {
            let type_str_chirho = match bracket_chirho.bracket_type_chirho {
                BracketTypeChirho::CoordinateChirho => "Coordinate",
                BracketTypeChirho::SubordinateChirho => "Subordinate",
            };

            let bracket_id_chirho = self.conn_chirho.query_row(
                "INSERT INTO brackets_chirho
                    (arc_id_chirho, label_chirho, bracket_type_chirho, display_order_chirho)
                 VALUES (?1, ?2, ?3, ?4)
                 RETURNING id_chirho",
                params![
                    arc_id_chirho,
                    bracket_chirho.label_chirho,
                    type_str_chirho,
                    bracket_chirho.display_order_chirho,
                ],
                |row_chirho| row_chirho.get::<_, i64>(0),
            )?;

            for prop_id_chirho in &bracket_chirho.proposition_ids_chirho {
                self.conn_chirho.execute(
                    "INSERT INTO bracket_propositions_chirho
                        (bracket_id_chirho, proposition_id_chirho)
                     VALUES (?1, ?2)",
                    params![bracket_id_chirho, prop_id_chirho.0],
                )?;
            }
        }
        Ok(())
    }

    fn load_propositions_chirho(
        &self,
        arc_id_chirho: i64,
    ) -> DiscourseResultChirho<Vec<PropositionChirho>> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, text_chirho, label_chirho,
                    start_book_chirho, start_chapter_chirho, start_verse_chirho,
                    end_book_chirho, end_chapter_chirho, end_verse_chirho,
                    start_char_chirho, end_char_chirho, notes_chirho, phrase_structure_chirho
             FROM propositions_chirho WHERE arc_id_chirho = ?1
             ORDER BY id_chirho",
        )?;

        let props_chirho = stmt_chirho
            .query_map(params![arc_id_chirho], |row_chirho| {
                let phrase_json_chirho: Option<String> = row_chirho.get(12)?;
                let phrase_structure_chirho = phrase_json_chirho
                    .and_then(|j_chirho| serde_json::from_str(&j_chirho).ok());

                let start_book_chirho: String = row_chirho.get(3)?;
                let start_ch_chirho: u16 = row_chirho.get(4)?;
                let start_v_chirho: u16 = row_chirho.get(5)?;
                let end_book_chirho: String = row_chirho.get(6)?;
                let end_ch_chirho: u16 = row_chirho.get(7)?;
                let end_v_chirho: u16 = row_chirho.get(8)?;

                Ok(PropositionChirho {
                    id_chirho: PropositionIdChirho(row_chirho.get(0)?),
                    text_chirho: row_chirho.get(1)?,
                    label_chirho: row_chirho.get(2)?,
                    verse_span_chirho: VerseSpanChirho {
                        start_ref_chirho: VerseRefChirho {
                            book_chirho: start_book_chirho,
                            chapter_chirho: start_ch_chirho,
                            verse_chirho: start_v_chirho,
                        },
                        end_ref_chirho: VerseRefChirho {
                            book_chirho: end_book_chirho,
                            chapter_chirho: end_ch_chirho,
                            verse_chirho: end_v_chirho,
                        },
                        start_char_chirho: row_chirho.get(9)?,
                        end_char_chirho: row_chirho.get(10)?,
                    },
                    notes_chirho: row_chirho.get(11)?,
                    phrase_structure_chirho,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(props_chirho)
    }

    fn load_relationships_chirho(
        &self,
        arc_id_chirho: i64,
    ) -> DiscourseResultChirho<Vec<RelationshipChirho>> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT source_id_chirho, target_id_chirho, relationship_type_chirho,
                    strength_chirho, notes_chirho
             FROM relationships_chirho WHERE arc_id_chirho = ?1",
        )?;

        let rels_chirho = stmt_chirho
            .query_map(params![arc_id_chirho], |row_chirho| {
                let type_str_chirho: String = row_chirho.get(2)?;
                Ok(RelationshipChirho {
                    source_id_chirho: PropositionIdChirho(row_chirho.get(0)?),
                    target_id_chirho: PropositionIdChirho(row_chirho.get(1)?),
                    relationship_type_chirho: RelationshipTypeChirho::from_str_chirho(
                        &type_str_chirho,
                    )
                    .unwrap_or(RelationshipTypeChirho::SeriesChirho),
                    strength_chirho: row_chirho.get(3)?,
                    notes_chirho: row_chirho.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rels_chirho)
    }

    fn load_brackets_chirho(
        &self,
        arc_id_chirho: i64,
    ) -> DiscourseResultChirho<Vec<BracketNodeChirho>> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT id_chirho, label_chirho, bracket_type_chirho, display_order_chirho
             FROM brackets_chirho WHERE arc_id_chirho = ?1
             ORDER BY display_order_chirho",
        )?;

        let brackets_chirho = stmt_chirho
            .query_map(params![arc_id_chirho], |row_chirho| {
                let bracket_id_chirho: i64 = row_chirho.get(0)?;
                let type_str_chirho: String = row_chirho.get(2)?;
                let bracket_type_chirho = if type_str_chirho == "Coordinate" {
                    BracketTypeChirho::CoordinateChirho
                } else {
                    BracketTypeChirho::SubordinateChirho
                };

                Ok((bracket_id_chirho, row_chirho.get(1)?, bracket_type_chirho, row_chirho.get(3)?))
            })?
            .collect::<Result<Vec<(i64, Option<String>, BracketTypeChirho, u32)>, _>>()?;

        let mut result_chirho = Vec::new();
        for (bid_chirho, label_chirho, btype_chirho, order_chirho) in brackets_chirho {
            let mut prop_stmt_chirho = self.conn_chirho.prepare(
                "SELECT proposition_id_chirho FROM bracket_propositions_chirho
                 WHERE bracket_id_chirho = ?1",
            )?;
            let prop_ids_chirho: Vec<PropositionIdChirho> = prop_stmt_chirho
                .query_map(params![bid_chirho], |row_chirho| {
                    Ok(PropositionIdChirho(row_chirho.get(0)?))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            result_chirho.push(BracketNodeChirho {
                label_chirho,
                bracket_type_chirho: btype_chirho,
                proposition_ids_chirho: prop_ids_chirho,
                display_order_chirho: order_chirho,
            });
        }

        Ok(result_chirho)
    }
}

impl DiscourseStoreChirho for SqliteDiscourseStoreChirho {
    fn save_arc_chirho(
        &self,
        arc_chirho: &ArcStructureChirho,
    ) -> DiscourseResultChirho<ArcIdChirho> {
        let tags_json_chirho = serde_json::to_string(&arc_chirho.tags_chirho)
            .unwrap_or_else(|_| "[]".to_string());

        let main_prop_chirho = arc_chirho
            .main_proposition_id_chirho
            .map(|id_chirho| id_chirho.0);

        // If we have an existing ID, delete and re-insert (simpler than partial updates).
        if let Some(existing_id_chirho) = arc_chirho.id_chirho {
            self.delete_arc_chirho(existing_id_chirho)?;
        }

        let arc_id_chirho: i64 = self.conn_chirho.query_row(
            "INSERT INTO arcs_chirho
                (title_chirho, start_book_chirho, start_chapter_chirho, start_verse_chirho,
                 end_book_chirho, end_chapter_chirho, end_verse_chirho,
                 main_proposition_id_chirho, tags_chirho)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             RETURNING id_chirho",
            params![
                arc_chirho.title_chirho,
                arc_chirho.passage_chirho.start_ref_chirho.book_chirho,
                arc_chirho.passage_chirho.start_ref_chirho.chapter_chirho,
                arc_chirho.passage_chirho.start_ref_chirho.verse_chirho,
                arc_chirho.passage_chirho.end_ref_chirho.book_chirho,
                arc_chirho.passage_chirho.end_ref_chirho.chapter_chirho,
                arc_chirho.passage_chirho.end_ref_chirho.verse_chirho,
                main_prop_chirho,
                tags_json_chirho,
            ],
            |row_chirho| row_chirho.get(0),
        )?;

        self.save_propositions_chirho(arc_id_chirho, &arc_chirho.propositions_chirho)?;
        self.save_relationships_chirho(arc_id_chirho, &arc_chirho.relationships_chirho)?;
        self.save_brackets_chirho(arc_id_chirho, &arc_chirho.brackets_chirho)?;

        Ok(ArcIdChirho(arc_id_chirho))
    }

    fn load_arc_chirho(
        &self,
        id_chirho: ArcIdChirho,
    ) -> DiscourseResultChirho<ArcStructureChirho> {
        let (title_chirho, sb_chirho, sc_chirho, sv_chirho, eb_chirho, ec_chirho, ev_chirho, main_chirho, tags_json_chirho): (String, String, u16, u16, String, u16, u16, Option<i64>, String) = self.conn_chirho.query_row(
            "SELECT title_chirho, start_book_chirho, start_chapter_chirho, start_verse_chirho,
                    end_book_chirho, end_chapter_chirho, end_verse_chirho,
                    main_proposition_id_chirho, tags_chirho
             FROM arcs_chirho WHERE id_chirho = ?1",
            params![id_chirho.0],
            |row_chirho| {
                Ok((
                    row_chirho.get(0)?,
                    row_chirho.get(1)?,
                    row_chirho.get(2)?,
                    row_chirho.get(3)?,
                    row_chirho.get(4)?,
                    row_chirho.get(5)?,
                    row_chirho.get(6)?,
                    row_chirho.get(7)?,
                    row_chirho.get(8)?,
                ))
            },
        ).map_err(|_| DiscourseErrorChirho::ArcNotFoundChirho { id_chirho: id_chirho.0 })?;

        let tags_chirho: Vec<String> =
            serde_json::from_str(&tags_json_chirho).unwrap_or_default();

        let propositions_chirho = self.load_propositions_chirho(id_chirho.0)?;
        let relationships_chirho = self.load_relationships_chirho(id_chirho.0)?;
        let brackets_chirho = self.load_brackets_chirho(id_chirho.0)?;

        Ok(ArcStructureChirho {
            id_chirho: Some(id_chirho),
            title_chirho,
            passage_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho {
                    book_chirho: sb_chirho,
                    chapter_chirho: sc_chirho,
                    verse_chirho: sv_chirho,
                },
                end_ref_chirho: VerseRefChirho {
                    book_chirho: eb_chirho,
                    chapter_chirho: ec_chirho,
                    verse_chirho: ev_chirho,
                },
                start_char_chirho: None,
                end_char_chirho: None,
            },
            propositions_chirho,
            relationships_chirho,
            main_proposition_id_chirho: main_chirho.map(PropositionIdChirho),
            tags_chirho,
            brackets_chirho,
        })
    }

    fn delete_arc_chirho(&self, id_chirho: ArcIdChirho) -> DiscourseResultChirho<()> {
        self.conn_chirho
            .execute("DELETE FROM arcs_chirho WHERE id_chirho = ?1", params![id_chirho.0])?;
        Ok(())
    }

    fn list_arcs_chirho(&self) -> DiscourseResultChirho<Vec<(ArcIdChirho, String)>> {
        let mut stmt_chirho = self
            .conn_chirho
            .prepare("SELECT id_chirho, title_chirho FROM arcs_chirho ORDER BY id_chirho")?;
        let arcs_chirho = stmt_chirho
            .query_map([], |row_chirho| {
                Ok((ArcIdChirho(row_chirho.get(0)?), row_chirho.get(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(arcs_chirho)
    }

    fn find_by_relationship_chirho(
        &self,
        relationship_type_chirho: RelationshipTypeChirho,
    ) -> DiscourseResultChirho<Vec<ArcIdChirho>> {
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT DISTINCT arc_id_chirho FROM relationships_chirho
             WHERE relationship_type_chirho = ?1",
        )?;
        let ids_chirho = stmt_chirho
            .query_map(params![relationship_type_chirho.as_str_chirho()], |row_chirho| {
                Ok(ArcIdChirho(row_chirho.get(0)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ids_chirho)
    }

    fn find_by_proposition_text_chirho(
        &self,
        text_query_chirho: &str,
    ) -> DiscourseResultChirho<Vec<ArcIdChirho>> {
        let pattern_chirho = format!("%{text_query_chirho}%");
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT DISTINCT arc_id_chirho FROM propositions_chirho
             WHERE text_chirho LIKE ?1",
        )?;
        let ids_chirho = stmt_chirho
            .query_map(params![pattern_chirho], |row_chirho| {
                Ok(ArcIdChirho(row_chirho.get(0)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ids_chirho)
    }

    fn count_arcs_chirho(&self) -> DiscourseResultChirho<u64> {
        let count_chirho: u64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM arcs_chirho",
            [],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn make_test_arc_chirho() -> ArcStructureChirho {
        ArcStructureChirho {
            id_chirho: None,
            title_chirho: "John 3:16-17 Analysis".to_string(),
            passage_chirho: VerseSpanChirho {
                start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 17).unwrap(),
                start_char_chirho: None,
                end_char_chirho: None,
            },
            propositions_chirho: vec![
                PropositionChirho {
                    id_chirho: PropositionIdChirho(1),
                    text_chirho: "God so loved the world".to_string(),
                    verse_span_chirho: VerseSpanChirho {
                        start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                        end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                        start_char_chirho: Some(4),
                        end_char_chirho: Some(26),
                    },
                    notes_chirho: None,
                    label_chirho: "1a".to_string(),
                    phrase_structure_chirho: None,
                },
                PropositionChirho {
                    id_chirho: PropositionIdChirho(2),
                    text_chirho: "that He gave His only begotten Son".to_string(),
                    verse_span_chirho: VerseSpanChirho {
                        start_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                        end_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                        start_char_chirho: Some(27),
                        end_char_chirho: Some(60),
                    },
                    notes_chirho: None,
                    label_chirho: "1b".to_string(),
                    phrase_structure_chirho: None,
                },
            ],
            relationships_chirho: vec![RelationshipChirho {
                source_id_chirho: PropositionIdChirho(1),
                target_id_chirho: PropositionIdChirho(2),
                relationship_type_chirho: RelationshipTypeChirho::ResultChirho,
                strength_chirho: 3,
                notes_chirho: Some("Result of God's love".to_string()),
            }],
            main_proposition_id_chirho: Some(PropositionIdChirho(1)),
            tags_chirho: vec!["gospel".to_string(), "love".to_string()],
            brackets_chirho: vec![BracketNodeChirho {
                label_chirho: Some("Main argument".to_string()),
                bracket_type_chirho: BracketTypeChirho::CoordinateChirho,
                proposition_ids_chirho: vec![PropositionIdChirho(1), PropositionIdChirho(2)],
                display_order_chirho: 0,
            }],
        }
    }

    #[test]
    fn test_save_and_load_arc_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        let arc_chirho = make_test_arc_chirho();

        let id_chirho = store_chirho.save_arc_chirho(&arc_chirho).unwrap();
        let loaded_chirho = store_chirho.load_arc_chirho(id_chirho).unwrap();

        assert_eq!(loaded_chirho.title_chirho, "John 3:16-17 Analysis");
        assert_eq!(loaded_chirho.propositions_chirho.len(), 2);
        assert_eq!(loaded_chirho.relationships_chirho.len(), 1);
        assert_eq!(loaded_chirho.brackets_chirho.len(), 1);
        assert_eq!(loaded_chirho.tags_chirho.len(), 2);
    }

    #[test]
    fn test_list_arcs_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        store_chirho.save_arc_chirho(&make_test_arc_chirho()).unwrap();

        let list_chirho = store_chirho.list_arcs_chirho().unwrap();
        assert_eq!(list_chirho.len(), 1);
        assert_eq!(list_chirho[0].1, "John 3:16-17 Analysis");
    }

    #[test]
    fn test_delete_arc_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        let id_chirho = store_chirho.save_arc_chirho(&make_test_arc_chirho()).unwrap();
        assert_eq!(store_chirho.count_arcs_chirho().unwrap(), 1);

        store_chirho.delete_arc_chirho(id_chirho).unwrap();
        assert_eq!(store_chirho.count_arcs_chirho().unwrap(), 0);
    }

    #[test]
    fn test_find_by_relationship_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        store_chirho.save_arc_chirho(&make_test_arc_chirho()).unwrap();

        let results_chirho = store_chirho
            .find_by_relationship_chirho(RelationshipTypeChirho::ResultChirho)
            .unwrap();
        assert_eq!(results_chirho.len(), 1);

        let empty_chirho = store_chirho
            .find_by_relationship_chirho(RelationshipTypeChirho::GroundChirho)
            .unwrap();
        assert!(empty_chirho.is_empty());
    }

    #[test]
    fn test_find_by_proposition_text_chirho() {
        let store_chirho = SqliteDiscourseStoreChirho::in_memory_chirho().unwrap();
        store_chirho.save_arc_chirho(&make_test_arc_chirho()).unwrap();

        let results_chirho = store_chirho
            .find_by_proposition_text_chirho("loved the world")
            .unwrap();
        assert_eq!(results_chirho.len(), 1);

        let empty_chirho = store_chirho
            .find_by_proposition_text_chirho("nonexistent")
            .unwrap();
        assert!(empty_chirho.is_empty());
    }
}
