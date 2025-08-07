mod config;
mod db;
mod handlers;
mod services;
mod models;
mod repository;
mod middleware;

use actix_web::{App, HttpServer, web};
use config::Config;
use db::init_db;
use sqlx::PgPool;
use middleware::api_key::ApiKeyAuth;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool: PgPool = init_db(&config).await;

    println!("🚀 Server running at http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .wrap(ApiKeyAuth::new(config.api_key.clone()))
            .app_data(web::Data::new(pool.clone()))
            // Register all services
            .service(handlers::province::get_all_provinces_with_search)
            .service(handlers::regency::get_regencies_by_province_id_with_search)
            .service(handlers::district::get_districts_by_regency_id_with_search)
            .service(handlers::village::get_villages_by_district_id_with_search)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
