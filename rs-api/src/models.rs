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


pub mod xai
{
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChatRequest {
        pub messages: Vec<Message>,
        pub model: String,
        pub stream: bool,
        pub temperature: f64,
    }

    impl ChatRequest
    {
        pub fn from_msg(msg : String) -> Self
        {
            Self{
                messages: vec![Message::helper(),Message::from_msg(msg)],
                model: String::from("grok-beta"),
                stream: false,
                temperature: 0.0,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    impl Message
    {
        pub fn helper() -> Self
        {
            Self
            {
                role: String::from("system"),
                content: String::from("You are a customer service staff."),
            }
        }

        pub fn from_msg(msg : String) -> Self
        {
            Self
            {
                role: String::from("user"),
                content: String::from(&msg),
            }
        }

    }


    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChatResponse {
        pub id: String,
        pub object: String,
        pub created: u64,
        pub model: String,
        pub choices: Vec<Choice>,
        pub usage: Usage,
        pub system_fingerprint: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Choice {
        pub index: u32,
        pub message: AssistantMessage,
        pub finish_reason: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssistantMessage {
        pub role: String,
        pub content: String,
        pub refusal: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Usage {
        pub prompt_tokens: u32,
        pub completion_tokens: u32,
        pub total_tokens: u32,
        pub prompt_tokens_details: PromptTokensDetails,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PromptTokensDetails {
        pub text_tokens: u32,
        pub audio_tokens: u32,
        pub image_tokens: u32,
        pub cached_tokens: u32,
    }


}




impl ChatResponse {
    pub fn from_xai(res: crate::models::xai::ChatResponse) -> Self
    {
        Self{
            message: res.choices[0].message.content.clone(),
            model: res.model,
        }
    }
}