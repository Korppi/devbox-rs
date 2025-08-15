# Devbox

![Rust](https://img.shields.io/badge/language-Rust-red) ![license: MIT](https://img.shields.io/badge/license-MIT-green) [![Coverage](https://github.com/Korppi/devbox-rs/actions/workflows/coverage.yaml/badge.svg)](https://github.com/Korppi/devbox-rs/actions/workflows/coverage.yaml) [![CI](https://github.com/Korppi/devbox-rs/actions/workflows/ci.yaml/badge.svg?branch=development)](https://github.com/Korppi/devbox-rs/actions/workflows/ci.yaml) [![Release](https://github.com/Korppi/devbox-rs/actions/workflows/release.yaml/badge.svg?branch=master)](https://github.com/Korppi/devbox-rs/actions/workflows/release.yaml)



Devbox is a modular CLI toolbox for Linux admins, developers and power-users.  
It’s built with Rust and Clap, and aggregates a growing set of small, focused commands—  
from networking helpers to Base64 encoding/decoding and beyond.

## ⚙️ Features

- **`base64 encode|decode`**  
  Encode or decode Base64 strings using RFC4648.
  - `--url`, `-u` Use URL-safe alphabet (`-` / `_`)  
  - `--no-pad` Remove padding (`=`)  
  - `--json`, `-j` Wrap result in JSON  
  - `--pretty` Indent JSON (works only with `--json`)
- **`ip`**  
  Show public and/or local IP address of this machine.
  - `--local`, `-l` Show local IP address
  - `--public`, `-p` Show public IP address
- **`now`**
  Show current date and time. 
  - `--iso`, `-I` Show datetime in iso format (ISO 8601)
  - `--tz` Show datetime at wanted timezone
  - `--utc`, `-u` Show UTC datetime
  - `--week` Show week number
- **`pigsay`**
  Make pig say things. 
  - `--eye`, `-e` Change pig eye
  - `--tail`, `-t` Change pig tail

## 🗺️ Roadmap

### Features 
- **pigsay** Cowsay clone! Make pig say things!
- **find** Find and optionally delete files by size/age
- **hangman** Hangman game.  

### Code quality
- Add more tests through `Cucumber` or `Robot framework`

## 🧪 Quality

- ✅ Automated tests run on every push via **GitHub Actions** including cargo fmt and cargo clippy checks
- 🚀 Release builds are created automatically when tags are pushed to `master`
- 🔀 Uses `master` / `development` / `feature/*` branching strategy
- 🧾 Commits follow the [Conventional Commits](https://www.conventionalcommits.org) standard

## 🚀 Installation

### Option 1 – Pre-built binary 
Please download from [Releases](https://github.com/Korppi/devbox-rs/releases).

### Option 2 – Build from source (recommended) 
Please install [Rust and Cargo](https://www.rust-lang.org/tools/install) first.

#### Windows (PowerShell)

```powershell
git clone https://github.com/Korppi/devbox-rs.git
cd devbox-rs
.\install.ps1
```
❗ If you see an error like script execution is disabled, run instead:
```powershell
powershell -ExecutionPolicy Bypass -File install.ps1
```
#### Linux

```bash
git clone https://github.com/Korppi/devbox-rs.git
cd devbox-rs
./install.sh
```
If it fails with "permission denied":
```bash
chmod +x install.sh
```

If it fails with "Could not find openssl via pkg-config" then do following if using Almalinux or similar:
```bash
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y openssl-devel pkgconf pkgconf-pkg-config
# check that pkg-config finds openssl, this will print something like -I/usr/include ... -lssl -lcrypto  
pkg-config --libs --cflags openssl
```

After this run install.sh again.
```bash
./install.sh
```

## 🛠️ Why Devbox?

- ❤️ **Hobby**  
  A fun side-project and portfolio showcase.  
- 🎓 **Learning**  
  Exploring Rust, Clap and CLI best practices.  
- 🚀 **Practical**  
  Simplify or improve upon complex existing tools (e.g. `find`)  

## 📄 License

MIT © [Teppo Kavander](https://github.com/Korppi)
