use clap::Parser;
use esetres::commands::{start, Commands};

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
    }

    Ok(())
}
