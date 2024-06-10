use crate::db;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = db::schema::Token::get_all().await?;

    if tokens.len() == 0 {
        println!("No tokens generated yet.");
    } else {
        let mut max_name_length = 6;
        let mut max_bucket_scope_length = 7;

        for token in &tokens {
            if max_name_length < token.name.len() {
                max_name_length = token.name.len();
            }

            if max_bucket_scope_length < token.bucket_scope.len() {
                max_bucket_scope_length = token.bucket_scope.len();
            }
        }

        let header = format!("{}   {}   Access  ", min_length("Name", max_name_length), min_length("Scope", max_bucket_scope_length));
        
        println!("{}", header);
        
        println!("{}", repeat("-", header.len()));

        for token in &tokens {
            println!(
                "{} | {} | {}",
                min_length(&token.name, max_name_length),
                min_length(&token.bucket_scope, max_bucket_scope_length),
                min_length(&token.access.to_string(), 5)
            );
        }
    }

    Ok(())
}

/// Returns a right padded string based on the min length
fn min_length(str: &str, min: usize) -> String {
    let mut str = String::from(str);

    let remaining = min - str.len();

    for _ in 0..remaining {
        str.push_str(" ");
    }

    str
}

fn repeat(char: &str, amount: usize) -> String {
    let mut str = String::new();
    for _ in 0..amount {
        str.push_str(char);
    }

    str
}