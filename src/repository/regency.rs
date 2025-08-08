use crate::models::regency::Regency;
use anyhow::Result;
use sqlx::PgPool;

pub async fn get_regencies_by_province_id(
    pool: &PgPool,
    province_id: &str,
) -> Result<Vec<Regency>> {
    let regencies = sqlx::query_as::<_, Regency>(
        "SELECT * FROM regencies WHERE province_id = $1 ORDER BY regency_name ASC",
    )
    .bind(province_id)
    .fetch_all(pool)
    .await?;

    Ok(regencies)
}

pub async fn search_regencies(
    pool: &PgPool,
    province_id: &str,
    query: &str,
) -> Result<Vec<Regency>> {
    let search_pattern = format!("%{}%", query);
    let regencies = sqlx::query_as::<_, Regency>(
        "SELECT * FROM regencies WHERE province_id = $1 AND LOWER(regency_name) LIKE LOWER($2) ORDER BY regency_name ASC"
    )
    .bind(province_id)
    .bind(search_pattern)
    .fetch_all(pool)
    .await?;

    Ok(regencies)
}
