-- For God so loved the world that he gave his only begotten Son,
-- that whoever believes in him should not perish but have eternal life.
-- John 3:16

-- Initial schema for discourse analysis storage.

CREATE TABLE IF NOT EXISTS arcs_chirho (
    id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
    title_chirho TEXT NOT NULL,
    start_book_chirho TEXT NOT NULL,
    start_chapter_chirho INTEGER NOT NULL,
    start_verse_chirho INTEGER NOT NULL,
    end_book_chirho TEXT NOT NULL,
    end_chapter_chirho INTEGER NOT NULL,
    end_verse_chirho INTEGER NOT NULL,
    main_proposition_id_chirho INTEGER,
    tags_chirho TEXT NOT NULL DEFAULT '[]',
    created_at_chirho TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at_chirho TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS propositions_chirho (
    id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
    arc_id_chirho INTEGER NOT NULL REFERENCES arcs_chirho(id_chirho) ON DELETE CASCADE,
    text_chirho TEXT NOT NULL,
    label_chirho TEXT NOT NULL,
    start_book_chirho TEXT NOT NULL,
    start_chapter_chirho INTEGER NOT NULL,
    start_verse_chirho INTEGER NOT NULL,
    end_book_chirho TEXT NOT NULL,
    end_chapter_chirho INTEGER NOT NULL,
    end_verse_chirho INTEGER NOT NULL,
    start_char_chirho INTEGER,
    end_char_chirho INTEGER,
    notes_chirho TEXT,
    phrase_structure_chirho TEXT
);

CREATE TABLE IF NOT EXISTS relationships_chirho (
    id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
    arc_id_chirho INTEGER NOT NULL REFERENCES arcs_chirho(id_chirho) ON DELETE CASCADE,
    source_id_chirho INTEGER NOT NULL,
    target_id_chirho INTEGER NOT NULL,
    relationship_type_chirho TEXT NOT NULL,
    strength_chirho INTEGER NOT NULL DEFAULT 2,
    notes_chirho TEXT
);

CREATE TABLE IF NOT EXISTS brackets_chirho (
    id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
    arc_id_chirho INTEGER NOT NULL REFERENCES arcs_chirho(id_chirho) ON DELETE CASCADE,
    label_chirho TEXT,
    bracket_type_chirho TEXT NOT NULL,
    display_order_chirho INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS bracket_propositions_chirho (
    bracket_id_chirho INTEGER NOT NULL REFERENCES brackets_chirho(id_chirho) ON DELETE CASCADE,
    proposition_id_chirho INTEGER NOT NULL,
    PRIMARY KEY (bracket_id_chirho, proposition_id_chirho)
);

CREATE INDEX IF NOT EXISTS idx_propositions_arc_chirho
    ON propositions_chirho(arc_id_chirho);

CREATE INDEX IF NOT EXISTS idx_relationships_arc_chirho
    ON relationships_chirho(arc_id_chirho);

CREATE INDEX IF NOT EXISTS idx_relationships_type_chirho
    ON relationships_chirho(relationship_type_chirho);

CREATE INDEX IF NOT EXISTS idx_relationships_source_chirho
    ON relationships_chirho(source_id_chirho);

CREATE INDEX IF NOT EXISTS idx_relationships_target_chirho
    ON relationships_chirho(target_id_chirho);

CREATE INDEX IF NOT EXISTS idx_brackets_arc_chirho
    ON brackets_chirho(arc_id_chirho);
