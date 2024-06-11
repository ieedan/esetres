use rusqlite::Connection;
use tokio::task;

pub mod schema;

const MIGRATION: &str = r#"
    CREATE TABLE IF NOT EXISTS tokens (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        bucket_scope TEXT NOT NULL,
        access TEXT NOT NULL, -- read / write / full
        token TEXT NOT NULL
    )
"#;

pub async fn migrate() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(_) = tokio::fs::read_dir("./db").await {
        tokio::fs::create_dir("./db").await?;
    }

    task::spawn_blocking(move || -> rusqlite::Result<()> {
        let conn = connect()?;

        conn.execute_batch(MIGRATION)?;

        Ok(())
    })
    .await??;

    Ok(())
}

pub fn connect() -> rusqlite::Result<Connection> {
    Ok(Connection::open("./db/database.db")?)
}
