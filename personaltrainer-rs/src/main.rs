use reqwest;
use serde::{Deserialize, Serialize};
use tokio;
use std::env;
use dotenv::dotenv;

mod models;
mod utils;


use utils::get_user_input;
use models::{User, Gender, FitnessLevel, FitnessGoal};


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

    // Get API endpoint and model name from environment variable
    let api_endpoint = env::var("OLLAMA_API_ENDPOINT").expect("OLLAMA_API_ENDPOINT must be set");
    let model_name = env::var("OLLAMA_MODEL").expect("OLLAMA_MODEL must be set");

    // Build user object
    let name = get_user_input("\nEnter your first name: ");
    let surname = get_user_input("Enter your last name: ");
    let age = get_user_input("Enter your age: ").parse::<u8>().unwrap_or(0);
    let height = get_user_input("Enter your height in cm: ").parse::<f32>().unwrap_or(0.0);
    let weight = get_user_input("Enter your weight in kg: ").parse::<f32>().unwrap_or(0.0);
    
    println!("\nSelect your gender:");
    println!("1. Male");
    println!("2. Female");
    let gender = match get_user_input("Enter number (1-2): ").as_str() {
        "1" => Gender::Male,
        "2" => Gender::Female,
        _ => Gender::Male,
    };

    println!("\nSelect your fitness level:");
    println!("1. Beginner");
    println!("2. Intermediate");
    println!("3. Advanced");
    let fitness_level = match get_user_input("Enter number (1-3): ").as_str() {
        "1" => FitnessLevel::Beginner,
        "2" => FitnessLevel::Intermediate,
        "3" => FitnessLevel::Advanced,
        _ => FitnessLevel::Beginner,
    };

    let physical_limitations = get_user_input("Enter any physical limitations (or press Enter if none): ");
    let time_available = get_user_input("How many hours per week can you dedicate to exercise? ").parse::<f32>().unwrap_or(3.0);
    let equipment = get_user_input("What equipment do you have access to? (press Enter if none) ");
    // let fitness_goal = get_user_input("What is your fitness goal? (press Enter if none)");
    let fitness_goal = match get_user_input("\n
    1. Weight loss
    2. Muscle gain
    3. Endurance
    4. Flexibility
    5. Balance
    6. Other
    Enter number (1-6): ").as_str() {
        "1" => FitnessGoal::WeightLoss,
        "2" => FitnessGoal::MuscleGain,
        "3" => FitnessGoal::Endurance,
        "4" => FitnessGoal::Flexibility,
        "5" => FitnessGoal::Balance,
        "6" => FitnessGoal::Other,
        _ => FitnessGoal::Other,
    };
    

    let user = User::new(
        name,
        surname,
        age,
        height,
        weight,
        gender,
        fitness_level,
        physical_limitations,
        time_available,
        equipment,
        fitness_goal,
    );

    let user_prompt = format!("I want you to act as a personal trainer. 
    I will provide you with all the information needed about me, because I am looking to become fitter, 
    stronger and healthier through physical training, and your role is to devise the best plan for me 
    depending on my current fitness level, goals and lifestyle habits. 
    You should use your knowledge of exercise science, nutrition advice, 
    and other relevant factors in order to create a plan suitable for me. 
    My name is {} {}, I'm {} years old, {}cm tall, I weights {}kg. 
    My fitness level is {}. I have {} as a physical limitations, {} hours a week available, and {} equipment.", 
    user.name, user.surname, user.age, user.height, user.weight, user.fitness_level.to_string(), user.physical_limitations, user.time_available, user.equipment);


    // Prepare the request payload
    let request = GenerateRequest {
        model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| model_name),
        prompt: user_prompt,
        stream: false,
    };

    println!("\nWaiting for the Personal TrAIner to generate a plan...\n");

    // Send POST request to Ollama API
    let response = client
        .post(&api_endpoint)
        .json(&request)
        .send()
        .await?;

    // Parse and print the response
    let result: GenerateResponse = response.json().await?;
    
    if let Some(response_text) = &result.response {
        println!("\nResponse: {}", response_text);
    } else {
        eprintln!("No response field in the API result");
    }

    Ok(())
}
