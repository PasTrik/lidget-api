use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct QuizCategory {
    pub id: String,
    pub slug: String,
    pub label: String,
}

#[derive(Deserialize)]
pub struct NewQuizCategory {
    pub slug: String,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub relationship_id: Option<String>,
    pub title: String,
    pub category_id: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct NewQuiz {
    pub id: String,
    pub relationship_id: Option<String>,
    pub title: String,
    pub category_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub quiz_id: String,
    pub question: String,
    pub type_: String,
    pub target: String,
    pub order: i64,
}

#[derive(Deserialize)]
pub struct NewQuizQuestion {
  pub quiz_id: String,
  pub question: String,
  pub type_: String,
  pub target: String,
  pub order: i64,
}

#[derive(Serialize, Deserialize)]
pub struct QuizChoice {
    pub id: String,
    pub question_id: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct NewQuizChoice {
    pub question_id: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuizAnswer {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub choice_id: Option<String>,
    pub question_id: String,
    pub custom_text: Option<String>,
    pub date_answer: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct NewQuizAnswer {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub question_id: String,
    pub choice_id: Option<String>,
    pub custom_text: Option<String>,
    pub date_answer: Option<String>,
}

/*
    Quiz categories
*/
pub async fn create_quiz_category(pool: &SqlitePool, id: &str, quiz_category: NewQuizCategory) ->
Result<QuizCategory, AppError> {
    sqlx::query_as!(
        QuizCategory,
        "INSERT INTO quiz_category (id, slug, label) VALUES (?, ?, ?) RETURNING id as \"id!\", slug, label",
        id, quiz_category.slug, quiz_category.label
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

pub async fn find_all_quiz_categories(pool: &SqlitePool) -> Result<Vec<QuizCategory>, AppError> {
  sqlx::query_as!(
        QuizCategory,
        "SELECT id as \"id!\", slug, label FROM quiz_category"
    )
    .fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn edit_quiz_category(pool: &SqlitePool, quiz_category: QuizCategory) -> Result<QuizCategory, AppError> {
  sqlx::query_as!(
        QuizCategory,
        "UPDATE quiz_category SET slug = ?, label = ? WHERE id = ? RETURNING id as \"id!\", slug, label",
        quiz_category.slug, quiz_category.label, quiz_category.id
    )
    .fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn delete_quiz_category(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM quiz_category WHERE id = ?",
        id
    ).execute(pool).await.map_err(|_| AppError::DatabaseError)?;
    Ok(())
}

/*
    Quizzes
*/
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

pub async fn edit_quiz(pool: &SqlitePool, quiz: NewQuiz) -> Result<Quiz, AppError> {
    sqlx::query_as!(
        Quiz,
        "UPDATE quizzes SET title = ?, category_id = ? WHERE id = ? RETURNING id as \"id!\", relationship_id, title, category_id, created_at",
        quiz.title, quiz.category_id, quiz.id
    )
    .fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn delete_quiz(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM quizzes WHERE id = ?",
        id
    ).execute(pool).await.map_err(|_| AppError::DatabaseError)?;
    Ok(())
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

pub async fn search_quiz_by_title(pool: &SqlitePool, title: &str, limit: i64, offset: i64) -> Result<Vec<Quiz>, AppError> {
    let pattern = format!("%{}%", title);
    sqlx::query_as!(
        Quiz,
        "SELECT id as \"id!\", relationship_id, title, category_id, created_at
         FROM quizzes
         WHERE title LIKE ?
         ORDER BY created_at DESC
         LIMIT ? OFFSET ?
         ",
        pattern, limit, offset
    )
    .fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_all_quizzes(pool: &SqlitePool, relationship_id: &str, limit: i64, offset: i64) -> Result<Vec<Quiz>, AppError> {
    sqlx::query_as!(
        Quiz,
        "SELECT id as \"id!\", relationship_id, title, category_id, created_at
         FROM quizzes
         WHERE relationship_id IS NULL OR relationship_id = ?
         ORDER BY created_at DESC
         LIMIT ? OFFSET ?
         ",
        relationship_id,
        limit,
        offset
    )
    .fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_available_quizzes(pool: &SqlitePool, user_id: &str, relationship_id: &str, limit: i64, offset: i64) -> Result<Vec<Quiz>, AppError> {
    sqlx::query_as!(
        Quiz,
        "SELECT id as \"id!\", relationship_id, title, category_id, created_at FROM quizzes q
         WHERE (q.relationship_id IS NULL OR q.relationship_id = ?)
         AND q.id NOT IN (
             SELECT qa.quiz_id FROM quiz_answers qa
             WHERE qa.user_id = ?
             GROUP BY qa.quiz_id
             HAVING COUNT(DISTINCT qa.question_id) = (
                 SELECT COUNT(*) FROM quiz_questions qq
                 WHERE qq.quiz_id = qa.quiz_id
             )
         )
        ORDER BY q.created_at DESC
        LIMIT ? OFFSET ?
        ",
        relationship_id, user_id, limit, offset
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_quiz_history(pool: &SqlitePool, user_id: &str, limit: i64, offset: i64) -> Result<Vec<Quiz>, AppError> {
    sqlx::query_as!(
        Quiz,
        "SELECT id as \"id!\", relationship_id, title, category_id, created_at FROM quizzes
         WHERE id IN (
             SELECT quiz_id FROM quiz_answers
             WHERE user_id = ?
         )
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        ",
        user_id, limit, offset
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn create_quiz_question(pool: &SqlitePool, id: &str, quiz_question: NewQuizQuestion) -> Result<QuizQuestion, AppError> {
    sqlx::query_as!(
        QuizQuestion,
        "INSERT INTO quiz_questions (id, quiz_id, question, type, target, \"order\")
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING id as \"id!\", quiz_id, question, type as \"type_\", target, \"order\"",
        id, quiz_question.quiz_id, quiz_question.question, quiz_question.type_, quiz_question.target, quiz_question.order
    ).fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_quiz_questions(pool: &SqlitePool, quiz_id: &str) -> Result<Vec<QuizQuestion>, AppError> {
    sqlx::query_as!(
        QuizQuestion,
        "SELECT id as \"id!\", quiz_id, question, type as \"type_\", target, \"order\"
         FROM quiz_questions
         WHERE quiz_id = ?",
        quiz_id
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn edit_quiz_question(pool: &SqlitePool, quiz_question: QuizQuestion) -> Result<QuizQuestion, AppError> {
  sqlx::query_as!(
        QuizQuestion,
        "UPDATE quiz_questions SET question = ?, type = ?, target = ?, \"order\" = ? WHERE id = ? RETURNING id as \"id!\", quiz_id, question, type as \"type_\", target, \"order\"",
        quiz_question.question, quiz_question.type_, quiz_question.target, quiz_question.order, quiz_question.id
    ).fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn delete_quiz_question(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM quiz_questions WHERE id = ?",
        id
    ).execute(pool).await.map_err(|_| AppError::DatabaseError)?;
    Ok(())
}

pub async fn create_quiz_choice(pool: &SqlitePool, id: &str, quiz_choice: NewQuizChoice) -> Result<QuizChoice, AppError> {
    sqlx::query_as!(
        QuizChoice,
        "INSERT INTO quiz_choices (id, question_id, content)
         VALUES (?, ?, ?)
         RETURNING id as \"id!\", question_id, content",
        id, quiz_choice.question_id, quiz_choice.content
    ).fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn edit_quiz_choice(pool: &SqlitePool, quiz_choice: QuizChoice) -> Result<QuizChoice, AppError> {
  sqlx::query_as!(
        QuizChoice,
        "UPDATE quiz_choices SET content = ? WHERE id = ? RETURNING id as \"id!\", question_id, content",
        quiz_choice.content, quiz_choice.id
    ).fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn delete_quiz_choice(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM quiz_choices WHERE id = ?",
        id
    ).execute(pool).await.map_err(|_| AppError::DatabaseError)?;
    Ok(())
}

pub async fn find_quiz_choices(pool: &SqlitePool, question_id: &str) -> Result<Vec<QuizChoice>, AppError> {
    sqlx::query_as!(
        QuizChoice,
        "SELECT id as \"id!\", question_id, content
         FROM quiz_choices
         WHERE question_id = ?",
        question_id
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_quiz_choice_by_quiz_id(
    pool: &SqlitePool,
    quiz_id: &str,
) -> Result<Vec<QuizChoice>, AppError> {
    sqlx::query_as!(
        QuizChoice,
        "SELECT qc.id as \"id!\", qc.question_id, qc.content
         FROM quiz_choices qc
         INNER JOIN quiz_questions qq ON  qq.id = qc.question_id
         WHERE qq.quiz_id = ?",
        quiz_id
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn create_quiz_answer(pool: &SqlitePool, quiz_answer: NewQuizAnswer) -> Result<QuizAnswer, AppError> {
    sqlx::query_as!(
        QuizAnswer,
        "INSERT INTO quiz_answers (id, quiz_id, user_id, question_id, choice_id, custom_text, date_answer)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         RETURNING id as \"id!\", quiz_id, user_id, question_id, choice_id, custom_text, date_answer, created_at",
        quiz_answer.id, quiz_answer.quiz_id, quiz_answer.user_id, quiz_answer.question_id, quiz_answer.choice_id, quiz_answer.custom_text, quiz_answer.date_answer
    ).fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_answers_by_quiz_and_user(pool: &SqlitePool, quiz_id: &str, user_id: &str) -> Result<Vec<QuizAnswer>, AppError> {
    sqlx::query_as!(
        QuizAnswer,
        "SELECT id as \"id!\", quiz_id, user_id, question_id, choice_id, custom_text, date_answer, created_at
         FROM quiz_answers
         WHERE quiz_id = ? AND user_id = ?",
        quiz_id, user_id
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_answers_by_quiz_and_users(
    pool: &SqlitePool,
    quiz_id: &str,
    user1_id: &str,
    user2_id: &str,
) -> Result<Vec<QuizAnswer>, AppError> {
    sqlx::query_as!(
        QuizAnswer,
        "SELECT id as \"id!\", quiz_id, user_id, question_id, choice_id, custom_text, date_answer, created_at
         FROM quiz_answers
         WHERE quiz_id = ? AND user_id IN (?, ?)",
        quiz_id, user1_id, user2_id
    ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}