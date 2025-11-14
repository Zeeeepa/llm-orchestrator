#!/bin/bash

# Fix and Republish - Yank old versions and publish corrected ones
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check authentication
if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
    if ! grep -q "token" ~/.cargo/credentials.toml 2>/dev/null; then
        print_error "Not logged in to crates.io!"
        echo "Please run: cargo login YOUR_TOKEN"
        exit 1
    fi
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FIX AND REPUBLISH CRATES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

print_warning "This will YANK the old 0.1.0 versions and republish with fixes"
echo ""
echo "Crates to yank and republish:"
echo "  - llm-orchestrator-providers"
echo "  - llm-orchestrator-audit"
echo "  - llm-orchestrator-core"
echo "  - llm-orchestrator-sdk"
echo ""

read -p "Proceed? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_warning "Operation cancelled"
    exit 0
fi

echo ""

# Yank old versions
print_step "Yanking old 0.1.0 versions..."
~/.cargo/bin/cargo yank --vers 0.1.0 llm-orchestrator-providers || print_warning "  Already yanked or not found: providers"
~/.cargo/bin/cargo yank --vers 0.1.0 llm-orchestrator-audit || print_warning "  Already yanked or not found: audit"
~/.cargo/bin/cargo yank --vers 0.1.0 llm-orchestrator-core || print_warning "  Already yanked or not found: core"
~/.cargo/bin/cargo yank --vers 0.1.0 llm-orchestrator-sdk || print_warning "  Already yanked or not found: sdk"
print_success "Yanking complete"
echo ""

# Wait for yank to propagate
print_warning "Waiting 30s for yank to propagate..."
sleep 30
print_success "Wait complete"
echo ""

# Publish in correct order
declare -a CRATES=(
    "llm-orchestrator-providers"
    "llm-orchestrator-audit"
    "llm-orchestrator-core"
    "llm-orchestrator-sdk"
    "llm-orchestrator-cli"
)

PUBLISHED=()
FAILED=()

for crate in "${CRATES[@]}"; do
    print_step "Publishing $crate..."

    cd "crates/$crate"

    # Publish with --allow-dirty in case of uncommitted changes
    if ~/.cargo/bin/cargo publish --allow-dirty 2>&1; then
        print_success "  ✓ Published $crate"
        PUBLISHED+=("$crate")
    else
        print_error "  ✗ Failed to publish $crate"
        FAILED+=("$crate")
        cd ../..
        continue
    fi

    cd ../..

    # Wait for indexing after key crates
    if [ "$crate" = "llm-orchestrator-providers" ] || [ "$crate" = "llm-orchestrator-core" ] || [ "$crate" = "llm-orchestrator-sdk" ]; then
        print_warning "Waiting 180s for crates.io to index $crate..."
        for ((i=180; i>0; i-=10)); do
            echo -n "  ${i}s remaining..."
            sleep 10
            echo " ✓"
        done
        print_success "Indexing wait complete"
    fi

    echo ""
done

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ ${#PUBLISHED[@]} -gt 0 ]; then
    print_success "Successfully published ${#PUBLISHED[@]} crates:"
    for crate in "${PUBLISHED[@]}"; do
        echo "  ✓ $crate"
        echo "    https://crates.io/crates/$crate"
    done
    echo ""
fi

if [ ${#FAILED[@]} -gt 0 ]; then
    print_error "Failed to publish ${#FAILED[@]} crates:"
    for crate in "${FAILED[@]}"; do
        echo "  ✗ $crate"
    done
    echo ""
    exit 1
fi

print_success "All crates published successfully! 🎉"
echo ""
