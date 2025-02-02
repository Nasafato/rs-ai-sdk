pub mod language_model;

use language_model::types::LanguageModel;
use language_model::types::Message;

pub async fn generate_text(
    model: Box<dyn LanguageModel>,
    system: String,
    prompt: String,
    messages: Vec<Message>,
) -> String {
    model.generate_text(system, prompt, messages).await
}
