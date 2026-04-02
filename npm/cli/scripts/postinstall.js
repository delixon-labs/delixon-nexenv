/* eslint-env node */
const { platform, arch } = process;
const { existsSync, copyFileSync, chmodSync } = require("fs");
const { join } = require("path");

const PLATFORM_MAP = {
  "win32-x64": "@delixon/cli-win32-x64",
  "linux-x64": "@delixon/cli-linux-x64",
  "darwin-arm64": "@delixon/cli-darwin-arm64",
  "darwin-x64": "@delixon/cli-darwin-x64",
};

const key = `${platform}-${arch}`;
const pkg = PLATFORM_MAP[key];

if (!pkg) {
  console.error(`Delixon: plataforma no soportada (${key})`);
  console.error("Plataformas soportadas: win32-x64, linux-x64, darwin-arm64, darwin-x64");
  process.exit(1);
}

const binaryName = platform === "win32" ? "delixon.exe" : "delixon";

try {
  const binaryPath = require.resolve(`${pkg}/${binaryName}`);
  const destPath = join(__dirname, "..", "bin", binaryName);

  copyFileSync(binaryPath, destPath);

  if (platform !== "win32") {
    chmodSync(destPath, 0o755);
  }
} catch {
  // El paquete opcional no se instalo (plataforma diferente)
  // Esto es normal en CI o cross-platform
}
