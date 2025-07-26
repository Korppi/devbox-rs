use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devbox", 
about = "Developer's toolbox")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show public or local IP of this machine
    Ip {
        /// Show only public IP
        #[arg(short, long)]
        public: bool,

        /// Show only local IP
        #[arg(short, long)]
        local: bool,
    },
}
