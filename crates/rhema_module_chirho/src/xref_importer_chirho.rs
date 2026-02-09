// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Cross-reference importer — populates the xref store from rsword_chirho
//! built-in parallel passages and other data sources.

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_contracts_chirho::xref_chirho::{CrossRefEntryChirho, CrossRefGraphChirho, XRefTypeChirho};
use rsword_chirho::{ParallelManagerChirho, ParallelPassageChirho, ParallelTypeChirho};

use crate::error_chirho::ModuleErrorChirho;
use crate::xref_store_chirho::XrefStoreChirho;

/// Map a rsword_chirho `ParallelTypeChirho` to our `XRefTypeChirho`.
fn map_parallel_type_chirho(pt_chirho: &ParallelTypeChirho) -> XRefTypeChirho {
    match pt_chirho {
        ParallelTypeChirho::SynopticChirho => XRefTypeChirho::ParallelChirho,
        ParallelTypeChirho::HistoryChirho => XRefTypeChirho::ParallelChirho,
        ParallelTypeChirho::ProphecyChirho => XRefTypeChirho::ProphecyFulfillmentChirho,
        ParallelTypeChirho::QuotationChirho => XRefTypeChirho::QuotationChirho,
        ParallelTypeChirho::ThematicChirho => XRefTypeChirho::AllusionChirho,
    }
}

/// Convert a `ParallelPassageChirho` into a `VerseRefChirho` using the
/// first verse of the passage range.
fn passage_to_verse_ref_chirho(
    passage_chirho: &ParallelPassageChirho,
) -> Option<VerseRefChirho> {
    let book_name_chirho = osis_abbrev_to_full_chirho(&passage_chirho.book_chirho)?;
    let chapter_chirho = passage_chirho.chapter_chirho as u16;
    let verse_chirho = passage_chirho.verse_start_chirho as u16;

    if chapter_chirho == 0 {
        return None;
    }

    Some(VerseRefChirho {
        book_chirho: book_name_chirho.to_string(),
        chapter_chirho,
        verse_chirho,
    })
}

/// Map OSIS book abbreviations used by rsword_chirho parallels to full names.
fn osis_abbrev_to_full_chirho(abbrev_chirho: &str) -> Option<&'static str> {
    match abbrev_chirho {
        "Matt" => Some("Matthew"),
        "Mark" => Some("Mark"),
        "Luke" => Some("Luke"),
        "John" => Some("John"),
        "Acts" => Some("Acts"),
        "Rom" => Some("Romans"),
        "1Cor" => Some("I Corinthians"),
        "2Cor" => Some("II Corinthians"),
        "Gal" => Some("Galatians"),
        "Eph" => Some("Ephesians"),
        "Phil" => Some("Philippians"),
        "Col" => Some("Colossians"),
        "1Thess" => Some("I Thessalonians"),
        "2Thess" => Some("II Thessalonians"),
        "1Tim" => Some("I Timothy"),
        "2Tim" => Some("II Timothy"),
        "Titus" => Some("Titus"),
        "Phlm" => Some("Philemon"),
        "Heb" => Some("Hebrews"),
        "Jas" => Some("James"),
        "1Pet" => Some("I Peter"),
        "2Pet" => Some("II Peter"),
        "1John" => Some("I John"),
        "2John" => Some("II John"),
        "3John" => Some("III John"),
        "Jude" => Some("Jude"),
        "Rev" => Some("Revelation of John"),
        "Gen" => Some("Genesis"),
        "Exod" => Some("Exodus"),
        "Lev" => Some("Leviticus"),
        "Num" => Some("Numbers"),
        "Deut" => Some("Deuteronomy"),
        "Josh" => Some("Joshua"),
        "Judg" => Some("Judges"),
        "Ruth" => Some("Ruth"),
        "1Sam" => Some("I Samuel"),
        "2Sam" => Some("II Samuel"),
        "1Kgs" => Some("I Kings"),
        "2Kgs" => Some("II Kings"),
        "1Chr" => Some("I Chronicles"),
        "2Chr" => Some("II Chronicles"),
        "Ezra" => Some("Ezra"),
        "Neh" => Some("Nehemiah"),
        "Esth" => Some("Esther"),
        "Job" => Some("Job"),
        "Ps" => Some("Psalms"),
        "Prov" => Some("Proverbs"),
        "Eccl" => Some("Ecclesiastes"),
        "Song" => Some("Song of Solomon"),
        "Isa" => Some("Isaiah"),
        "Jer" => Some("Jeremiah"),
        "Lam" => Some("Lamentations"),
        "Ezek" => Some("Ezekiel"),
        "Dan" => Some("Daniel"),
        "Hos" => Some("Hosea"),
        "Joel" => Some("Joel"),
        "Amos" => Some("Amos"),
        "Obad" => Some("Obadiah"),
        "Jonah" => Some("Jonah"),
        "Mic" => Some("Micah"),
        "Nah" => Some("Nahum"),
        "Hab" => Some("Habakkuk"),
        "Zeph" => Some("Zephaniah"),
        "Hag" => Some("Haggai"),
        "Zech" => Some("Zechariah"),
        "Mal" => Some("Malachi"),
        _ => None,
    }
}

