use clap::Subcommand;

use crate::db::schema::Access;

pub mod start;

pub mod tokens;

pub mod migrate;

pub mod init;

pub mod buckets;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Takes you through the setup process
    Init,
    /// Starts the server
    Start,
    /// Allows you to manage tokens from the CLI
    #[command(subcommand)]
    Tokens(Tokens),
    /// Creates the sqlite database if it doesn't exist and creates the required tables
    Migrate,
    /// Allows you to manage buckets from the CLI
    #[command(subcommand)]
    Buckets(Buckets),
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

#[derive(Subcommand, Debug)]
pub enum Buckets {
    /// List all buckets
    List,
    /// Create a new bucket
    Create {
        /// Name of the token
        name: String,
    },
    Delete {
        /// Deletes the bucket
        name: String
    }
}