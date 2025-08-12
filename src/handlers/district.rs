use crate::services::district::{fetch_districts_by_regency_id, search_district_service};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
}

#[utoipa::path(
    get,
    path = "/regencies/{regency_id}/districts",
    params(
        ("X-API-KEY" = String, Header, description = "x-api-key token"),
        ("regency_id" = String, Path, description = "Regency ID"),
        ("search" = Option<String>, Query, description = "Search districts by name (optional)")
    ),
    responses(
        (status = 200, description = "Ok", body = [District]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Districts"
)]
#[get("/regencies/{regency_id}/districts")]
pub async fn get_districts_by_regency_id_with_search(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let regency_id = path.into_inner();
    if let Some(search_query) = &query.search {
        match search_district_service(&pool, &regency_id, search_query).await {
            Ok(districts) => return HttpResponse::Ok().json(districts),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
    match fetch_districts_by_regency_id(&pool, &regency_id).await {
        Ok(districts) => HttpResponse::Ok().json(districts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
