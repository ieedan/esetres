use clap::Subcommand;

pub mod start;

pub mod tokens;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Starts the server
    Start {
        /// Port to start the server on defaults to `3000` when the `--local` flag is set and `8080` otherwise
        #[clap(short, long, value_parser)]
        port: Option<u16>,
        /// Determines how the server should start whether is should start on `localhost` or `0.0.0.0`
        #[clap(short, long, action = clap::ArgAction::SetTrue)]
        local: bool,
    },
    /// Allows you to manage tokens from the CLI
    #[command(subcommand)]
    Tokens(Tokens),
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
        scope: Option<String>
    },
    /// Revoke an existing token
    Revoke {
        /// Name of the token
        name: String,
    },
}
