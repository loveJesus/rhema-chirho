-- For God so loved the world that he gave his only begotten Son,
-- that whoever believes in him should not perish but have eternal life.
-- John 3:16

-- Module metadata
CREATE TABLE IF NOT EXISTS module_meta_chirho (
    key_chirho TEXT PRIMARY KEY,
    value_chirho TEXT NOT NULL
);

-- Verses: one row per verse
CREATE TABLE IF NOT EXISTS verses_chirho (
    id_chirho INTEGER PRIMARY KEY,
    book_chirho TEXT NOT NULL,
    chapter_chirho INTEGER NOT NULL,
    verse_chirho INTEGER NOT NULL,
    text_chirho TEXT NOT NULL,
    module_chirho TEXT NOT NULL,
    UNIQUE(book_chirho, chapter_chirho, verse_chirho, module_chirho)
);

-- Word-level tokens with morphology
CREATE TABLE IF NOT EXISTS tokens_chirho (
    id_chirho INTEGER PRIMARY KEY,
    verse_id_chirho INTEGER NOT NULL REFERENCES verses_chirho(id_chirho),
    word_index_chirho INTEGER NOT NULL,
    surface_chirho TEXT NOT NULL,
    lemma_chirho TEXT,
    strong_chirho TEXT,
    morph_raw_chirho TEXT,
    pos_chirho TEXT,
    tense_chirho TEXT,
    voice_chirho TEXT,
    mood_chirho TEXT,
    case_chirho TEXT,
    number_chirho TEXT,
    gender_chirho TEXT,
    person_chirho TEXT,
    hebrew_stem_chirho TEXT,
    hebrew_state_chirho TEXT,
    language_chirho TEXT NOT NULL DEFAULT 'Unknown'
);

CREATE INDEX IF NOT EXISTS idx_tokens_verse_chirho ON tokens_chirho(verse_id_chirho);
CREATE INDEX IF NOT EXISTS idx_tokens_pos_chirho ON tokens_chirho(pos_chirho);
CREATE INDEX IF NOT EXISTS idx_tokens_lemma_chirho ON tokens_chirho(lemma_chirho);
CREATE INDEX IF NOT EXISTS idx_tokens_strong_chirho ON tokens_chirho(strong_chirho);
CREATE INDEX IF NOT EXISTS idx_tokens_tense_chirho ON tokens_chirho(tense_chirho);
CREATE INDEX IF NOT EXISTS idx_verses_book_chirho ON verses_chirho(book_chirho, chapter_chirho);
