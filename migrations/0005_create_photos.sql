CREATE TABLE IF NOT EXISTS photos
(
    id              TEXT PRIMARY KEY,
    relationship_id TEXT NOT NULL REFERENCES relationships (id) ON DELETE CASCADE,
    sender_id       TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    file_url        TEXT NOT NULL,
    caption         TEXT,
    type            TEXT NOT NULL CHECK (type IN ('photo', 'drawing')),
    created_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
)