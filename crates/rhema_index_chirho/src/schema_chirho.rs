// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Rhema index schema — defines the Tantivy schema with structured fields
//! for books, chapters, verses, Strong's numbers, morphology, and full text.

use tantivy::schema::{Field, Schema, FAST, INDEXED, STORED, STRING, TEXT};

/// Schema version — bump when fields change (invalidates existing indexes).
pub const SCHEMA_VERSION_CHIRHO: u32 = 2;

/// Field names for the Rhema Tantivy schema.
pub const FIELD_KEY_CHIRHO: &str = "key";
pub const FIELD_BOOK_CHIRHO: &str = "book";
pub const FIELD_CHAPTER_CHIRHO: &str = "chapter";
pub const FIELD_VERSE_CHIRHO: &str = "verse";
pub const FIELD_TEXT_CHIRHO: &str = "text";
pub const FIELD_STRONGS_CHIRHO: &str = "strongs";
pub const FIELD_MODULE_CHIRHO: &str = "module";

// Phase 3: Morphology fields
pub const FIELD_MORPH_CHIRHO: &str = "morph";
pub const FIELD_LEMMA_CHIRHO: &str = "lemma";
pub const FIELD_POS_CHIRHO: &str = "pos";
pub const FIELD_TENSE_CHIRHO: &str = "tense";
pub const FIELD_VOICE_CHIRHO: &str = "voice";
pub const FIELD_MOOD_CHIRHO: &str = "mood";
pub const FIELD_CASE_CHIRHO: &str = "case_field";
pub const FIELD_NUMBER_CHIRHO: &str = "number_field";
pub const FIELD_GENDER_CHIRHO: &str = "gender";
pub const FIELD_PERSON_CHIRHO: &str = "person";

/// The Rhema index schema with all field handles.
#[derive(Clone, Debug)]
pub struct RhemaSchemaChirho {
    /// The built Tantivy schema.
    pub schema_chirho: Schema,
    /// Composite key field — e.g., "Genesis 1:1" (stored, not tokenized).
    pub key_field_chirho: Field,
    /// Book name — e.g., "Genesis" (stored, keyword for filtering).
    pub book_field_chirho: Field,
    /// Chapter number (stored, indexed, fast).
    pub chapter_field_chirho: Field,
    /// Verse number (stored, indexed, fast).
    pub verse_field_chirho: Field,
    /// Full text of the verse — tokenized and indexed for search.
    pub text_field_chirho: Field,
    /// Strong's numbers — e.g., "H430 H1254" (tokenized for term search).
    pub strongs_field_chirho: Field,
    /// Module name — e.g., "KJV" (stored, keyword for multi-module indexes).
    pub module_field_chirho: Field,

    // Phase 3: Morphology fields
    /// Raw morph code(s) — tokenized text (multiple per verse).
    pub morph_field_chirho: Field,
    /// Lemma strings — tokenized text (multiple per verse).
    pub lemma_field_chirho: Field,
    /// Part of speech facet — keyword (e.g., "VerbChirho").
    pub pos_field_chirho: Field,
    /// Tense facet — keyword.
    pub tense_field_chirho: Field,
    /// Voice facet — keyword.
    pub voice_field_chirho: Field,
    /// Mood facet — keyword.
    pub mood_field_chirho: Field,
    /// Case facet — keyword.
    pub case_field_chirho: Field,
    /// Grammatical number facet — keyword.
    pub number_field_chirho: Field,
    /// Gender facet — keyword.
    pub gender_field_chirho: Field,
    /// Person facet — keyword.
    pub person_field_chirho: Field,
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

        // Phase 3: Morphology fields
        let morph_field_chirho =
            builder_chirho.add_text_field(FIELD_MORPH_CHIRHO, TEXT);
        let lemma_field_chirho =
            builder_chirho.add_text_field(FIELD_LEMMA_CHIRHO, TEXT);
        let pos_field_chirho =
            builder_chirho.add_text_field(FIELD_POS_CHIRHO, STRING);
        let tense_field_chirho =
            builder_chirho.add_text_field(FIELD_TENSE_CHIRHO, STRING);
        let voice_field_chirho =
            builder_chirho.add_text_field(FIELD_VOICE_CHIRHO, STRING);
        let mood_field_chirho =
            builder_chirho.add_text_field(FIELD_MOOD_CHIRHO, STRING);
        let case_field_chirho =
            builder_chirho.add_text_field(FIELD_CASE_CHIRHO, STRING);
        let number_field_chirho =
            builder_chirho.add_text_field(FIELD_NUMBER_CHIRHO, STRING);
        let gender_field_chirho =
            builder_chirho.add_text_field(FIELD_GENDER_CHIRHO, STRING);
        let person_field_chirho =
            builder_chirho.add_text_field(FIELD_PERSON_CHIRHO, STRING);

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
            morph_field_chirho,
            lemma_field_chirho,
            pos_field_chirho,
            tense_field_chirho,
            voice_field_chirho,
            mood_field_chirho,
            case_field_chirho,
            number_field_chirho,
            gender_field_chirho,
            person_field_chirho,
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_schema_fields_chirho() {
        let schema_chirho = RhemaSchemaChirho::build_chirho();
        // Verify all original fields are present
        assert!(schema_chirho.schema_chirho.get_field(FIELD_KEY_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_TEXT_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_STRONGS_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_BOOK_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_MODULE_CHIRHO).is_ok());
    }

    #[test]
    fn test_morph_fields_exist_chirho() {
        let schema_chirho = RhemaSchemaChirho::build_chirho();
        assert!(schema_chirho.schema_chirho.get_field(FIELD_MORPH_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_LEMMA_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_POS_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_TENSE_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_VOICE_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_MOOD_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_CASE_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_NUMBER_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_GENDER_CHIRHO).is_ok());
        assert!(schema_chirho.schema_chirho.get_field(FIELD_PERSON_CHIRHO).is_ok());
    }

    #[test]
    fn test_schema_version_chirho() {
        assert_eq!(SCHEMA_VERSION_CHIRHO, 2);
    }

    #[test]
    fn test_total_field_count_chirho() {
        let schema_chirho = RhemaSchemaChirho::build_chirho();
        // 7 original + 10 morph = 17 fields
        let field_count_chirho = schema_chirho.schema_chirho.fields().count();
        assert_eq!(field_count_chirho, 17);
    }
}
