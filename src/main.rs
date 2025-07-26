mod cli;
mod commands;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ip { public, local } => {
            let _ = commands::ip::run(
                *public,
                *local,
                &mut std::io::stdout(),
                &mut std::io::stderr(),
            );
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(*shell, &mut cmd, "devbox", &mut std::io::stdout());
        }
    }
}
