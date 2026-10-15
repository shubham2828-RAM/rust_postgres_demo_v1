
pub mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Step 1: Connect to PostgreSQL
    let pool = db::create_pool().await?;

    // Step 2: Fetch data
    db::get_users(&pool).await?;

    Ok(())
}
