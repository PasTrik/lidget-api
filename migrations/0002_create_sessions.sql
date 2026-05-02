CREATE TABLE IF NOT EXISTS sessions
(
    id
    TEXT
    PRIMARY
    KEY,
    user_id
    TEXT
    NOT
    NULL
    REFERENCES
    users
(
    id
) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    device_name TEXT NOT NULL,
    last_seen TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