/// Import all built-in parallel passages from rsword_chirho into the xref store.
///
/// Creates bidirectional edges between the first verse of each parallel passage
/// in each parallel set. Returns the number of edges inserted.
pub fn import_parallel_passages_chirho(
    store_chirho: &XrefStoreChirho,
) -> Result<usize, ModuleErrorChirho> {
    let manager_chirho = ParallelManagerChirho::new_chirho();
    let mut entries_chirho: Vec<CrossRefEntryChirho> = Vec::new();

    // Iterate all parallel types
    for parallel_type_chirho in &[
        ParallelTypeChirho::SynopticChirho,
        ParallelTypeChirho::HistoryChirho,
        ParallelTypeChirho::ProphecyChirho,
        ParallelTypeChirho::QuotationChirho,
        ParallelTypeChirho::ThematicChirho,
    ] {
        let sets_chirho = manager_chirho.get_by_type_chirho(*parallel_type_chirho);
        let xref_type_chirho = map_parallel_type_chirho(parallel_type_chirho);

        for set_chirho in &sets_chirho {
            // Convert passages to verse refs
            let refs_chirho: Vec<VerseRefChirho> = set_chirho
                .passages_chirho
                .iter()
                .filter_map(passage_to_verse_ref_chirho)
                .collect();

            // Create bidirectional edges between all pairs
            for i_chirho in 0..refs_chirho.len() {
                for j_chirho in (i_chirho + 1)..refs_chirho.len() {
                    // Forward edge: i → j
                    entries_chirho.push(
                        CrossRefEntryChirho::new_chirho(
                            refs_chirho[i_chirho].clone(),
                            refs_chirho[j_chirho].clone(),
                            xref_type_chirho,
                        )
                        .with_confidence_chirho(4)
                        .with_note_chirho(&set_chirho.description_chirho)
                        .with_dataset_chirho("parallel"),
                    );

                    // Reverse edge: j → i
                    entries_chirho.push(
                        CrossRefEntryChirho::new_chirho(
                            refs_chirho[j_chirho].clone(),
                            refs_chirho[i_chirho].clone(),
                            xref_type_chirho,
                        )
                        .with_confidence_chirho(4)
                        .with_note_chirho(&set_chirho.description_chirho)
                        .with_dataset_chirho("parallel"),
                    );
                }
            }
        }
    }

    store_chirho.insert_xrefs_batch_chirho(&entries_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_import_parallel_count_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let count_chirho = import_parallel_passages_chirho(&store_chirho).unwrap();
        // Should insert a non-trivial number of edges
        assert!(count_chirho > 0, "Expected some edges, got {}", count_chirho);
    }

    #[test]
    fn test_synoptic_edges_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        import_parallel_passages_chirho(&store_chirho).unwrap();

        // The baptism of Jesus is in Matt, Mark, Luke
        // Matt 3:13 should have outgoing edges to Mark and Luke parallels
        let from_matt_chirho = store_chirho
            .xrefs_from_chirho(&VerseRefChirho {
                book_chirho: "Matthew".to_string(),
                chapter_chirho: 3,
                verse_chirho: 13,
            })
            .unwrap();

        // Should have at least edges to Mark and Luke baptism accounts
        assert!(
            from_matt_chirho.len() >= 2,
            "Expected at least 2 edges from Matt 3:13, got {}",
            from_matt_chirho.len()
        );
    }

    #[test]
    fn test_history_edges_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        import_parallel_passages_chirho(&store_chirho).unwrap();

        // David becomes king: 2Sam 5:1 ↔ 1Chr 11:1
        let from_sam_chirho = store_chirho
            .xrefs_from_chirho(&VerseRefChirho {
                book_chirho: "II Samuel".to_string(),
                chapter_chirho: 5,
                verse_chirho: 1,
            })
            .unwrap();

        assert!(
            !from_sam_chirho.is_empty(),
            "Expected edges from II Samuel 5:1"
        );
    }

    #[test]
    fn test_bidirectional_edges_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        import_parallel_passages_chirho(&store_chirho).unwrap();

        // Check that edges are bidirectional
        let from_matt_chirho = store_chirho
            .xrefs_from_chirho(&VerseRefChirho {
                book_chirho: "Matthew".to_string(),
                chapter_chirho: 3,
                verse_chirho: 13,
            })
            .unwrap();

        // Find a target
        if let Some(first_target_chirho) = from_matt_chirho.first() {
            let reverse_chirho = store_chirho
                .xrefs_from_chirho(&first_target_chirho.target_chirho)
                .unwrap();

            let has_reverse_chirho = reverse_chirho.iter().any(|e_chirho| {
                e_chirho.target_chirho.book_chirho == "Matthew"
                    && e_chirho.target_chirho.chapter_chirho == 3
                    && e_chirho.target_chirho.verse_chirho == 13
            });

            assert!(has_reverse_chirho, "Expected reverse edge back to Matthew 3:13");
        }
    }

    #[test]
    fn test_type_mapping_chirho() {
        assert_eq!(
            map_parallel_type_chirho(&ParallelTypeChirho::SynopticChirho),
            XRefTypeChirho::ParallelChirho
        );
        assert_eq!(
            map_parallel_type_chirho(&ParallelTypeChirho::HistoryChirho),
            XRefTypeChirho::ParallelChirho
        );
        assert_eq!(
            map_parallel_type_chirho(&ParallelTypeChirho::ProphecyChirho),
            XRefTypeChirho::ProphecyFulfillmentChirho
        );
        assert_eq!(
            map_parallel_type_chirho(&ParallelTypeChirho::QuotationChirho),
            XRefTypeChirho::QuotationChirho
        );
        assert_eq!(
            map_parallel_type_chirho(&ParallelTypeChirho::ThematicChirho),
            XRefTypeChirho::AllusionChirho
        );
    }

    #[test]
    fn test_passage_to_verse_ref_chirho() {
        let passage_chirho = ParallelPassageChirho::new_chirho(
            "Matt",
            3,
            13,
            17,
            ParallelTypeChirho::SynopticChirho,
        );
        let ref_chirho = passage_to_verse_ref_chirho(&passage_chirho).unwrap();
        assert_eq!(ref_chirho.book_chirho, "Matthew");
        assert_eq!(ref_chirho.chapter_chirho, 3);
        assert_eq!(ref_chirho.verse_chirho, 13);
    }

    #[test]
    fn test_passage_to_verse_ref_unknown_book_chirho() {
        let passage_chirho = ParallelPassageChirho::new_chirho(
            "Fake",
            1,
            1,
            5,
            ParallelTypeChirho::ThematicChirho,
        );
        assert!(passage_to_verse_ref_chirho(&passage_chirho).is_none());
    }

    #[test]
    fn test_dataset_label_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        import_parallel_passages_chirho(&store_chirho).unwrap();

        let from_matt_chirho = store_chirho
            .xrefs_from_chirho(&VerseRefChirho {
                book_chirho: "Matthew".to_string(),
                chapter_chirho: 3,
                verse_chirho: 13,
            })
            .unwrap();

        if let Some(entry_chirho) = from_matt_chirho.first() {
            assert_eq!(entry_chirho.source_dataset_chirho, "parallel");
        }
    }

    #[test]
    fn test_bfs_through_parallels_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        import_parallel_passages_chirho(&store_chirho).unwrap();

        // BFS depth 1 from Matt 3:13 should find Mark/Luke parallels
        let expanded_chirho = store_chirho
            .bfs_expand_chirho(
                &VerseRefChirho {
                    book_chirho: "Matthew".to_string(),
                    chapter_chirho: 3,
                    verse_chirho: 13,
                },
                1,
                false,
            )
            .unwrap();

        assert!(
            expanded_chirho.len() >= 2,
            "Expected at least 2 BFS results, got {}",
            expanded_chirho.len()
        );
    }

    #[test]
    fn test_idempotent_import_chirho() {
        let store_chirho = XrefStoreChirho::in_memory_chirho().unwrap();
        let count1_chirho = import_parallel_passages_chirho(&store_chirho).unwrap();
        let count2_chirho = import_parallel_passages_chirho(&store_chirho).unwrap();

        // Second import should insert 0 due to UNIQUE + INSERT OR IGNORE
        assert_eq!(count2_chirho, 0);

        // Total count should match first import
        assert_eq!(store_chirho.count_xrefs_chirho().unwrap(), count1_chirho as u64);
    }
}
