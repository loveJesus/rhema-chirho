-- For God so loved the world that he gave his only begotten Son,
-- that whoever believes in him should not perish but have eternal life.
-- John 3:16

-- Syntax/clause schema

CREATE TABLE IF NOT EXISTS clauses_chirho (
    clause_id_chirho  INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id_chirho  INTEGER,
    clause_type_chirho TEXT NOT NULL,
    book_chirho       TEXT NOT NULL,
    chapter_chirho    INTEGER NOT NULL,
    verse_chirho      INTEGER NOT NULL,
    FOREIGN KEY (parent_id_chirho) REFERENCES clauses_chirho (clause_id_chirho)
);

CREATE INDEX IF NOT EXISTS idx_clause_type_chirho
    ON clauses_chirho (clause_type_chirho);
CREATE INDEX IF NOT EXISTS idx_clause_verse_chirho
    ON clauses_chirho (book_chirho, chapter_chirho, verse_chirho);
