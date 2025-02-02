use ai_sdk::language_model::{self, Message, LanguageModel};
#[tokio::test]
async fn test_local_huggingface() {
    use ai_sdk::language_model::huggingface_local;
    
    let model = huggingface_local("https://huggingface.co/Xenova/GIST-small-Embedding-v0/resolve/main/onnx/model_int8.onnx").await;
    let embeddings = model.embed("test input".to_string()).await;
    
    // Since we don't know the exact values, let's just verify the structure
    assert!(embeddings.len() > 0);
    // Optionally check if values are within expected range
    for value in embeddings.iter() {
        assert!(*value >= -1.0 && *value <= 1.0);
    }
} 