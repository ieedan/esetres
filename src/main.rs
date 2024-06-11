use clap::Parser;
use esetres::commands::{self, buckets, init, migrate, start, tokens, Commands};

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Cli {
    /// Commands to execute
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Start => start::run().await?,
        Commands::Tokens(cmd) => match cmd {
            commands::Tokens::List => tokens::list::run().await?,
            commands::Tokens::Mint {
                name,
                scope,
                access,
            } => tokens::mint::run(name, scope, access).await?,
            commands::Tokens::Revoke { name } => tokens::revoke::run(name).await?,
        },
        Commands::Migrate => migrate::run().await?,
        Commands::Init => init::run().await?,
        Commands::Buckets(cmd) => match cmd {
            commands::Buckets::List => buckets::list::run().await?,
            commands::Buckets::Create { name } => buckets::create::run(&name).await?,
            commands::Buckets::Delete { name } => buckets::delete::run(&name).await?,
        },
    }

    Ok(())
}
