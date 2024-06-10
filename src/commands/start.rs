use crate::{config, mime, router::{self, AppState}};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get();
    
    let mime_types = mime::get(&config).await?;

    // If not exists create
    if let Err(_) = std::fs::read_dir(&config.root_directory) {
        tokio::fs::create_dir(&config.root_directory).await?;
        println!("Created directory {}", &config.root_directory);
    }

    let address = format!("{}:{}", &config.ip, &config.port);

    let app = router::get(AppState { config, mime_types });

    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Listening at {address}...");

    axum::serve(listener, app).await?;

    Ok(())
}
