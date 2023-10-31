use crate::domains::namespace::NamespaceError;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Proxy {
    pub id: Uuid,
    pub slug: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateProxyData {
    pub slug: String,
}

pub type ProxyResult<R> = Result<R, ProxyError>;

#[derive(Debug, thiserror::Error)]
pub enum ProxyError {
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
}

impl From<sqlx::Error> for ProxyError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl From<NamespaceError> for ProxyError {
    fn from(value: NamespaceError) -> Self {
        todo!()
    }
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        todo!()
    }
}
