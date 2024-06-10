use crate::db;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = db::schema::Token::get_all().await?;

    if tokens.len() == 0 {
        println!("No tokens generated yet.");
    } else {
        for token in tokens {
            println!("{} - Scope: {}", token.name, token.scope);
        }
    }

    Ok(())
}
