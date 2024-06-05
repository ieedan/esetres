use std::collections::HashMap;

use serde_json::Value;

use crate::config::{self};

const MIME_TYPES_URL: &str = "https://cdn.jsdelivr.net/gh/jshttp/mime-db@master/db.json";

struct Mime {

}

/// Checks the folder path defined by `MIME_TYPES_LOCAL_PATH` for
/// files saved locally before fetching them from the source
pub async fn get<'a>(
    config: &config::Object,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file_types = HashMap::new();

    let file = if let Ok(str) = tokio::fs::read_to_string(&config.mime_types.local_path).await {
        str
    } else {
        println!("Getting mime types from {MIME_TYPES_URL}...");

        // get file from remote
        let client = reqwest::Client::new();
        let res = client
            .get(MIME_TYPES_URL)
            .header("User-Agent", "esetres/0.1.0")
            .send()
            .await?;

        let text = res.text().await?;

        tokio::fs::write(&config.mime_types.local_path, &text).await?;

        text
    };

    let v: Value = serde_json::from_str(&file)?;

    for types in v.as_object().into_iter() {
        for (k, value) in types {
            let extensions = value["extensions"].as_array();
            if let Some(exts) = extensions {
                for ext in exts {
                    let key = k.to_owned();
                    let extension = ext.as_str().unwrap().to_string();
                    file_types.insert(extension, key);
                }
            }
        }
    }

    Ok(file_types)
}
