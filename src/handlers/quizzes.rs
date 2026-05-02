use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::quiz;
use crate::models::quiz::Quiz;
use crate::pagination::PaginationParams;
use crate::state::AppState;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::upload;

#[derive(Serialize)]
pub struct QuizChoiceResponse {
    pub id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct QuizQuestionResponse {
    pub id: String,
    pub question: String,
    pub type_: String,
    pub target: String,
    pub order: i64,
    pub choices: Vec<QuizChoiceResponse>,
}

#[derive(Serialize)]
pub struct QuizDetailResponse {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub questions: Vec<QuizQuestionResponse>,
}

#[derive(Deserialize)]
pub struct TextChoiceBody {
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct QuizAnswerBody {
    pub question_id: String,
    pub choice_id: Option<String>,
    pub custom_text: Option<String>,
    pub date_answer: Option<String>,
}

#[derive(Deserialize)]
pub struct QuizBody {
    pub relationship_id: Option<String>,
    pub title: String,
    pub category_id: String,
}

pub async fn create_quiz_category(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(body): Json<quiz::NewQuizCategory>,
) -> Result<(StatusCode, Json<()>), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    quiz::create_quiz_category(&state.db, &id, body).await?;
    Ok((StatusCode::OK, Json(())))
}

pub async fn find_all_quizzes_categories(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<quiz::QuizCategory>>), AppError> {
    let categories = quiz::find_all_quiz_categories(&state.db).await?;
    Ok((StatusCode::OK, Json(categories)))
}

pub async fn edit_quiz_category(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<quiz::NewQuizCategory>,
) -> Result<(StatusCode, Json<quiz::QuizCategory>), AppError> {
    let category = quiz::edit_quiz_category(
        &state.db,
        quiz::QuizCategory {
            id,
            slug: body.slug,
            label: body.label,
        },
    ).await?;
    Ok((StatusCode::OK, Json(category)))
}

pub async fn delete_quiz_category(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, ()), AppError> {
    quiz::delete_quiz_category(&state.db, &id).await?;
    Ok((StatusCode::OK, ()))
}

pub async fn create_quiz(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(body): Json<QuizBody>,
) -> Result<(StatusCode, Json<Quiz>), AppError> {
    let quiz = quiz::create_quiz(
        &state.db,
        quiz::NewQuiz {
            id: uuid::Uuid::new_v4().to_string(),
            relationship_id: body.relationship_id,
            title: body.title,
            category_id: body.category_id,
        },
    ).await.map_err(|_| AppError::DatabaseError)?;
    Ok((StatusCode::OK, Json(quiz)))
}

pub async fn find_all_quizzes(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<(StatusCode, Json<Vec<Quiz>>), AppError> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    let quizzes = quiz::find_all_quizzes(
        &state.db,
        &auth_user.relationship.as_ref().map(|r| r.id.as_str()).unwrap_or(""),
        limit,
        offset,
    ).await?;
    Ok((StatusCode::OK, Json(quizzes)))
}

pub async fn find_quiz_by_id(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<QuizDetailResponse>), AppError> {
    let quiz = quiz::find_quiz_by_id(&state.db, &id)
        .await?
        .ok_or(AppError::NotFound)?;

    let questions = quiz::find_quiz_questions(&state.db, &quiz.id).await?;
    let all_choices = quiz::find_quiz_choice_by_quiz_id(&state.db, &quiz.id).await?;

    let questions_response: Vec<QuizQuestionResponse> = questions.iter().map(|q| {
        let choices = all_choices.iter()
            .filter(|c| c.question_id == q.id)
            .map(|c| QuizChoiceResponse {
                id: c.id.clone(),
                content: c.content.clone(),
            })
            .collect();
        QuizQuestionResponse {
            id: q.id.clone(),
            question: q.question.clone(),
            type_: q.type_.clone(),
            target: q.target.clone(),
            order: q.order,
            choices,
        }
    }).collect();

    Ok((
        StatusCode::OK,
        Json(QuizDetailResponse {
            id: quiz.id.clone(),
            title: quiz.title.clone(),
            category_id: quiz.category_id.clone(),
            questions: questions_response,
        }),
    ))
}

pub async fn find_available_quizzes(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<(StatusCode, Json<Vec<Quiz>>), AppError> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    let quizzes = quiz::find_available_quizzes(
        &state.db,
        &auth_user.user.id,
        &auth_user.relationship.as_ref().map(|r| r.id.as_str()).unwrap_or(""),
        limit,
        offset,
    ).await?;
    Ok((StatusCode::OK, Json(quizzes)))
}

pub async fn find_quiz_history(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<(StatusCode, Json<Vec<Quiz>>), AppError> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    let quizzes = quiz::find_quiz_history(&state.db, &auth_user.user.id, limit, offset).await?;
    Ok((StatusCode::OK, Json(quizzes)))
}

pub async fn edit_quiz(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<QuizBody>,
) -> Result<(StatusCode, Json<Quiz>), AppError> {
    let quiz = quiz::edit_quiz(
        &state.db,
        quiz::NewQuiz {
            id,
            relationship_id: body.relationship_id,
            title: body.title,
            category_id: body.category_id,
        },
    ).await?;
    Ok((StatusCode::OK, Json(quiz)))
}

pub async fn delete_quiz(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, ()), AppError> {
    quiz::delete_quiz(&state.db, &id).await?;
    Ok((StatusCode::OK, ()))
}

pub async fn create_quiz_question(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(body): Json<quiz::NewQuizQuestion>,
) -> Result<(StatusCode, Json<quiz::QuizQuestion>), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let quiz_question = quiz::create_quiz_question(&state.db, &id, body).await?;
    Ok((StatusCode::OK, Json(quiz_question)))
}

