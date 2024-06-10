use crate::{
    config, db::{self, schema::{Access, Token}}, jwt
};

pub async fn run(name: String, scope: Option<String>, access: Option<Access>) -> Result<(), Box<dyn std::error::Error>> {
    let scope = scope.unwrap_or("*".to_string());
    let token = jwt::create(name.clone(), scope.clone())?;

    db::schema::Token::create(Token {
        id: 0,
        name: name.clone(),
        token: token.clone(),
        access: access.unwrap_or(Access::READ),
        bucket_scope: scope.clone(),
    }).await?;

    println!("New token ({name}) created for scope ({scope}).");
    println!("{token}");

    // The user won't care if this worked

    let config = config::get();

    let client = reqwest::Client::new();

    let _ = client.post(format!("http://{}:{}/cache/invalidate", &config.ip, &config.port)).send().await;

    Ok(())
}
