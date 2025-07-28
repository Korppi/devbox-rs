## [0.2.1] – 2025‑07‑28
### Added
- `base64` -subcommand now has alias b64

## [0.2.0] – 2025‑07‑27
### Added  
- `base64 encode` -subcommand
- `base64 decode` -subcommand
- `install.sh` now generates man pages on linux

### Changed
- Hide `completions`‑subcommand, so it wont show up in `--help` listing

### Fixed
- Fixed `install.sh`:
  - Script automatically changes to correct directory (`cd "$(dirname …)"`)  
  - Check `Cargo.toml` exists  
  - Makes sure `~/.cargo/bin` is in the `PATH` and added in `~/.bashrc`/`~/.zshrc`  
