/* eslint-env node */
const { platform, arch } = process;
const https = require("https");
const { createWriteStream, chmodSync, existsSync, mkdirSync, unlinkSync } = require("fs");
const { join } = require("path");

const ASSET_MAP = {
  "win32-x64": "nexenv-cli-win32-x64.exe",
  "linux-x64": "nexenv-cli-linux-x64",
  "darwin-arm64": "nexenv-cli-darwin-arm64",
  "darwin-x64": "nexenv-cli-darwin-x64",
};

const key = `${platform}-${arch}`;
const asset = ASSET_MAP[key];

if (!asset) {
  console.error(`Nexenv: plataforma no soportada (${key})`);
  console.error("Plataformas soportadas: win32-x64, linux-x64, darwin-arm64, darwin-x64");
  process.exit(1);
}

if (process.env.NEXENV_SKIP_POSTINSTALL === "1") {
  process.exit(0);
}

const pkg = require("../package.json");
const version = pkg.version;
const url = `https://github.com/delixon-labs/delixon-nexenv/releases/download/v${version}/${asset}`;
const binaryName = platform === "win32" ? "nexenv.exe" : "nexenv";
const binDir = join(__dirname, "..", "bin");
const destPath = join(binDir, binaryName);

if (!existsSync(binDir)) {
  mkdirSync(binDir, { recursive: true });
}

function download(u, dest, maxRedirects) {
  return new Promise((resolve, reject) => {
    function attempt(current, remaining) {
      if (remaining < 0) {
        reject(new Error("demasiados redirects"));
        return;
      }
      https
        .get(current, (res) => {
          if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
            attempt(res.headers.location, remaining - 1);
            return;
          }
          if (res.statusCode !== 200) {
            reject(new Error(`HTTP ${res.statusCode} al descargar ${current}`));
            return;
          }
          const file = createWriteStream(dest);
          res.pipe(file);
          file.on("finish", () => file.close(() => resolve()));
          file.on("error", (err) => {
            try { unlinkSync(dest); } catch { /* ignore */ }
            reject(err);
          });
        })
        .on("error", reject);
    }
    attempt(u, maxRedirects);
  });
}

console.log(`Nexenv: descargando binario v${version} para ${key}...`);
download(url, destPath, 5)
  .then(() => {
    if (platform !== "win32") chmodSync(destPath, 0o755);
    console.log(`Nexenv: binario instalado en ${destPath}`);
  })
  .catch((err) => {
    console.error(`Nexenv: error descargando el binario: ${err.message}`);
    console.error(`URL intentada: ${url}`);
    console.error("Descarga manual: https://github.com/delixon-labs/delixon-nexenv/releases");
    console.error("Asigna NEXENV_SKIP_POSTINSTALL=1 si estas en un entorno sin red.");
    process.exit(1);
  });
