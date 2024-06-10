use super::connect;
use bcrypt::hash;
use tokio::task;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: i32,
    pub name: String,
    pub scope: String,
    pub token: String,
}

impl Token {
    pub async fn get_all() -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let tokens = task::spawn_blocking(move || -> rusqlite::Result<Vec<Token>> {
            let conn = connect()?;

            let mut stmt = conn.prepare("SELECT Id, Name, Scope, Token FROM tokens ORDER BY Id DESC")?;
            let tokens_iter = stmt.query_map([], |row| {
                Ok(Token {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    scope: row.get(2)?,
                    token: row.get(3)?,
                })
            })?;

            let mut tokens: Vec<Token> = vec![];

            for token in tokens_iter {
                if let Ok(t) = token {
                    tokens.push(t);
                }
            }

            Ok(tokens)
        })
        .await??;

        Ok(tokens)
    }

    pub async fn create(self) -> Result<(), Box<dyn std::error::Error>> {
        let token_hash = hash(&self.token, 6)?;
        task::spawn_blocking(move || -> rusqlite::Result<()> {
            let conn = connect()?;

            conn.execute(
                "INSERT INTO tokens (name, token, scope) VALUES (?1, ?2, ?3)",
                [&self.name, &token_hash, &self.scope],
            )?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    pub async fn delete(name: String) -> Result<(), Box<dyn std::error::Error>> {
        task::spawn_blocking(move || -> rusqlite::Result<()> {
            let conn = connect()?;

            conn.execute(
                "DELETE FROM tokens WHERE name = ?1",
                [name],
            )?;

            Ok(())
        })
        .await??;

        Ok(())
    }
}
