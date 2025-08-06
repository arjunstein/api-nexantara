use crate::models::regency::Regency;
use crate::repository::regency::{get_regencies_by_province_id, search_regencies};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_regencies_by_province_id(pool: &PgPool, province_id: Uuid) -> Result<Vec<Regency>> {
    get_regencies_by_province_id(pool, province_id).await
}

pub async fn search_regencies_service(pool: &PgPool, province_id: Uuid, query: &str) -> Result<Vec<Regency>> {
    search_regencies(pool, province_id, query).await
}