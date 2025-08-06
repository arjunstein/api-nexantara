use crate::models::province::Province;
use crate::repository::province::{get_all_provinces, search_provinces};
use sqlx::PgPool;
use anyhow::Result;

pub async fn fetch_all_provinces(pool: &PgPool) -> Result<Vec<Province>> {
    get_all_provinces(pool).await
}

pub async fn search_provinces_service(pool: &PgPool, query: &str) -> Result<Vec<Province>> {
    search_provinces(pool, query).await
}
