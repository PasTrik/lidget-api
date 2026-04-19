CREATE TABLE IF NOT EXISTS notifications_queue (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL CHECK ( event_type IN (
                                                    'new_photo',
                                                    'new_drawing',
                                                    'quiz_answered',
                                                    'new_quiz_available',
                                                    'location_update',
                                                    'event_created',
                                                    'partner_joined',
                                                    'quiz_streak_reminder'
        )),
    payload TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    delivered_at TEXT
)