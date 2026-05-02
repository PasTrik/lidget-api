use crate::errors::AppError;

pub enum UploadFolder {
  Quiz,
  Photos,
  Avatars,
}

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const ALLOWED_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

impl UploadFolder {
  pub fn as_str(&self) -> &str {
    match self {
      UploadFolder::Quiz => "quiz",
      UploadFolder::Photos => "photos",
      UploadFolder::Avatars => "avatars",
    }
  }
}

pub async fn save_upload(
  bytes: Vec<u8>,
  content_type: &str,
  folder: UploadFolder,
) -> Result<String, AppError> {
  if bytes.len() > MAX_FILE_SIZE {
    return Err(AppError::FileTooLarge);
  }

  if !ALLOWED_TYPES.contains(&content_type) {
    return Err(AppError::InvalidFileType);
  }

  let extension = match content_type {
    "image/jpeg" => "jpg",
    "image/png" => "png",
    "image/webp" => "webp",
    _ => return Err(AppError::InvalidFileType),
  };

  let filename = format!("{}.{}", uuid::Uuid::new_v4(), extension);
  let path = format!("uploads/{}/{}", folder.as_str(), filename);

  tokio::fs::create_dir_all(format!("uploads/{}", folder.as_str()))
    .await
    .map_err(|_| AppError::FileSaveError)?;

  tokio::fs::write(&path, bytes)
    .await
    .map_err(|_| AppError::FileSaveError)?;

  Ok(path)
}