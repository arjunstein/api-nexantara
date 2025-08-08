use crate::services::regency::{fetch_regencies_by_province_id, search_regencies_service};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
}

#[get("/api/v1/provinces/{province_id}/regencies")]
pub async fn get_regencies_by_province_id_with_search(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let province_id = path.into_inner();
    if let Some(search_query) = &query.search {
        match search_regencies_service(&pool, &province_id, search_query).await {
            Ok(regencies) => return HttpResponse::Ok().json(regencies),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
    match fetch_regencies_by_province_id(&pool, &province_id).await {
        Ok(regencies) => HttpResponse::Ok().json(regencies),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
