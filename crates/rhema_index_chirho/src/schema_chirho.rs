// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Rhema index schema — defines the Tantivy schema with structured fields
//! for books, chapters, verses, Strong's numbers, and full text.

use tantivy::schema::{Field, Schema, FAST, INDEXED, STORED, STRING, TEXT};

/// Field names for the Rhema Tantivy schema.
pub const FIELD_KEY_CHIRHO: &str = "key";
pub const FIELD_BOOK_CHIRHO: &str = "book";
pub const FIELD_CHAPTER_CHIRHO: &str = "chapter";
pub const FIELD_VERSE_CHIRHO: &str = "verse";
pub const FIELD_TEXT_CHIRHO: &str = "text";
pub const FIELD_STRONGS_CHIRHO: &str = "strongs";
pub const FIELD_MODULE_CHIRHO: &str = "module";

/// The Rhema index schema with all field handles.
#[derive(Clone, Debug)]
pub struct RhemaSchemaChirho {
    /// The built Tantivy schema.
    pub schema_chirho: Schema,
    /// Composite key field — e.g., "Genesis 1:1" (stored, not tokenized).
    pub key_field_chirho: Field,
    /// Book name — e.g., "Genesis" (stored, keyword for filtering).
    pub book_field_chirho: Field,
    /// Chapter number as string — e.g., "1" (stored, keyword for filtering).
    pub chapter_field_chirho: Field,
    /// Verse number as string — e.g., "1" (stored, keyword for filtering).
    pub verse_field_chirho: Field,
    /// Full text of the verse — tokenized and indexed for search.
    pub text_field_chirho: Field,
    /// Strong's numbers — e.g., "H430 H1254" (tokenized for term search).
    pub strongs_field_chirho: Field,
    /// Module name — e.g., "KJV" (stored, keyword for multi-module indexes).
    pub module_field_chirho: Field,
}

impl RhemaSchemaChirho {
    /// Build the Rhema index schema.
    pub fn build_chirho() -> Self {
        let mut builder_chirho = Schema::builder();

        // Key: stored + keyword (exact match, not tokenized)
        let key_field_chirho =
            builder_chirho.add_text_field(FIELD_KEY_CHIRHO, STRING | STORED);
        // Book: stored + keyword for filtering
        let book_field_chirho =
            builder_chirho.add_text_field(FIELD_BOOK_CHIRHO, STRING | STORED);
        // Chapter: stored + keyword (treated as string for faceting)
        let chapter_field_chirho =
            builder_chirho.add_u64_field(FIELD_CHAPTER_CHIRHO, INDEXED | STORED | FAST);
        // Verse: stored + keyword
        let verse_field_chirho =
            builder_chirho.add_u64_field(FIELD_VERSE_CHIRHO, INDEXED | STORED | FAST);
        // Text: full-text indexed (tokenized), stored for snippets
        let text_field_chirho =
            builder_chirho.add_text_field(FIELD_TEXT_CHIRHO, TEXT | STORED);
        // Strong's numbers: tokenized text field for term-level searching
        let strongs_field_chirho =
            builder_chirho.add_text_field(FIELD_STRONGS_CHIRHO, TEXT);
        // Module: keyword for multi-module indexes
        let module_field_chirho =
            builder_chirho.add_text_field(FIELD_MODULE_CHIRHO, STRING | STORED);

        let schema_chirho = builder_chirho.build();

        Self {
            schema_chirho,
            key_field_chirho,
            book_field_chirho,
            chapter_field_chirho,
            verse_field_chirho,
            text_field_chirho,
            strongs_field_chirho,
            module_field_chirho,
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_schema_fields_chirho() {
        let schema_chirho = RhemaSchemaChirho::build_chirho();
        // Verify all fields are present
        assert!(
            schema_chirho
                .schema_chirho
                .get_field(FIELD_KEY_CHIRHO)
                .is_ok()
        );
        assert!(
            schema_chirho
                .schema_chirho
                .get_field(FIELD_TEXT_CHIRHO)
                .is_ok()
        );
        assert!(
            schema_chirho
                .schema_chirho
                .get_field(FIELD_STRONGS_CHIRHO)
                .is_ok()
        );
        assert!(
            schema_chirho
                .schema_chirho
                .get_field(FIELD_BOOK_CHIRHO)
                .is_ok()
        );
        assert!(
            schema_chirho
                .schema_chirho
                .get_field(FIELD_MODULE_CHIRHO)
                .is_ok()
        );
    }
}
