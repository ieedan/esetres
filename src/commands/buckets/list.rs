use chrono::{DateTime, Local};

use crate::{
    config,
    util::{min_length, repeat},
};

struct BucketInfo {
    name: String,
    modified: DateTime<Local>,
}

pub async fn run() -> Result<(), std::io::Error> {
    let config = config::get();

    let mut buckets: Vec<BucketInfo> = vec![];

    let mut largest: usize = 4;

    for entry in std::fs::read_dir(&config.root_directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let bucket_name = path.file_name().unwrap().to_str().unwrap().to_string();
            if bucket_name.len() > largest {
                largest = bucket_name.len();
            }
            let modified = entry.metadata()?.modified().unwrap();
            buckets.push(BucketInfo {
                name: bucket_name,
                modified: modified.into(),
            });
        }
    }

    if buckets.len() == 0 {
        println!("No buckets created yet.")
    } else {
        let header = format!(
            "{} {}",
            min_length("Name", largest + 2),
            min_length("Modified", buckets[0].modified.to_string().len() + 2)
        );
        println!("{header}");
        println!("{}", repeat("-", header.len() + 2));
        for bucket in buckets {
            println!(
                "{} | {}",
                min_length(&bucket.name, largest),
                &bucket.modified
            );
        }
    }

    Ok(())
}
