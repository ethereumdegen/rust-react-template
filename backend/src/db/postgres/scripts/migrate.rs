use degen_sql::db::postgres::postgres_db::{Database, DatabaseCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = DatabaseCredentials::from_env();

    let mut database = Database::connect(credentials, None).await?;

    let _migration = database.migrate().await?;

    println!("Migration complete");

    Ok(())
}
