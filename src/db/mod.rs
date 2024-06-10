use rusqlite::Connection;
use tokio::{fs, task};

pub mod schema;

pub async fn migrate() -> Result<(), Box<dyn std::error::Error>> {
    let qry = fs::read_to_string("./db/migration.sql").await?;

    task::spawn_blocking(move || -> rusqlite::Result<()> {
        let conn = connect()?;

        conn.execute_batch(&qry)?;

        Ok(())
    })
    .await??;

    Ok(())
}

pub fn connect() -> rusqlite::Result<Connection> {
    Ok(Connection::open("./db/database.db")?)
}
