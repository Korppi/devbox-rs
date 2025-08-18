## [0.4.3] – 2025‑08-18
### Fixed  
- `install.sh` script now handles man page generation better, even is installed multiple time by building from source

## [0.4.2] – 2025‑08-18
### Fixed  
- `pigsay` -subcommand bubble width and padding is now calculated by character count, not byte count

## [0.4.1] – 2025‑08-17
### Fixed  
- `pigsay` -subcommand now builds bubble better, if width is not given by user and given input is short

## [0.4.0] – 2025‑08-17
### Added  
- `pigsay` -subcommand

## [0.3.0] – 2025‑08-12
### Added  
- `now` -subcommand

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
