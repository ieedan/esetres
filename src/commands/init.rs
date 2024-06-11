use colored::Colorize;
use dialoguer::{Confirm, FuzzySelect, Input, Password};
use get_if_addrs::get_if_addrs;
use rand::Rng;
use std::net::IpAddr;

use crate::{commands, config, db};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to esetres cli!");
    let pipe = "|".truecolor(128, 128, 128);
    let o = "o".truecolor(50, 50, 255);
    let check = "✓".bright_green();

    println!("{pipe}");

    let create_env: bool;

    if let Ok(_) = tokio::fs::read(".env").await {
        let overwrite = Confirm::new()
            .with_prompt(format!(
                "{o} There is already an env file would you like to overwrite it?"
            ))
            .default(false)
            .interact()?;

        create_env = overwrite;

        println!("{pipe}");
    } else {
        create_env = true;
    }

    if create_env {
        let should_generate = Confirm::new()
            .with_prompt(format!("{o} Do you want us to generate the token secret?"))
            .default(true)
            .interact()?;

        println!("{pipe}");

        let token_secret: String;

        if should_generate {
            token_secret = generate_secret();
            println!("{check} Generated token secret.");
            println!("{pipe}");
        } else {
            token_secret = Password::new()
                .with_prompt(format!("{o} Enter your token secret"))
                .interact()?;
            println!("{pipe}");
        }

        let mut ips: Vec<String> = vec![];

        match get_if_addrs() {
            Ok(if_addrs) => {
                for if_addr in if_addrs {
                    let ip_addr = if_addr.addr.ip();
                    if !is_local_address(&ip_addr) {
                        match ip_addr {
                            IpAddr::V4(ipv4_addr) => {
                                ips.push(format!("{ipv4_addr}"));
                            }
                            IpAddr::V6(ipv6_addr) => {
                                ips.push(format!("{ipv6_addr}"));
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }

        ips.push("Enter my own".to_string());

        let selected_ip = FuzzySelect::new()
            .with_prompt(format!("{o} Select your the ip"))
            .items(&ips)
            .default(ips.len() - 1)
            .interact()?;

        println!("{pipe}");

        let ip_address: String;

        if ips[selected_ip] == "Enter my own" {
            ip_address = Input::new()
                .with_prompt(format!("{o} Enter your ip address"))
                .interact_text()?;

            println!("{pipe}");
        } else {
            ip_address = ips[selected_ip].to_string();
        }

        let port = Input::<u16>::new()
            .with_prompt(format!("{o} Enter the port"))
            .default(8080)
            .interact_text()?;

        println!("{pipe}");

        let use_https = Confirm::new()
            .with_prompt(format!("{o} Use https?"))
            .default(false)
            .interact()?;

        let https = if use_https { 1 } else { 0 };

        println!("");

        let env_file =
        format!("TOKEN_SECRET=\"{token_secret}\"\r\n\r\nIP=\"{ip_address}\"\r\nPORT={port}\r\nHTTPS={https}");

        println!(
            "TOKEN_SECRET=\"[hidden]\"\r\n\r\nIP=\"{ip_address}\"\r\nPORT={port}\r\nHTTPS={https}"
        );
        println!("");

        let should_create = Confirm::new()
            .with_prompt(format!("{o} .env file Ok?"))
            .default(true)
            .interact()?;

        if should_create {
            println!("{pipe}");
            tokio::fs::write(".env", env_file).await?;
            println!("{check} Created .env file.");
        } else {
            return Ok(());
        }

        println!("{pipe}");
    }

    let run_migration = Confirm::new()
        .with_prompt(format!("{o} Run sqlite migration?"))
        .default(true)
        .interact()?;

    println!("{pipe}");

    if run_migration {
        db::migrate().await?;
        println!("{check} Ran sqlite migration.");
        println!("{pipe}");
    }

    let config = config::get();

    if let Err(_) = tokio::fs::read_dir(&config.root_directory).await {
        let create_bucket = Confirm::new()
            .with_prompt(format!("{o} Create a bucket?"))
            .default(true)
            .interact()?;

        println!("{pipe}");

        if create_bucket {
            let bucket_name = Input::new()
                .with_prompt(format!("{o} Enter the bucket name"))
                .default("default".to_string())
                .allow_empty(true)
                .interact_text()?;

            println!("{pipe}");

            commands::buckets::create::run(&bucket_name).await?;

            println!("{check} Bucket [{bucket_name}] created.");

            println!("{pipe}");
        }
    }

    println!("{check} Completed initialization.");

    Ok(())
}

fn generate_secret() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

fn is_local_address(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4_addr) => {
            ipv4_addr.is_link_local() || ipv4_addr.is_loopback() || ipv4_addr.is_broadcast()
        }
        IpAddr::V6(ipv6_addr) => ipv6_addr.is_loopback(),
    }
}
