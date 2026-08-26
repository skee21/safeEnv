#!/bin/bash
set -e

INSTALL_DIR="$HOME/.safeEnv/bin"
EXE_PATH="$INSTALL_DIR/safeEnv"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$OS" = "linux" ]; then
    ASSET_NAME="safeEnv-linux-amd64"
    if [ "$ARCH" != "x86_64" ]; then
        echo "Warning: Linux builds are currently only provided for amd64."
    fi
elif [ "$OS" = "darwin" ]; then
    if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ASSET_NAME="safeEnv-macos-arm64"
    else
        ASSET_NAME="safeEnv-macos-amd64"
    fi
else
    echo "Unsupported OS: $OS"
    exit 1
fi

REPO_URL="https://github.com/skee21/safeEnv/releases/latest/download/$ASSET_NAME"

mkdir -p "$INSTALL_DIR"

echo "Downloading safeEnv..."
curl -sSL "$REPO_URL" -o "$EXE_PATH"
chmod +x "$EXE_PATH"

echo "Installing to $INSTALL_DIR"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Attempting to add to PATH..."
    PROFILE_FILE="$HOME/.bashrc"
    if [ -n "$ZSH_VERSION" ]; then
        PROFILE_FILE="$HOME/.zshrc"
    fi
    
    if echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$PROFILE_FILE"; then
        echo "Successfully added $INSTALL_DIR to your PATH in $PROFILE_FILE."
        echo "Run 'source $PROFILE_FILE' or restart your terminal."
    else
        echo -e "\033[33mWARNING: Failed to automatically add to PATH.\033[0m"
        echo -e "\033[33mPlease manually add this line to your shell profile:\033[0m"
        echo -e "\033[33mexport PATH=\"\$PATH:$INSTALL_DIR\"\033[0m"
    fi
else
    echo "Directory $INSTALL_DIR is already in your PATH."
fi