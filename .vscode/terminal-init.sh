#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# Delixon — Inicialización de terminal Git Bash
# Esta configuración es EXCLUSIVA de este proyecto.
# No modifica el .bashrc global.
# ─────────────────────────────────────────────────────────────────────────────

PROJECT_NAME="Delixon"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Ir a la raíz del proyecto al abrir la terminal
cd "$PROJECT_ROOT" || exit

# ─── Variables de entorno del proyecto ───────────────────────────────────────
export DELIXON_ENV="development"
export DELIXON_LOG_LEVEL="debug"
export DELIXON_DATA_DIR="$PROJECT_ROOT/dev-data"

# ─── Cargar .env.local si existe ─────────────────────────────────────────────
if [ -f "$PROJECT_ROOT/.env.local" ]; then
    set -a
    # shellcheck disable=SC1091
    source "$PROJECT_ROOT/.env.local"
    set +a
fi

# ─── Colores ─────────────────────────────────────────────────────────────────
CYAN='\033[0;36m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
MAGENTA='\033[0;35m'
GRAY='\033[0;90m'
BOLD='\033[1m'
RESET='\033[0m'

# ─── Funciones de apoyo ───────────────────────────────────────────────────────

git_branch() {
    git rev-parse --abbrev-ref HEAD 2>/dev/null
}

git_dirty() {
    [ -n "$(git status --porcelain 2>/dev/null)" ] && echo "*"
}

node_version() {
    node --version 2>/dev/null | sed 's/^v//'
}

rust_version() {
    rustc --version 2>/dev/null | awk '{print $2}'
}

short_path() {
    echo "${PWD/$PROJECT_ROOT/~}"
}

# ─── Prompt personalizado ─────────────────────────────────────────────────────
build_prompt() {
    local branch dirty node rust path_short

    branch=$(git_branch)
    dirty=$(git_dirty)
    node=$(node_version)
    rust=$(rust_version)
    path_short=$(short_path)

    local prompt=""

    # Nombre del proyecto
    prompt+="\n${BOLD}${CYAN} $PROJECT_NAME ${RESET}"

    # Rama Git
    if [ -n "$branch" ]; then
        prompt+=" ${YELLOW} ${branch}${dirty}${RESET}"
    fi

    # Runtimes
    if [ -n "$node" ]; then
        prompt+=" ${GREEN} node:${node}${RESET}"
    fi
    if [ -n "$rust" ]; then
        prompt+=" ${MAGENTA} rust:${rust}${RESET}"
    fi

    # Ruta
    prompt+="\n${GRAY} ${path_short}${RESET}"

    # Cursor
    prompt+=" ${CYAN}❯${RESET} "

    echo -e "$prompt"
}

export PS1='$(build_prompt)'

# ─── Banner de bienvenida ─────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}  ██████╗ ███████╗██╗     ██╗██╗  ██╗ ██████╗ ███╗   ██╗${RESET}"
echo -e "${CYAN}  ██╔══██╗██╔════╝██║     ██║╚██╗██╔╝██╔═══██╗████╗  ██║${RESET}"
echo -e "${CYAN}  ██║  ██║█████╗  ██║     ██║ ╚███╔╝ ██║   ██║██╔██╗ ██║${RESET}"
echo -e "${CYAN}  ██║  ██║██╔══╝  ██║     ██║ ██╔██╗ ██║   ██║██║╚██╗██║${RESET}"
echo -e "${CYAN}  ██████╔╝███████╗███████╗██║██╔╝ ██╗╚██████╔╝██║ ╚████║${RESET}"
echo -e "${CYAN}  ╚═════╝ ╚══════╝╚══════╝╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝${RESET}"
echo ""
echo -e "${GRAY}  Workspace : $PROJECT_ROOT${RESET}"
echo -e "${GRAY}  Entorno   : $DELIXON_ENV${RESET}"

branch=$(git_branch)
[ -n "$branch" ] && echo -e "${YELLOW}  Rama      : $branch${RESET}"

node=$(node_version)
[ -n "$node" ] && echo -e "${GREEN}  Node      : $node${RESET}"

rust=$(rust_version)
[ -n "$rust" ] && echo -e "${MAGENTA}  Rust      : $rust${RESET}"

echo ""
echo -e "${GRAY}  npm run tauri dev   → arrancar en desarrollo${RESET}"
echo -e "${GRAY}  npm run test        → ejecutar tests${RESET}"
echo ""
