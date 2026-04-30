use crate::errors::AppError;
use sqlx::SqlitePool;

pub struct QuizCategory {
    pub id: String,
    pub slug: String,
    pub label: String,
}

pub struct Quiz {
    pub id: String,
    pub relationship_id: Option<String>,
    pub title: String,
    pub category_id: String,
    pub created_at: String,
}

pub struct NewQuiz {
    pub id: String,
    pub relationship_id: Option<String>,
    pub title: String,
    pub category_id: String,
}

pub struct QuizQuestions {
    pub id: String,
    pub quiz_id: String,
    pub question: String,
    pub type_: String,
    pub target: String,
    pub order: String,
}

pub struct QuizChoices {
    pub id: String,
    pub question_id: String,
    pub content: String,
}

pub struct QuizAnswers {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub choice_id: Option<String>,
    pub custom_text: Option<String>,
    pub date_answer: Option<String>,
    pub created_at: String,
}

pub async fn create_quiz_category(pool: &SqlitePool, quiz_category: QuizCategory) ->
Result<QuizCategory, AppError> {
    sqlx::query_as!(
        QuizCategory,
        "INSERT INTO quiz_category (id, slug, label) VALUES (?, ?, ?) RETURNING id as \"id!\", slug, label",
        quiz_category.id, quiz_category.slug, quiz_category.label
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AppError::DatabaseError)
}

pub async fn find_quiz_category_by_id(pool: &SqlitePool, id: &str) -> Result<Option<QuizCategory>, AppError> {
    sqlx::query_as!(
        QuizCategory,
        "SELECT id as \"id!\", slug, label FROM quiz_category WHERE id = ?",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::DatabaseError)
}

pub async fn create_quiz(pool: &SqlitePool, quiz: NewQuiz) -> Result<Quiz, AppError> {
    sqlx::query_as!(
        Quiz,
        "INSERT INTO quizzes (id, relationship_id, title, category_id)
         VALUES (?, ?, ?, ?)
         RETURNING id as \"id!\", relationship_id, title, category_id, created_at",
        quiz.id, quiz.relationship_id, quiz.title, quiz.category_id
    )
    .fetch_one(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn find_quiz_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Quiz>, AppError> {
    sqlx::query_as!(
        Quiz,
        "SELECT id as \"id!\", relationship_id, title, category_id, created_at
         FROM quizzes
         WHERE id = ?",
        id
    )
    .fetch_optional(pool).await.map_err(|_| AppError::DatabaseError)
}