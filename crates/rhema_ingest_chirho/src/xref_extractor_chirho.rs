// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Cross-reference extraction from OSIS and GBF markup.
//!
//! Extracts `<reference osisRef="...">` and `<note type="crossReference">` from
//! OSIS markup, and `<RX>...<Rx>` from GBF markup.

use regex::Regex;
use std::sync::LazyLock;

use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_contracts_chirho::xref_chirho::{CrossRefEntryChirho, XRefTypeChirho};

// ── Regex patterns ───────────────────────────────────────────────

static OSIS_REF_RE_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<reference\s+osisRef="([^"]+)"[^>]*>"#).unwrap()
});

static OSIS_NOTE_RE_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<note\s+type="crossReference"[^>]*>(.*?)</note>"#).unwrap()
});

static GBF_XREF_RE_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<RX>(.*?)<Rx>").unwrap()
});

// ── OSIS book name mapping ───────────────────────────────────────

/// Map OSIS abbreviated book name to full book name.
pub fn osis_book_to_name_chirho(osis_chirho: &str) -> Option<&'static str> {
    match osis_chirho {
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
        _ => None,
    }
}

/// Parse an OSIS reference like `Gen.1.1` or `John.3.16` into a VerseRefChirho.
pub fn parse_osis_ref_chirho(osis_ref_chirho: &str) -> Vec<VerseRefChirho> {
    let mut results_chirho = Vec::new();

    // Handle semicolon-separated references: "Gen.1.1;John.3.16"
    for part_chirho in osis_ref_chirho.split(';') {
        let trimmed_chirho = part_chirho.trim();
        if trimmed_chirho.is_empty() {
            continue;
        }

        // Handle range: "Gen.1.1-Gen.1.5" → take first
        let range_part_chirho = if let Some(dash_chirho) = trimmed_chirho.find('-') {
            &trimmed_chirho[..dash_chirho]
        } else {
            trimmed_chirho
        };

        let segments_chirho: Vec<&str> = range_part_chirho.split('.').collect();
        if segments_chirho.len() >= 3 {
            let book_osis_chirho = segments_chirho[0];
            if let (Some(book_chirho), Ok(ch_chirho), Ok(v_chirho)) = (
                osis_book_to_name_chirho(book_osis_chirho),
                segments_chirho[1].parse::<u16>(),
                segments_chirho[2].parse::<u16>(),
            )
                && ch_chirho > 0
            {
                results_chirho.push(VerseRefChirho {
                    book_chirho: book_chirho.to_string(),
                    chapter_chirho: ch_chirho,
                    verse_chirho: v_chirho,
                });
            }
        } else if segments_chirho.len() == 2 {
            // Chapter-level: "Gen.1"
            let book_osis_chirho = segments_chirho[0];
            if let (Some(book_chirho), Ok(ch_chirho)) = (
                osis_book_to_name_chirho(book_osis_chirho),
                segments_chirho[1].parse::<u16>(),
            )
                && ch_chirho > 0
            {
                results_chirho.push(VerseRefChirho {
                    book_chirho: book_chirho.to_string(),
                    chapter_chirho: ch_chirho,
                    verse_chirho: 0,
                });
            }
        }
    }

    results_chirho
}

/// Parse a human-readable reference like "John 3:16" or "I Corinthians 13:4".
pub fn parse_human_ref_chirho(ref_str_chirho: &str) -> Option<VerseRefChirho> {
    static HUMAN_REF_RE_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(.+?)\s+(\d+):(\d+)$").unwrap()
    });

    let caps_chirho = HUMAN_REF_RE_CHIRHO.captures(ref_str_chirho.trim())?;
    let book_chirho = caps_chirho.get(1)?.as_str();
    let chapter_chirho: u16 = caps_chirho.get(2)?.as_str().parse().ok()?;
    let verse_chirho: u16 = caps_chirho.get(3)?.as_str().parse().ok()?;

    if chapter_chirho == 0 {
        return None;
    }

    Some(VerseRefChirho {
        book_chirho: book_chirho.to_string(),
        chapter_chirho,
        verse_chirho,
    })
}

