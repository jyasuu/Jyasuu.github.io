use crate::models::{ChatRequest, ChatResponse};
use crate::errors::ServiceError;
use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn send_chat_to_ollama(
    request: &ChatRequest
) -> Result<ChatResponse, ServiceError> {
    // Retrieve Ollama API URL from environment, with a default fallback
    let ollama_api_url = env::var("OLLAMA_API_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());

    let client = Client::new();
    
    let ollama_payload = json!({
        "model": request.model,
        "prompt": request.message,
        "stream": false
    });

    let response = client
        .post(format!("{}/api/generate", ollama_api_url))
        .json(&ollama_payload)
        .send()
        .await
        .map_err(|e| ServiceError::OllamaApiError(e.to_string()))?;

    let chat_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| ServiceError::OllamaApiError(e.to_string()))?;

    Ok(ChatResponse {
        message: chat_response["response"]
            .as_str()
            .unwrap_or("No response")
            .to_string(),
        model: request.model.clone()
    })
}

pub async fn get_ollama_models() -> Result<Vec<String>, ServiceError> {
    // Retrieve Ollama API URL from environment, with a default fallback
    let ollama_api_url = env::var("OLLAMA_API_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());

    let client = Client::new();
    
    let response = client
        .get(format!("{}/api/tags", ollama_api_url))
        .send()
        .await
        .map_err(|e| ServiceError::OllamaApiError(e.to_string()))?;

    let models: serde_json::Value = response
        .json()
        .await
        .map_err(|e| ServiceError::OllamaApiError(e.to_string()))?;

    let model_names = models["models"]
        .as_array()
        .map(|m| 
            m.iter()
                .filter_map(|model| model["name"].as_str().map(|s| s.to_string()))
                .collect()
        )
        .unwrap_or_default();

    Ok(model_names)
}