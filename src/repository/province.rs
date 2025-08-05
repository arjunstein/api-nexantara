use crate::models::province::Province;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub async fn get_all_provinces(pool: &PgPool) -> Result<Vec<Province>> {
    let provinces = sqlx::query_as::<_, Province>("SELECT * FROM provinces ORDER BY province_name ASC")
    .fetch_all(pool)
    .await?;

    Ok(provinces)
}

pub async fn get_province_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Province>> {
    let province = sqlx::query_as::<_, Province>("SELECT * FROM provinces WHERE id = $1")
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(province)
}
