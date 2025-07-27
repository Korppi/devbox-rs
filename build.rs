// build.rs
use clap::CommandFactory;
use clap_mangen::Man;
use std::{fs, path::Path};

// Ladataan CLI-määrittely suoraan lähdekoodista
#[path = "src/cli.rs"]
mod cli;

fn main() {
    // 1) Hae Cargo:n build-scriptien ulostulohakemisto
    let binding = std::env::var_os("OUT_DIR");
    let out_dir = binding
        .as_deref() // Option<OsString> → Option<&OsStr>
        .map(Path::new) // nyt Path::new saa &OsStr
        .expect("OUT_DIR not set");

    // 2) Luo hakemisto jos ei ole
    fs::create_dir_all(out_dir).expect("ei voitu luoda OUT_DIR-hakemistoa");

    // 3) Rakenna Clap-komento ja generoi man-sivu
    let cmd = cli::Cli::command();
    let man = Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer).expect("man-render epäonnistui");

    // 4) Kirjoita man-sivu tiedostoksi OUT_DIR/devbox.1
    let dest = out_dir.join("devbox.1");
    fs::write(&dest, buffer).expect("ei voitu kirjoittaa devbox.1");

    // Jos cli.rs muuttuu, rerun build-script
    println!("cargo:rerun-if-changed=src/cli.rs");
}
