use std::net::Ipv4Addr;

use crate::{config, mime, router::{self, AppState}};

pub async fn run(port: Option<u16>, local: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get();
    
    let default = if local { 3000 } else { 8080 };
    let port = port.unwrap_or(default);
    let ip = if local {
        Ipv4Addr::new(127, 0, 0, 1)
    } else {
        Ipv4Addr::new(0, 0, 0, 0)
    };
    
    let mime_types = mime::get(&config).await?;

    // If not exists create
    if let Err(_) = std::fs::read_dir(&config.root_directory) {
        tokio::fs::create_dir(&config.root_directory).await?;
        println!("Created directory {}", &config.root_directory);
    }

    let address = format!("{}:{}", &ip, &port);

    let app = router::get(AppState { ip, port, config, mime_types });

    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Listening at {address}...");

    axum::serve(listener, app).await?;

    Ok(())
}
