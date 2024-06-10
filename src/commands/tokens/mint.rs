use crate::{
    config, db::{self, schema::Token}, jwt
};

pub async fn run(name: String, scope: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let scope = scope.unwrap_or("*".to_string());
    let token = jwt::create(name.clone(), scope.clone())?;

    db::schema::Token::create(Token {
        id: 0,
        name: name.clone(),
        token: token.clone(),
        scope: scope.clone(),
    }).await?;

    let config = config::get();

    let client = reqwest::Client::new();

    client.post(format!("http://{}:{}/cache/invalidate", &config.ip, &config.port)).send().await?;

    println!("New token ({name}) created for scope ({scope}).");
    println!("{token}");

    Ok(())
}
