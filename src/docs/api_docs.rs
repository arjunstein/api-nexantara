// src/docs/api_doc.rs
use crate::models::province::Province;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::province::get_all_provinces_with_search,
    ),
    components(schemas(Province)),
    tags(
        (name = "Provinces")
    ),
)]
pub struct ApiDoc;
