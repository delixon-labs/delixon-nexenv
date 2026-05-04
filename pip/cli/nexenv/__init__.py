"""Nexenv — Workspace manager for developers."""

from importlib.metadata import PackageNotFoundError
from importlib.metadata import version as _pkg_version

try:
    __version__ = _pkg_version("nexenv")
except PackageNotFoundError:  # paquete no instalado (modo dev sin install)
    __version__ = "0.0.0"
