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
