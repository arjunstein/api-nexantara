use actix_web::{get, web, HttpResponse, Responder};
use crate::services::village::{fetch_villages_by_district_id};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/api/v1/districts/{district_id}/villages")]
pub async fn get_villages_by_district_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let district_id = path.into_inner();
    match fetch_villages_by_district_id(&pool, district_id).await {
        Ok(villages) => HttpResponse::Ok().json(villages),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
