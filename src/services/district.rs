use crate::models::district::District;
use crate::repository::district::{get_districts_by_regency_id, search_districts};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_districts_by_regency_id(pool: &PgPool, regency_id: Uuid) -> Result<Vec<District>> {
    get_districts_by_regency_id(pool, regency_id).await
}

pub async fn search_district_service(pool: &PgPool, regency_id: Uuid, query: &str) -> Result<Vec<District>> {
    search_districts(pool, regency_id, query).await
}