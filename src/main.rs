mod cli;
mod commands;
mod utils;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use devbox::cli::{Base64Commands, Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let stdout = &mut std::io::stdout();
    let stderr = &mut std::io::stderr();

    match &cli.command {
        Commands::Ip { public, local } => {
            let _ = commands::ip::run(*public, *local, stdout, stderr);
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(*shell, &mut cmd, "devbox", &mut std::io::stdout());
        }
        Commands::Base64 { command } => match command {
            Base64Commands::Encode {
                json,
                pretty,
                no_pad,
                urlsafe,
                input,
            } => {
                let _ = commands::b64::encode::run(
                    *json,
                    *pretty,
                    *no_pad,
                    *urlsafe,
                    input.clone(),
                    stdout,
                    stderr,
                );
            }
            Base64Commands::Decode {
                json,
                pretty,
                urlsafe,
                input,
            } => {
                let _ = commands::b64::decode::run(
                    *json,
                    *pretty,
                    *urlsafe,
                    input.clone(),
                    stdout,
                    stderr,
                );
            }
        },
        Commands::Now { iso, utc, tz, week } => {
            let _ = commands::now::run(*iso, *utc, *week, tz.clone(), stdout, stderr);
        }
        Commands::Pigsay { eye, tail, input, width } => {
            let _ = commands::pigsay::run(*eye, *tail, input.clone(), *width as usize, stdout, stderr);
        }
    }
}
