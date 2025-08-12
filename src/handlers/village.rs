use crate::services::village::{fetch_villages_by_district_id, search_villages_service};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
}

#[utoipa::path(
    get,
    path = "/districts/{district_id}/villages",
    params(
        ("X-API-KEY" = String, Header, description = "x-api-key token"),
        ("district_id" = String, Path, description = "District ID"),
        ("search" = Option<String>, Query, description = "Search villages by name (optional)")
    ),
    responses(
        (status = 200, description = "Ok", body = [Village]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Villages"
)]
#[get("/districts/{district_id}/villages")]
pub async fn get_villages_by_district_id_with_search(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let district_id = path.into_inner();
    if let Some(search_query) = &query.search {
        match search_villages_service(&pool, &district_id, search_query).await {
            Ok(villages) => return HttpResponse::Ok().json(villages),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
    match fetch_villages_by_district_id(&pool, &district_id).await {
        Ok(villages) => HttpResponse::Ok().json(villages),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
