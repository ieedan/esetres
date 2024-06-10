use crate::db;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    db::migrate().await?;

    println!("Migration complete");

    Ok(())
}
