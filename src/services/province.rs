use crate::models::province::Province;
use crate::repository::province::{get_all_provinces, get_province_by_id};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_all_provinces(pool: &PgPool) -> Result<Vec<Province>> {
    get_all_provinces(pool).await
}

pub async fn fetch_province_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Province>> {
    get_province_by_id(pool, id).await
}