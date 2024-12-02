use actix_web::{web, HttpResponse};
use crate::models::{ChatRequest, ChatResponse};
use crate::services::{get_ollama_models, send_chat_to_ollama, send_chat_to_xai};
use crate::errors::ServiceError;


pub async fn ollama_chat_handler(
    request: web::Json<ChatRequest>
) -> Result<HttpResponse, ServiceError> {
    let chat_response = send_chat_to_ollama(&request.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(chat_response))
}

pub async fn xai_chat_handler(
    request: web::Json<ChatRequest>
) -> Result<HttpResponse, ServiceError> {
    let req = crate::models::xai::ChatRequest::from_msg(request.message.clone());
    let chat_response = send_chat_to_xai(&req).await?;
    let res  = ChatResponse::from_xai(chat_response);

    
    Ok(HttpResponse::Ok().json(res))
}

pub async fn list_models() -> Result<HttpResponse, ServiceError> {
    let models = get_ollama_models().await?;
    
    Ok(HttpResponse::Ok().json(models))
}