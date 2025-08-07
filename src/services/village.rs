use crate::models::village::Village;
use crate::repository::village::{get_villages_by_district_id, search_villages};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_villages_by_district_id(pool: &PgPool, district_id: Uuid) -> Result<Vec<Village>> {
    get_villages_by_district_id(pool, district_id).await
}

pub async fn search_villages_service(pool: &PgPool, district_id: Uuid, query: &str) -> Result<Vec<Village>> {
    search_villages(pool, district_id, query).await
}