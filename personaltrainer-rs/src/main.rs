use reqwest;
use serde::{Deserialize, Serialize};
use tokio;
use std::env;

mod models;

use models::User;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct GenerateResponse {
    response: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP client
    let client = reqwest::Client::new();

    // Get API endpoint and model name from environment variable or use default
    let api_endpoint = env::var("OLLAMA_API_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());
    let model_name = env::var("OLLAMA_MODEL")
        .unwrap_or_else(|_| "llama3.2".to_string());

    // Prepare the request payload
    let request = GenerateRequest {
        model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| model_name),
        prompt: "I would like to lose weight and gain muscle mass. What would you recommend?".to_string(),
        stream: false,
    };

    // Send POST request to Ollama API
    let response = client
        .post(&api_endpoint)
        .json(&request)
        .send()
        .await?;

    // Parse and print the response
    let result: GenerateResponse = response.json().await?;
    
    if let Some(response_text) = &result.response {
        println!("Response: {}", response_text);
    } else {
        println!("No response field in the API result");
    }

    Ok(())
}
