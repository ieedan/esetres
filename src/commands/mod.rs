use clap::Subcommand;

pub mod start;

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
}