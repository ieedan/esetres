use std::env;

#[derive(Debug)]
pub struct Object {
    pub root_directory: String,
    pub mime_types: MimeTypes,
    pub ip: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct MimeTypes {
    pub local_path: String,
}

pub fn get() -> Object {
    Object {
        ip: env::var("IP").unwrap(),
        port: env::var("PORT").unwrap().parse().unwrap(),
        root_directory: "./buckets".to_string(),
        mime_types: MimeTypes {
            local_path: "./mime-db.json".to_string(),
        },
    }
}
