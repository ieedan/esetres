use std::path::Path;

use crate::config;

pub async fn run(name: &str) -> Result<(), std::io::Error> {
    let config = config::get();
    let path = Path::new(&config.root_directory).join(name);

    if let Err(_) = tokio::fs::read_dir(&config.root_directory).await {
        tokio::fs::create_dir(&config.root_directory).await?;
    }

    if let Ok(_) = tokio::fs::read_dir(&path).await {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "The bucket already exists"));
    }

    tokio::fs::create_dir(&path).await?;

    tokio::fs::create_dir(&path.join("public")).await?;

    tokio::fs::create_dir(&path.join("private")).await?;

    Ok(())
}