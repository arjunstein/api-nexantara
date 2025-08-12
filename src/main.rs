mod config;
mod db;
mod docs;
mod handlers;
mod middleware;
mod models;
mod repository;
mod services;

use actix_web::{App, HttpServer, web};
use config::Config;
use db::init_db;
use docs::api_docs::ApiDoc;
use middleware::api_key::ApiKeyAuth;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool: PgPool = init_db(&config).await;

    println!("🚀 Server running...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", ApiDoc::openapi()))
            .service(
                web::scope("/api/v1")
                    .wrap(ApiKeyAuth::new(config.api_key.clone()))
                    .service(handlers::province::get_all_provinces_with_search)
                    .service(handlers::regency::get_regencies_by_province_id_with_search)
                    .service(handlers::district::get_districts_by_regency_id_with_search)
                    .service(handlers::village::get_villages_by_district_id_with_search),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
