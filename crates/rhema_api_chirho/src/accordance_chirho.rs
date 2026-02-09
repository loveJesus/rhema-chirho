// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Accordance Bible software URL scheme generator.
//!
//! Generates `accordance://` URLs for deep-linking into Accordance.

/// Generate an Accordance URL for a verse reference.
///
/// Format: `accordance://read/?q=Book+Chapter:Verse`
pub fn accordance_url_chirho(verse_ref_chirho: &str) -> String {
    let encoded_chirho = verse_ref_chirho.replace(' ', "+");
    format!("accordance://read/?q={encoded_chirho}")
}

/// Generate an Accordance search URL.
///
/// Format: `accordance://search/?q=search+terms`
pub fn accordance_search_url_chirho(query_chirho: &str) -> String {
    let encoded_chirho = query_chirho.replace(' ', "+");
    format!("accordance://search/?q={encoded_chirho}")
}

/// Map a SWORD book name to an Accordance-compatible book abbreviation.
pub fn sword_to_accordance_book_chirho(sword_book_chirho: &str) -> &str {
    match sword_book_chirho {
        "Genesis" => "Gen",
        "Exodus" => "Exod",
        "Leviticus" => "Lev",
        "Numbers" => "Num",
        "Deuteronomy" => "Deut",
        "Joshua" => "Josh",
        "Judges" => "Judg",
        "Ruth" => "Ruth",
        "I Samuel" => "1Sam",
        "II Samuel" => "2Sam",
        "I Kings" => "1Kgs",
        "II Kings" => "2Kgs",
        "I Chronicles" => "1Chr",
        "II Chronicles" => "2Chr",
        "Ezra" => "Ezra",
        "Nehemiah" => "Neh",
        "Esther" => "Esth",
        "Job" => "Job",
        "Psalms" => "Ps",
        "Proverbs" => "Prov",
        "Ecclesiastes" => "Eccl",
        "Song of Solomon" => "Song",
        "Isaiah" => "Isa",
        "Jeremiah" => "Jer",
        "Lamentations" => "Lam",
        "Ezekiel" => "Ezek",
        "Daniel" => "Dan",
        "Hosea" => "Hos",
        "Joel" => "Joel",
        "Amos" => "Amos",
        "Obadiah" => "Obad",
        "Jonah" => "Jonah",
        "Micah" => "Mic",
        "Nahum" => "Nah",
        "Habakkuk" => "Hab",
        "Zephaniah" => "Zeph",
        "Haggai" => "Hag",
        "Zechariah" => "Zech",
        "Malachi" => "Mal",
        "Matthew" => "Matt",
        "Mark" => "Mark",
        "Luke" => "Luke",
        "John" => "John",
        "Acts" => "Acts",
        "Romans" => "Rom",
        "I Corinthians" => "1Cor",
        "II Corinthians" => "2Cor",
        "Galatians" => "Gal",
        "Ephesians" => "Eph",
        "Philippians" => "Phil",
        "Colossians" => "Col",
        "I Thessalonians" => "1Thess",
        "II Thessalonians" => "2Thess",
        "I Timothy" => "1Tim",
        "II Timothy" => "2Tim",
        "Titus" => "Titus",
        "Philemon" => "Phlm",
        "Hebrews" => "Heb",
        "James" => "Jas",
        "I Peter" => "1Pet",
        "II Peter" => "2Pet",
        "I John" => "1John",
        "II John" => "2John",
        "III John" => "3John",
        "Jude" => "Jude",
        "Revelation of John" => "Rev",
        other_chirho => other_chirho,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_single_verse_url_chirho() {
        let url_chirho = accordance_url_chirho("John 3:16");
        assert_eq!(url_chirho, "accordance://read/?q=John+3:16");
    }

    #[test]
    fn test_chapter_url_chirho() {
        let url_chirho = accordance_url_chirho("Romans 8");
        assert_eq!(url_chirho, "accordance://read/?q=Romans+8");
    }

    #[test]
    fn test_book_name_mapping_chirho() {
        assert_eq!(sword_to_accordance_book_chirho("I Corinthians"), "1Cor");
        assert_eq!(sword_to_accordance_book_chirho("Song of Solomon"), "Song");
        assert_eq!(
            sword_to_accordance_book_chirho("Revelation of John"),
            "Rev"
        );
    }

    #[test]
    fn test_multi_word_book_url_chirho() {
        let url_chirho = accordance_url_chirho("Song of Solomon 2:1");
        assert_eq!(url_chirho, "accordance://read/?q=Song+of+Solomon+2:1");
    }

    #[test]
    fn test_search_url_chirho() {
        let url_chirho = accordance_search_url_chirho("love AND world");
        assert_eq!(url_chirho, "accordance://search/?q=love+AND+world");
    }

    #[test]
    fn test_empty_ref_url_chirho() {
        let url_chirho = accordance_url_chirho("");
        assert_eq!(url_chirho, "accordance://read/?q=");
    }
}
