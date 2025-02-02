use async_trait::async_trait;
use futures::Stream;
use crate::language_model::types::{LanguageModel, Message};
use std::pin::Pin;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAiLanguageModel {
    model: String,
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
}

#[derive(Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

impl OpenAiLanguageModel {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            client: Client::new(),
            api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
        }
    }
}

#[async_trait]
impl LanguageModel for OpenAiLanguageModel {
    async fn generate_text(&self, system: String, prompt: String, messages: Vec<Message>) -> String {
        let request = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![
                OpenAiMessage { role: "system".to_string(), content: system },
                OpenAiMessage { role: "user".to_string(), content: prompt },
            ],
        };

        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await.unwrap()
            .json::<OpenAiResponse>()
            .await.unwrap();

        response.choices[0].message.content.clone()
    }

    async fn stream_text(&self, system: String, prompt: String, messages: Vec<Message>) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        // Implementation here
        todo!()
    }

    async fn embed(&self, input: String) -> Vec<f32> {
        // Implementation here
        todo!()
    }

    async fn embed_many(&self, inputs: Vec<String>) -> Vec<Vec<f32>> {
        // Implementation here
        todo!()
    }
}
