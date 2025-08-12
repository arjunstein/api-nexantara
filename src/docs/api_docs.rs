// src/docs/api_doc.rs
use crate::models::{district::District, province::Province, regency::Regency, village::Village};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::province::get_all_provinces_with_search,
        crate::handlers::regency::get_regencies_by_province_id_with_search,
        crate::handlers::district::get_districts_by_regency_id_with_search,
        crate::handlers::village::get_villages_by_district_id_with_search
    ),
    components(
        schemas(
            Province,
            Regency,
            District,
            Village
        )
    ),
    tags(
        (name = "Provinces", description = "Get All Provinces"),
        (name = "Regencies", description = "Get Regencies by Province ID"),
        (name = "Districts", description = "Get Districts by Regency ID"),
        (name = "Villages", description = "Get Villages by District ID"),
    ),
    servers(
        (url = "/api/v1", description = "Base path")
    )
)]
pub struct ApiDoc;
