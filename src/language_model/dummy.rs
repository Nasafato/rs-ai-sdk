use async_trait::async_trait;
use futures::{Stream, stream};
use std::pin::Pin;
use crate::language_model::types::{LanguageModel, Message};

pub struct DummyLanguageModel;

impl DummyLanguageModel {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LanguageModel for DummyLanguageModel {
    async fn generate_text(&self, _system: String, _prompt: String, _messages: Vec<Message>) -> String {
        "This is a dummy response.".to_string()
    }

    async fn stream_text(&self, _system: String, _prompt: String, _messages: Vec<Message>) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        let responses = vec![
            "This is the first part of the response...",
            " then comes the second part...",
            " and finally the conclusion.",
        ].into_iter().map(String::from);
        
        Box::pin(stream::iter(responses))
    }

    async fn embed(&self, _input: String) -> Vec<f32> {
        vec![0.1, 0.2, 0.3]
    }

    async fn embed_many(&self, inputs: Vec<String>) -> Vec<Vec<f32>> {
        inputs.iter().map(|_| vec![0.1, 0.2, 0.3]).collect()
    }
} 