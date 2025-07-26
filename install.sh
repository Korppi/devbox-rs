#!/usr/bin/env bash
set -e

# 1) Siirry skriptin omaan hakemistoon (projektin juureen)
cd "$(dirname "${BASH_SOURCE[0]}")"

# 2) Varmista, että ollaaan projektin juuressa
if [ ! -f Cargo.toml ]; then
  echo "Error: Cargo.toml not found. Please run install.sh from the project root." >&2
  exit 1
fi

# 3) Buildaa release‑binääri
echo "Building Devbox with cargo..."
cargo build --release

# 4) Asenna binääri käyttäjälle (~/.cargo/bin)
echo "Installing Devbox into Cargo bin directory..."
cargo install --path . --force

# 5) Generoi bash/zsh‑täydennys
COMPLETION_DIR="${HOME}/.devbox"
mkdir -p "$COMPLETION_DIR"
echo "Generating bash completions to $COMPLETION_DIR/devbox.bash..."
./target/release/devbox completions bash > "$COMPLETION_DIR/devbox.bash"

# 6) Varmista ~/.cargo/bin on PATHissa – lisää tarvittaessa ~/.bashrc:ään
if ! grep -q 'export PATH="$HOME/.cargo/bin' "${HOME}/.bashrc"; then
  echo "Adding ~/.cargo/bin to PATH in ~/.bashrc"
  {
    echo ""
    echo "# Add Cargo binaries"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"'
  } >> "${HOME}/.bashrc"
fi

# 7) Päivitä shell‐profiili
PROFILE_FILE="${HOME}/.bashrc"
if [[ "$SHELL" == *zsh* ]]; then
    PROFILE_FILE="${HOME}/.zshrc"
fi

if ! grep -q 'source ~/.devbox/devbox.bash' "$PROFILE_FILE"; then
    echo "Adding completion script to $PROFILE_FILE"
    {
        echo ""
        echo "# Devbox completions"
        echo "source ~/.devbox/devbox.bash"
    } >> "$PROFILE_FILE"
else
    echo "Completion script already present in $PROFILE_FILE"
fi

echo "Done! Restart your terminal or run:"
echo "  source ~/.bashrc"
echo "  source ~/.devbox/devbox.bash"
echo "Then you can use 'devbox' from anywhere."
