use reqwest;
use serde::{Deserialize, Serialize};
use tokio;
use std::env;
use dotenv::dotenv;

mod models;

use models::{User, Gender, FitnessLevel};


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
    
    // Load environment variables from .env file
    dotenv().ok();
    
    // Create an HTTP client
    let client = reqwest::Client::new();

    // Get API endpoint and model name from environment variable or use default
    let api_endpoint = env::var("OLLAMA_API_ENDPOINT").expect("OLLAMA_API_ENDPOINT must be set");
    let model_name = env::var("OLLAMA_MODEL").expect("OLLAMA_MODEL must be set");


    // Create a user
    let user = User::new(
        "John".to_string(),
        "Doe".to_string(),
        30,
        180.0,
        70.0,
        Gender::Male,
        FitnessLevel::Beginner,
        "".to_string(),
        3.0,
        "".to_string(),
    );


    let user_prompt = format!("I want you to act as a personal trainer. 
    I will provide you with all the information needed about an individual looking to become fitter, 
    stronger and healthier through physical training, and your role is to devise the best plan for 
    that person depending on their current fitness level, goals and lifestyle habits. 
    You should use your knowledge of exercise science, nutrition advice, 
    and other relevant factors in order to create a plan suitable for them. The person name is {} {}, he's {} years old, {}cm tall, he weights {}kg. 
    His fitness level is {}. 
    He has {} physical limitations, {} time available, and {} equipment.", user.name, user.surname, user.age, user.height, user.weight, user.fitness_level.to_string(), user.physical_limitations, user.time_available, user.equipment);


    // Prepare the request payload
    let request = GenerateRequest {
        model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| model_name),
        prompt: user_prompt,
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
        eprintln!("No response field in the API result");
    }

    Ok(())
}
