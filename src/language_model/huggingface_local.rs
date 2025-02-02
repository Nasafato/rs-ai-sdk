use async_trait::async_trait;
use tokenizers::{Tokenizer}
use futures::{Stream, stream};
use std::pin::Pin;
use tokenizers::Tokenizer;
use crate::language_model::types::{LanguageModel, Message};
use std::path::Path;
use std::fs;
use reqwest::Client as HttpClient;
use tokio::io::AsyncWriteExt;
use ort::session::{builder::GraphOptimizationLevel, Session};
use ort::value::{Tensor};
use dirs;
 

    pub async fn get_or_download_model(url: &str) -> std::io::Result<String> {
        // Parse the URL to create cache path
        let url_parts: Vec<&str> = url.split("/").collect();
        let org_name = url_parts[3];
        let model_name = url_parts[4];
        let file_name = url_parts.last().unwrap();

        // Create cache directory path
        let cache_dir = dirs::home_dir()
            .unwrap()
            .join(".cache")
            .join("ai-sdk")
            .join("huggingface")
            .join(org_name)
            .join(model_name);

        // Create full file path
        let file_path = cache_dir.join(file_name);

        // If file exists, return its path
        if file_path.exists() {
            return Ok(file_path.to_str().unwrap().to_string());
        }

        // Create directories if they don't exist
        fs::create_dir_all(&cache_dir)?;

        // Download the file
        let client = HttpClient::new();
        let response = client.get(url)
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut file = tokio::fs::File::create(&file_path)
            .await?;

        let content = response.bytes()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        file.write_all(&content)
            .await?;

        Ok(file_path.to_str().unwrap().to_string())
    }


pub struct LocalHuggingFaceModel {
    session: Session,
}

impl LocalHuggingFaceModel {

    pub async fn new_from_huggingface_url(url: &str) -> Result<Self, ort::Error> {
        let model_path = get_or_download_model(url).await.unwrap();
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(model_path)?;

        Ok(LocalHuggingFaceModel::from_session(session))
    }

    async fn encode(&self, text: &str) -> Result<Vec<i64>, ort::Error> {
        // Create a single-element vector with your input text
        let data = vec![text.to_string()];
        // Create tensor with shape [1] (batch size of 1) and your input
        let value = Tensor::from_string_array(([1], data.into_boxed_slice()))?;
        let inputs = ort::inputs![value]?;
        let outputs = self.session.run(inputs)?;
        let first = &outputs[0];
        
        // Extract the tensor values
        let embedding = first.try_extract_string_tensor()?;
        Ok(embedding.iter()
            .map(|s| s.parse::<i64>().unwrap_or(0))
            .collect::<Vec<i64>>())
    }

    fn from_session(session: Session) -> Self {
        Self { session }
    }
}

#[async_trait]
impl LanguageModel for LocalHuggingFaceModel {
    async fn generate_text(&self, system: String, prompt: String, _messages: Vec<Message>) -> String {
        todo!("Text generation requires more complex handling with local models")
    }

    async fn stream_text(&self, _system: String, _prompt: String, _messages: Vec<Message>) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        todo!("Streaming not implemented for local models")
    }

    async fn embed(&self, input: String) -> Vec<f32> {
        let input_ids = self.encode(&input).await.unwrap()
            .into_iter()
            .map(|x| x as f32)
            .collect::<Vec<f32>>();
        input_ids
    }

    async fn embed_many(&self, inputs: Vec<String>) -> Vec<Vec<f32>> {
        todo!()
        // inputs.into_iter()
        //     .map(|input| self.embed(input))
        //     .collect::<futures::future::join_all>()
        //     .await
    }
} 