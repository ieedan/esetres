#[derive(Debug)]
pub struct Object {
    pub root_directory: String,
    pub mime_types: MimeTypes,
}

#[derive(Debug)]
pub struct MimeTypes {
    pub local_path: String,
}

pub fn get() -> Object {
    Object {
        root_directory: "./buckets".to_string(),
        mime_types: MimeTypes {
            local_path: "./mime-db.json".to_string(),
        },
    }
}
