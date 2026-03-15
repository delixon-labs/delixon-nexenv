# ─────────────────────────────────────────────────────────────────────────────
# Delixon — Inicialización de terminal PowerShell
# Esta configuración es EXCLUSIVA de este proyecto.
# No modifica el perfil global de PowerShell.
# ─────────────────────────────────────────────────────────────────────────────

$PROJECT_NAME  = "Delixon"
$PROJECT_ROOT  = Split-Path $MyInvocation.MyCommand.Path | Split-Path
$PROJECT_COLOR = "Cyan"

# Ir a la raíz del proyecto siempre al abrir la terminal
Set-Location $PROJECT_ROOT

# ─── Variables de entorno del proyecto ───────────────────────────────────────
$env:DELIXON_ENV      = "development"
$env:DELIXON_LOG_LEVEL = "debug"
$env:DELIXON_DATA_DIR  = Join-Path $PROJECT_ROOT "dev-data"

# ─── Funciones de apoyo para el prompt ───────────────────────────────────────

function Get-GitBranch {
    try {
        $branch = git rev-parse --abbrev-ref HEAD 2>$null
        if ($branch) { return $branch }
    } catch {}
    return $null
}

function Get-GitStatus {
    try {
        $status = git status --porcelain 2>$null
        if ($status) { return "*" }
    } catch {}
    return ""
}

function Get-NodeVersion {
    try {
        $v = node --version 2>$null
        if ($v) { return $v.TrimStart("v") }
    } catch {}
    return $null
}

function Get-RustVersion {
    try {
        $v = rustc --version 2>$null
        if ($v) { return ($v -split " ")[1] }
    } catch {}
    return $null
}

# ─── Prompt personalizado ─────────────────────────────────────────────────────
function prompt {
    $branch    = Get-GitBranch
    $dirty     = Get-GitStatus
    $node      = Get-NodeVersion
    $rust      = Get-RustVersion
    $location  = (Get-Location).Path.Replace($PROJECT_ROOT, "~")

    # Línea 1: nombre del proyecto
    Write-Host ""
    Write-Host " $PROJECT_NAME " -BackgroundColor DarkCyan -ForegroundColor White -NoNewline

    # Rama Git
    if ($branch) {
        Write-Host "  $branch$dirty" -ForegroundColor Yellow -NoNewline
    }

    # Runtimes activos
    if ($node) {
        Write-Host "  node:$node" -ForegroundColor Green -NoNewline
    }
    if ($rust) {
        Write-Host "  rust:$rust" -ForegroundColor Magenta -NoNewline
    }

    Write-Host ""

    # Línea 2: ruta actual
    Write-Host " $location" -ForegroundColor DarkGray -NoNewline
    Write-Host " ❯ " -ForegroundColor $PROJECT_COLOR -NoNewline

    return " "
}

# ─── Banner de bienvenida ─────────────────────────────────────────────────────
Write-Host ""
Write-Host "  ██████╗ ███████╗██╗     ██╗██╗  ██╗ ██████╗ ███╗   ██╗" -ForegroundColor Cyan
Write-Host "  ██╔══██╗██╔════╝██║     ██║╚██╗██╔╝██╔═══██╗████╗  ██║" -ForegroundColor Cyan
Write-Host "  ██║  ██║█████╗  ██║     ██║ ╚███╔╝ ██║   ██║██╔██╗ ██║" -ForegroundColor Cyan
Write-Host "  ██║  ██║██╔══╝  ██║     ██║ ██╔██╗ ██║   ██║██║╚██╗██║" -ForegroundColor Cyan
Write-Host "  ██████╔╝███████╗███████╗██║██╔╝ ██╗╚██████╔╝██║ ╚████║" -ForegroundColor Cyan
Write-Host "  ╚═════╝ ╚══════╝╚══════╝╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Workspace : $PROJECT_ROOT" -ForegroundColor DarkGray
Write-Host "  Entorno   : $($env:DELIXON_ENV)" -ForegroundColor DarkGray

$branch = Get-GitBranch
if ($branch) {
    Write-Host "  Rama      : $branch" -ForegroundColor Yellow
}

$node = Get-NodeVersion
if ($node) {
    Write-Host "  Node      : $node" -ForegroundColor Green
}

$rust = Get-RustVersion
if ($rust) {
    Write-Host "  Rust      : $rust" -ForegroundColor Magenta
}

Write-Host ""
Write-Host "  npm run tauri dev   → arrancar en desarrollo" -ForegroundColor DarkGray
Write-Host "  npm run test        → ejecutar tests" -ForegroundColor DarkGray
Write-Host ""
