use crate::models::village::Village;
use crate::repository::village::get_villages_by_district_id;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn fetch_villages_by_district_id(pool: &PgPool, district_id: Uuid) -> Result<Vec<Village>> {
    get_villages_by_district_id(pool, district_id).await
}