use anyhow::{Context, Result};
use chrono::Utc;
use futures::stream::{FuturesUnordered, StreamExt};
use log::{debug, error, info};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{PgPool, QueryBuilder, postgres::PgPoolOptions};
use std::{collections::HashMap, env};

#[derive(Debug, Deserialize)]
struct WilayahData {
    code: String,
    name: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();

    info!("🚀 Starting data import from Wilayah.id...");

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let base_url = env::var("API_WILAYAH").expect("environment variable is not set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let provinces = fetch_wilayah_data("provinces").await?;
    let mut total_provinces = 0;
    let mut total_regencies = 0;
    let mut total_districts = 0;
    let mut total_villages = 0;

    for province in &provinces {
        import_province(&pool, province).await?;
        total_provinces += 1;

        let regencies = fetch_wilayah_data(&format!("regencies/{}", province.code)).await?;
        let mut regency_futures = FuturesUnordered::new();

        for regency in regencies {
            let pool = pool.clone();
            let province_code = province.code.clone();
            let regency = regency;
            regency_futures.push(tokio::spawn(async move {
                if let Err(e) = import_regency(&pool, &regency, &province_code).await {
                    error!("Regency error: {}", e);
                    return None;
                }
                Some(regency)
            }));
        }

        while let Some(Ok(Some(regency))) = regency_futures.next().await {
            total_regencies += 1;
            let districts = fetch_wilayah_data(&format!("districts/{}", regency.code)).await?;
            let mut district_futures = FuturesUnordered::new();

            for district in districts {
                let pool = pool.clone();
                let regency_code = regency.code.clone();
                let district = district;
                district_futures.push(tokio::spawn(async move {
                    if let Err(e) = import_district(&pool, &district, &regency_code).await {
                        error!("District error: {}", e);
                        return None;
                    }
                    Some(district)
                }));
            }

            while let Some(Ok(Some(district))) = district_futures.next().await {
                total_districts += 1;
                let villages = fetch_wilayah_data(&format!("villages/{}", district.code)).await?;

                // Batched village insert
                let now = Utc::now();
                let mut builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
                    r#"
                    INSERT INTO villages (id, village_name, district_id, postal_code, created_at, updated_at)
                "#,
                );

                builder.push_values(villages.iter(), |mut b, v| {
                    let postal = v
                        .extra
                        .get("postal")
                        .and_then(|v| v.as_str())
                        .unwrap_or("00000");
                    b.push_bind(&v.code)
                        .push_bind(&v.name)
                        .push_bind(&district.code)
                        .push_bind(postal)
                        .push_bind(now)
                        .push_bind(now);
                });

                builder.push(
                    r#"
                    ON CONFLICT (id) DO UPDATE
                    SET village_name = EXCLUDED.village_name,
                        district_id = EXCLUDED.district_id,
                        postal_code = EXCLUDED.postal_code,
                        updated_at = EXCLUDED.updated_at
                    "#,
                );

                builder.build().execute(&pool).await?;
                total_villages += villages.len();

                if total_villages % 500 == 0 {
                    info!("✅ {} villages imported so far...", total_villages);
                }
            }
        }
    }

    info!(
        "✅ Import done! Total: {} provinces, {} regencies, {} districts, {} villages",
        total_provinces, total_regencies, total_districts, total_villages
    );

    Ok(())
}

async fn fetch_wilayah_data(endpoint: &str) -> Result<Vec<WilayahData>> {
    let url = format!("{}/{}.json", BASE_URL, endpoint);
    debug!("Fetching: {}", url);
    let res = reqwest::get(&url).await?;
    let text = res.text().await?;

    if let Ok(data) = serde_json::from_str::<Vec<WilayahData>>(&text) {
        return Ok(data);
    }

    #[derive(Deserialize)]
    struct Wrapper {
        data: Vec<WilayahData>,
    }

    let wrapper: Wrapper = serde_json::from_str(&text)?;
    Ok(wrapper.data)
}

async fn import_province(pool: &PgPool, data: &WilayahData) -> Result<()> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO provinces (id, province_name, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO UPDATE
        SET province_name = EXCLUDED.province_name,
            updated_at = EXCLUDED.updated_at
        "#,
        &data.code,
        &data.name,
        now,
        now,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn import_regency(pool: &PgPool, data: &WilayahData, province_code: &str) -> Result<()> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO regencies (id, regency_name, province_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET regency_name = EXCLUDED.regency_name,
            province_id = EXCLUDED.province_id,
            updated_at = EXCLUDED.updated_at
        "#,
        &data.code,
        &data.name,
        province_code,
        now,
        now,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn import_district(pool: &PgPool, data: &WilayahData, regency_code: &str) -> Result<()> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO districts (id, district_name, regency_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET district_name = EXCLUDED.district_name,
            regency_id = EXCLUDED.regency_id,
            updated_at = EXCLUDED.updated_at
        "#,
        &data.code,
        &data.name,
        regency_code,
        now,
        now,
    )
    .execute(pool)
    .await?;
    Ok(())
}
