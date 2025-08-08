use crate::models::regency::Regency;
use crate::repository::regency::{get_regencies_by_province_id, search_regencies};
use anyhow::Result;
use sqlx::PgPool;

pub async fn fetch_regencies_by_province_id(
    pool: &PgPool,
    province_id: &str,
) -> Result<Vec<Regency>> {
    get_regencies_by_province_id(pool, province_id).await
}

pub async fn search_regencies_service(
    pool: &PgPool,
    province_id: &str,
    query: &str,
) -> Result<Vec<Regency>> {
    search_regencies(pool, province_id, query).await
}
