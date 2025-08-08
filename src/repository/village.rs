use crate::models::village::Village;
use anyhow::Result;
use sqlx::PgPool;

pub async fn get_villages_by_district_id(pool: &PgPool, district_id: &str) -> Result<Vec<Village>> {
    let villages = sqlx::query_as::<_, Village>(
        "SELECT * FROM villages WHERE district_id = $1 ORDER BY village_name ASC",
    )
    .bind(district_id)
    .fetch_all(pool)
    .await?;

    Ok(villages)
}

pub async fn search_villages(
    pool: &PgPool,
    district_id: &str,
    query: &str,
) -> Result<Vec<Village>> {
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
