use crate::models::village::Village;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn get_villages_by_district_id(pool: &PgPool, district_id: Uuid) -> Result<Vec<Village>> {
    let villages = sqlx::query_as::<_, Village>("SELECT * FROM villages WHERE district_id = $1 ORDER BY village_name ASC")
    .bind(district_id)
    .fetch_all(pool)
    .await?;

    Ok(villages)
}

pub async fn search_villages(pool: &PgPool, district_id: Uuid, query: &str) -> Result<Vec<Village>> {
    let search_pattern = format!("%{}%", query);
    let villages = sqlx::query_as::<_, Village>(
        "SELECT * FROM villages WHERE district_id = $1 AND (LOWER(village_name) LIKE LOWER($2) OR LOWER(postal_code) LIKE LOWER($2)) ORDER BY village_name ASC"
    )
    .bind(district_id)
    .bind(search_pattern)
    .fetch_all(pool)
    .await?;

    Ok(villages)
}