#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the platform-specific package name
function getPlatformPackage() {
  const platform = process.platform;
  const arch = process.arch;

  const packageMap = {
    'linux-x64': '@llm-dev-ops/llm-orchestrator-linux-x64',
    'linux-arm64': '@llm-dev-ops/llm-orchestrator-linux-arm64',
    'darwin-x64': '@llm-dev-ops/llm-orchestrator-darwin-x64',
    'darwin-arm64': '@llm-dev-ops/llm-orchestrator-darwin-arm64',
    'win32-x64': '@llm-dev-ops/llm-orchestrator-win32-x64',
  };

  const key = `${platform}-${arch}`;
  const packageName = packageMap[key];

  if (!packageName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error('Supported platforms: linux-x64, linux-arm64, darwin-x64, darwin-arm64, win32-x64');
    process.exit(1);
  }

  return packageName;
}

// Find the binary path
function getBinaryPath() {
  const packageName = getPlatformPackage();
  const binaryName = process.platform === 'win32' ? 'llm-orchestrator.exe' : 'llm-orchestrator';

  // Try to find the binary in node_modules
  let binaryPath = path.join(__dirname, '..', '..', packageName.replace('@llm-dev-ops/', ''), 'bin', binaryName);

  // Check if it exists
  if (!fs.existsSync(binaryPath)) {
    // Try alternative path (when installed globally or in different structure)
    binaryPath = path.join(__dirname, '..', '..', '..', packageName, 'bin', binaryName);
  }

  if (!fs.existsSync(binaryPath)) {
    console.error(`Could not find binary at: ${binaryPath}`);
    console.error(`Make sure ${packageName} is installed`);
    process.exit(1);
  }

  return binaryPath;
}

// Run the binary
function main() {
  try {
    const binaryPath = getBinaryPath();

    // Make sure the binary is executable (Unix-like systems)
    if (process.platform !== 'win32') {
      try {
        fs.chmodSync(binaryPath, 0o755);
      } catch (err) {
        // Ignore errors - might already be executable
      }
    }

    // Spawn the binary with all arguments
    const child = spawn(binaryPath, process.argv.slice(2), {
      stdio: 'inherit',
      windowsHide: false,
    });

    child.on('exit', (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
      } else {
        process.exit(code || 0);
      }
    });

    // Forward signals to child process
    process.on('SIGINT', () => child.kill('SIGINT'));
    process.on('SIGTERM', () => child.kill('SIGTERM'));
  } catch (err) {
    console.error('Error running llm-orchestrator:', err.message);
    process.exit(1);
  }
}

main();
