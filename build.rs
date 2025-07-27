// build.rs
use std::{fs, path::Path};

use clap::CommandFactory;
use clap_mangen::Man;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    // 1) Luo man/-hakemisto
    let out_dir = Path::new("man");
    fs::create_dir_all(out_dir).expect("could not create man directory");

    // 2) Tee Clap-Command
    let cmd = cli::Cli::command();

    // 3) Generoi man-sivu
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer).expect("could not render man page");

    // 4) Tallenna tiedostoksi
    fs::write(out_dir.join("devbox.1"), buffer).expect("could not write man/devbox.1");
}
