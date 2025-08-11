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
    /// Encode or decode Base64 strings using RFC4648.
    #[command(visible_alias = "b64")]
    Base64 {
        #[command(subcommand)]
        command: Base64Commands,
    },
    /// Show public or local IP of this machine
    #[command(
        about = "Show public and/or local IP address.",
        after_help = "Examples:\n  devbox ip \n  devbox ip --public \n  devbox ip -p"
    )]
    Ip {
        /// Show only local IP address
        #[arg(short, long)]
        local: bool,

        /// Show only public IP address
        #[arg(short, long)]
        public: bool,
    },
    /// Show current date and time
    #[command(about = "Show current date, time, weekday and ISO week number")]
    Now {
        /// Use ISO 8601 format (e.g. "2025-07-28T14:23:00+02:00")
        #[arg(short = 'I', long)]
        iso: bool,

        /// Specify a timezone (e.g. "America/New_York")
        #[arg(long, value_name = "TZ", conflicts_with = "utc")]
        tz: Option<String>,

        /// Show UTC time instead of local
        #[arg(short, long, conflicts_with = "tz")]
        utc: bool,

        /// Show week number
        #[arg(long)]
        week: bool,

        /// Show weekday
        #[arg(long)]
        weekday: bool,
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

    /// Encode text to base64
    Encode {
        /// Set output to JSON format
        #[arg(short, long)]
        json: bool,

        /// Omit base64 padding
        #[arg(long)]
        no_pad: bool,

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
