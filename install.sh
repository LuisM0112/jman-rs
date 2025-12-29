#!/usr/bin/env bash

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

API_URL="https://api.github.com/repos/$REPO/releases"

LATEST_TAG="$(curl -fsSL "$API_URL/latest" 2>/dev/null \
  | grep '"tag_name"' \
  | cut -d '"' -f4 || true)"

if [[ -z "$LATEST_TAG" ]]; then
  echo "No stable release found, falling back to pre-release..."

  LATEST_TAG="$(curl -fsSL "$API_URL" \
    | grep '"tag_name"' \
    | head -n 1 \
    | cut -d '"' -f4)"

  if [[ -z "$LATEST_TAG" ]]; then
    echo "No releases or pre-releases found"
    exit 1
  fi
fi

FILE="${BINARY_NAME}-${PLATFORM}-${ARCH}"
URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$FILE"

echo "Downloading $URL"
curl -fL "$URL" -o "$BINARY_NAME"
chmod +x "$BINARY_NAME"

APP_DIR="$HOME/.jman"
mkdir -p $APP_DIR/bin
mv "$BINARY_NAME" "$APP_DIR/bin"

ENV_FILE="$APP_DIR/env"
touch "$ENV_FILE"

{
  echo "#!/bin/sh"
  echo "# jman shell setup"
  echo 'export PATH="$HOME/.jman/bin:$PATH"'
  echo 'export JAVA_HOME="$HOME/.jman/current"'
  echo 'export PATH="$JAVA_HOME/bin:$PATH"'
} >> "$ENV_FILE"

BASHRC="$HOME/.bashrc"
touch "$BASHRC"

if ! grep -q '. $HOME/.jman/env' "$BASHRC"; then
  {
    echo '. $HOME/.jman/env'
  } >> "$BASHRC"

  ADDED_TO_PATH=true
else
  ADDED_TO_PATH=false
fi

echo ""
echo "Installed jman to $APP_DIR"

if [[ "$ADDED_TO_PATH" = true ]]; then
  echo "Added ~/.jman/bin to your PATH in ~/.bashrc"
  echo "Restart your terminal or run: source ~/.bashrc"
else
  echo "~/.jman/bin already in PATH configuration"
fi

echo ""
echo "Run: jman --help"
