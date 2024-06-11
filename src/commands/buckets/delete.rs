use std::path::Path;

use crate::config;

pub async fn run(name: &str) -> Result<(), std::io::Error> {
    let config = config::get();
    let path = Path::new(&config.root_directory).join(name);
    
    tokio::fs::remove_dir_all(path).await?;

    println!("Removed {name}.");

    Ok(())
}