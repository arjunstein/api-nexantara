use crate::models::district::District;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn get_districts_by_regency_id(pool: &PgPool, regency_id: Uuid) -> Result<Vec<District>> {
    let districts = sqlx::query_as::<_, District>("SELECT * FROM districts WHERE regency_id = $1 ORDER BY district_name ASC")
    .bind(regency_id)
    .fetch_all(pool)
    .await?;

    Ok(districts)
}

pub async fn search_districts(pool: &PgPool, regency_id: Uuid, query: &str) -> Result<Vec<District>> {
    let search_pattern = format!("%{}%", query);
    let districts = sqlx::query_as::<_, District>(
        "SELECT * FROM districts WHERE regency_id = $1 AND LOWER(district_name) LIKE LOWER($2) ORDER BY district_name ASC"
    )
    .bind(regency_id)
    .bind(search_pattern)
    .fetch_all(pool)
    .await?;

    Ok(districts)
}