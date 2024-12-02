use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {

    #[error("Ollama API error: {0}")]
    OllamaApiError(String),

    #[error("XAI API error: {0}")]
    XaiApiError(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::OllamaApiError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": msg
            })),
            ServiceError::XaiApiError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": msg
            })),
        }
    }
}
