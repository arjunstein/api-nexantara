use crate::models::district::District;
use crate::repository::district::get_districts_by_regency_id;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_districts_by_regency_id(pool: &PgPool, regency_id: Uuid) -> Result<Vec<District>> {
    get_districts_by_regency_id(pool, regency_id).await
}