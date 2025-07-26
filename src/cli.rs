use clap::{ColorChoice, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devbox", 
about = "Developer's toolbox",
color = ColorChoice::Auto)]
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
}
