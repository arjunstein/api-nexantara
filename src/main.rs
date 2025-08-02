mod config;
mod db;

use actix_web::{get, App, HttpServer, Responder};
use config::Config;
use db::init_db;
use sqlx::PgPool;
use actix_web::web;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, from Nexantara API"
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool: PgPool = init_db(&config).await;

    println!("🚀 Server running at http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(index)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
