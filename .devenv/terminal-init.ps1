# ==============================================================================
# Delixon -- Terminal PowerShell (configuracion exclusiva de este proyecto)
# No modifica el perfil global. Solo afecta esta ventana de terminal.
# ==============================================================================

$PROJECT_ROOT = Split-Path $MyInvocation.MyCommand.Path | Split-Path
Set-Location $PROJECT_ROOT

# --- Variables de entorno del proyecto ---------------------------------------
$env:DELIXON_ENV       = "development"
$env:DELIXON_LOG_LEVEL = "debug"
$env:DELIXON_DATA_DIR  = Join-Path $PROJECT_ROOT "dev-data"

$envLocal = Join-Path $PROJECT_ROOT ".env.local"
if (Test-Path $envLocal) {
    Get-Content $envLocal | ForEach-Object {
        if ($_ -match '^\s*([^#][^=]+)=(.*)$') {
            [System.Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim(), "Process")
        }
    }
}

# --- Historial aislado para este proyecto ------------------------------------
$histFile = Join-Path $PROJECT_ROOT ".vscode\terminal-history.txt"
Set-PSReadLineOption -HistorySavePath $histFile

# --- PSReadLine -- mismo estilo que el perfil global -------------------------
try {
    Set-PSReadLineOption -PredictionSource History
    Set-PSReadLineOption -PredictionViewStyle ListView
    Set-PSReadLineOption -HistorySearchCursorMovesToEnd
    Set-PSReadLineKeyHandler -Key UpArrow       -Function HistorySearchBackward
    Set-PSReadLineKeyHandler -Key DownArrow     -Function HistorySearchForward
    Set-PSReadLineKeyHandler -Key Tab           -Function MenuComplete
    Set-PSReadLineKeyHandler -Key Ctrl+Spacebar -Function AcceptSuggestion
} catch {}

# --- Oh My Posh -- tema night-owl-clean del perfil global --------------------
$ompTheme = "$env:USERPROFILE\.config\oh-my-posh\night-owl-clean.omp.json"
if (Test-Path $ompTheme) {
    oh-my-posh init pwsh --config $ompTheme | Invoke-Expression
}

# --- Aliases utiles para este proyecto ---------------------------------------
function dev   { npm run tauri dev }
function test  { npm run test }
function build { npm run tauri build }
function lint  { npm run lint }

# --- Banner de bienvenida ----------------------------------------------------
Write-Host ""
Write-Host " Delixon " -BackgroundColor DarkCyan -ForegroundColor White -NoNewline
Write-Host " workspace listo" -ForegroundColor DarkGray
Write-Host ""

$branch = git rev-parse --abbrev-ref HEAD 2>$null
if ($branch) { Write-Host "  rama     : $branch" -ForegroundColor Yellow }

$node = node --version 2>$null
if ($node)   { Write-Host "  node     : $node"   -ForegroundColor Green }

if (Get-Command rustc -ErrorAction SilentlyContinue) {
    $rust = rustc --version 2>$null | ForEach-Object { ($_ -split " ")[1] }
    if ($rust) { Write-Host "  rust     : $rust" -ForegroundColor Magenta }
} else {
    Write-Host "  rust     : no instalado  ->  winget install Rustlang.Rustup" -ForegroundColor Red
}

Write-Host "  entorno  : $($env:DELIXON_ENV)" -ForegroundColor DarkGray
Write-Host ""
Write-Host "  dev   -> npm run tauri dev"   -ForegroundColor DarkGray
Write-Host "  test  -> npm run test"         -ForegroundColor DarkGray
Write-Host "  build -> npm run tauri build"  -ForegroundColor DarkGray
Write-Host ""
