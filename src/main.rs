use ai_sdk::{generate_text, language_model::{self, Message}};
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("secrets.toml"))
        .build()
        .unwrap();

    let openai_api_key = config.get::<String>("OPENAI_API_KEY").unwrap();
    std::env::set_var("OPENAI_API_KEY", openai_api_key);

    let model = language_model::openai("gpt-4o-mini");
    let response = generate_text(
        Box::new(model),
        "You are a helpful assistant.".to_string(),
        "What is the capital of France?".to_string(),
        vec![Message {
            role: "user".to_string(),
            content: "What is the capital of France?".to_string()
        }]
    ).await;

    println!("{}", response);
}
