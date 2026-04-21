"""Nexenv CLI wrapper — downloads and executes the native binary."""

import os
import platform
import subprocess
import sys
import urllib.request
from pathlib import Path

__version__ = "1.0.0"

PLATFORM_MAP = {
    ("Windows", "AMD64"): "nexenv-cli-win32-x64.exe",
    ("Windows", "x86_64"): "nexenv-cli-win32-x64.exe",
    ("Linux", "x86_64"): "nexenv-cli-linux-x64",
    ("Darwin", "arm64"): "nexenv-cli-darwin-arm64",
    ("Darwin", "x86_64"): "nexenv-cli-darwin-x64",
}

RELEASE_URL = "https://github.com/delixon-labs/delixon-nexenv/releases/download/v{version}/{binary}"


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
    binary_name = get_binary_name()
    # Use simple name for the local binary
    local_name = "nexenv.exe" if platform.system() == "Windows" else "nexenv"
    return get_bin_dir() / local_name


def download_binary(version: str) -> Path:
    """Download the native binary from GitHub Releases."""
    binary_name = get_binary_name()
    url = RELEASE_URL.format(version=version, binary=binary_name)
    dest = get_binary_path()

    print(f"Nexenv: descargando binario v{version}...")
    try:
        urllib.request.urlretrieve(url, dest)
    except Exception as e:
        print(f"Nexenv: error descargando binario: {e}", file=sys.stderr)
        print(f"URL: {url}", file=sys.stderr)
        sys.exit(1)

    # Make executable on Unix
    if platform.system() != "Windows":
        os.chmod(dest, 0o755)

    print(f"Nexenv: binario instalado en {dest}")
    return dest


def main():
    """Entry point — run the native binary or download it first."""
    binary = get_binary_path()

    # Download if not exists
    if not binary.exists():
        binary = download_binary(__version__)

    # Execute the native binary with all arguments
    try:
        result = subprocess.run([str(binary)] + sys.argv[1:])
        sys.exit(result.returncode)
    except FileNotFoundError:
        print("Nexenv: binario no encontrado. Reinstala con: pip install --force-reinstall nexenv", file=sys.stderr)
        sys.exit(1)
    except PermissionError:
        print("Nexenv: sin permisos de ejecucion. Reinstala con: pip install --force-reinstall nexenv", file=sys.stderr)
        sys.exit(1)
