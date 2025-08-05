use actix_web::{get, web, HttpResponse, Responder};
use crate::services::district::{fetch_districts_by_regency_id};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/api/v1/regencies/{regency_id}/districts")]
pub async fn get_districts_by_regency_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let regency_id = path.into_inner();
    match fetch_districts_by_regency_id(&pool, regency_id).await {
        Ok(districts) => HttpResponse::Ok().json(districts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}