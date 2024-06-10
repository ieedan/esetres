use std::{env, fs};

fn main() {
    println!("cargo::rerun-if-changed=./db/migration.sql");
    println!("cargo::rerun-if-changed=.env");

    let out_dir = env::var("OUT_DIR").unwrap();

    let path = if out_dir.contains("release") {
        let start = out_dir.rfind("release").unwrap();
        &out_dir[..start + "release".len()]
    } else {
        let start = out_dir.rfind("debug").unwrap();
        &out_dir[..start + "debug".len()]
    };

    println!("Out Dir: {path}");

    if let Err(_) = fs::read_dir(format!("{path}/db")) {
        fs::create_dir(format!("{path}/db")).unwrap();
    }

    fs::copy("./db/migration.sql", format!("{path}/db/migration.sql")).unwrap();
    fs::copy(".env", format!("{path}/.env")).unwrap();

    println!("Completed copying files!");
}
