#!/bin/bash
#
# Better New Tab Release Build Script
# Builds the Vue frontend and Rust server for distribution
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the root directory of the project
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Output directory
DIST_DIR="${PROJECT_ROOT}/dist"

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}    Better New Tab Release Build${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build Vue frontend
echo -e "${YELLOW}Building Vue frontend...${NC}"
cd "${PROJECT_ROOT}/client"

if ! command -v npm &> /dev/null; then
    echo -e "${RED}npm is not installed. Please install Node.js and npm.${NC}"
    exit 1
fi

npm install
npm run build

if [ ! -d "dist" ]; then
    echo -e "${RED}Frontend build failed - dist directory not found${NC}"
    exit 1
fi

echo -e "${GREEN}Frontend build complete!${NC}"

# Build Rust server
echo -e "${YELLOW}Building Rust server...${NC}"
cd "${PROJECT_ROOT}/server"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}cargo is not installed. Please install Rust.${NC}"
    exit 1
fi

cargo build --release

# Find the built binary
BINARY_NAME="BetterNewTab"
TARGET_DIR="${PROJECT_ROOT}/server/target/release"

if [ -f "${TARGET_DIR}/${BINARY_NAME}" ]; then
    cp "${TARGET_DIR}/${BINARY_NAME}" "${DIST_DIR}/"
    echo -e "${GREEN}Binary copied to ${DIST_DIR}/${BINARY_NAME}${NC}"
elif [ -f "${TARGET_DIR}/${BINARY_NAME}.exe" ]; then
    cp "${TARGET_DIR}/${BINARY_NAME}.exe" "${DIST_DIR}/"
    echo -e "${GREEN}Binary copied to ${DIST_DIR}/${BINARY_NAME}.exe${NC}"
else
    echo -e "${RED}Could not find built binary${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}    Build Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Output files are in: ${DIST_DIR}"
echo ""

# Display file sizes
echo "Build artifacts:"
ls -lh "${DIST_DIR}"
echo ""

# Platform-specific notes
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
if [ "$OS" = "darwin" ]; then
    echo -e "${YELLOW}Note: To create a macOS .app bundle, use additional tooling.${NC}"
elif [ "$OS" = "linux" ]; then
    echo -e "${YELLOW}Note: For cross-compilation, install the appropriate targets:${NC}"
    echo "  rustup target add x86_64-pc-windows-gnu"
    echo "  rustup target add aarch64-apple-darwin"
fi