pub async fn edit_quiz_question(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<quiz::NewQuizQuestion>,
) -> Result<(StatusCode, Json<quiz::QuizQuestion>), AppError> {
    let json = quiz::edit_quiz_question(
        &state.db,
        quiz::QuizQuestion {
            id,
            quiz_id: body.quiz_id,
            question: body.question,
            type_: body.type_,
            target: body.target,
            order: body.order,
        },
    ).await.map_err(|_| AppError::DatabaseError)?;

  Ok((StatusCode::OK, Json(json)))
}

pub async fn delete_quiz_question(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, ()), AppError> {
    quiz::delete_quiz_question(&state.db, &id).await.map_err(|_| AppError::DatabaseError)?;
    Ok((StatusCode::OK, ()))
}

pub async fn create_quiz_text_choice(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path((_quiz_id, question_id)): Path<(String, String)>,
    Json(body): Json<TextChoiceBody>,
) -> Result<(StatusCode, Json<quiz::QuizChoice>), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let choice = quiz::create_quiz_choice(
        &state.db,
        &id,
        quiz::NewQuizChoice {
            question_id,
            content: body.content,
        },
    ).await?;
    Ok((StatusCode::CREATED, Json(choice)))
}

pub async fn create_quiz_image_choice(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path((_quiz_id, question_id)): Path<(String, String)>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<quiz::QuizChoice>), AppError> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut content_type = String::new();

    while let Some(field) = multipart.next_field().await
        .map_err(|_| AppError::BadRequest("Invalid multipart".to_string()))?
    {
        if field.name() == Some("image") {
            content_type = field.content_type()
                .ok_or(AppError::InvalidFileType)?
                .to_string();
            file_bytes = Some(
                field.bytes().await
                    .map_err(|_| AppError::BadRequest("Failed to read image".to_string()))?
                    .to_vec(),
            );
        }
    }

    let bytes = file_bytes.ok_or(AppError::BadRequest("No image provided".to_string()))?;
    let url = upload::save_upload(bytes, &content_type, upload::UploadFolder::Quiz).await?;

    let id = uuid::Uuid::new_v4().to_string();
    let choice = quiz::create_quiz_choice(
        &state.db,
        &id,
        quiz::NewQuizChoice {
            question_id,
            content: url,
        },
    ).await?;

    Ok((StatusCode::CREATED, Json(choice)))
}

pub async fn answer_quiz(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
    Json(body): Json<Vec<QuizAnswerBody>>,
) -> Result<(StatusCode, Json<()>), AppError> {
    for answer_raw in body.iter() {
        let id = uuid::Uuid::new_v4().to_string();
        let quiz_answer = quiz::NewQuizAnswer {
            id,
            quiz_id: quiz_id.clone(),
            user_id: auth_user.user.id.clone(),
            choice_id: answer_raw.choice_id.clone(),
            question_id: answer_raw.question_id.clone(),
            custom_text: answer_raw.custom_text.clone(),
            date_answer: answer_raw.date_answer.clone(),
        };
        quiz::create_quiz_answer(&state.db, quiz_answer).await?;
    }

    Ok((StatusCode::OK, Json(())))
}