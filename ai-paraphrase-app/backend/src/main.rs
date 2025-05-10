use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, Responder, HttpResponse, http::header};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Deserialize)]
struct Input {
    text: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[post("/paraphrase")]
async fn paraphrase(body: web::Json<Input>) -> impl Responder {
    let api_key = std::env::var("OPENAI_API_KEY").expect("API key missing");
    let client = Client::new();

    let prompt = format!("Paraphrase this: {}", body.text);

    let request_body = OpenAIRequest {
        model: "mistralai/mistral-7b-instruct:free".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let res = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "http://localhost:5173") // Update to deployed frontend URL later
        .json(&request_body)
        .send()
        .await;

    match res {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(json) => {
                    let output = json["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("Sorry, no paraphrased result.")
                        .to_string();
                    HttpResponse::Ok().json(serde_json::json!({ "result": output }))
                },
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse AI response"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to reach AI provider"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Allow your dev frontend
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(paraphrase)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
