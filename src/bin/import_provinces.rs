use chrono;
use csv::ReaderBuilder;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tokio;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct ProvinceRecord {
    province_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let path = Path::new("files/province.csv");
    let file = File::open(path)?;

    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut total_imported = 0;
    let mut total_duplicates = 0;

    for result in rdr.deserialize() {
        let record: ProvinceRecord = result?;
        let id = Uuid::new_v4();
        let province_name = record.province_name.trim().to_uppercase();
        let created_at = chrono::Utc::now();
        let updated_at = chrono::Utc::now();

        // Check if province already exists
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM provinces WHERE province_name = $1)",
            province_name
        )
        .fetch_one(&pool)
        .await?;

        if exists.unwrap_or(false) {
            println!("Skipping duplicate province: {}", province_name);
            total_duplicates += 1;
            continue;
        }

        match sqlx::query!(
            r#"
            INSERT INTO provinces (id, province_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            province_name,
            created_at,
            updated_at,
        )
        .execute(&pool)
        .await
        {
            Ok(_) => {
                println!("Imported province: {}", province_name);
                total_imported += 1;
            }
            Err(e) => {
                if e.to_string().contains("duplicate key") {
                    println!(
                        "Skipping duplicate province (race condition): {}",
                        province_name
                    );
                    total_duplicates += 1;
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    println!("\nImport completed!");
    println!("Total imported: {}", total_imported);
    println!("Total duplicates skipped: {}", total_duplicates);

    Ok(())
}
