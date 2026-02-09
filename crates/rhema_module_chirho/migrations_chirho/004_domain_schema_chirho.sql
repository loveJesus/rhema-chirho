-- For God so loved the world that he gave his only begotten Son,
-- that whoever believes in him should not perish but have eternal life.
-- John 3:16

-- Semantic domain schema

CREATE TABLE IF NOT EXISTS senses_chirho (
    sense_id_chirho   TEXT PRIMARY KEY NOT NULL,
    domain_chirho     TEXT NOT NULL,
    gloss_chirho      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS sense_verses_chirho (
    sense_id_chirho   TEXT NOT NULL,
    book_chirho       TEXT NOT NULL,
    chapter_chirho    INTEGER NOT NULL,
    verse_chirho      INTEGER NOT NULL,
    FOREIGN KEY (sense_id_chirho) REFERENCES senses_chirho (sense_id_chirho)
);

CREATE INDEX IF NOT EXISTS idx_sense_domain_chirho
    ON senses_chirho (domain_chirho);
CREATE INDEX IF NOT EXISTS idx_sense_verse_chirho
    ON sense_verses_chirho (sense_id_chirho);
CREATE INDEX IF NOT EXISTS idx_sense_verse_ref_chirho
    ON sense_verses_chirho (book_chirho, chapter_chirho, verse_chirho);
