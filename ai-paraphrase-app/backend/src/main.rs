use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, http::header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

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
async fn paraphrase(body: web::Json<Input>, api_key: web::Data<String>) -> impl Responder {
    let client = Client::new();

    let prompt = format!(
        "Paraphrase the following text into one clear, high-quality sentence. Do not provide multiple options, explanations, or commentary. Output ONLY the paraphrased sentence:\n\n{}",
        body.text
    );

    let request_body = OpenAIRequest {
        model: "mistralai/mistral-7b-instruct:free".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let res = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key.get_ref()))
        .header("HTTP-Referer", "https://task-hye8.vercel.app")
        .json(&request_body)
        .send()
        .await;

    match res {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(json) => {
                let raw_output = json["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("Sorry, no paraphrased result.");

                let first_sentence = raw_output
                    .split_terminator(['.', '\n', '!', '?'])
                    .next()
                    .unwrap_or(raw_output)
                    .trim();

                let clean_output = if first_sentence.ends_with(['.', '!', '?']) {
                    first_sentence.to_string()
                } else {
                    format!("{}.", first_sentence)
                };

                HttpResponse::Ok().json(serde_json::json!({ "result": clean_output }))
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to parse AI response"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to reach AI provider"),
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl Fn(&mut web::ServiceConfig) + Clone + Send + 'static> {
    let api_key = secrets
        .get("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not found");

    let api_key_data = web::Data::new(api_key);

    let factory = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .allowed_origin("https://task-hye8.vercel.app") // Only allow Vercel
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::ORIGIN,
                header::AUTHORIZATION,
            ])
            .max_age(3600);

        cfg.app_data(api_key_data.clone())
            .wrap(cors)
            .service(paraphrase);
    };

    Ok(factory.into())
}
