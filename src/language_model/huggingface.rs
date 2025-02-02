use async_trait::async_trait;
use futures::{Stream, stream};
use std::pin::Pin;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::language_model::types::{LanguageModel, Message};

pub struct HuggingFaceModel {
    model: String,
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct TextGenerationRequest {
    inputs: String,
    parameters: TextGenerationParameters,
}

#[derive(Serialize)]
struct TextGenerationParameters {
    max_new_tokens: u32,
    return_full_text: bool,
}

#[derive(Serialize)]
struct EmbeddingRequest {
    inputs: Vec<String>,
}

#[derive(Deserialize)]
struct TextGenerationResponse {
    generated_text: String,
}

impl HuggingFaceModel {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            client: Client::new(),
            api_key: std::env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY must be set"),
        }
    }
}

#[async_trait]
impl LanguageModel for HuggingFaceModel {
    async fn generate_text(&self, system: String, prompt: String, _messages: Vec<Message>) -> String {
        let full_prompt = format!("{}\n{}", system, prompt);
        let request = TextGenerationRequest {
            inputs: full_prompt,
            parameters: TextGenerationParameters {
                max_new_tokens: 1000,
                return_full_text: false,
            },
        };

        let response = self.client
            .post(format!("https://api-inference.huggingface.co/models/{}", self.model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await.unwrap()
            .json::<Vec<TextGenerationResponse>>()
            .await.unwrap();

        response[0].generated_text.clone()
    }

    async fn stream_text(&self, _system: String, _prompt: String, _messages: Vec<Message>) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        // HuggingFace doesn't support streaming yet
        todo!()
    }

    async fn embed(&self, input: String) -> Vec<f32> {
        let request = EmbeddingRequest {
            inputs: vec![input],
        };

        self.client
            .post(format!("https://api-inference.huggingface.co/models/{}", self.model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await.unwrap()
            .json::<Vec<Vec<f32>>>()
            .await.unwrap()
            .remove(0)
    }

    async fn embed_many(&self, inputs: Vec<String>) -> Vec<Vec<f32>> {
        let request = EmbeddingRequest { inputs };

        self.client
            .post(format!("https://api-inference.huggingface.co/models/{}", self.model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap()
    }
} 