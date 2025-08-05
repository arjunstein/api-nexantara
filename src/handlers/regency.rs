use actix_web::{get, web, HttpResponse, Responder};
use crate::services::regency::{fetch_regencies_by_province_id};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/api/v1/provinces/{province_id}/regencies")]
pub async fn get_regencies_by_province_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let province_id = path.into_inner();
    match fetch_regencies_by_province_id(&pool, province_id).await {
        Ok(regencies) => HttpResponse::Ok().json(regencies),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}