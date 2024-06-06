use clap::Parser;
use esetres::commands::{self, start, Commands};

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
        Commands::Start { port, local } => start::run(port, local).await?,
        Commands::Tokens(cmd) => {
            match cmd {
                commands::Tokens::List => todo!(),
                commands::Tokens::Mint { name, scope } => todo!(),
                commands::Tokens::Revoke { name } => todo!(),
            }
        },
    }

    Ok(())
}
