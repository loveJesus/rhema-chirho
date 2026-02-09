-- For God so loved the world that he gave his only begotten Son,
-- that whoever believes in him should not perish but have eternal life.
-- John 3:16

-- Cross-reference graph: directed edges between verses.
CREATE TABLE IF NOT EXISTS cross_refs_chirho (
    id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
    source_book_chirho TEXT NOT NULL,
    source_chapter_chirho INTEGER NOT NULL,
    source_verse_chirho INTEGER NOT NULL,
    target_book_chirho TEXT NOT NULL,
    target_chapter_chirho INTEGER NOT NULL,
    target_verse_chirho INTEGER NOT NULL,
    xref_type_chirho TEXT NOT NULL DEFAULT 'Direct',
    confidence_chirho INTEGER NOT NULL DEFAULT 3,
    note_chirho TEXT,
    source_dataset_chirho TEXT NOT NULL DEFAULT 'unknown',
    UNIQUE(source_book_chirho, source_chapter_chirho, source_verse_chirho,
           target_book_chirho, target_chapter_chirho, target_verse_chirho,
           source_dataset_chirho)
);

CREATE INDEX IF NOT EXISTS idx_xref_source_chirho
    ON cross_refs_chirho(source_book_chirho, source_chapter_chirho, source_verse_chirho);

CREATE INDEX IF NOT EXISTS idx_xref_target_chirho
    ON cross_refs_chirho(target_book_chirho, target_chapter_chirho, target_verse_chirho);

CREATE INDEX IF NOT EXISTS idx_xref_type_chirho
    ON cross_refs_chirho(xref_type_chirho);

CREATE INDEX IF NOT EXISTS idx_xref_dataset_chirho
    ON cross_refs_chirho(source_dataset_chirho);
