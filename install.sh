#!/usr/bin/env bash
set -e

REPO="LuisM0112/jman-rs"
BINARY_NAME="jman"

OS="$(uname -s)"
ARCH="$(uname -m)"

if [[ "$OS" == "Linux" ]]; then
  PLATFORM="linux"
elif [[ "$OS" == "Darwin" ]]; then
  PLATFORM="macos"
else
  echo "Unsupported OS: $OS"
  exit 1
fi

if [[ "$ARCH" == "x86_64" ]]; then
  ARCH="x86_64"
elif [[ "$ARCH" == "arm64" || "$ARCH" == "aarch64" ]]; then
  ARCH="aarch64"
else
  echo "Unsupported architecture: $ARCH"
  exit 1
fi

LATEST=$(curl -s https://api.github.com/repos/$REPO/releases/latest \
  | grep '"tag_name"' \
  | cut -d '"' -f4)

if [[ -z "$LATEST" ]]; then
  echo "Could not detect latest version"
  exit 1
fi

FILE="${BINARY_NAME}-${PLATFORM}-${ARCH}"

URL="https://github.com/$REPO/releases/download/$LATEST/$FILE"

echo "Downloading $URL"

curl -fL "$URL" -o "$BINARY_NAME"

chmod +x "$BINARY_NAME"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

mv "$BINARY_NAME" "$INSTALL_DIR/"

echo ""
echo "✅ Installed jman to $INSTALL_DIR"
echo "Make sure this is in your PATH:"
echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "Run: jman --help"
