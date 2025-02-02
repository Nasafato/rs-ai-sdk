pub mod openai;
pub mod types;
pub mod dummy;
pub mod huggingface;
pub mod huggingface_local;

use openai::OpenAiLanguageModel;
use dummy::DummyLanguageModel;
use huggingface::HuggingFaceModel;
use huggingface_local::LocalHuggingFaceModel;
pub use types::{LanguageModel, Message};

pub fn openai(model: &str) -> OpenAiLanguageModel {
    OpenAiLanguageModel::new(model)
}

pub fn dummy() -> DummyLanguageModel {
    DummyLanguageModel::new()
}

pub fn huggingface(model: &str) -> HuggingFaceModel {
    HuggingFaceModel::new(model)
}

pub async fn huggingface_local(url: &str) -> LocalHuggingFaceModel {
    LocalHuggingFaceModel::new_from_huggingface_url(url).await.unwrap()
}

// pub fn huggingface_local_from_exported(model_name: &str) -> LocalHuggingFaceModel {
//     LocalHuggingFaceModel::new_from_exported(model_name)
// }

// pub async fn huggingface_snowflake() -> LocalHuggingFaceModel {
//     LocalHuggingFaceModel::new_snowflake().await
// }

// Future providers could be added like:
// pub fn anthropic(model: &str) -> AnthropicModel { ... }
// pub fn google(model: &str) -> GoogleModel { ... }