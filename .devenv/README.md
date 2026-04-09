# .devenv — Configuracion del entorno de desarrollo

Esta carpeta contiene la configuracion del entorno de trabajo del proyecto.
Es parte del repositorio y debe estar presente en todas las maquinas.

## Contenido

| Archivo | Descripcion |
|---------|-------------|
| `terminal-init.ps1` | Script de inicio de terminal PowerShell |
| `terminal-init.sh` | Script de inicio de terminal Git Bash |
| `settings.vscode.json` | Plantilla de configuracion VSCode para este proyecto |

## Setup inicial (una sola vez por desarrollador)

### 1. Configurar VSCode

Copia la plantilla de settings a tu carpeta .vscode local:

```powershell
# En PowerShell desde la raiz del proyecto
Copy-Item .devenv\settings.vscode.json .vscode\settings.json
```

```bash
# En Git Bash
cp .devenv/settings.vscode.json .vscode/settings.json
```

Tu `.vscode/settings.json` es personal y no va al repositorio.
Puedes modificarlo a tu gusto sin afectar al resto del equipo.

### 2. Abrir la terminal del proyecto

La terminal se configura automaticamente al abrir VSCode con los perfiles:
- **PowerShell (Nexenv)** — usa `terminal-init.ps1`
- **Git Bash (Nexenv)** — usa `terminal-init.sh`

Ambos cargan el entorno del proyecto de forma aislada sin modificar
el perfil global de tu sistema.
