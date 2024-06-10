use clap::Subcommand;

pub mod start;

pub mod tokens;

pub mod migrate;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Starts the server
    Start,
    /// Allows you to manage tokens from the CLI
    #[command(subcommand)]
    Tokens(Tokens),
    Migrate,
}

#[derive(Subcommand, Debug)]
pub enum Tokens {
    /// List all of the active tokens
    List,
    /// Mint a new token
    Mint {
        /// Name of the token
        name: String,
        /// The bucket scope of the token
        #[clap(long, value_parser)]
        scope: Option<String>,
    },
    /// Revoke an existing token
    Revoke {
        /// Name of the token
        name: String,
    },
}
