use crate::models::province::Province;
use sqlx::PgPool;
use anyhow::Result;

pub async fn get_all_provinces(pool: &PgPool) -> Result<Vec<Province>> {
    let provinces = sqlx::query_as::<_, Province>("SELECT * FROM provinces ORDER BY province_name ASC")
    .fetch_all(pool)
    .await?;

    Ok(provinces)
}

pub async fn search_provinces(pool: &PgPool, query: &str) -> Result<Vec<Province>> {
    let search_pattern = format!("%{}%", query);
    let provinces = sqlx::query_as::<_, Province>(
        "SELECT * FROM provinces WHERE LOWER(province_name) LIKE LOWER($1) ORDER BY province_name ASC"
    )
    .bind(search_pattern)
    .fetch_all(pool)
    .await?;

    Ok(provinces)
}
