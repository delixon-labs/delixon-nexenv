/* eslint-env node */
const { platform, arch } = process;
const { existsSync, copyFileSync, chmodSync } = require("fs");
const { join } = require("path");

const PLATFORM_MAP = {
  "win32-x64": "nexenv-win32-x64",
  "linux-x64": "nexenv-linux-x64",
  "darwin-arm64": "nexenv-darwin-arm64",
  "darwin-x64": "nexenv-darwin-x64",
};

const key = `${platform}-${arch}`;
const pkg = PLATFORM_MAP[key];

if (!pkg) {
  console.error(`Nexenv: plataforma no soportada (${key})`);
  console.error("Plataformas soportadas: win32-x64, linux-x64, darwin-arm64, darwin-x64");
  process.exit(1);
}

const binaryName = platform === "win32" ? "nexenv.exe" : "nexenv";

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
