## [0.2.0] – 2025‑07‑26
### Added  
- `base64 encode` -subcommand
- `base64 decode` -subcommand

### Changed
- Hide `completions`‑subcommand, so it wont show up in `--help` listing

### Fixed
- Fixed `install.sh`:
  - Script automatically changes to correct directory (`cd "$(dirname …)"`)  
  - Check `Cargo.toml` exists  
  - Makes sure `~/.cargo/bin` is in the `PATH` and added in `~/.bashrc`/`~/.zshrc`  
