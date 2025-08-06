use actix_web::{get, web, HttpResponse, Responder};
use crate::services::province::{fetch_all_provinces, fetch_province_by_id, search_provinces_service};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
}

#[get("/api/v1/provinces")]
pub async fn get_all(pool: web::Data<PgPool>, query: web::Query<QueryParams>) -> impl Responder {
    if let Some(search_query) = &query.search {
        match search_provinces_service(&pool, search_query).await {
            Ok(provinces) => return HttpResponse::Ok().json(provinces),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    match fetch_all_provinces(&pool).await {
        Ok(provinces) => HttpResponse::Ok().json(provinces),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/v1/provinces/{id}")]
pub async fn get_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    match fetch_province_by_id(&pool, id).await {
        Ok(Some(province)) => HttpResponse::Ok().json(province),
        Ok(None) => HttpResponse::NotFound().body("Province not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}