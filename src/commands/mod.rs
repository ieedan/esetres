use clap::Subcommand;

use crate::db::schema::Access;

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
    /// Creates the sqlite database if it doesn't exist and creates the required tables
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
        /// The access level of the token
        #[arg(long, short, value_parser)]
        access: Option<Access>
    },
    /// Revoke an existing token
    Revoke {
        /// Name of the token
        name: String,
    },
}
