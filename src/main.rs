use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use url::Url;

mod utils;

struct AppState {
    pool: MySqlPool,
    blacklist: Vec<String>,
}

#[derive(Deserialize)]
struct CreateUrl {
    url: String,
}

#[derive(Serialize)]
struct ShortUrl {
    short: String,
}

#[derive(Serialize)]
struct ResponseError {
    error: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "https://docs.tuna2134.dev/api/shorten/"))
        .finish()
}

#[get("/{short}")]
async fn redirect(data: web::Data<AppState>, path: web::Path<(String,)>) -> impl Responder {
    let short = path.into_inner().0;
    let url = utils::get_url(&data.pool, short).await;
    match url {
        Ok(url) => {
            HttpResponse::Found()
                .append_header(("Location", url))
                .finish()
        }
        Err(_) => {
            HttpResponse::NotFound()
                .body("Not found")
        }
    }
}

#[post("/")]
async fn create_url(data: web::Data<AppState>, body: web::Json<CreateUrl>) -> impl Responder {
    let url = Url::parse(&body.url).unwrap();
    if url.scheme() != "https" {
        return HttpResponse::BadRequest().json(ResponseError { error: "Use HTTPS scheme".to_string() });
    }
    if data.blacklist.contains(&url.host_str().unwrap().to_string()) {
        return HttpResponse::BadRequest().json(ResponseError { error: "URL is blacklisted".to_string() });
    }
    if let Some(short) = utils::get_existed(&data.pool, url.to_string()).await {
        return HttpResponse::Ok().json(ShortUrl { short });
    }
    let short = utils::create_random();
    utils::create_short(&data.pool, short.clone(), url.to_string()).await.unwrap();
    HttpResponse::Ok().json(ShortUrl { short })
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await?;
    let blacklist = std::include_str!("../blacklist.txt")
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let app_state = web::Data::new(AppState { pool, blacklist });
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .service(create_url)
            .service(redirect)
            .service(index)
            .app_data(app_state.clone())
            .wrap(cors)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;
    Ok(())
}
