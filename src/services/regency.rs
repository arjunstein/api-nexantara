use crate::models::regency::Regency;
use crate::repository::regency::{get_regency_by_province_id};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_regency_by_province_id(pool: &PgPool, province_id: Uuid) -> Result<Vec<Regency>> {
    get_regency_by_province_id(pool, province_id).await
}