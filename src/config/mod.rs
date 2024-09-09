use std::env;

#[derive(Debug, Clone)]
pub struct Object {
    pub root_directory: String,
    pub token_secret: String,
    pub ip: String,
    pub port: u16,
    pub https: bool,
    pub max_size_mb: usize,
}

impl Object {
    pub fn address(&self) -> String {
        let prefix = if self.https { "https" } else { "http" };
        format!("{prefix}://{}:{}", self.ip, self.port)
    }
}

#[derive(Debug)]
pub struct MimeTypes {
    pub local_path: String,
}

pub fn get() -> Object {
    dotenvy::dotenv().ok();
    Object {
        token_secret: env::var("TOKEN_SECRET").expect(
            "TOKEN_SECRET must be configured run `esetres init` or provide it in the .env file.",
        ),
        https: env::var("HTTPS")
            .expect("HTTPS must be configured run `esetres init` or provide it in the .env file.")
            .parse::<i32>()
            .unwrap()
            == 1,
        ip: env::var("IP")
            .expect("IP must be configured run `esetres init` or provide it in the .env file."),
        port: env::var("PORT")
            .expect("PORT must be configured run `esetres init` or provide it in the .env file.")
            .parse()
            .unwrap(),
        root_directory: "./buckets".to_string(),
        max_size_mb: env::var("MAX_SIZE_MB")
            .unwrap_or("2".to_string())
            .parse()
            .unwrap(),
    }
}
