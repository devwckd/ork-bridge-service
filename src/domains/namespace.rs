use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Namespace {
    pub id: Uuid,
    pub slug: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateNamespaceData {
    pub slug: String,
}

pub type NamespaceResult<R> = Result<R, NamespaceError>;

#[derive(Debug, thiserror::Error)]
pub enum NamespaceError {
    #[error("namespace not found")]
    NotFound,
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
}

impl From<sqlx::Error> for NamespaceError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl IntoResponse for NamespaceError {
    fn into_response(self) -> Response {
        todo!()
    }
}
