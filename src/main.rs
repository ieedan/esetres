use clap::Parser;
use esetres::commands::{self, migrate, start, tokens, Commands};

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Cli {
    /// Commands to execute
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let args = Cli::parse();

    match args.command {
        Commands::Start => start::run().await?,
        Commands::Tokens(cmd) => match cmd {
            commands::Tokens::List => tokens::list::run().await?,
            commands::Tokens::Mint { name, scope } => tokens::mint::run(name, scope).await?,
            commands::Tokens::Revoke { name } => tokens::revoke::run(name).await?,
        },
        Commands::Migrate => migrate::run().await?,
    }

    Ok(())
}
