
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("PostgreSQL connected successfully!");

    Ok(pool)
}

pub async fn get_users(pool: &PgPool) -> Result<String, sqlx::Error> {
    let users = sqlx::query_as::<_, (i32, String,String)>(
        "SELECT id, name, email FROM users_data"
    )
        .fetch_all(pool)
        .await?;

    for (id, name,email) in users.to_owned() {
        println!("ID: {}, Name: {} Email: {}", id, name,email);
    }

    let final_string = format!("{:?}", users).to_string();
    Ok(final_string)
}
