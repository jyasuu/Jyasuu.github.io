use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Invalid request")]
    BadRequest,

    #[error("Ollama API error: {0}")]
    OllamaApiError(String),

    #[error("XAI API error: {0}")]
    XaiApiError(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest => HttpResponse::BadRequest().json(json!({
                "error": "Invalid request"
            })),
            ServiceError::OllamaApiError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": msg
            })),
            ServiceError::XaiApiError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": msg
            })),
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }
}