/// Extract cross-references from OSIS markup.
pub fn extract_osis_xrefs_chirho(
    raw_text_chirho: &str,
    source_verse_chirho: &VerseRefChirho,
) -> Vec<CrossRefEntryChirho> {
    let mut results_chirho = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();

    // Extract from <reference osisRef="..."> tags
    for cap_chirho in OSIS_REF_RE_CHIRHO.captures_iter(raw_text_chirho) {
        let osis_ref_chirho = &cap_chirho[1];
        let targets_chirho = parse_osis_ref_chirho(osis_ref_chirho);
        for target_chirho in targets_chirho {
            if seen_chirho.insert(target_chirho.clone()) {
                results_chirho.push(
                    CrossRefEntryChirho::new_chirho(
                        source_verse_chirho.clone(),
                        target_chirho,
                        XRefTypeChirho::DirectChirho,
                    )
                    .with_dataset_chirho("osis"),
                );
            }
        }
    }

    // Extract from <note type="crossReference"> tags (may contain OSIS refs inside)
    for cap_chirho in OSIS_NOTE_RE_CHIRHO.captures_iter(raw_text_chirho) {
        let note_content_chirho = &cap_chirho[1];
        // Look for osisRef inside the note
        for inner_cap_chirho in OSIS_REF_RE_CHIRHO.captures_iter(note_content_chirho) {
            let osis_ref_chirho = &inner_cap_chirho[1];
            let targets_chirho = parse_osis_ref_chirho(osis_ref_chirho);
            for target_chirho in targets_chirho {
                if seen_chirho.insert(target_chirho.clone()) {
                    results_chirho.push(
                        CrossRefEntryChirho::new_chirho(
                            source_verse_chirho.clone(),
                            target_chirho,
                            XRefTypeChirho::DirectChirho,
                        )
                        .with_dataset_chirho("osis"),
                    );
                }
            }
        }
    }

    results_chirho
}

