use crate::services::province::{fetch_all_provinces, search_provinces_service};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/provinces",
    params(
        ("search" = Option<String>, Query, description = "Search provinces (optional)")
    ),
    responses(
        (status = 200, description = "List provinces", body = [Province]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Provinces"
)]
#[get("/provinces")]
pub async fn get_all_provinces_with_search(
    pool: web::Data<PgPool>,
    query: web::Query<QueryParams>,
) -> impl Responder {
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
