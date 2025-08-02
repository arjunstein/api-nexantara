mod config;
mod db;
mod handlers;
mod services;
mod models;
mod repository;

use actix_web::{App, HttpServer, web};
use handlers::province::{get_all, get_by_id};
use config::Config;
use db::init_db;
use sqlx::PgPool;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool: PgPool = init_db(&config).await;

    println!("🚀 Server running at http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(get_all)
        .service(get_by_id)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
