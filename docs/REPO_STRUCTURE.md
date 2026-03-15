# Delixon — Estructura del Repositorio y Configuración de Desarrollo

> Organización GitHub: `deli-labs` · Repositorio: `delixon`
> URL: `https://github.com/deli-labs/delixon`

---

## Índice

1. [Configuración de GitHub](#1-configuración-de-github)
2. [Estructura de carpetas del repositorio](#2-estructura-de-carpetas-del-repositorio)
3. [Entornos de desarrollo](#3-entornos-de-desarrollo)
4. [Cómo arrancar el proyecto localmente](#4-cómo-arrancar-el-proyecto-localmente)
5. [Estrategia de ramas](#5-estrategia-de-ramas)
6. [Flujo de trabajo diario](#6-flujo-de-trabajo-diario)
7. [Pruebas locales](#7-pruebas-locales)
8. [CI/CD básico en GitHub Actions](#8-cicd-básico-en-github-actions)

---

## 1. Configuración de GitHub

### Organización

```
Organización : deli-labs
Owner        : dRaydel (único propietario de momento)
Repositorio  : delixon
Visibilidad  : Privado durante desarrollo, público en lanzamiento
URL          : https://github.com/deli-labs/delixon
```

### Pasos para crear la organización y el repo

```bash
# 1. Crear organización en GitHub
#    → github.com → "+" → "New organization" → Plan Free → Nombre: deli-labs

# 2. Dentro de la organización, crear repositorio
#    → Nombre: delixon
#    → Privado
#    → Inicializar con README
#    → .gitignore: Node (lo extenderemos manualmente)
#    → Licencia: MIT (o privada según decisión)

# 3. Clonar localmente
git clone https://github.com/deli-labs/delixon.git
cd delixon
```

### Configuración inicial del repositorio

```bash
# Configurar identidad de Git para este repo
git config user.name "dRaydel"
git config user.email "tu@email.com"

# Establecer rama principal como main
git branch -M main
```

---

## 2. Estructura de carpetas del repositorio

```
delixon/
│
├── .github/                          # Configuración de GitHub
│   ├── workflows/                    # GitHub Actions (CI/CD)
│   │   ├── build.yml                 # Build en push a main/dev
│   │   ├── test.yml                  # Tests automáticos en PRs
│   │   └── release.yml               # Generar instalador en releases
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── pull_request_template.md
│
├── src-tauri/                        # Backend — Rust / Tauri
│   ├── src/
│   │   ├── main.rs                   # Punto de entrada de la app
│   │   ├── lib.rs                    # Librerías internas
│   │   ├── commands/                 # Comandos IPC (frontend ↔ backend)
│   │   │   ├── mod.rs
│   │   │   ├── projects.rs           # Crear, abrir, eliminar proyectos
│   │   │   ├── environments.rs       # Gestión de variables de entorno
│   │   │   ├── dependencies.rs       # Resolución y caché de dependencias
│   │   │   ├── runtimes.rs           # Gestión de versiones (Node, Python...)
│   │   │   └── shell.rs              # Integración con terminales del SO
│   │   ├── models/                   # Estructuras de datos (structs)
│   │   │   ├── mod.rs
│   │   │   ├── project.rs            # Modelo de proyecto
│   │   │   ├── template.rs           # Modelo de plantilla
│   │   │   └── config.rs             # Configuración global de Delixon
│   │   └── utils/                    # Utilidades del sistema
│   │       ├── mod.rs
│   │       ├── fs.rs                 # Operaciones de sistema de archivos
│   │       ├── process.rs            # Lanzar y gestionar procesos
│   │       └── platform.rs           # Detección del SO (Win/Linux/Mac)
│   ├── Cargo.toml                    # Dependencias Rust
│   ├── Cargo.lock
│   ├── build.rs                      # Script de build
│   └── tauri.conf.json               # Configuración Tauri (nombre, permisos, íconos)
│
├── src/                              # Frontend — React + TypeScript
│   ├── assets/                       # Imágenes, íconos, fuentes
│   ├── components/
│   │   ├── ui/                       # Componentes base (shadcn/radix)
│   │   ├── layout/                   # Sidebar, topbar, wrappers
│   │   ├── dashboard/                # Vista principal de proyectos
│   │   ├── project-card/             # Tarjeta individual de proyecto
│   │   ├── project-editor/           # Formulario de configuración
│   │   ├── template-gallery/         # Galería de plantillas disponibles
│   │   ├── dependency-viewer/        # Vista de dependencias del proyecto
│   │   └── terminal-panel/           # Terminal integrada (fase 2)
│   ├── pages/
│   │   ├── Dashboard.tsx             # Página principal
│   │   ├── ProjectDetail.tsx         # Detalle de un proyecto
│   │   ├── Templates.tsx             # Galería de plantillas
│   │   └── Settings.tsx              # Configuración de Delixon
│   ├── stores/                       # Estado global (Zustand)
│   │   ├── projects.ts
│   │   ├── settings.ts
│   │   └── ui.ts
│   ├── hooks/                        # Custom hooks React
│   │   ├── useProjects.ts
│   │   ├── useEnvironment.ts
│   │   ├── useRuntimes.ts
│   │   └── useTauri.ts               # Bridge tipado con el backend Tauri
│   ├── lib/                          # Utilidades frontend
│   │   ├── tauri.ts                  # Llamadas IPC al backend
│   │   ├── templates.ts              # Lógica de carga de plantillas
│   │   └── utils.ts                  # Helpers generales
│   ├── types/                        # Tipos TypeScript compartidos
│   │   ├── project.ts
│   │   ├── template.ts
│   │   └── config.ts
│   ├── App.tsx
│   ├── main.tsx                      # Punto de entrada React
│   └── index.css                     # Estilos globales / Tailwind
│
├── templates/                        # Plantillas de proyectos gestionados
│   ├── _base/                        # Archivos comunes a todas las plantillas
│   │   ├── .gitignore
│   │   └── README.template.md
│   ├── node-express/
│   │   ├── template.json             # Metadatos de la plantilla
│   │   ├── package.json
│   │   └── src/
│   ├── react-vite/
│   ├── python-fastapi/
│   ├── python-django/
│   ├── fullstack-react-python/
│   ├── rust-cli/
│   └── docker-compose/
│
├── docs/                             # Documentación del proyecto
│   ├── PLAN.md                       # Plan general del producto
│   ├── REPO_STRUCTURE.md             # Este documento
│   ├── INVESTOR_PITCH.md             # Pitch para inversores
│   ├── architecture.md               # Decisiones de arquitectura técnica
│   ├── contributing.md               # Guía para contribuidores
│   └── templates-guide.md            # Cómo crear nuevas plantillas
│
├── tests/                            # Tests del backend Rust
│   ├── integration/
│   └── unit/
│
├── .gitignore
├── .gitattributes
├── package.json                      # Dependencias Node (frontend + Tauri CLI)
├── package-lock.json
├── tsconfig.json                     # Configuración TypeScript
├── vite.config.ts                    # Configuración Vite (bundler frontend)
├── tailwind.config.ts                # Configuración TailwindCSS
└── README.md                         # Presentación pública del proyecto
```

---

## 3. Entornos de desarrollo

Delixon usa dos tecnologías principales en su desarrollo: **Node.js** (para el frontend React) y **Rust** (para el backend Tauri). Cada una tiene su propio entorno de dependencias.

### 3.1 Requisitos del sistema

```
Node.js    >= 18.x   (recomendado: 20.x LTS)
Rust       >= 1.77   (se instala con rustup)
Cargo                (incluido con Rust)
Git        >= 2.40
```

Para Windows, adicionalmente:
```
Microsoft C++ Build Tools  (requerido por Tauri en Windows)
WebView2 Runtime           (incluido en Windows 11, instalable en W10)
```

### 3.2 Entorno Node (frontend)

Las dependencias Node se gestionan con `npm` y están aisladas en `node_modules/` dentro del repo. No requieren instalación global ni venv separado.

```bash
# Instalar dependencias del frontend
npm install

# Esto instala:
# - React 18 + TypeScript
# - Vite (bundler)
# - TailwindCSS
# - Zustand, React Query
# - shadcn/radix UI
# - @tauri-apps/api (bridge con el backend)
# - @tauri-apps/cli (herramientas de desarrollo Tauri)
```

El archivo `package.json` es el "requirements" del frontend. Cualquier persona que clone el repo y ejecute `npm install` tiene el entorno frontend listo.

### 3.3 Entorno Rust (backend)

Rust no usa virtualenvs — su gestor de dependencias `Cargo` es por diseño aislado por proyecto mediante `Cargo.toml` y `Cargo.lock`.

```bash
# Instalar Rust (si no está instalado)
# Windows: descargar rustup-init.exe desde rustup.rs
rustup-init.exe

# Verificar instalación
rustc --version
cargo --version

# Las dependencias Rust se instalan automáticamente al compilar
cargo build   # descarga e instala todo lo declarado en Cargo.toml
```

`Cargo.lock` garantiza que todos los colaboradores usen exactamente las mismas versiones de dependencias Rust.

### 3.4 Variables de entorno para desarrollo

Crear un archivo `.env.local` en la raíz (está en `.gitignore`, nunca va al repo):

```bash
# .env.local — NO subir al repositorio
DELIXON_ENV=development
DELIXON_LOG_LEVEL=debug
DELIXON_DATA_DIR=./dev-data    # Carpeta local para datos de prueba
```

El archivo `.env.example` en el repo documenta qué variables existen sin exponer valores reales:

```bash
# .env.example — SÍ va al repositorio
DELIXON_ENV=development
DELIXON_LOG_LEVEL=debug
DELIXON_DATA_DIR=
```

---

## 4. Cómo arrancar el proyecto localmente

### Primera vez (setup completo)

```bash
# 1. Clonar el repositorio
git clone https://github.com/deli-labs/delixon.git
cd delixon

# 2. Instalar dependencias Node
npm install

# 3. Copiar variables de entorno
cp .env.example .env.local
# Editar .env.local con los valores correctos

# 4. Compilar dependencias Rust (primera vez tarda ~2-5 min)
npm run tauri build -- --debug

# 5. Arrancar en modo desarrollo
npm run tauri dev
```

### Desarrollo diario

```bash
# Arrancar app en modo desarrollo (hot reload en frontend + recompila backend)
npm run tauri dev

# Solo frontend (sin backend, útil para trabajar en UI)
npm run dev

# Solo compilar backend Rust
cargo build --manifest-path src-tauri/Cargo.toml
```

### Scripts disponibles

| Comando | Qué hace |
|---------|----------|
| `npm run dev` | Arranca solo el frontend en el navegador (Vite) |
| `npm run tauri dev` | Arranca la app completa en modo desarrollo |
| `npm run build` | Compila el frontend para producción |
| `npm run tauri build` | Genera el instalador final (.msi en Windows) |
| `npm run test` | Ejecuta tests del frontend |
| `cargo test` | Ejecuta tests del backend Rust |
| `npm run lint` | Verifica el código con ESLint |
| `npm run format` | Formatea con Prettier |

---

## 5. Estrategia de ramas

```
main          ← Producción. Solo recibe merges desde release/*
              ← Protegida: requiere PR + aprobación

develop       ← Integración. Base de trabajo diario
              ← Aquí se mergean las features terminadas

feature/*     ← Una rama por funcionalidad
              ← Ejemplo: feature/project-isolation
              ← Ejemplo: feature/template-gallery
              ← Se crea desde develop, se mergea a develop

fix/*         ← Correcciones de bugs
              ← Ejemplo: fix/env-vars-not-loading
              ← Se crea desde develop (o main si es crítico)

release/*     ← Preparación de versión
              ← Ejemplo: release/v0.1.0
              ← Solo ajustes finales, luego mergea a main y develop

docs/*        ← Solo cambios de documentación
              ← Ejemplo: docs/update-contributing-guide
```

### Flujo visual

```
develop
   │
   ├── feature/project-isolation ──► develop ──► release/v0.1.0 ──► main (tag v0.1.0)
   │
   ├── feature/template-gallery  ──► develop
   │
   └── fix/env-not-loading       ──► develop
```

### Nombrado de commits

Seguimos **Conventional Commits**:

```
feat: añadir aislamiento de historial de terminal
fix: corregir carga de variables de entorno en Windows
docs: actualizar guía de plantillas
refactor: extraer lógica de detección de runtimes
test: añadir tests para resolución de dependencias
chore: actualizar dependencias Rust
```

---

## 6. Flujo de trabajo diario

```bash
# 1. Asegurarse de estar actualizado
git checkout develop
git pull origin develop

# 2. Crear rama para la nueva funcionalidad
git checkout -b feature/nombre-de-la-feature

# 3. Desarrollar con hot reload
npm run tauri dev

# 4. Hacer commits frecuentes con mensajes claros
git add .
git commit -m "feat: descripción concisa de lo que hace"

# 5. Subir la rama
git push origin feature/nombre-de-la-feature

# 6. Abrir Pull Request en GitHub hacia develop
#    → Descripción de qué hace el cambio
#    → Screenshot o gif si hay cambios visuales
#    → Checklist de testing manual hecho

# 7. Mergear y limpiar
git checkout develop
git pull origin develop
git branch -d feature/nombre-de-la-feature
```

---

## 7. Pruebas locales

### Probar la app como usuario final (sin instalar)

```bash
# Modo desarrollo — la app se abre como ventana nativa
npm run tauri dev
# Cualquier cambio en src/ recarga la UI automáticamente
# Cambios en src-tauri/ recompilan el backend automáticamente
```

### Tests automatizados

```bash
# Tests unitarios del frontend (Vitest)
npm run test

# Tests unitarios del backend Rust
cargo test --manifest-path src-tauri/Cargo.toml

# Tests de integración
cargo test --manifest-path src-tauri/Cargo.toml -- integration
```

### Generar instalador local para probar la app instalada

```bash
# Genera el instalador en src-tauri/target/release/bundle/
npm run tauri build

# En Windows genera:
# - delixon_0.1.0_x64_en-US.msi   (instalador MSI)
# - delixon_0.1.0_x64-setup.exe   (instalador NSIS)
```

### Probar con datos de desarrollo

La variable `DELIXON_DATA_DIR=./dev-data` en `.env.local` hace que Delixon use una carpeta local `dev-data/` para almacenar proyectos de prueba. Esto evita tocar los datos reales del sistema.

```bash
# La carpeta dev-data/ está en .gitignore
# Contiene proyectos y configuraciones solo para testing
dev-data/
├── projects/
│   ├── proyecto-prueba-node/
│   └── proyecto-prueba-python/
└── config.json
```

---

## 8. CI/CD básico en GitHub Actions

### Build automático en cada push

`.github/workflows/build.yml`:
```yaml
name: Build

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [develop]

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies
        run: npm install

      - name: Run frontend tests
        run: npm run test

      - name: Run Rust tests
        run: cargo test --manifest-path src-tauri/Cargo.toml

      - name: Build app
        run: npm run tauri build
```

### Release automático al crear un tag

`.github/workflows/release.yml`:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'   # Se activa con tags tipo v0.1.0, v1.0.0...

jobs:
  release-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install dependencies
        run: npm install
      - name: Build installer
        run: npm run tauri build
      - name: Upload release artifacts
        uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/msi/*.msi
            src-tauri/target/release/bundle/nsis/*.exe
```

### Cómo hacer un release

```bash
# 1. Asegurarse de que main está actualizado y estable
git checkout main
git pull origin main

# 2. Crear y subir el tag de versión
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions detecta el tag y genera el instalador automáticamente
# El instalador aparece en la sección Releases del repo
```

---

*Delixon — deli-labs/delixon · Organización GitHub: deli-labs*
