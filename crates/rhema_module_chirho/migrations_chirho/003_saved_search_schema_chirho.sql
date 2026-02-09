-- For God so loved the world that he gave his only begotten Son,
-- that whoever believes in him should not perish but have eternal life.
-- John 3:16

-- Saved searches schema

CREATE TABLE IF NOT EXISTS saved_searches_chirho (
    id_chirho         TEXT PRIMARY KEY NOT NULL,
    name_chirho       TEXT NOT NULL UNIQUE,
    query_text_chirho TEXT NOT NULL,
    created_at_chirho TEXT NOT NULL DEFAULT (datetime('now')),
    last_used_at_chirho TEXT NOT NULL DEFAULT (datetime('now')),
    usage_count_chirho INTEGER NOT NULL DEFAULT 0,
    tags_chirho       TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_saved_search_name_chirho
    ON saved_searches_chirho (name_chirho);
CREATE INDEX IF NOT EXISTS idx_saved_search_tags_chirho
    ON saved_searches_chirho (tags_chirho);
