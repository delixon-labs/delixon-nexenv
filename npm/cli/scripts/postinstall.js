/* eslint-env node */
const { platform, arch } = process;
const https = require("https");
const crypto = require("crypto");
const {
  createWriteStream,
  chmodSync,
  existsSync,
  mkdirSync,
  readFileSync,
  unlinkSync,
} = require("fs");
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
const releaseBase = `https://github.com/delixon-labs/delixon-nexenv/releases/download/v${version}`;
const binaryUrl = `${releaseBase}/${asset}`;
const checksumsUrl = `${releaseBase}/SHA256SUMS`;
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

// Descarga texto plano (para SHA256SUMS). Devuelve null si 404 — el caller
// decide si es fatal o backward-compat con warning.
function fetchText(u, maxRedirects) {
  return new Promise((resolve, reject) => {
    function attempt(current, remaining) {
      if (remaining < 0) return reject(new Error("demasiados redirects"));
      https
        .get(current, (res) => {
          if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
            attempt(res.headers.location, remaining - 1);
            return;
          }
          if (res.statusCode === 404) return resolve(null);
          if (res.statusCode !== 200) {
            return reject(new Error(`HTTP ${res.statusCode} al descargar ${current}`));
          }
          const chunks = [];
          res.on("data", (c) => chunks.push(c));
          res.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
          res.on("error", reject);
        })
        .on("error", reject);
    }
    attempt(u, maxRedirects);
  });
}

// Parsea formato GNU coreutils sha256sum: cada linea "<sha256>  <filename>".
// Tolera "<sha256> *<filename>" (modo binario) y CRLF.
function parseSha256Sums(text) {
  const map = {};
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const m = line.match(/^([a-f0-9]{64})\s+\*?(.+)$/i);
    if (m) map[m[2].trim()] = m[1].toLowerCase();
  }
  return map;
}

function sha256OfFile(path) {
  const h = crypto.createHash("sha256");
  h.update(readFileSync(path));
  return h.digest("hex");
}

async function main() {
  console.log(`Nexenv: descargando binario v${version} para ${key}...`);
  await download(binaryUrl, destPath, 5);

  // Integridad: bajar SHA256SUMS y verificar. Si falta el asset (releases
  // antiguas pre-checksums), warning + continuar (forward-compat).
  console.log("Nexenv: verificando integridad (SHA-256)...");
  const sumsText = await fetchText(checksumsUrl, 5);
  if (!sumsText) {
    console.warn(
      `Nexenv: WARNING — el release v${version} no incluye SHA256SUMS.\n` +
      "  El binario fue descargado por HTTPS pero NO se verifico integridad.\n" +
      "  A partir de la proxima version los releases incluiran SHA256SUMS y\n" +
      "  esta verificacion sera obligatoria.",
    );
  } else {
    const sums = parseSha256Sums(sumsText);
    const expected = sums[asset];
    if (!expected) {
      try { unlinkSync(destPath); } catch { /* ignore */ }
      throw new Error(`SHA256SUMS no contiene entrada para ${asset}`);
    }
    const actual = sha256OfFile(destPath);
    if (actual !== expected) {
      try { unlinkSync(destPath); } catch { /* ignore */ }
      throw new Error(
        `checksum mismatch para ${asset}\n  esperado: ${expected}\n  obtenido: ${actual}`,
      );
    }
    console.log(`Nexenv: integridad OK (sha256: ${expected.slice(0, 16)}...)`);
  }

  if (platform !== "win32") chmodSync(destPath, 0o755);
  console.log(`Nexenv: binario instalado en ${destPath}`);
}

main().catch((err) => {
  console.error(`Nexenv: error en postinstall: ${err.message}`);
  console.error(`URL del binario: ${binaryUrl}`);
  console.error("Descarga manual: https://github.com/delixon-labs/delixon-nexenv/releases");
  console.error("Asigna NEXENV_SKIP_POSTINSTALL=1 si estas en un entorno sin red.");
  process.exit(1);
});
