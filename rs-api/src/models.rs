use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default = "default_model")]
    pub model: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,
    pub model: String,
}

fn default_model() -> String {
    "llama2".to_string()
}