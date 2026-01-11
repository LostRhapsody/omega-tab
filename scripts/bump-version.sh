#!/bin/bash
#
# BetterNewTab Version Bump Script
# Bumps version, commits, tags, and optionally pushes to trigger a release
#
# Usage:
#   ./scripts/bump-version.sh <version>     # Set specific version (e.g., 0.2.0)
#   ./scripts/bump-version.sh patch         # Bump patch version (0.1.0 -> 0.1.1)
#   ./scripts/bump-version.sh minor         # Bump minor version (0.1.0 -> 0.2.0)
#   ./scripts/bump-version.sh major         # Bump major version (0.1.0 -> 1.0.0)
#
# Options:
#   --push    Push commits and tags to trigger release workflow
#   --dry-run Show what would happen without making changes
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the root directory of the project
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="${PROJECT_ROOT}/server/Cargo.toml"

# Parse arguments
VERSION_ARG=""
PUSH=false
DRY_RUN=false

for arg in "$@"; do
    case $arg in
        --push)
            PUSH=true
            ;;
        --dry-run)
            DRY_RUN=true
            ;;
        *)
            VERSION_ARG="$arg"
            ;;
    esac
done

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Parse semver into components
parse_version() {
    local version="$1"
    echo "$version" | sed 's/\./ /g'
}

# Calculate new version based on bump type
calculate_new_version() {
    local current="$1"
    local bump_type="$2"

    read -r major minor patch <<< "$(parse_version "$current")"

    case $bump_type in
        major)
            echo "$((major + 1)).0.0"
            ;;
        minor)
            echo "${major}.$((minor + 1)).0"
            ;;
        patch)
            echo "${major}.${minor}.$((patch + 1))"
            ;;
        *)
            # Assume it's a specific version number
            echo "$bump_type"
            ;;
    esac
}

# Validate version format
validate_version() {
    local version="$1"
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
        echo -e "${RED}Error: Invalid version format '$version'${NC}"
        echo "Expected format: MAJOR.MINOR.PATCH (e.g., 1.2.3) or MAJOR.MINOR.PATCH-PRERELEASE (e.g., 1.2.3-beta.1)"
        exit 1
    fi
}

# Update version in Cargo.toml
update_cargo_toml() {
    local new_version="$1"
    if [ "$DRY_RUN" = true ]; then
        echo -e "${BLUE}[DRY RUN] Would update Cargo.toml version to $new_version${NC}"
    else
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML"
        rm -f "${CARGO_TOML}.bak"
        echo -e "${GREEN}Updated Cargo.toml version to $new_version${NC}"
    fi
}

# Update Cargo.lock by running cargo check
update_cargo_lock() {
    if [ "$DRY_RUN" = true ]; then
        echo -e "${BLUE}[DRY RUN] Would update Cargo.lock${NC}"
    else
        echo -e "${YELLOW}Updating Cargo.lock...${NC}"
        cd "${PROJECT_ROOT}/server"
        cargo check --quiet 2>/dev/null || true
        cd "$PROJECT_ROOT"
        echo -e "${GREEN}Cargo.lock updated${NC}"
    fi
}

# Create git commit and tag
create_commit_and_tag() {
    local new_version="$1"
    local tag="v${new_version}"

    if [ "$DRY_RUN" = true ]; then
        echo -e "${BLUE}[DRY RUN] Would commit with message: 'chore: bump version to $tag'${NC}"
        echo -e "${BLUE}[DRY RUN] Would create tag: $tag${NC}"
    else
        cd "$PROJECT_ROOT"

        # Stage changes
        git add server/Cargo.toml server/Cargo.lock 2>/dev/null || git add server/Cargo.toml

        # Commit
        git commit -m "chore: bump version to $tag"
        echo -e "${GREEN}Created commit${NC}"

        # Tag
        git tag -a "$tag" -m "Release $tag"
        echo -e "${GREEN}Created tag: $tag${NC}"
    fi
}

# Push to remote
push_to_remote() {
    local tag="v${1}"

    if [ "$DRY_RUN" = true ]; then
        echo -e "${BLUE}[DRY RUN] Would push commits and tag $tag to origin${NC}"
    else
        echo -e "${YELLOW}Pushing to origin...${NC}"
        git push origin HEAD
        git push origin "$tag"
        echo -e "${GREEN}Pushed commits and tag to origin${NC}"
        echo ""
        echo -e "${GREEN}Release workflow triggered!${NC}"
        echo -e "View progress at: ${BLUE}https://github.com/LostRhapsody/betternewtab/actions${NC}"
    fi
}

# Main script
main() {
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}    BetterNewTab Version Bump${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""

    # Check for version argument
    if [ -z "$VERSION_ARG" ]; then
        echo -e "${RED}Error: Version argument required${NC}"
        echo ""
        echo "Usage:"
        echo "  $0 <version>     # Set specific version (e.g., 0.2.0)"
        echo "  $0 patch         # Bump patch version"
        echo "  $0 minor         # Bump minor version"
        echo "  $0 major         # Bump major version"
        echo ""
        echo "Options:"
        echo "  --push           Push to trigger release workflow"
        echo "  --dry-run        Show what would happen"
        exit 1
    fi

    # Get current version
    CURRENT_VERSION=$(get_current_version)
    echo -e "Current version: ${YELLOW}$CURRENT_VERSION${NC}"

    # Calculate new version
    NEW_VERSION=$(calculate_new_version "$CURRENT_VERSION" "$VERSION_ARG")

    # Validate
    validate_version "$NEW_VERSION"

    echo -e "New version:     ${GREEN}$NEW_VERSION${NC}"
    echo ""

    # Check if tag already exists
    if git tag -l "v$NEW_VERSION" | grep -q "v$NEW_VERSION"; then
        echo -e "${RED}Error: Tag v$NEW_VERSION already exists${NC}"
        exit 1
    fi

    # Check for uncommitted changes
    if ! git diff --quiet HEAD 2>/dev/null; then
        echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
        if [ "$DRY_RUN" = false ]; then
            read -p "Continue anyway? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                exit 1
            fi
        fi
    fi

    # Perform updates
    update_cargo_toml "$NEW_VERSION"
    update_cargo_lock
    echo ""

    create_commit_and_tag "$NEW_VERSION"
    echo ""

    if [ "$PUSH" = true ]; then
        push_to_remote "$NEW_VERSION"
    else
        echo -e "${YELLOW}To trigger the release workflow, run:${NC}"
        echo "  git push origin HEAD"
        echo "  git push origin v$NEW_VERSION"
        echo ""
        echo -e "${YELLOW}Or run this script with --push:${NC}"
        echo "  $0 $VERSION_ARG --push"
    fi

    echo ""
    echo -e "${GREEN}Done!${NC}"
}

main
