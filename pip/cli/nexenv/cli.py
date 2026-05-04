"""Nexenv CLI wrapper — downloads and executes the native binary."""

import hashlib
import os
import platform
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request
from importlib.metadata import PackageNotFoundError
from importlib.metadata import version as _pkg_version
from pathlib import Path


def _resolve_version() -> str:
    """Lee la version del package metadata. Cae a 0.0.0 solo si el package
    no esta instalado (modo dev sin pip install -e)."""
    try:
        return _pkg_version("nexenv")
    except PackageNotFoundError:
        return "0.0.0"


__version__ = _resolve_version()

PLATFORM_MAP = {
    ("Windows", "AMD64"): "nexenv-cli-win32-x64.exe",
    ("Windows", "x86_64"): "nexenv-cli-win32-x64.exe",
    ("Linux", "x86_64"): "nexenv-cli-linux-x64",
    ("Darwin", "arm64"): "nexenv-cli-darwin-arm64",
    ("Darwin", "x86_64"): "nexenv-cli-darwin-x64",
}

RELEASE_BASE = "https://github.com/delixon-labs/delixon-nexenv/releases/download/v{version}"
DOWNLOAD_TIMEOUT_S = 60
DOWNLOAD_MAX_RETRIES = 3
DOWNLOAD_RETRY_DELAY_S = 2


def get_bin_dir() -> Path:
    """Directory where the native binary is stored."""
    home = Path.home() / ".nexenv" / "bin"
    home.mkdir(parents=True, exist_ok=True)
    return home


def get_binary_name() -> str:
    """Get the correct binary name for this platform."""
    system = platform.system()
    machine = platform.machine()
    key = (system, machine)

    binary = PLATFORM_MAP.get(key)
    if not binary:
        print(f"Nexenv: plataforma no soportada ({system} {machine})", file=sys.stderr)
        print("Plataformas soportadas: Windows x64, Linux x64, macOS arm64/x64", file=sys.stderr)
        sys.exit(1)

    return binary


def get_binary_path() -> Path:
    """Get the full path to the native binary."""
    get_binary_name()
    local_name = "nexenv.exe" if platform.system() == "Windows" else "nexenv"
    return get_bin_dir() / local_name


def _http_get(url: str, *, timeout: int = DOWNLOAD_TIMEOUT_S):
    """GET con retry y timeout. Devuelve bytes, o None si 404."""
    last_err = None
    for attempt in range(1, DOWNLOAD_MAX_RETRIES + 1):
        try:
            with urllib.request.urlopen(url, timeout=timeout) as r:
                return r.read()
        except urllib.error.HTTPError as e:
            if e.code == 404:
                return None
            last_err = e
        except (urllib.error.URLError, TimeoutError, OSError) as e:
            last_err = e
        if attempt < DOWNLOAD_MAX_RETRIES:
            time.sleep(DOWNLOAD_RETRY_DELAY_S * attempt)
    raise RuntimeError(f"fallo al descargar {url}: {last_err}")


def _parse_sha256sums(text: str) -> dict:
    """Parsea formato GNU coreutils sha256sum: '<sha>  <filename>'."""
    result = {}
    for raw in text.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        m = re.match(r"^([a-fA-F0-9]{64})\s+\*?(.+)$", line)
        if m:
            result[m.group(2).strip()] = m.group(1).lower()
    return result


def download_binary(version: str) -> Path:
    """Download the native binary from GitHub Releases (with SHA-256 verify)."""
    binary_name = get_binary_name()
    base = RELEASE_BASE.format(version=version)
    binary_url = f"{base}/{binary_name}"
    sums_url = f"{base}/SHA256SUMS"
    dest = get_binary_path()

    print(f"Nexenv: descargando binario v{version}...")
    try:
        data = _http_get(binary_url)
    except Exception as e:
        print(f"Nexenv: error descargando binario: {e}", file=sys.stderr)
        print(f"URL: {binary_url}", file=sys.stderr)
        sys.exit(1)
    if data is None:
        print(f"Nexenv: el release v{version} no contiene {binary_name}", file=sys.stderr)
        print(f"URL: {binary_url}", file=sys.stderr)
        sys.exit(1)

    # Verificacion SHA-256. Si SHA256SUMS no existe (releases pre-checksums)
    # warning y continuar — forward-compat, en proximas versiones obligatorio.
    print("Nexenv: verificando integridad (SHA-256)...")
    try:
        sums_raw = _http_get(sums_url)
    except Exception as e:
        print(f"Nexenv: error descargando SHA256SUMS: {e}", file=sys.stderr)
        sys.exit(1)

    if sums_raw is None:
        print(
            f"Nexenv: WARNING — el release v{version} no incluye SHA256SUMS.\n"
            "  El binario fue descargado por HTTPS pero NO se verifico integridad.\n"
            "  A partir de la proxima version los releases incluiran SHA256SUMS\n"
            "  y esta verificacion sera obligatoria.",
            file=sys.stderr,
        )
    else:
        sums = _parse_sha256sums(sums_raw.decode("utf-8"))
        expected = sums.get(binary_name)
        if not expected:
            print(
                f"Nexenv: SHA256SUMS no contiene entrada para {binary_name}",
                file=sys.stderr,
            )
            sys.exit(1)
        actual = hashlib.sha256(data).hexdigest()
        if actual != expected:
            print(
                f"Nexenv: checksum mismatch para {binary_name}\n"
                f"  esperado: {expected}\n"
                f"  obtenido: {actual}",
                file=sys.stderr,
            )
            sys.exit(1)
        print(f"Nexenv: integridad OK (sha256: {expected[:16]}...)")

    # Escribir solo despues de verificar (no dejar binarios sin validar
    # en disco si SHA256SUMS fallo).
    dest.write_bytes(data)
    if platform.system() != "Windows":
        os.chmod(dest, 0o755)

    print(f"Nexenv: binario instalado en {dest}")
    return dest


def main():
    """Entry point — run the native binary or download it first."""
    binary = get_binary_path()

    if not binary.exists():
        binary = download_binary(__version__)

    try:
        result = subprocess.run([str(binary), *sys.argv[1:]])  # noqa: S603
        sys.exit(result.returncode)
    except FileNotFoundError:
        print(
            "Nexenv: binario no encontrado. Reinstala con: pip install --force-reinstall nexenv",
            file=sys.stderr,
        )
        sys.exit(1)
    except PermissionError:
        print(
            "Nexenv: sin permisos de ejecucion. Reinstala con: pip install --force-reinstall nexenv",
            file=sys.stderr,
        )
        sys.exit(1)
