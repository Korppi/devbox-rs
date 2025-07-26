mod cli;
mod commands;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ip { public, local } => {
            let _ = commands::ip::run(*public, *local, &mut std::io::stdout(), &mut std::io::stderr());
        }
    }
}
