use actix_web::{web, HttpResponse, Responder};
use crate::models::ChatRequest;
use crate::services::{send_chat_to_ollama, get_ollama_models};
use crate::errors::ServiceError;

pub async fn chat_handler(
    request: web::Json<ChatRequest>
) -> Result<HttpResponse, ServiceError> {
    let chat_response = send_chat_to_ollama(&request.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(chat_response))
}

pub async fn list_models() -> Result<HttpResponse, ServiceError> {
    let models = get_ollama_models().await?;
    
    Ok(HttpResponse::Ok().json(models))
}