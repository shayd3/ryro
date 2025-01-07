use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};

// Models
#[derive(Deserialize)]
struct CreateShortUrlRequest {
    long_url: String,
}

#[derive(Serialize)]
struct ShortUrl {
    slug: String,
    long_url: String,
    creation_date: DateTime<Utc>,
}

fn generate_slug() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

// Request handlers
#[post("/url")]
async fn create_short_url(data: web::Json<CreateShortUrlRequest>) -> impl Responder {
    let slug = generate_slug();
    let short_url = ShortUrl {
        slug: slug,
        long_url: data.long_url.clone(),
        creation_date: Utc::now(),
    };
    HttpResponse::Ok().json(short_url)
}

#[get("/url/{short_url}")]
async fn get_short_url() -> impl Responder {
    HttpResponse::Ok().body("you just called get_short_url()")
}

#[get("/urls")]
async fn get_urls() -> impl Responder {
    HttpResponse::Ok().body("you just called get_urls()")
}



// Create App to register and serve the request handlers
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_short_url)
            .service(get_short_url)
            .service(get_urls)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
