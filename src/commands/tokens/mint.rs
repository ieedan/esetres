use crate::jwt;

pub async fn run(name: String, scope: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let scope = scope.unwrap_or("*".to_string());
    let token = jwt::create(name.clone(), scope.clone())?;

    println!("New token ({name}) created for scope ({scope}).");
    println!("{token}");

    Ok(())
}
