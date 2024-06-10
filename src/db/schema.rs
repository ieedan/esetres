use std::fmt::Display;

use super::connect;
use bcrypt::hash;
use clap::ValueEnum;
use rusqlite::types::{FromSql, FromSqlError};
use tokio::task;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: i32,
    pub name: String,
    pub bucket_scope: String,
    pub access: Access,
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum Access {
    READ,
    WRITE,
    FULL,
}

impl FromSql for Access {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let str = value.as_str()?;

        match str {
            "full" => Ok(Access::FULL),
            "write" => Ok(Access::WRITE),
            "read" => Ok(Access::READ),
            &_ => Err(FromSqlError::InvalidType),
        }
    }
}

impl Access {
    fn to_string(&self) -> String {
        match self {
            Access::READ => "read".to_string(),
            Access::WRITE => "write".to_string(),
            Access::FULL => "full".to_string(),
        }
    }
}

impl Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Access::READ => write!(f, "read"),
            Access::WRITE => write!(f, "write"),
            Access::FULL => write!(f, "full"),
        }
    }
}

impl Token {
    pub async fn get_all() -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let tokens = task::spawn_blocking(move || -> rusqlite::Result<Vec<Token>> {
            let conn = connect()?;

            let mut stmt = conn.prepare(
                "SELECT id, name, bucket_scope, access, token FROM tokens ORDER BY Id DESC",
            )?;
            let tokens_iter = stmt.query_map([], |row| {
                Ok(Token {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    bucket_scope: row.get(2)?,
                    access: row.get(3)?,
                    token: row.get(4)?,
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
                "INSERT INTO tokens (name, token, access, bucket_scope) VALUES (?1, ?2, ?3, ?4)",
                [
                    &self.name,
                    &token_hash,
                    &self.access.to_string(),
                    &self.bucket_scope,
                ],
            )?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    pub async fn delete(name: String) -> Result<(), Box<dyn std::error::Error>> {
        task::spawn_blocking(move || -> rusqlite::Result<()> {
            let conn = connect()?;

            conn.execute("DELETE FROM tokens WHERE name = ?1", [name])?;

            Ok(())
        })
        .await??;

        Ok(())
    }
}
