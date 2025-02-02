pub mod openai;
pub mod types;

use openai::OpenAiLanguageModel;
pub use types::{LanguageModel, Message};

pub fn openai(model: &str) -> OpenAiLanguageModel {
    OpenAiLanguageModel::new(model)
}

// Future providers could be added like:
// pub fn anthropic(model: &str) -> AnthropicModel { ... }
// pub fn google(model: &str) -> GoogleModel { ... }