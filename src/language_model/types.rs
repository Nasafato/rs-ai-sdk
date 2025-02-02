use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

pub struct Message {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait LanguageModel {
    async fn generate_text(&self, system: String, prompt: String, messages: Vec<Message>) -> String;
    async fn stream_text(&self, system: String, prompt: String, messages: Vec<Message>) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    async fn embed(&self, input: String) -> Vec<f32>;
    async fn embed_many(&self, inputs: Vec<String>) -> Vec<Vec<f32>>;
}


