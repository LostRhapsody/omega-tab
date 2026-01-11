#!/bin/bash
#
# Better New Tab Installer
# Downloads and installs the latest release from GitHub
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

REPO="LostRhapsody/betternewtab"
BINARY_NAME="BetterNewTab"

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$OS" in
        linux)
            OS="linux"
            ;;
        darwin)
            OS="macos"
            ;;
        mingw*|msys*|cygwin*)
            OS="windows"
            ;;
        *)
            echo -e "${RED}Unsupported operating system: $OS${NC}"
            exit 1
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="aarch64"
            ;;
        *)
            echo -e "${RED}Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac

    echo -e "${GREEN}Detected platform: ${OS}-${ARCH}${NC}"
}

# Get the latest release version from GitHub
get_latest_version() {
    echo -e "${YELLOW}Fetching latest release...${NC}"
    VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$VERSION" ]; then
        echo -e "${RED}Failed to fetch latest version${NC}"
        exit 1
    fi

    echo -e "${GREEN}Latest version: ${VERSION}${NC}"
}

# Download the binary
download_binary() {
    FILENAME="${BINARY_NAME}-${OS}-${ARCH}"
    if [ "$OS" = "windows" ]; then
        FILENAME="${FILENAME}.exe"
    fi

    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"

    echo -e "${YELLOW}Downloading from: ${DOWNLOAD_URL}${NC}"

    TMP_DIR=$(mktemp -d)
    TMP_FILE="${TMP_DIR}/${FILENAME}"

    if ! curl -L -o "$TMP_FILE" "$DOWNLOAD_URL"; then
        echo -e "${RED}Failed to download binary${NC}"
        rm -rf "$TMP_DIR"
        exit 1
    fi

    chmod +x "$TMP_FILE"
    echo -e "${GREEN}Downloaded successfully${NC}"
}

# Install the binary
install_binary() {
    if [ "$OS" = "linux" ] || [ "$OS" = "macos" ]; then
        INSTALL_DIR="/usr/local/bin"
        INSTALL_PATH="${INSTALL_DIR}/${BINARY_NAME}"

        echo -e "${YELLOW}Installing to ${INSTALL_PATH}...${NC}"

        if [ -w "$INSTALL_DIR" ]; then
            mv "$TMP_FILE" "$INSTALL_PATH"
        else
            echo -e "${YELLOW}Requesting sudo access to install to ${INSTALL_DIR}${NC}"
            sudo mv "$TMP_FILE" "$INSTALL_PATH"
        fi

        echo -e "${GREEN}Installed successfully!${NC}"
        echo -e "${GREEN}Run '${BINARY_NAME}' to start Better New Tab${NC}"
    elif [ "$OS" = "windows" ]; then
        INSTALL_DIR="$HOME/AppData/Local/BetterNewTab"
        INSTALL_PATH="${INSTALL_DIR}/${BINARY_NAME}.exe"

        mkdir -p "$INSTALL_DIR"
        mv "$TMP_FILE" "$INSTALL_PATH"

        echo -e "${GREEN}Installed to ${INSTALL_PATH}${NC}"
        echo -e "${YELLOW}Note: You may want to add ${INSTALL_DIR} to your PATH${NC}"
    fi

    rm -rf "$TMP_DIR"
}

# Main installation flow
main() {
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}    Better New Tab Installer${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""

    detect_platform
    get_latest_version
    download_binary
    install_binary

    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}    Installation Complete!${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo "Better New Tab is now installed."
    echo "Run 'BetterNewTab' to start the application."
    echo ""
    echo "The application will:"
    echo "  - Start a local server on http://127.0.0.1:3000"
    echo "  - Create a system tray icon"
    echo "  - Store data in your local app data directory"
    echo ""
}

main "$@"
