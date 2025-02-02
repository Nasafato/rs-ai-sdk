pub mod openai;
pub mod types;
pub mod dummy;

use openai::OpenAiLanguageModel;
use dummy::DummyLanguageModel;
pub use types::{LanguageModel, Message};

pub fn openai(model: &str) -> OpenAiLanguageModel {
    OpenAiLanguageModel::new(model)
}

pub fn dummy() -> DummyLanguageModel {
    DummyLanguageModel::new()
}

// Future providers could be added like:
// pub fn anthropic(model: &str) -> AnthropicModel { ... }
// pub fn google(model: &str) -> GoogleModel { ... }