use crate::{config, db};

pub async fn run(name: String) -> Result<(), Box<dyn std::error::Error>> {
    db::schema::Token::delete(name.clone()).await?;

    println!("{name} revoked.");

    // The user won't care if this worked

    let config = config::get();

    let client = reqwest::Client::new();

    let _ = client.post(format!("http://{}:{}/cache/invalidate", &config.ip, &config.port)).send().await;

    Ok(())
}