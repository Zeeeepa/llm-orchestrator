#!/bin/bash

# Publish npm packages to @llm-dev-ops organization
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

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    print_error "npm is not installed. Please install Node.js and npm first."
    exit 1
fi

# Check if logged into npm
if ! npm whoami &> /dev/null; then
    print_error "Not logged into npm. Please run: npm login"
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "PUBLISH LLM ORCHESTRATOR TO NPM"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

print_warning "This script will publish the following packages to npm:"
echo "  - @llm-dev-ops/llm-orchestrator-linux-x64"
echo "  - @llm-dev-ops/llm-orchestrator (main package)"
echo ""
echo "Note: For other platforms (macOS, Windows, ARM), use GitHub Actions"
echo "or build the binaries manually and add them to the respective npm packages."
echo ""

read -p "Continue? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_warning "Publishing cancelled"
    exit 0
fi

echo ""

# Check if Linux binary exists
if [ ! -f "npm/llm-orchestrator-linux-x64/bin/llm-orchestrator" ]; then
    print_error "Linux x64 binary not found!"
    print_step "Building release binary..."
    cargo build --release -p llm-orchestrator-cli
    mkdir -p npm/llm-orchestrator-linux-x64/bin
    cp target/release/llm-orchestrator npm/llm-orchestrator-linux-x64/bin/
    chmod +x npm/llm-orchestrator-linux-x64/bin/llm-orchestrator
    print_success "Binary built and copied"
fi

# Publish Linux x64 package
print_step "Publishing @llm-dev-ops/llm-orchestrator-linux-x64..."
cd npm/llm-orchestrator-linux-x64
if npm publish --access public; then
    print_success "Published @llm-dev-ops/llm-orchestrator-linux-x64"
else
    print_error "Failed to publish llm-orchestrator-linux-x64"
    exit 1
fi
cd ../..

# Wait for npm to index
print_warning "Waiting 30s for npm to index the package..."
sleep 30

# Publish main package
print_step "Publishing @llm-dev-ops/llm-orchestrator..."
cd npm/llm-orchestrator
if npm publish --access public; then
    print_success "Published @llm-dev-ops/llm-orchestrator"
else
    print_error "Failed to publish llm-orchestrator"
    exit 1
fi
cd ../..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "SUCCESS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

print_success "Packages published to npm!"
echo ""
echo "Published packages:"
echo "  ✓ @llm-dev-ops/llm-orchestrator-linux-x64@0.1.1"
echo "    https://www.npmjs.com/package/@llm-dev-ops/llm-orchestrator-linux-x64"
echo ""
echo "  ✓ @llm-dev-ops/llm-orchestrator@0.1.1"
echo "    https://www.npmjs.com/package/@llm-dev-ops/llm-orchestrator"
echo ""
echo "Install with:"
echo "  npm install -g @llm-dev-ops/llm-orchestrator"
echo ""
echo "Note: Currently only Linux x64 is available."
echo "To publish for other platforms, use GitHub Actions:"
echo "  git tag v0.1.1"
echo "  git push origin v0.1.1"
echo ""
