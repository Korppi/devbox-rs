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

# 5) Generoi bash/zsh-täydennys
COMPLETION_DIR="${HOME}/.devbox"
mkdir -p "$COMPLETION_DIR"
echo "Generating bash completions to $COMPLETION_DIR/devbox.bash..."
DEVBOX_BIN="$(command -v devbox || true)"
if [ -z "$DEVBOX_BIN" ]; then
  DEVBOX_BIN="$HOME/.cargo/bin/devbox"
fi
"$DEVBOX_BIN" completions bash > "$COMPLETION_DIR/devbox.bash"


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

# ===============================
# Man-sivun asennus (robusti)
# ===============================
# Selvitä Cargo:n target-hakemisto JSON:lla
TARGET_DIR=$(cargo metadata --format-version=1 --no-deps | jq -r '.target_directory')

# Etsi uusin devbox.1 release-buildista
echo "Searching for generated man page (devbox.1)..."
MAN_SOURCE=$(find "$TARGET_DIR/release/build" -type f -path "*/out/devbox.1" -printf "%T@ %p\n" 2>/dev/null | sort -n | tail -1 | cut -d' ' -f2-)

if [ -z "$MAN_SOURCE" ] || [ ! -f "$MAN_SOURCE" ]; then
  echo "Huom: man-sivua ei löytynyt. Ohitetaan man-asennus." >&2
else
  echo "Asennetaan man-sivu: $MAN_SOURCE → /usr/local/share/man/man1/devbox.1"
  if sudo -n true 2>/dev/null; then
    sudo install -Dm644 "$MAN_SOURCE" /usr/local/share/man/man1/devbox.1
    if command -v mandb >/dev/null 2>&1; then
      sudo mandb
    fi
    echo "Man-sivu asennettu."
  else
    # Fallback käyttäjän omaan hakemistoon jos sudoa ei ole
    USER_MAN="$HOME/.local/share/man/man1"
    mkdir -p "$USER_MAN"
    install -Dm644 "$MAN_SOURCE" "$USER_MAN/devbox.1"
    if command -v mandb >/dev/null 2>&1; then
      mandb --user
    fi
    echo "Man-sivu asennettu käyttäjän hakemistoon: $USER_MAN/devbox.1"
    echo "Lisää tarvittaessa PATH man:ille: export MANPATH=\"$HOME/.local/share/man:\$MANPATH\""
  fi
fi

echo "Man-sivu asennettu."
echo "Asennus valmis! Sulje ja avaa terminaali uudelleen, tai aja:"
echo "   source ~/.bashrc"
echo "   source ~/.devbox/devbox.bash"
