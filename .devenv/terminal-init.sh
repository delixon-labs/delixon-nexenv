#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# Nexenv — Inicialización de terminal Git Bash
# Esta configuración es EXCLUSIVA de este proyecto.
# No modifica el .bashrc global.
# ─────────────────────────────────────────────────────────────────────────────

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT" || exit

# ─── Variables de entorno del proyecto ───────────────────────────────────────
export NEXENV_ENV="development"
export NEXENV_LOG_LEVEL="debug"
export NEXENV_DATA_DIR="$PROJECT_ROOT/dev-data"

[ -f "$PROJECT_ROOT/.env.local" ] && set -a && source "$PROJECT_ROOT/.env.local" && set +a

# ─── Historial aislado para este proyecto ────────────────────────────────────
export HISTFILE="$PROJECT_ROOT/.vscode/bash-history.txt"
export HISTSIZE=5000
export HISTFILESIZE=5000
export HISTCONTROL=ignoredups:erasedups
shopt -s histappend

# ─── Oh My Posh — mismo tema night-owl-clean del perfil global ───────────────
OMP_THEME_WIN="$USERPROFILE\\.config\\oh-my-posh\\night-owl-clean.omp.json"
OMP_THEME_BASH=$(echo "$OMP_THEME_WIN" | sed 's|\\|/|g' | sed 's|^\([A-Za-z]\):|/\L\1|')

if command -v oh-my-posh &>/dev/null; then
    if [ -f "$OMP_THEME_BASH" ]; then
        eval "$(oh-my-posh init bash --config "$OMP_THEME_BASH")"
    else
        eval "$(oh-my-posh init bash)"
    fi
fi

# ─── Aliases útiles para este proyecto ───────────────────────────────────────
alias dev='npm run tauri dev'
alias test='npm run test'
alias build='npm run tauri build'
alias lint='npm run lint'

# ─── Banner de bienvenida ─────────────────────────────────────────────────────
CYAN='\033[0;36m'; YELLOW='\033[0;33m'; GREEN='\033[0;32m'
MAGENTA='\033[0;35m'; GRAY='\033[0;90m'; RESET='\033[0m'

echo ""
echo -e "${CYAN} Nexenv ${RESET}${GRAY} workspace listo${RESET}"
echo ""

branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null)
[ -n "$branch" ] && echo -e "${YELLOW}  rama     : $branch${RESET}"

node_v=$(node --version 2>/dev/null)
[ -n "$node_v" ] && echo -e "${GREEN}  node     : $node_v${RESET}"

rust_v=$(rustc --version 2>/dev/null | awk '{print $2}')
[ -n "$rust_v" ] && echo -e "${MAGENTA}  rust     : $rust_v${RESET}"

echo -e "${GRAY}  entorno  : $NEXENV_ENV${RESET}"
echo ""
echo -e "${GRAY}  dev    → npm run tauri dev${RESET}"
echo -e "${GRAY}  test   → npm run test${RESET}"
echo -e "${GRAY}  build  → npm run tauri build${RESET}"
echo ""