/// Extract cross-references from GBF markup.
pub fn extract_gbf_xrefs_chirho(
    raw_text_chirho: &str,
    source_verse_chirho: &VerseRefChirho,
) -> Vec<CrossRefEntryChirho> {
    let mut results_chirho = Vec::new();

    for cap_chirho in GBF_XREF_RE_CHIRHO.captures_iter(raw_text_chirho) {
        let ref_text_chirho = cap_chirho[1].trim();
        if ref_text_chirho.is_empty() {
            continue;
        }

        // Try parsing as human-readable reference first
        if let Some(target_chirho) = parse_human_ref_chirho(ref_text_chirho) {
            results_chirho.push(
                CrossRefEntryChirho::new_chirho(
                    source_verse_chirho.clone(),
                    target_chirho,
                    XRefTypeChirho::DirectChirho,
                )
                .with_dataset_chirho("gbf"),
            );
        }
    }

    results_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    // ── OSIS extraction tests ───────────────────────────────────

    #[test]
    fn test_extract_osis_simple_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 3,
            verse_chirho: 16,
        };
        let markup_chirho = r#"text <reference osisRef="Rom.5.8">Romans 5:8</reference> more"#;
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].target_chirho.book_chirho, "Romans");
        assert_eq!(refs_chirho[0].target_chirho.chapter_chirho, 5);
        assert_eq!(refs_chirho[0].target_chirho.verse_chirho, 8);
        assert_eq!(refs_chirho[0].source_dataset_chirho, "osis");
    }

    #[test]
    fn test_extract_osis_multiple_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "Matthew".to_string(),
            chapter_chirho: 4,
            verse_chirho: 4,
        };
        let markup_chirho = r#"<reference osisRef="Deut.8.3">Deut 8:3</reference> and <reference osisRef="Ps.91.11">Ps 91:11</reference>"#;
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 2);
    }

    #[test]
    fn test_extract_osis_note_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "Romans".to_string(),
            chapter_chirho: 3,
            verse_chirho: 23,
        };
        let markup_chirho = r#"all have sinned <note type="crossReference"><reference osisRef="Gen.3.6">Gen 3:6</reference></note>"#;
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].target_chirho.book_chirho, "Genesis");
    }

    #[test]
    fn test_extract_osis_range_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "Acts".to_string(),
            chapter_chirho: 2,
            verse_chirho: 17,
        };
        let markup_chirho =
            r#"<reference osisRef="Joel.2.28-Joel.2.32">Joel 2:28-32</reference>"#;
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].target_chirho.book_chirho, "Joel");
        assert_eq!(refs_chirho[0].target_chirho.chapter_chirho, 2);
        assert_eq!(refs_chirho[0].target_chirho.verse_chirho, 28);
    }

    #[test]
    fn test_extract_osis_semicolons_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "Hebrews".to_string(),
            chapter_chirho: 11,
            verse_chirho: 1,
        };
        let markup_chirho =
            r#"<reference osisRef="Rom.1.17;Gal.3.11">Rom 1:17; Gal 3:11</reference>"#;
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 2);
        assert_eq!(refs_chirho[0].target_chirho.book_chirho, "Romans");
        assert_eq!(refs_chirho[1].target_chirho.book_chirho, "Galatians");
    }

    #[test]
    fn test_extract_osis_no_refs_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "Gen".to_string(),
            chapter_chirho: 1,
            verse_chirho: 1,
        };
        let markup_chirho = "In the beginning God created the heavens and the earth.";
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert!(refs_chirho.is_empty());
    }

    // ── GBF extraction tests ────────────────────────────────────

    #[test]
    fn test_extract_gbf_single_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 3,
            verse_chirho: 16,
        };
        let markup_chirho = "text <RX>Romans 5:8<Rx> more";
        let refs_chirho = extract_gbf_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].target_chirho.book_chirho, "Romans");
        assert_eq!(refs_chirho[0].source_dataset_chirho, "gbf");
    }

    #[test]
    fn test_extract_gbf_multiple_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 1,
            verse_chirho: 1,
        };
        let markup_chirho = "<RX>Genesis 1:1<Rx> and <RX>Colossians 1:16<Rx>";
        let refs_chirho = extract_gbf_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho.len(), 2);
    }

    #[test]
    fn test_extract_gbf_empty_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "Gen".to_string(),
            chapter_chirho: 1,
            verse_chirho: 1,
        };
        let markup_chirho = "<RX><Rx>";
        let refs_chirho = extract_gbf_xrefs_chirho(markup_chirho, &source_chirho);
        assert!(refs_chirho.is_empty());
    }

    // ── OSIS ref parsing tests ──────────────────────────────────

    #[test]
    fn test_parse_osis_ref_simple_chirho() {
        let refs_chirho = parse_osis_ref_chirho("Gen.1.1");
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].book_chirho, "Genesis");
        assert_eq!(refs_chirho[0].chapter_chirho, 1);
        assert_eq!(refs_chirho[0].verse_chirho, 1);
    }

    #[test]
    fn test_parse_osis_ref_nt_chirho() {
        let refs_chirho = parse_osis_ref_chirho("John.3.16");
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].book_chirho, "John");
    }

    #[test]
    fn test_parse_osis_ref_range_chirho() {
        let refs_chirho = parse_osis_ref_chirho("Gen.1.1-Gen.1.5");
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].verse_chirho, 1); // Takes the start of range
    }

    #[test]
    fn test_parse_osis_ref_semicolons_chirho() {
        let refs_chirho = parse_osis_ref_chirho("Gen.1.1;John.3.16;Rev.22.21");
        assert_eq!(refs_chirho.len(), 3);
    }

    #[test]
    fn test_parse_osis_ref_chapter_level_chirho() {
        let refs_chirho = parse_osis_ref_chirho("Ps.23");
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].book_chirho, "Psalms");
        assert_eq!(refs_chirho[0].chapter_chirho, 23);
        assert_eq!(refs_chirho[0].verse_chirho, 0);
    }

    #[test]
    fn test_parse_osis_ref_unknown_book_chirho() {
        let refs_chirho = parse_osis_ref_chirho("FakeBook.1.1");
        assert!(refs_chirho.is_empty());
    }

    // ── Human ref parsing tests ─────────────────────────────────

    #[test]
    fn test_parse_human_ref_standard_chirho() {
        let ref_chirho = parse_human_ref_chirho("John 3:16").unwrap();
        assert_eq!(ref_chirho.book_chirho, "John");
        assert_eq!(ref_chirho.chapter_chirho, 3);
        assert_eq!(ref_chirho.verse_chirho, 16);
    }

    #[test]
    fn test_parse_human_ref_multiword_chirho() {
        let ref_chirho = parse_human_ref_chirho("I Corinthians 13:4").unwrap();
        assert_eq!(ref_chirho.book_chirho, "I Corinthians");
        assert_eq!(ref_chirho.chapter_chirho, 13);
        assert_eq!(ref_chirho.verse_chirho, 4);
    }

    #[test]
    fn test_parse_human_ref_invalid_chirho() {
        assert!(parse_human_ref_chirho("invalid").is_none());
        assert!(parse_human_ref_chirho("").is_none());
        assert!(parse_human_ref_chirho("John").is_none());
    }

    // ── Book mapping tests ──────────────────────────────────────

    #[test]
    fn test_osis_book_mapping_ot_chirho() {
        assert_eq!(osis_book_to_name_chirho("Gen"), Some("Genesis"));
        assert_eq!(osis_book_to_name_chirho("Exod"), Some("Exodus"));
        assert_eq!(osis_book_to_name_chirho("Mal"), Some("Malachi"));
    }

    #[test]
    fn test_osis_book_mapping_nt_chirho() {
        assert_eq!(osis_book_to_name_chirho("Matt"), Some("Matthew"));
        assert_eq!(osis_book_to_name_chirho("Rev"), Some("Revelation of John"));
        assert_eq!(osis_book_to_name_chirho("1John"), Some("I John"));
    }

    #[test]
    fn test_osis_book_mapping_unknown_chirho() {
        assert_eq!(osis_book_to_name_chirho("FakeBook"), None);
    }

    // ── Source dataset tests ────────────────────────────────────

    #[test]
    fn test_osis_source_dataset_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 3,
            verse_chirho: 16,
        };
        let markup_chirho = r#"<reference osisRef="Rom.5.8">Rom</reference>"#;
        let refs_chirho = extract_osis_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho[0].source_dataset_chirho, "osis");
    }

    #[test]
    fn test_gbf_source_dataset_chirho() {
        let source_chirho = VerseRefChirho {
            book_chirho: "John".to_string(),
            chapter_chirho: 3,
            verse_chirho: 16,
        };
        let markup_chirho = "<RX>Romans 5:8<Rx>";
        let refs_chirho = extract_gbf_xrefs_chirho(markup_chirho, &source_chirho);
        assert_eq!(refs_chirho[0].source_dataset_chirho, "gbf");
    }
}
