use actix_web::{get, web, HttpResponse, Responder};
use crate::services::district::{fetch_districts_by_regency_id, search_district_service};
use sqlx::PgPool;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
}

#[get("/api/v1/regencies/{regency_id}/districts")]
pub async fn get_districts_by_regency_id_with_search(pool: web::Data<PgPool>, path: web::Path<Uuid>, query: web::Query<QueryParams>) -> impl Responder {
    let regency_id = path.into_inner();
    if let Some(search_query) = &query.search {
        match search_district_service(&pool, regency_id, search_query).await {
            Ok(districts) => return HttpResponse::Ok().json(districts),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
    match fetch_districts_by_regency_id(&pool, regency_id).await {
        Ok(districts) => HttpResponse::Ok().json(districts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}