# Devbox

![Rust](https://img.shields.io/badge/language-Rust-red) ![license: MIT](https://img.shields.io/badge/license-MIT-green)

Devbox is a modular CLI toolbox for Linux admins, developers and power-users.  
It’s built with Rust and Clap, and aggregates a growing set of small, focused commands—  
from networking helpers to Base64 encoding/decoding and beyond.

## ⚙️ Features

- **`ip`**  
  Show public and/or local IP address of this machine.

## 🗺️ Roadmap

- **base64** Ability for base64 encoding and decoding
- **now** Show current date, time, weekday & ISO week number  
- **clean-files** Find and optionally delete files by size/age 

## 🚀 Installation

### Option 1 – Pre-built binary 
➡️ *Coming soon on [Releases](https://github.com/Korppi/devbox-rs/releases)*

### Option 2 – Build from source 
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

## 🛠️ Why Devbox?

- ❤️ **Hobby**  
  A fun side-project and portfolio showcase.  
- 🎓 **Learning**  
  Exploring Rust, Clap and CLI best practices.  
- 🚀 **Practical**  
  Simplify or improve upon complex existing tools (e.g. `find`)  

## 📄 License

MIT © [Teppo Kavander](https://github.com/Korppi)
