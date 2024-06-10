use crate::db;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    Ok(db::migrate().await?)
}
