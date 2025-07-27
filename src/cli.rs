use clap::{ColorChoice, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devbox", 
about = "Developer's toolbox",
color = ColorChoice::Auto,
version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show public or local IP of this machine
    #[command(
        about = "Show public and/or local IP address.",
        after_help = "Examples:\n  devbox ip \n  devbox ip --public \n  devbox ip -p"
    )]
    Ip {
        /// Show only public IP address
        #[arg(short, long)]
        public: bool,

        /// Show only local IP address
        #[arg(short, long)]
        local: bool,
    },
    /// Encode or decode Base64 strings using RFC4648.
    Base64 {
        #[command(subcommand)]
        command: Base64Commands,
    },
    /// Generate shell completion script
    #[command(hide = true)]
    Completions {
        /// Shell to generate completions for (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
pub enum Base64Commands {
    /// Encode text to base64
    Encode {
        /// Set output to JSON format
        #[arg(short, long)]
        json: bool,

        /// Pretty-print JSON (only with --json)
        #[arg(long, requires = "json")]
        pretty: bool,

        /// Omit base64 padding
        #[arg(long)]
        no_pad: bool,

        /// Use URL-safe base64 variant
        #[arg(short, long)]
        urlsafe: bool,

        /// Input text (optional if using pipe)
        input: Option<String>,
    },

    /// Decode base64 to text
    Decode {
        /// Set output to JSON format
        #[arg(short, long)]
        json: bool,

        /// Pretty-print JSON (only with --json)
        #[arg(long, requires = "json")]
        pretty: bool,

        /// Use URL-safe base64 variant
        #[arg(short, long)]
        urlsafe: bool,

        /// Input text (optional if using pipe)
        input: Option<String>,
    },
}
