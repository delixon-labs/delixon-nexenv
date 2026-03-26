# Delixon CLI — Referencia Completa de Comandos

> **delixon-cli** es la interfaz de terminal de Delixon. Comparte el mismo core (`delixon_lib`) que la GUI,
> por lo que ambas interfaces operan sobre los mismos datos, manifests y persistencia.

## Instalacion

```bash
# Desde la raiz del repo
cargo build --release --manifest-path src-tauri/Cargo.toml --bin delixon-cli

# El binario queda en src-tauri/target/release/delixon-cli(.exe)

# O desde el directorio src-tauri/
cd src-tauri && cargo build --release --bin delixon-cli

# Para ejecutar directamente sin build previo
cargo run --manifest-path src-tauri/Cargo.toml --bin delixon-cli -- <comando>
```

---

## Comandos Disponibles (28 comandos)

### Gestion de Proyectos

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `list` | Lista todos los proyectos registrados | `delixon-cli list` |
| `open <name>` | Abre proyecto en el editor configurado (busqueda parcial) | `delixon-cli open mi-proyecto` |
| `create <name> --path <ruta> [--template <id>]` | Crea y registra un nuevo proyecto | `delixon-cli create mi-app --path ./apps --template react-vite` |
| `scan <path>` | Detecta stack de un proyecto existente y lo registra | `delixon-cli scan ./mi-proyecto` |
| `status <project>` | Muestra estado Git del proyecto (rama, cambios, remoto) | `delixon-cli status mi-app` |

#### Detalle: `list`
```
delixon-cli list
```
Muestra nombre, ruta, estado (Active/Archived) y runtimes de cada proyecto registrado.

#### Detalle: `open`
```
delixon-cli open cliente-b
```
Busca por nombre parcial. Abre en el editor configurado (VS Code, Cursor, Zed, Neovim, etc.) con el entorno del proyecto.

#### Detalle: `create`
```
delixon-cli create ecommerce --path /projects/cliente-a --template node-express
```
Crea el proyecto, lo registra en Delixon, y opcionalmente aplica un template.

#### Detalle: `scan`
```
delixon-cli scan /projects/legacy-api
```
Analiza el directorio y detecta: lenguaje, framework, package manager, ORM, DB, Docker, CI, testing, linters, TypeScript, y genera un "readiness score".

---

### Scaffold y Templates

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `new <name> --path <ruta> [--type <tipo>] [--profile <perfil>] [--techs <t1,t2>]` | Genera proyecto completo desde scaffold | `delixon-cli new api --path ./apps --type api --profile standard --techs rust,docker` |

#### Detalle: `new`
```
delixon-cli new dashboard --path ./projects --type fullstack --profile production --techs react,nodejs,postgresql,prisma
```
**Tipos**: `api`, `frontend`, `fullstack`, `cli`, `desktop`
**Perfiles**: `rapid`, `standard`, `production`

Genera: estructura de directorios, `.gitignore`, `.env.example`, `docker-compose.yml`, `Makefile`, CI workflows, manifest, README, configuracion de VS Code.

---

### Catalogo y Validacion

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `catalog [id]` | Navega catalogo de tecnologias (sin ID lista todas) | `delixon-cli catalog` / `delixon-cli catalog rust` |
| `validate <techs...>` | Valida combinacion de tecnologias | `delixon-cli validate react nodejs postgresql prisma` |

#### Detalle: `catalog`
```
delixon-cli catalog
delixon-cli catalog postgresql
```
Muestra metadatos: categoria, version, descripcion, dependencias, puertos default, health checks.

#### Detalle: `validate`
```
delixon-cli validate nextjs postgresql prisma redis
```
Resuelve dependencias automaticas, detecta incompatibilidades, asigna puertos, sugiere tecnologias complementarias.

---

### Recipes

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `recipes` | Lista todas las recipes disponibles | `delixon-cli recipes` |
| `add <recipe> [--project <name>] [--preview]` | Aplica una recipe al proyecto | `delixon-cli add docker --project mi-app --preview` |

#### Recipes disponibles:
- `testing-vitest` — Setup de testing con Vitest
- `testing-pytest` — Setup de testing con Pytest
- `docker` — Configuracion Docker
- `ci-github` — GitHub Actions CI
- `linting-biome` — Linting con Biome
- `prisma` — Setup de Prisma ORM

#### Detalle: `add`
```
# Preview sin aplicar
delixon-cli add ci-github --project mi-api --preview

# Aplicar
delixon-cli add ci-github --project mi-api
```
Genera archivos, sugiere dependencias, inyecta env vars y scripts en el manifest.

