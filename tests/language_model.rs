use ai_sdk::language_model::{self, Message, LanguageModel};

#[tokio::test]
async fn test_generate_text() {
    let model = language_model::dummy();
    let response = model.generate_text(
        "system prompt".to_string(),
        "user prompt".to_string(),
        vec![Message {
            role: "user".to_string(),
            content: "test message".to_string(),
        }],
    ).await;

    assert_eq!(response, "This is a dummy response.");
}

#[tokio::test]
async fn test_embed() {
    let model = language_model::dummy();
    let embedding = model.embed("test input".to_string()).await;
    
    assert_eq!(embedding, vec![0.1, 0.2, 0.3]);
}

#[tokio::test]
async fn test_embed_many() {
    let model = language_model::dummy();
    let embeddings = model.embed_many(vec![
        "test input 1".to_string(),
        "test input 2".to_string(),
    ]).await;
    
    assert_eq!(embeddings, vec![
        vec![0.1, 0.2, 0.3],
        vec![0.1, 0.2, 0.3],
    ]);
}

#[tokio::test]
async fn test_stream_text() {
    use futures::StreamExt;
    
    let model = language_model::dummy();
    let mut stream = model.stream_text(
        "system prompt".to_string(),
        "user prompt".to_string(),
        vec![],
    ).await;
    
    let mut result = String::new();
    while let Some(response) = stream.next().await {
        result.push_str(&response);
    }
    
    assert_eq!(result, "This is the first part of the response... then comes the second part... and finally the conclusion.");
} 