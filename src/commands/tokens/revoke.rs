use crate::{config, db};

pub async fn run(name: String) -> Result<(), Box<dyn std::error::Error>> {
    db::schema::Token::delete(name.clone()).await?;
    
    let config = config::get();

    let client = reqwest::Client::new();

    client.post(format!("http://{}:{}/cache/invalidate", &config.ip, &config.port)).send().await?;

    println!("{name} revoked.");

    Ok(())
}