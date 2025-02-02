use std::process::Command;

fn main() {
    // Install optimum with exporters
    Command::new("pip")
        .args(["install", "optimum[exporters]"])
        .status()
        .expect("Failed to install optimum");

    // Export the model using optimum-cli
    // Here we're exporting an embedding model as an example
    Command::new("optimum-cli")
        .args([
            "export",
            "onnx",
            "--model",
            "sentence-transformers/all-MiniLM-L6-v2",  // Change this to your desired model
            "models/embeddings/",  // Output directory
        ])
        .status()
        .expect("Failed to export model");
} 