---

### Diagnostico y Salud

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `doctor` | Verifica estado del sistema completo | `delixon-cli doctor` |
| `health <project>` | Ejecuta health checks del proyecto | `delixon-cli health mi-app` |

#### Detalle: `doctor`
```
delixon-cli doctor
```
Verifica: directorio de datos, configuracion, proyectos registrados, runtimes instalados (Node, Python, Rust, Go, PHP, Ruby), Docker, Git.

#### Detalle: `health`
```
delixon-cli health mi-app
```
Chequea: directorio existe, README, Git init, .gitignore, dependencias instaladas, runtimes disponibles, puertos libres.

---

### Variables de Entorno

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `env <project> get` | Muestra variables de entorno del proyecto | `delixon-cli env mi-app get` |
| `env <project> set <key> <value>` | Establece una variable de entorno | `delixon-cli env mi-app set DATABASE_URL postgres://localhost/mydb` |

---

### Manifest

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `manifest <project>` | Muestra el manifest completo del proyecto | `delixon-cli manifest mi-app` |

El manifest (`.delixon/manifest.yaml`) unifica: tecnologias, servicios, env vars, comandos, puertos, recipes aplicadas, health checks.

---

### Export / Import

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `export <project> [-o <archivo>]` | Exporta proyecto como archivo `.delixon` | `delixon-cli export mi-app -o backup.delixon` |
| `import <file> --path <ruta>` | Importa proyecto desde archivo `.delixon` | `delixon-cli import backup.delixon --path ./projects` |

Formato portable JSON con metadatos del proyecto, lista de env vars (sin valores sensibles).

---

### Docker Compose

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `docker up <project>` | Inicia servicios (`docker compose up -d`) | `delixon-cli docker up mi-app` |
| `docker down <project>` | Detiene servicios (`docker compose down`) | `delixon-cli docker down mi-app` |
| `docker status <project>` | Estado de servicios Docker | `delixon-cli docker status mi-app` |
| `docker logs <project> [--lines N]` | Muestra logs (default: 50 lineas) | `delixon-cli docker logs mi-app --lines 100` |

---

### Versionado y Snapshots

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `snapshot save <project>` | Guarda snapshot del manifest actual | `delixon-cli snapshot save mi-app` |
| `snapshot list <project>` | Lista todos los snapshots guardados | `delixon-cli snapshot list mi-app` |
| `snapshot diff <project> <v1> <v2>` | Compara dos versiones del manifest | `delixon-cli snapshot diff mi-app 1 2` |
| `snapshot rollback <project> <version>` | Restaura manifest a version anterior | `delixon-cli snapshot rollback mi-app 1` |
| `diff <project>` | Muestra cambios desde el ultimo snapshot | `delixon-cli diff mi-app` |

---

### Scripts y Procesos

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `run <script> [--project <name>]` | Ejecuta script definido en el manifest | `delixon-cli run dev --project mi-app` |
| `ps [project]` | Lista procesos en puertos del proyecto | `delixon-cli ps mi-app` |
| `ports` | Muestra todos los puertos en uso | `delixon-cli ports` |

#### Detalle: `run`
Ejecuta scripts del manifest con shell nativo (cmd en Windows, sh en Unix). Whitelist de ejecutables permitidos por seguridad.

---

### Notas

| Comando | Descripcion | Ejemplo |
|---|---|---|
| `note <project> [text]` | Gestiona notas del proyecto | `delixon-cli note mi-app "TODO: migrar a v2"` |

Sin texto: lista notas existentes. Con texto: agrega nueva nota con timestamp y UUID.

---

## Arquitectura

```
delixon-cli (binario)
    |
    v
delixon_lib (core compartido)
    |
    v
Misma persistencia JSON / YAML
    ^
    |
delixon (GUI Tauri + React)
```

Ambas interfaces (CLI y GUI) son fachadas del mismo motor. No compiten, se complementan:
- **CLI**: acciones rapidas, automatizacion, scripting, testing, power users
- **GUI**: exploracion visual, configuracion, dashboard, onboarding, usuarios menos tecnicos

---

## Plataformas

| SO | Shell | Notas |
|---|---|---|
| Windows | cmd / PowerShell | Rutas con `\`, `tasklist` para procesos |
| Linux | bash / zsh | Rutas POSIX, `lsof` para procesos |
| macOS | bash / zsh | Igual que Linux |
