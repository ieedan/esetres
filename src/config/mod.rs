use std::env;

#[derive(Debug)]
pub struct Object {
    pub root_directory: String,
    pub mime_types: MimeTypes,
    pub ip: String,
    pub port: u16,
    pub https: bool,
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
    Object {
        https: env::var("HTTPS").unwrap().parse::<i32>().unwrap() == 1,
        ip: env::var("IP").unwrap(),
        port: env::var("PORT").unwrap().parse().unwrap(),
        root_directory: "./buckets".to_string(),
        mime_types: MimeTypes {
            local_path: "./mime-db.json".to_string(),
        },
    }
}
