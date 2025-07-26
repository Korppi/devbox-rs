#!/usr/bin/env bash
set -e

# Switch to the directory where this script lives
cd "$(dirname "${BASH_SOURCE[0]}")"

# Optional guard
if [ ! -f Cargo.toml ]; then
  echo "Error: Cargo.toml not found. Please run install.sh from the project root." >&2
  exit 1
fi

echo "Building Devbox with cargo..."
echo "🔧 Building Devbox with cargo..."
cargo build --release

echo "🧩 Generating bash completions..."
mkdir -p ~/.devbox
./target/release/devbox completions bash > ~/.devbox/devbox.bash

# Shell profile update
PROFILE_FILE="${HOME}/.bashrc"
if [[ "$SHELL" == *zsh ]]; then
    PROFILE_FILE="${HOME}/.zshrc"
fi

if ! grep -q 'devbox.bash' "$PROFILE_FILE"; then
    echo "➕ Adding completion script to $PROFILE_FILE"
    echo "" >> "$PROFILE_FILE"
    echo "# Devbox completions" >> "$PROFILE_FILE"
    echo "source ~/.devbox/devbox.bash" >> "$PROFILE_FILE"
else
    echo "✅ Completion script already present in $PROFILE_FILE"
fi

echo "✅ Done! Restart your terminal or run 'source ~/.devbox/devbox.bash' to enable completions immediately."
