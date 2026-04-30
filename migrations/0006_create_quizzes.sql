CREATE TABLE IF NOT EXISTS quiz_category
(
    id         TEXT PRIMARY KEY,
    slug       TEXT NOT NULL UNIQUE, -- 'freaky', 'amour' etc. (identifiant technique)
    label      TEXT NOT NULL,        -- 'Freaky', 'Amour' etc. (affiché dans l'app)
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quizzes
(
    id              TEXT PRIMARY KEY,
    relationship_id TEXT REFERENCES relationships (id) ON DELETE CASCADE,
    title           TEXT NOT NULL,
    category_id     TEXT NOT NULL REFERENCES quiz_category (id) ON DELETE CASCADE,
    created_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quiz_questions
(
    id         TEXT PRIMARY KEY,
    quiz_id    TEXT                                                                    NOT NULL REFERENCES quizzes (id) ON DELETE CASCADE,
    question   TEXT                                                                    NOT NULL,
    type       TEXT CHECK ( type IN ('text_choices', 'image_choices', 'date_answer') ) NOT NULL,
    target     TEXT                                                                    NOT NULL CHECK ( target IN ('self', 'partner') ),
    "order"    INTEGER                                                                 NOT NULL,
    created_at TEXT                                                                    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quiz_choices
(
    id          TEXT PRIMARY KEY,
    question_id TEXT NOT NULL REFERENCES quiz_questions (id) ON DELETE CASCADE,
    content     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS quiz_answers
(
    id          TEXT PRIMARY KEY,
    quiz_id     TEXT NOT NULL REFERENCES quizzes (id) ON DELETE CASCADE,
    user_id     TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    question_id TEXT NOT NULL REFERENCES quiz_questions (id) ON DELETE CASCADE,
    choice_id   TEXT REFERENCES quiz_choices (id) ON DELETE CASCADE,
    custom_text TEXT,
    date_answer TEXT,
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
)