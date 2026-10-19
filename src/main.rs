
pub mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Step 1: Connect to PostgreSQL
    let pool = db::create_pool().await?;

    // Step 2: Fetch data
    let post_data=db::get_users(&pool).await;
    let data = post_data.unwrap_or("".to_string());
    println!("data received to main-> {}", data);

    Ok(())
}
