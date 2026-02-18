#!/bin/sh
set -e

# Detect OS and Arch
OS=$(uname -s)
ARCH=$(uname -m)

REPO="SAMIR897/rfetch"

case "$OS" in
    Linux)
        ASSET="rfetch-linux-amd64"
        ;;
    Darwin)
        if [ "$ARCH" = "arm64" ]; then
            ASSET="rfetch-macos-arm64"
        else
            ASSET="rfetch-macos-amd64"
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Downloading rfetch for $OS ($ARCH)..."

# Get latest release tag
LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "Error: Could not find latest release tag."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$ASSET.tar.gz"

curl -L -o rfetch.tar.gz "$URL"

echo "Extracting..."
tar -xzf rfetch.tar.gz

echo "Installing to /usr/local/bin..."
sudo mv rfetch /usr/local/bin/
rm rfetch.tar.gz

echo "Done! Run 'rfetch' to test it."
