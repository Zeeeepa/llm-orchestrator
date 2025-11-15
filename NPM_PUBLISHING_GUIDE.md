# NPM Publishing Guide

This guide explains how to publish the LLM Orchestrator to npm under the `@llm-dev-ops` organization.

## Package Structure

The npm distribution uses a multi-package approach (similar to esbuild, swc, turbo):

- **Main package**: `@llm-dev-ops/llm-orchestrator` - Platform detection and CLI wrapper
- **Platform packages**: Platform-specific binaries
  - `@llm-dev-ops/llm-orchestrator-linux-x64`
  - `@llm-dev-ops/llm-orchestrator-linux-arm64`
  - `@llm-dev-ops/llm-orchestrator-darwin-x64`
  - `@llm-dev-ops/llm-orchestrator-darwin-arm64`
  - `@llm-dev-ops/llm-orchestrator-win32-x64`

## Prerequisites

### For Manual Publishing

1. **npm account**: You need an npm account that is part of the `@llm-dev-ops` organization
2. **npm authentication**: Run `npm login` and authenticate

### For GitHub Actions Publishing

1. **NPM_SECRET**: Add your npm authentication token to GitHub repository secrets
   - Go to: Settings → Secrets and variables → Actions → New repository secret
   - Name: `NPM_SECRET`
   - Value: Your npm token (get from https://www.npmjs.com/settings/USERNAME/tokens)

## Publishing Methods

### Method 1: Automated Publishing via GitHub Actions (Recommended)

This method builds binaries for all platforms automatically:

```bash
# Create and push a version tag
git tag v0.1.1
git push origin v0.1.1
```

The GitHub Actions workflow will:
1. Build binaries for all platforms (Linux x64/ARM64, macOS x64/ARM64, Windows x64)
2. Publish all platform-specific packages
3. Publish the main package
4. Create a GitHub release

### Method 2: Manual Publishing (Linux only)

For quick testing or Linux-only distribution:

```bash
# Login to npm
npm login

# Run the publishing script
./scripts/publish-npm.sh
```

This will:
1. Build the Linux x64 binary (if not already built)
2. Publish `@llm-dev-ops/llm-orchestrator-linux-x64`
3. Publish `@llm-dev-ops/llm-orchestrator` (main package)

**Note**: With this method, only Linux x64 will be supported. Users on other platforms will get an error.

## Package Versions

All packages must have matching versions. Current version: **0.1.1**

To bump versions:

1. Update `Cargo.toml` (workspace version)
2. Update all `npm/*/package.json` files
3. Commit and tag

## Installation

After publishing, users can install with:

```bash
# Global installation
npm install -g @llm-dev-ops/llm-orchestrator

# Project installation
npm install @llm-dev-ops/llm-orchestrator
```

## Usage

### CLI

```bash
llm-orchestrator validate workflow.yaml
llm-orchestrator run workflow.yaml --input '{"key": "value"}'
```

### Programmatic API

```javascript
const orchestrator = require('@llm-dev-ops/llm-orchestrator');

// Run a workflow
const result = await orchestrator.run('workflow.yaml', {
  input: JSON.stringify({ query: 'What is AI?' })
});

console.log(result.stdout);
```

## Troubleshooting

### "Package already exists"

You cannot republish the same version. Bump the version number:

```bash
# Update versions in:
# - Cargo.toml (workspace.package.version)
# - npm/*/package.json files
```

### "Unsupported platform"

If you published with Method 2 (manual), only Linux x64 is available. Use Method 1 (GitHub Actions) to publish for all platforms.

### "Permission denied"

Ensure you're logged into npm and have permissions for the `@llm-dev-ops` organization:

```bash
npm login
npm access ls-packages @llm-dev-ops
```

## Links

- npm organization: https://www.npmjs.com/org/llm-dev-ops
- Main package: https://www.npmjs.com/package/@llm-dev-ops/llm-orchestrator
- GitHub repository: https://github.com/globalbusinessadvisors/llm-orchestrator
