#[derive(Debug)]
pub struct Object {
    pub root_directory: String,
    pub host_ip: String,
    pub port: u16,
    pub mime_types: MimeTypes,
}

#[derive(Debug)]
pub struct MimeTypes {
    pub local_path: String
}

pub fn get() -> Result<Object, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    Ok(Object {
        root_directory: std::env::var("ROOT_DIRECTORY")?,
        host_ip: std::env::var("HOST_IP")?,
        port: std::env::var("PORT")?.parse()?,
        mime_types: MimeTypes {
            local_path: std::env::var("MIME_TYPES_LOCAL_PATH")?,
        }
    })
}
