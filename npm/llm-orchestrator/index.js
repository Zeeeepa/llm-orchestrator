const { execFile } = require('child_process');
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
  return packageMap[key];
}

// Find the binary path
function getBinaryPath() {
  const packageName = getPlatformPackage();
  if (!packageName) {
    throw new Error(`Unsupported platform: ${process.platform}-${process.arch}`);
  }

  const binaryName = process.platform === 'win32' ? 'llm-orchestrator.exe' : 'llm-orchestrator';

  // Try to find the binary in node_modules
  let binaryPath = path.join(__dirname, '..', packageName.replace('@llm-dev-ops/', ''), 'bin', binaryName);

  // Check if it exists
  if (!fs.existsSync(binaryPath)) {
    // Try alternative path
    binaryPath = path.join(__dirname, '..', '..', packageName, 'bin', binaryName);
  }

  if (!fs.existsSync(binaryPath)) {
    throw new Error(`Could not find binary. Make sure ${packageName} is installed`);
  }

  return binaryPath;
}

/**
 * Execute llm-orchestrator CLI
 * @param {string[]} args - Command line arguments
 * @param {object} options - Options to pass to execFile
 * @returns {Promise<{stdout: string, stderr: string}>}
 */
function execute(args = [], options = {}) {
  return new Promise((resolve, reject) => {
    const binaryPath = getBinaryPath();

    execFile(binaryPath, args, options, (error, stdout, stderr) => {
      if (error) {
        error.stdout = stdout;
        error.stderr = stderr;
        reject(error);
      } else {
        resolve({ stdout, stderr });
      }
    });
  });
}

/**
 * Validate a workflow file
 * @param {string} filePath - Path to workflow YAML file
 * @returns {Promise<{stdout: string, stderr: string}>}
 */
async function validate(filePath) {
  return execute(['validate', filePath]);
}

/**
 * Run a workflow
 * @param {string} filePath - Path to workflow YAML file
 * @param {object} options - Options
 * @param {string} options.input - Input JSON string or file path
 * @param {number} options.maxConcurrency - Maximum concurrent tasks
 * @returns {Promise<{stdout: string, stderr: string}>}
 */
async function run(filePath, options = {}) {
  const args = ['run', filePath];

  if (options.input) {
    args.push('--input', options.input);
  }

  if (options.maxConcurrency) {
    args.push('--max-concurrency', String(options.maxConcurrency));
  }

  return execute(args);
}

module.exports = {
  execute,
  validate,
  run,
  getBinaryPath,
};
