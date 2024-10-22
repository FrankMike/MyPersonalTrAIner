use actix_web::{
    get, post, App, HttpResponse, HttpServer, Responder
};
use langchain_rust::{
    language_models::llm::LLM,
    llm::{
        claude::Claude, ollama::client::Ollama, openai::OpenAI, OpenAIConfig
    }
};
use dotenv::dotenv;
use std::env;
use serde_json::json;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/chat_claude")]
async fn chat_claude(prompt: String) -> impl Responder {
    dotenv().ok();
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
    
    let llm = Claude::default().with_api_key(api_key);
    
    let response = llm.invoke(&prompt).await;   


    match response {
        Ok(response) => {
            HttpResponse::Ok().json(json!({ "response": response }))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({ "error": format!("LLM generation failed: {e}") }))
        }
    }
}

#[post("/chat_gpt")]
async fn chat_gpt(prompt: String) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let open_ai = OpenAI::default().with_config(
        OpenAIConfig::default()
            .with_api_key(api_key),
    );

    let response = open_ai.invoke(&prompt).await;
    
    match response {
        Ok(response) => {
            HttpResponse::Ok().json(json!({ "response": response }))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({ "error": format!("LLM generation failed: {}", e) }))
        }
    }
}

#[post("/chat_ollama")]
async fn chat_ollama(prompt: String) -> impl Responder {
    let ollama = Ollama::default().with_model("llama3.2");
    let response = ollama.invoke(&prompt).await;

    match response {
        Ok(response) => {
            HttpResponse::Ok().json(json!({ "response": response }))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({ "error": format!("LLM generation failed: {}", e) }))
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(chat_claude)
            .service(chat_gpt)
            .service(chat_ollama)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use serde_json::Value;

    #[actix_web::test]
    async fn test_hello_endpoint() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, world!");
    }

    #[actix_web::test]
    async fn test_chat_claude_endpoint() {
        let app = test::init_service(App::new().service(chat_claude)).await;
        let req = test::TestRequest::post()
            .uri("/chat_claude")
            .set_payload("Test prompt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: Value = test::read_body_json(resp).await;
        assert!(body.get("response").is_some());
    }

    #[actix_web::test]
    async fn test_chat_gpt_endpoint() {
        let app = test::init_service(App::new().service(chat_gpt)).await;
        let req = test::TestRequest::post()
            .uri("/chat_gpt")
            .set_payload("Test prompt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: Value = test::read_body_json(resp).await;
        assert!(body.get("response").is_some());
    }

    #[actix_web::test]
    async fn test_chat_ollama_endpoint() {
        let app = test::init_service(App::new().service(chat_ollama)).await;
        let req = test::TestRequest::post()
            .uri("/chat_ollama")
            .set_payload("Test prompt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: Value = test::read_body_json(resp).await;
        assert!(body.get("response").is_some());
    }
}
