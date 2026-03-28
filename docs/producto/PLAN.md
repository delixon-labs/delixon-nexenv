# Delixon — Gestor de Workspaces para Desarrolladores

> *Deja de configurar. Empieza a construir.*

> *Un clic. Proyecto abierto. Entorno correcto.*

---

## Índice

1. [El problema real](#1-el-problema-real)
2. [La solución: Delixon](#2-la-solución-delixon)
3. [Qué hace Delixon](#3-qué-hace-delixon)
4. [Ejemplos prácticos](#4-ejemplos-prácticos)
5. [Stack tecnológico](#5-stack-tecnológico-tauri--react)
6. [Arquitectura del sistema](#6-arquitectura-del-sistema)
7. [Las capas de Delixon](#7-las-capas-de-delixon)
8. [Estructura del proyecto](#8-estructura-del-proyecto)
9. [Arquitectura por capas](#9-arquitectura-por-capas)
10. [Hoja de ruta y fases](#10-hoja-de-ruta-y-fases)
11. [Objetivos por fase](#11-objetivos-por-fase)
12. [Logros esperados con métricas](#12-logros-esperados-con-métricas)
13. [Comparativa con herramientas existentes](#13-comparativa-con-herramientas-existentes)
14. [Funcionalidades diferenciadoras](#14-funcionalidades-diferenciadoras)
15. [Gobernanza y equipos/empresa](#15-gobernanza-y-equiposempresa)
16. [Landing page y waitlist](#16-landing-page-y-waitlist)
17. [Opinión sincera y riesgos](#17-opinión-sincera-y-riesgos)
18. [Resumen ejecutivo](#18-resumen-ejecutivo)
19. [Analisis de estado y vision final](#19-analisis-de-estado-y-vision-final)
20. [Glosario de tecnologías y siglas](#20-glosario-de-tecnologías-y-siglas)
21. [Checklist general de implementación](#22-checklist-general-de-implementación)
22. [Vision del producto — Que no deberia faltar](#23-vision-del-producto--que-no-deberia-faltar)

---

## 1. El problema real

Un desarrollador que trabaja en varios proyectos simultáneamente pierde entre **2 y 5 horas semanales** en tareas que no aportan valor productivo:

| Problema | Impacto |
|----------|---------|
| Configurar entornos desde cero en cada proyecto nuevo | 1-3 horas por proyecto |
| Instalar dependencias que ya existen en otro proyecto | Tiempo + espacio en disco duplicado |
| Mezclar variables de entorno entre proyectos | Bugs silenciosos difíciles de rastrear |
| Ejecutar comandos en el proyecto equivocado | Errores costosos, a veces irreversibles |
| Historial de terminal mezclado entre proyectos | Pérdida de contexto y productividad |
| Recordar qué versión de runtime usa cada proyecto | Inconsistencias en builds y comportamientos |
| Incorporar un nuevo desarrollador al equipo | 4-8 horas de onboarding técnico |
| Puertos de red que colisionan entre proyectos | Bugs silenciosos o fallos al arrancar |
| Retomar un proyecto olvidado meses después | 30+ min de "recordar cómo funcionaba" |

Estos problemas **tienden a multiplicarse** con cada proyecto nuevo. No son molestias menores — representan horas de trabajo real perdidas, errores frecuentes, y fricción constante que desgasta al desarrollador.

---

## 2. La solución: Delixon

Delixon es una **aplicacion de escritorio local + CLI** que actua como capa de organizacion e inteligencia entre el desarrollador y sus proyectos.

**No reemplaza ninguna herramienta.** El desarrollador sigue usando VSCode, su terminal preferida, Git, Docker, npm, pip — todo lo que ya conoce y domina. Delixon se encarga de que cada proyecto viva en su propio mundo perfectamente configurado, listo para trabajar desde el primer segundo.

### Estrategia dual: GUI + CLI

Delixon no es "GUI o CLI". Es **ambos, para todos los usuarios**:

| Interfaz | Para quien | Uso principal |
|---|---|---|
| **GUI** (Tauri + React) | Producto principal para el cliente. Usuarios visuales, onboarding, exploracion | Dashboard, configuracion, wizards, health visual, browse catalogo |
| **CLI** (Rust + clap) | Power users, devs tecnicos, automatizacion, y herramienta interna de desarrollo | Acciones rapidas desde terminal, scripting, CI/CD, testing del core |

**Tres perfiles de usuario:**

1. **Usuario visual** — Usa solo GUI. Quiere clics, dashboards, colores. La GUI es su producto.
2. **Usuario tecnico** — Usa solo CLI. Vive en la terminal. `delixon open`, `delixon scan`, `delixon run`.
3. **Usuario mixto** — Usa GUI para explorar/configurar y CLI para operar rapido. El caso mas comun.

> **Ambas interfaces comparten el mismo core (`delixon_lib`).** Misma logica, mismos datos, misma persistencia JSON/YAML. No son productos separados. La CLI no es "la version pobre" — es la version rapida. La GUI no es "la version lenta" — es la version visual.

### Principio central

> El desarrollador abre Delixon, selecciona el proyecto, hace clic en "Abrir" y ya está trabajando. El entorno correcto, la terminal correcta, las variables correctas, las dependencias correctas. Sin pasos manuales. Sin documentos de 30 puntos. Sin sorpresas.

### Mensajes clave para comunicar

- **"No necesita Docker"** — Diferenciador enorme. Muchos devs odian la complejidad de Docker para desarrollo local. Delixon usa symlinks y scopes, no contenedores.
- **"No reemplaza nada"** — Se pone encima de las herramientas que ya usas.
- **"Funciona offline"** — Como app de escritorio (Tauri), todo es local. Importante para empresas con restricciones de nube.
- **"5 MB, no 200 MB"** — Tauri vs Electron. El instalador es tiny.
- **"El archivo `.delixon`"** — Exportar/importar config completa. Killer feature para equipos.

---

## 3. Qué hace Delixon

### 3.1 Aislamiento completo por proyecto

Cada proyecto registrado en Delixon tiene su propio contexto completamente independiente:

- **Terminal aislada**: historial de comandos propio, variables de entorno propias, PATH personalizado
- **Versiones de runtimes independientes**: Node 18 en un proyecto, Node 20 en otro, Python 3.10 en uno, 3.12 en otro — sin conflictos
- **Configuración de herramientas propia**: cada proyecto tiene su ESLint, Prettier, Black, Flake8, etc.
- **Secrets y credenciales locales**: nunca se comparten entre proyectos, nunca van al repositorio

**Qué se aísla exactamente:**
- Variables de entorno → Cada proyecto tiene su propio .env scope
- Historial de terminal → Comandos separados por proyecto
- Versiones de runtime → Node, Python, Go, Java por proyecto
- Puertos de red → Detección de conflictos automática
- Procesos en background → Saber qué corre en cada proyecto
- Git hooks → Pre-commit del proyecto correcto

**Qué NO se aísla (por diseño):**
- Tu editor (VSCode, Cursor, etc.) → lo comparten todos
- Tu configuración global de Git → misma identidad
- Tus herramientas del sistema → Delixon linkea, no duplica

### 3.2 Gestión inteligente de dependencias

Delixon no instala ciegamente. Antes de instalar una dependencia:

1. **Detecta** si ya existe una versión compatible en el sistema o en la caché de Delixon
2. **Vincula** la dependencia compartida si la versión es compatible (ahorro de disco y tiempo)
3. **Instala aislada** si se necesita una versión diferente, solo para ese proyecto
4. **Documenta** todo en los archivos de configuración del proyecto

Si el proyecto se mueve a otra máquina, los archivos de configuración tienen todo lo necesario para reconstruirlo desde cero con un solo comando.

### 3.3 Plantillas preconstruidas con mejores prácticas

Para las tecnologías más comunes, Delixon incluye plantillas listas para usar:

- Estructura de carpetas estándar y probada
- Configuración de linter y formatter lista desde el día 1
- Git hooks preconfigurados (commits limpios, tests antes de push)
- Archivos `.env.example` generados automáticamente
- Scripts de desarrollo, build, test y despliegue listos
- `.gitignore` completo y actualizado
- README inicial con estructura profesional

### 3.4 Apertura instantánea con contexto completo

Al abrir un proyecto desde Delixon (en menos de 2 segundos):
1. Activa el runtime correcto (Node 20, Python 3.11...)
2. Carga las variables de entorno del proyecto
3. Establece el historial de terminal del proyecto
4. Abre VSCode con el workspace del proyecto
5. Activa las extensiones específicas del proyecto
6. Si hay servicios (Docker, BD), los levanta
7. Muestra el estado del proyecto en el dashboard

### 3.5 Dashboard de proyectos

Vista central con el estado de todos los proyectos:
- Tecnologías usadas
- Última actividad
- Estado de dependencias (actualizadas, con vulnerabilidades conocidas, obsoletas)
- Tamaño en disco
- Rama de Git activa
- Cambios pendientes, PRs abiertas

---

## 4. Ejemplos prácticos

### Ejemplo A — Desarrollador freelance

**Sin Delixon:**
```
Lunes: Trabaja en proyecto-cliente-A (Node 18, PostgreSQL)
Martes: Cambia a proyecto-cliente-B (Node 20, MySQL)
- Tiene que cambiar la versión de Node manualmente
- Las variables de entorno de cliente-A siguen activas
- Ejecuta npm run dev y conecta a la base de datos de cliente-A por error
- Pierde 45 minutos depurando por qué "todo está raro"
```

**Con Delixon:**
```
$ delixon list
  📂 cliente-a/ecommerce    Node 18.17  ● corriendo
  📂 cliente-b/dashboard    Node 20.10  ○ parado
  📂 cliente-c/api          Python 3.11 ○ parado

$ delixon open cliente-b/dashboard
  ✅ Node 20.10 activado
  ✅ 12 variables de entorno cargadas
  ✅ VSCode abierto en /projects/cliente-b/dashboard
  ✅ Terminal lista con historial del proyecto

Ana cambia de proyecto en 2 segundos. Zero riesgo de cruzar datos.
```

### Ejemplo B — Nuevo desarrollador en el equipo

**Sin Delixon (día típico de onboarding):**
```
09:00  Clonar repos (3 repos, 15 min)
09:15  Instalar Node... ¿qué versión? El README dice 16 pero usan 20
09:45  npm install → falla por node-gyp en Windows
10:30  Configurar .env → pedir valores por Slack uno a uno
11:30  Docker para la BD → la versión de Docker no es compatible
12:30  Almuerzo. Todavía no ha visto el código.
15:30  Primer npm run dev exitoso
17:00  Se va a casa. Productividad del día: ~0
```

**Con Delixon:**
```
09:00  Instalar Delixon (2 min)
09:02  $ delixon clone startupx/main-api
       → Clona, detecta stack, instala runtime, carga env, abre editor
09:10  Carlos está leyendo el código con todo funcionando
09:30  Primer commit de Carlos
10:00  Productivo. Día 1.
```

### Ejemplo C — El proyecto olvidado

**Sin Delixon:**
```
$ cd mi-proyecto-viejo
$ npm start
→ Error: Node 16 required, you have Node 20
$ nvm install 16 && nvm use 16
$ npm install  → 47 warnings de seguridad, 3 errores de peer deps
$ cat .env.example  → ¿Cuáles eran los valores reales?
(30 minutos después, si hay suerte)
```

**Con Delixon:**
```
$ delixon open mi-proyecto-viejo
✅ Node 16.20 activado (lo tenía registrado)
✅ Variables de entorno restauradas
✅ Dependencias verificadas
✅ Listo en 3 segundos

Como si nunca te hubieras ido.
```

### Ejemplo D — Microservicios

**Sin Delixon:**
```
Terminal 1: cd auth && go run .
Terminal 2: cd api && nvm use 20 && npm run dev
Terminal 3: cd ml && pyenv activate ml-env && python main.py
Terminal 4: cd web && nvm use 18 && npm run dev
Terminal 5: docker-compose up postgres redis
+ recordar el orden de inicio
+ rezar para que los puertos no colisionen
```

**Con Delixon:**
```
$ delixon workspace open mi-producto
✅ auth      → Go 1.21, puerto 8080
✅ api       → Node 20, puerto 3000
✅ ml        → Python 3.11, puerto 5000
✅ web       → Node 18, puerto 5173
✅ postgres  → Docker, puerto 5432
✅ redis     → Docker, puerto 6379

Un comando. Todo orquestado. Puertos verificados.
```

---

## 5. Stack tecnológico: Tauri + React

### Por qué Tauri

| Criterio | Tauri | Electron | .NET/WPF |
|----------|-------|----------|----------|
| Peso del instalador | ~5 MB | ~80-150 MB | ~50 MB |
| Uso de memoria RAM | Bajo (~50 MB) | Alto (~200-500 MB) | Medio |
| Rendimiento | Nativo | Aceptable | Nativo |
| Cross-platform (Win/Linux/Mac) | Sí, nativo | Sí | Parcial |
| Acceso al sistema operativo | Rust (máximo control) | Node.js | .NET |
| Seguridad | Alta (modelo de permisos estricto) | Media | Alta |

**Para Delixon, Tauri es la elección adecuada porque:**
- Necesitamos interactuar profundamente con el sistema (procesos, archivos, variables de entorno, PATH)
- Rust nos da ese control con máximo rendimiento y seguridad
- React en el frontend nos permite una UI moderna y mantenible
- La base está preparada para Windows, Linux y macOS desde el principio

### Dependencias clave

```
Frontend (React):
- React 18
- TypeScript
- TailwindCSS
- Zustand (estado global)
- React Query (datos asincrónicos)
- Radix UI / shadcn (componentes accesibles)

Backend (Rust/Tauri):
- Tauri 2.x
- Serde (serialización)
- Tokio (operaciones asíncronas)
- which (detección de binarios en el sistema)
- dirs (rutas del sistema operativo)
```

---

## 6. Arquitectura del sistema

```
┌─────────────────────────────────────────────────────────┐
│                    Delixon App                          │
│                                                         │
│  ┌─────────────────────┐   ┌─────────────────────────┐  │
│  │   Frontend (React)  │   │    Backend (Rust/Tauri)  │  │
│  │                     │◄──►                         │  │
│  │  - Dashboard        │   │  - Project Manager      │  │
│  │  - Project Editor   │   │  - Environment Manager  │  │
│  │  - Template Gallery │   │  - Dependency Resolver  │  │
│  │  - Settings         │   │  - Shell Integrator     │  │
│  └─────────────────────┘   │  - Runtime Manager      │  │
│                            └────────────┬────────────┘  │
└─────────────────────────────────────────┼───────────────┘
                                          │
              ┌───────────────────────────┼──────────────┐
              │                           │              │
    ┌─────────▼────────┐  ┌──────────────▼──┐  ┌────────▼──────┐
    │  Sistema de       │  │  Gestor de       │  │  Integraciones│
    │  Archivos         │  │  Runtimes        │  │  externas     │
    │                   │  │                  │  │               │
    │  ~/.delixon/      │  │  - nvm/fnm       │  │  - VSCode     │
    │  ├── projects/    │  │  - pyenv         │  │  - Git        │
    │  ├── templates/   │  │  - rustup        │  │  - Docker     │
    │  ├── shared-deps/ │  │  - go toolchain  │  │  - Terminals  │
    │  └── config.json  │  └──────────────────┘  └───────────────┘
    └───────────────────┘
```

### Flujo de datos de un proyecto

```
Usuario crea proyecto
        │
        ▼
Delixon lee la plantilla seleccionada
        │
        ▼
Genera estructura de carpetas
        │
        ▼
Detecta runtimes disponibles en el sistema
        │
        ├── Runtime disponible → vincula
        └── Runtime no disponible → instala versión correcta
        │
        ▼
Configura variables de entorno aisladas
        │
        ▼
Inicializa Git con hooks preconfigurados
        │
        ▼
Registra proyecto en Delixon con su perfil completo
        │
        ▼
Abre VSCode con el workspace del proyecto listo
```

---

## 7. Las capas de Delixon

Delixon no es solo un gestor de workspaces. Es una **plataforma integral para el ciclo de vida completo del desarrollo**, organizada en capas donde cada una construye sobre la anterior.

```
┌─────────────────────────────────────────────────────────────┐
│              CAPA 5: EQUIPOS Y EMPRESA (futuro)             │
│  Onboarding · Secrets vault · Gobernanza · Catalogos corp   │
├─────────────────────────────────────────────────────────────┤
│              CAPA 4: INTELIGENCIA (futuro)                   │
│  Asistente IA · Auditoria automatica · Agentes · Sugeren-   │
│  cias contextuales · Aprendizaje adaptativo                  │
├─────────────────────────────────────────────────────────────┤
│              CAPA 3: OPERACION DIARIA  ✅                    │
│  Docker mgmt · Git · Scripts · Procesos · Puertos ·         │
│  Health checks · Doctor · Versionado · Notas                 │
├─────────────────────────────────────────────────────────────┤
│              CAPA 2: SCAFFOLDING  ✅                         │
│  Catalogo tecnologico · RulesEngine · Templates ·            │
│  Recipes · Scaffold wizard · Scan · Perfiles de madurez      │
├─────────────────────────────────────────────────────────────┤
│              CAPA 1: WORKSPACE  ✅                           │
│  Aislamiento · Env vars · Runtimes · Terminal · Dashboard    │
│  · Apertura en editor · Export/Import · Settings             │
├─────────────────────────────────────────────────────────────┤
│              CAPA 0: NUCLEO DECLARATIVO  ✅                  │
│  Project Manifest (.delixon/manifest.yaml)                    │
│  Todas las capas leen y escriben sobre el                     │
└─────────────────────────────────────────────────────────────┘
```

**Capa 0 (Nucleo)** — El manifest que unifica toda la info del proyecto. Sin el, todo lo demas seria una coleccion de botones inconexos.
**Capa 1 (Workspace)** — El core. Aislamiento, env vars, runtimes, terminal, dashboard. Lo que hace que Delixon sea util TODOS los dias.
**Capa 2 (Scaffolding)** — Motor de generacion y composicion. Crear, escanear, evolucionar el stack, validar arquitectura.
**Capa 3 (Operacion)** — Trabajo diario real. Docker, Git, scripts, health, doctor, versionado. Lo que retiene usuarios.
**Capa 4 (Inteligencia)** — Asistente que aprende, audita, sugiere y automatiza. Solo cuando las capas 0-3 esten blindadas con CI/CD, tests y metricas reales.
**Capa 5 (Equipos)** — Colaboracion, onboarding, gobernanza. Lo que monetiza a escala.

> **Capa 6 (vision futura): Servidor y cloud** — Delixon como servicio headless para gestionar entornos en servidores de desarrollo, CI/CD pipelines, y entornos de staging. La misma logica de capas 0-3 pero sin GUI, operando via CLI o API remota.

### 7.1 Capa 2: Motor de scaffolding

#### Catálogo tecnológico declarativo

```yaml
# Ejemplo: definición de una tecnología
id: fastapi
name: FastAPI
category: backend
runtime: python
version: "0.115"
compatibility:
  requires: [python]
  recommends: [postgresql, redis]
  conflicts: [django]
ports:
  default: 8000
scaffold:
  command: "pip install fastapi uvicorn"
  structure:
    - app/main.py
    - app/routes/
    - app/models/
    - app/services/
    - requirements.txt
```

#### Cobertura tecnológica objetivo

| Categoría | Tecnologías |
|---|---|
| **Runtime** | Node.js, Python, Go, Rust, Bun, Deno, PHP, Java |
| **Frontend** | Next.js, React, Vue, Nuxt, Svelte, Astro, Angular, Remix, Solid |
| **Backend** | Express, FastAPI, Django, Flask, NestJS, Gin, Actix, Spring Boot |
| **Base de datos** | PostgreSQL, MySQL, MongoDB, Redis, SQLite, Supabase |
| **ORM** | Prisma, Drizzle, SQLAlchemy, TypeORM, Mongoose, Diesel |
| **Auth** | NextAuth, Clerk, Lucia, Supabase Auth, JWT manual |
| **Styling** | Tailwind CSS, shadcn/ui, Chakra UI, Material UI, CSS Modules |
| **Servicios** | Nginx, Traefik, Mailpit, MinIO, RabbitMQ, Grafana, Prometheus |
| **DevOps** | Docker, GitHub Actions, Vitest, Jest, Playwright, ESLint, Prettier |

**+80 tecnologías** cubrirían el 95% de los stacks modernos.

#### Templates prearmados

| Template | Stack |
|---|---|
| **SaaS Starter** | Next.js + Prisma + PostgreSQL + NextAuth + Tailwind + Stripe |
| **API REST** | FastAPI + SQLAlchemy + PostgreSQL + Docker + pytest |
| **Full Stack MERN** | React + Express + MongoDB + Mongoose + JWT |
| **Dashboard interno** | Next.js + Prisma + PostgreSQL + shadcn + RBAC |
| **Monorepo** | Turborepo + React + Node API + shared packages |
| **Desktop app** | Tauri + React + TypeScript + Tailwind |
| **Microservicio** | Go/Rust + Docker + health checks + Prometheus |
| **Landing + API** | Astro + FastAPI + Supabase |

#### Validación inteligente de stacks

```
✅ Compatibilidad confirmada: Next.js + Prisma + PostgreSQL
⚠️ Advertencia: Seleccionaste Prisma Y TypeORM — ambos son ORMs, ¿cuál prefieres?
❌ Conflicto: Django requiere Python, pero seleccionaste Node.js como runtime
💡 Sugerencia: Si usas FastAPI, considera agregar Redis para caching
```

#### Generación orientada por tipo de producto

```
$ delixon create

  ¿Qué vas a construir?
  → SaaS B2B / Dashboard interno / API pública / Landing page /
    App móvil (backend) / E-commerce / Herramienta CLI / App de escritorio

  Seleccionaste: SaaS B2B

  Stack recomendado:
  ├── Frontend: Next.js 14 (App Router)
  ├── Database: PostgreSQL + Prisma
  ├── Auth: NextAuth (Google, GitHub, email)
  ├── Pagos: Stripe
  ├── Deploy: Vercel + Supabase
  └── Coste estimado: ~$0/mes hasta 1000 usuarios

  ¿Aceptar o personalizar?
```

#### Recipes: módulos que se añaden a un proyecto existente

```
$ delixon add auth --provider nextauth
  ✅ Instalado next-auth
  ✅ Creado app/api/auth/[...nextauth]/route.ts
  ✅ Creado lib/auth.ts con providers configurados
  ✅ Actualizado .env.example con variables de auth

$ delixon add database --type postgresql --orm prisma
  ✅ Instalado prisma y @prisma/client
  ✅ Creado prisma/schema.prisma con modelo User base
  ✅ Agregado PostgreSQL a docker-compose.yml
```

**Recipes disponibles:**
Auth, Base de datos, Pagos, Email, Colas, Storage, Observabilidad, Testing, CI/CD, Docker, Admin panel.

#### Modo "analizar proyecto existente"

```
$ delixon scan ./mi-proyecto-viejo

  Análisis completo:
  ├── Runtime: Node.js 18.17
  ├── Frontend: React 18 + Vite
  ├── Backend: Express 4.18
  ├── Testing: ❌ No detectado
  ├── Docker: ❌ No detectado
  └── Score: 4/10 (production readiness)

  Recomendaciones:
  1. ⚠️ Agregar testing (recipe: vitest)
  2. ⚠️ Agregar Docker (recipe: docker)
  3. 💡 Score mejoraría a 8/10 con estas adiciones

  ¿Aplicar recomendaciones? [seleccionar]
```

#### Perfiles de madurez (production hardening)

| Perfil | Qué incluye |
|---|---|
| **rapid** | Scaffold mínimo, arrancar rápido |
| **standard** | Linter, formatter, tests base, .env, Docker dev |
| **production** | Todo anterior + CI, health checks, logging, CORS, rate limiting |
| **enterprise** | Todo anterior + auditoría, RBAC, secrets policy, compliance |

### 7.2 Capa 3: Inteligencia (futuro — depende de validacion de capas 0-3)

> **Prerequisito:** Esta capa no se planifica ni implementa hasta que las capas 0-2 esten blindadas con CI/CD cross-platform, tests automatizados, y metricas reales de uso. La fortaleza de Delixon hoy es manifest + scan + open + doctor + scaffold + recipes + GUI/CLI dual. La IA amplifica eso, no lo reemplaza.

**Asistente IA integrado** — aprende de patrones de uso, se adapta al developer, sugiere stacks basados en preferencias anteriores. Memoria persistente de decisiones y errores resueltos.

**Agentes especializados** — SecurityGuard (OWASP, secrets, CVEs), CodeReviewer (calidad, complejidad), TestBuilder (genera tests, cobertura), PerfAnalyzer (bundle, queries, N+1), DocWriter (README, API docs), InfraOps (Docker, CI/CD), DataOptimizer (schema, indices), APIDesigner (REST/GraphQL).

**Pipeline de auditoria** — `delixon audit` ejecuta todos los agentes en un solo comando y genera un score general del proyecto. Es el producto premium para equipos.

### 7.3 Flujo entre capas

```
1. CREAR (Capa 2)      → delixon new / scaffold wizard en GUI
2. REGISTRAR (Capa 0+1) → Manifest generado, proyecto aislado con env vars y runtime
3. TRABAJAR (Capa 1+3)  → delixon open → editor + terminal + Docker + health en 2s
4. EVOLUCIONAR (Capa 2) → delixon add auth / add payments / snapshot save
5. DIAGNOSTICAR (Capa 3) → delixon doctor / health / status / diff
6. AUDITAR (Capa 4)     → delixon audit → seguridad, calidad, tests (futuro)
7. COMPARTIR (Capa 1+5) → delixon export → archivo .delixon para onboarding
```

> Este flujo funciona igual en **Windows, Linux y macOS**. El core Rust maneja rutas, shells y procesos de forma platform-aware (`cfg(target_os)`).

---

## 8. Estructura del proyecto

```
delixon/
├── src-tauri/                    # Backend Rust
│   ├── src/
│   │   ├── main.rs               # Punto de entrada Tauri
│   │   ├── commands/             # Comandos expuestos al frontend
│   │   │   ├── projects.rs       # CRUD de proyectos
│   │   │   ├── environments.rs   # Gestión de entornos
│   │   │   ├── dependencies.rs   # Resolución de dependencias
│   │   │   ├── runtimes.rs       # Gestión de versiones de runtimes
│   │   │   └── shell.rs          # Integración con terminales
│   │   ├── models/               # Estructuras de datos
│   │   │   ├── project.rs
│   │   │   └── config.rs
│   │   └── utils/                # Utilidades del sistema
│   │       ├── fs.rs             # Operaciones de archivo
│   │       ├── process.rs        # Gestión de procesos
│   │       └── platform.rs       # Detección de SO
│   └── tauri.conf.json
│
├── src/                          # Frontend React
│   ├── components/
│   │   ├── ui/                   # Componentes reutilizables (PathInput, ScrollRow...)
│   │   ├── dashboard/            # ProjectCard, modales (Create, Register, Import)
│   │   ├── project/              # Tabs del detalle de proyecto (9 tabs)
│   │   ├── templates/            # UseTemplateModal
│   │   └── layout/               # Sidebar
│   ├── pages/                    # Dashboard, ProjectDetail, Catalog, Templates, Scaffold, Settings
│   ├── stores/                   # Zustand stores
│   ├── styles/
│   │   ├── delixon/              # Paleta de colores, fonts, base CSS
│   │   └── tech/                 # Colores de marca por tecnologia (brand + catalog)
│   ├── lib/                      # Tauri bridge, tech-meta, catalog helpers
│   └── i18n/                     # Traducciones (es.json, en.json)
│
└── docs/
```

### Archivo de configuración delixon.yaml

```yaml
name: mi-api
stack: node

runtime:
  node: "20.10"
  npm: "10.2"

env:
  DATABASE_URL: "postgresql://localhost:5432/mydb"
  API_KEY: "${vault:api-key-prod}"   # referencia segura
  PORT: 3000

services:
  - docker-compose up -d postgres redis

scripts:
  start: "npm run dev"
  test: "npm run test"
  lint: "npm run lint"

editor:
  vscode:
    extensions:
      - dbaeumer.vscode-eslint
      - esbenp.prettier-vscode
    settings:
      editor.formatOnSave: true

on_open:
  - npm install --silent
  - echo "✅ Proyecto listo"
```

---

## 9. Arquitectura por capas

### Vision central

> **Delixon es el sistema operativo local del developer.**

No es una coleccion de features sueltas. Es un sistema por capas donde cada capa construye sobre la anterior. **Delixon crea, entiende, ejecuta, aisla, amplia, valida y repara proyectos localmente desde una sola app.**

### Capas del producto

```
Capa 5 — Equipos y empresa (futuro)
  Onboarding, secrets vault, gobernanza, catalogos corporativos

Capa 4 — Inteligencia (largo plazo)
  IA adaptativa, agentes, auditoria, sugerencias contextuales

Capa 3 — Operacion diaria (implementado)
  Docker management, Git, scripts, procesos, puertos,
  health checks, doctor, versionado/snapshots, notas

Capa 2 — Scaffolding y composicion (implementado)
  Catalogo tecnologico, RulesEngine, templates, recipes,
  scaffold wizard, perfiles de madurez, scan/deteccion

Capa 1 — Workspace (implementado)
  Gestion de proyectos (CRUD), env vars aisladas, runtimes,
  apertura en editor, terminal, dashboard, export/import

Capa 0 — Nucleo declarativo (implementado)
  Project Manifest (.delixon/manifest.yaml)
  Todas las capas leen y escriben sobre el
```

### Que aporta cada capa

| Capa | Responsabilidad | Estado | Modulos principales |
|---|---|---|---|
| **Capa 0 — Nucleo** | Manifest que unifica toda la info del proyecto | ✅ Implementado | `manifest.rs` |
| **Capa 1 — Workspace** | Gestion, aislamiento, entorno por proyecto | ✅ Implementado | `storage`, `config`, `portable`, `vscode` |
| **Capa 2 — Scaffolding** | Creacion, validacion y composicion de stacks | ✅ Implementado | `catalog`, `rules`, `scaffold`, `recipes`, `templates`, `detection` |
| **Capa 3 — Operacion** | Trabajo diario, diagnostico, evolucion | ✅ Implementado | `docker`, `git`, `scripts`, `health`, `doctor`, `versioning`, `snapshots`, `notes`, `ports`, `processes` |
| **Capa 4 — Inteligencia** | IA, agentes, auditoria automatizada | Pendiente | — |
| **Capa 5 — Equipos** | Colaboracion, onboarding, gobernanza | Pendiente | — |

### Decisiones descartadas

| Concepto | Razon |
|---|---|
| DevContainers generation (.devcontainer/) | Va contra la filosofia "sin Docker para dev". Solo como export opcional futuro |
| Monorepo con Turborepo | Delixon es monolito Tauri. No aplica |
| SQLite como DB local | JSON funciona para <100 proyectos. Evaluar a futuro si escala a equipos |

### El nucleo declarativo (CRITICO)

Sin una capa declarativa solida, el producto seria fragil — "una coleccion de botones" en vez de un sistema coherente.

**Project Manifest** — formato interno que define cada proyecto:

```yaml
# .delixon/manifest.yaml (generado automáticamente, editable)
schemaVersion: 1
name: mi-saas
projectType: saas-b2b
profile: standard
runtime: node@20
technologies:
  - nextjs@14
  - prisma@5
  - postgresql@16
  - tailwindcss@3
  - nextauth@4
services:
  - name: postgresql
    port: 5432
    docker: true
    healthCheck: "pg_isready -U postgres"
  - name: redis
    port: 6379
    docker: true
    healthCheck: "redis-cli ping"
envVars:
  required: [DATABASE_URL, NEXTAUTH_SECRET, NEXTAUTH_URL]
  optional: [REDIS_URL, STRIPE_KEY]
commands:
  dev: "npm run dev"
  build: "npm run build"
  test: "npm run test"
  lint: "npm run lint"
ports: [3000, 5432, 6379]
recipesApplied: [auth-nextauth, database-prisma, docker-services]
healthChecks: []
metadata:
  description: "SaaS B2B de gestion de ventas"
  createdAt: "2026-03-27T10:00:00Z"
  author: "equipo-ventas"
editor: code
```

> **Protecciones del manifest:**
> - `schemaVersion` permite migraciones futuras sin romper manifests existentes
> - `validate_manifest()` rechaza: name vacio, puertos 0 o duplicados, env vars con valores (`KEY=value`)
> - `normalize_manifest()` deduplica puertos/techs/recipes y limpia whitespace antes de guardar
> - `save_manifest()` SIEMPRE valida y normaliza — nunca se escribe basura al disco
> - `metadata` (description, createdAt, author) — responde "quien creo esto y cuando"
> - `editor` opcional — permite editor distinto al global por proyecto
> - `envVars` solo guarda NOMBRES de variables, nunca valores (los valores van en `envs/*.json`, fuera del repositorio)

**Este manifiesto es la columna vertebral.** Todo lo demás (dashboard, health checks, doctor, scan, recipes, versionado) lee y escribe sobre él. Sin él, cada feature es un silo independiente.

### Flujo ideal del producto

```
1. CREAR o IMPORTAR
   ├── Nuevo: elegir tipo → proponer stack → validar reglas → generar
   └── Existente: scan → detectar stack → generar manifest → registrar

2. CONFIGURAR (automático)
   ├── Env vars aisladas
   ├── Runtime correcto activado
   ├── Servicios Docker levantados
   ├── Puertos verificados
   └── Health check: todo OK

3. TRABAJAR (día a día)
   ├── Abrir en VSCode con contexto
   ├── Terminal aislada con env vars
   ├── Dashboard con estado real de todos los proyectos
   ├── Doctor: ¿qué falta? ¿qué se rompió?
   └── Health: ¿DB arriba? ¿puertos libres? ¿deps ok?

4. EVOLUCIONAR
   ├── Recipes: agregar auth, payments, testing, docker
   ├── Preview de cambios antes de aplicar
   ├── Versionado: save → diff → rollback si no convence
   └── Perfiles de madurez: subir de rapid a production

5. COMPARTIR
   ├── Archivo .delixon → otro dev reproduce el entorno en 5 min
   ├── .delixon-team → config de equipo sincronizada
   └── Secrets vault → no más "pásame el .env por Slack"
```

### Principios de diseno

**Fundamentos:**
- **Una sola app, una sola UI** — GUI y CLI son interfaces al mismo core (`delixon_lib`)
- **Calidad sobre cantidad** — 30+ tecnologias solidas > 83 a medias; 7 templates probados > 20 sin mantener
- **Cada capa construye sobre la anterior** — sin el manifest (Capa 0), todo lo demas seria un silo independiente
- **Sin Docker para el dev** — Docker solo para servicios de infra (PostgreSQL, Redis). Runtimes nativos

**4 verbos canonicos — filtro de producto:**

Todo lo que Delixon hace cabe en 4 verbos. Si una feature no cabe en ninguno, no pertenece al producto:

| Verbo | Que hace | Comandos asociados |
|---|---|---|
| **scan** | Entender que hay | `scan`, `detect`, `catalog`, `validate` |
| **open** | Activar el entorno correcto | `open`, `env`, `run`, `docker up` |
| **doctor** | Diagnosticar que falta o falla | `doctor`, `health`, `ports`, `ps`, `diff` |
| **evolve** | Cambiar el proyecto sin romperlo | `add`, `new`, `scaffold`, `snapshot`, `rollback` |

> Si alguien propone una feature y no cabe en scan/open/doctor/evolve, la respuesta es "no, gracias".

**`open` es la feature sagrada:**

`open` es el comando que crea el habito. Si `delixon open mi-proyecto` no es la forma mas rapida de empezar a trabajar, el producto fracasa. Garantias:
- Abre en <2 segundos, sin excepciones
- Activa el runtime correcto (nvm/fnm, pyenv, rustup)
- Carga env vars aisladas
- Si algo falla, dice exactamente que y como arreglarlo
- Es el primer comando que un nuevo usuario prueba y el que usa todos los dias

**Separacion de estados:**

| Tipo | Donde vive | Quien lo escribe | Ejemplo |
|---|---|---|---|
| **Estado deseado** | `.delixon/manifest.yaml` | El developer (via scaffold, add, edit) | "Este proyecto necesita Node 20, PostgreSQL, puerto 3000" |
| **Estado observado** | Resultado de `doctor`/`health` | Delixon (via diagnostico en tiempo real) | "Node 20 no esta instalado, Puerto 3000 esta ocupado" |

El manifest dice QUE deberia existir. Doctor/health dicen QUE existe realmente. El gap entre ambos es lo que Delixon ayuda a cerrar. Nunca mezclar: el manifest no se modifica segun lo observado automaticamente.

**Patron universal preview/diff/confirm:**

Toda operacion que modifica estado en disco sigue el mismo flujo:
1. **Preview** — mostrar que va a hacer
2. **Diff** — mostrar que va a cambiar
3. **Confirm** — pedir confirmacion (saltable con `--yes`)

Aplica a: `scaffold`, `recipe apply`, `rollback`, `import`, y cualquier comando futuro que escriba archivos. No es un "modo confianza" — es disciplina de UX obligatoria.

**Madurez interna de features:**

Cada feature del producto tiene un nivel de madurez interno (no expuesto al usuario):
- **Estable** — tests, validacion, docs, ≥1 release sin breaking changes → aparece en onboarding y docs
- **Beta** — funciona, tests basicos, API puede cambiar → aparece en `doctor` y CLI pero no en onboarding
- **Experimental** — existe pero sin garantias → solo accesible via CLI, sin docs publicos

El criterio es objetivo: tests + validacion + estabilidad. Toda feature empieza como experimental y asciende.

**Mensajes de error — requisitos de calidad:**

Todo error que Delixon muestre al usuario debe incluir 4 cosas:
1. **Que intento hacer** — "Intenté abrir el proyecto X en VS Code"
2. **Que detecto** — "VS Code no está instalado o no está en el PATH"
3. **Por que fallo** — "Sin editor disponible, no puedo abrir el proyecto"
4. **Que hacer** — "Instala VS Code y asegúrate de que `code` esté en el PATH, o configura otro editor en Settings"

Un error sin "que hacer" es un error inutil. Un error sin contexto ("Error: file not found") es peor que no mostrar nada.

---

## 10. Hoja de ruta y fases

### Estado actual — Lo que YA funciona

**Capa 0 — Nucleo declarativo:**
- [x] Project Manifest (`.delixon/manifest.yaml`) — techs, servicios, env vars, comandos, puertos, recipes, health checks
- [x] schema_version, metadata (description, created_at, author), editor opcional
- [x] Validacion y normalizacion obligatoria antes de guardar (validate + normalize en save_manifest)

**Capa 1 — Workspace:**
- [x] App de escritorio con Tauri 2 + React
- [x] CRUD de proyectos (crear, abrir, eliminar, actualizar)
- [x] Aislamiento de env vars por proyecto (JSON por proyecto)
- [x] Deteccion de runtimes: Node.js, Python, Rust, Go, .NET, PHP, Ruby
- [x] Apertura de proyecto en editor configurado (VS Code, Cursor, Zed, Neovim, etc.)
- [x] Apertura de terminal con env vars cargadas
- [x] Dashboard con busqueda, filtros y grid de proyectos
- [x] Pagina de detalle de proyecto con gestion de env vars
- [x] Export/import de configuracion (.delixon portable)
- [x] Settings: editor, tema, idioma, deteccion de runtimes
- [x] Sidebar con navegacion y proyectos recientes
- [x] Persistencia local (JSON en `~/.local/share/delixon/`)

**Capa 2 — Scaffolding y composicion:**
- [x] Catalogo de 30+ tecnologias en YAML con metadatos completos y UI de browse/search
- [x] RulesEngine: validacion, dependencias automaticas, conflictos, puertos, sugerencias
- [x] ScaffoldOrchestrator: genera estructura, docker-compose, .env, README, CI/CD, scripts, Makefile, VS Code config
- [x] 7 templates funcionales modularizados (1 archivo .rs por template + registry.rs): Node+Express, React+Vite, FastAPI, Django, Fullstack, Rust CLI, Docker Compose
- [x] 6 recipes aplicables (vitest, pytest, docker, ci-github, biome, prisma)
- [x] Full-stack detection (frontend/ + backend/) con readiness score
- [x] Perfiles de madurez (rapid/standard/production)
- [x] Scan de proyectos existentes (964 lineas de logica de deteccion)

**Capa 3 — Operacion diaria:**
- [x] 28 comandos CLI (ver `docs/cli/CLI_REFERENCE.md`)
- [x] Docker management (up/down/status/logs) — GUI + CLI
- [x] Git integration (rama, cambios, remoto, commits) — GUI + CLI
- [x] Scripts unificados (ejecutar desde manifest) — GUI + CLI
- [x] Health checks por proyecto con sugerencias de fix — GUI + CLI
- [x] Doctor del sistema (runtimes, Docker, Git, config) — GUI + CLI
- [x] Versionado de stacks (save/diff/rollback) — GUI + CLI
- [x] Snapshots de entorno
- [x] Notas por proyecto (CRUD con UUID y timestamps)
- [x] Gestion de puertos y procesos — GUI + CLI

**Sistema de diseno (GUI):**
- [x] Paleta semantica Delixon: info (azul), success (verde), warning (ambar), error (rojo), dlx-grays (6 niveles fondo + 6 texto)
- [x] Colores de marca por tecnologia en CSS (`src/styles/tech/`): brand.css (texto en project cards) y catalog.css (fondos en catalog cards)
- [x] Safelist de clases Tailwind para colores dinamicos (`src/lib/tech-safelist.ts`)
- [x] Sistema de aliases de tech IDs (node→nodejs, postgres→postgresql, etc.) en `tech-meta.ts`
- [x] Font packs configurables (modern, classic, developer)
- [x] Componentes reutilizables: PathInput (browse nativo), ScrollRow (carousel horizontal), TechCard
- [x] Botones con patron uniforme: accion→success, navegacion→info, peligro→error, neutral→dlx-grays (hover solo fondo, texto fijo)
- [x] i18n parcial (es/en) con react-i18next

**Landing page (delixon-web):**
- [x] Landing completa con paneles expandibles, efectos 3D, i18n ES/EN
- [x] Waitlist backend: Fastify + PostgreSQL + Docker
- [x] Admin panel, referidos, double opt-in

### Principio rector

> **"Primero indispensable. Luego potente. Después ambicioso."**
>
> Hoy, "indispensable" para Delixon es: manifest, open, doctor, health, scan, CLI, 3-5 templates perfectos, 2-4 recipes impecables. Todo lo demás puede esperar.

**Instalacion insultantemente facil:**

Si un developer no puede ir de "descargar Delixon" a "primer proyecto abierto con entorno correcto" en menos de 5 minutos, hay un problema de producto, no de usuario. El onboarding no es una feature secundaria — es la primera impresion y para muchos la unica oportunidad.

**Windows es el campo de batalla principal:**

No "compatible con Windows". Windows PRIMERO. Mas del 40% de developers usan Windows. Cada decision de diseño se prueba primero en Windows, luego se verifica en Linux y macOS. Cada `Path::new()`, cada deteccion de shell, cada `tasklist` en vez de `lsof`. Si funciona en Windows y no en macOS, es un bug pendiente. Si funciona en macOS y no en Windows, es un bug critico.

### CORTO PLAZO (1-3 meses) — "Que funcione de verdad"

> **Base declarativa + operación local + CLI + cross-platform desde el día 1.**
>
> Sin la base declarativa, todo lo demás será frágil. Sin operación local útil, nadie lo usa dos veces.
> Sin CLI, pierdes a la mayoría de devs. Sin Linux, pierdes a la mitad del público.

**P0 — Nucleo declarativo (la columna vertebral):**
- [x] Definir formato de `project manifest` (.delixon/manifest.yaml) — techs, servicios, env vars, comandos, puertos, recipes, health checks
- [x] Catalogo YAML con 30+ tecnologias, metadatos completos, UI de browse/search
- [x] Integrar RulesEngine con dependencias auto, incompatibilidades, puertos, sugerencias
- [x] Generar manifest automaticamente al crear o importar un proyecto

**P0 — Completar workspace (Capa 1):**
- [ ] Historial de terminal aislado por proyecto
- [ ] Activacion automatica de runtimes al abrir proyecto — integracion con nvm/fnm (Node), pyenv (Python), rustup (Rust)
- [x] Exportar/importar configuracion de proyecto (archivo `.delixon`) — portable.rs + UI
- [x] Deteccion de conflictos de puertos entre proyectos — ports.rs con TCP check

**P0 — CLI desde el día 1 (los devs viven en la terminal):**
- [x] Arquitectura CLI con subcomandos (clap en Rust, binario separado que invoca el mismo core)
- [x] La CLI y la GUI comparten el mismo core (misma lógica, mismos datos, misma persistencia)
- [ ] Instalable globalmente: el dev escribe `delixon` desde cualquier terminal

**28 comandos implementados (delixon-cli):**

| Comando | Descripción | Uso |
|---|---|---|
| `list` | Lista todos los proyectos registrados | `delixon-cli list` |
| `open <name>` | Abre proyecto en el editor configurado | `delixon-cli open mi-proyecto` |
| `create <name> --path <ruta>` | Crea un nuevo proyecto | `delixon-cli create mi-app --path ./apps --template id` |
| `scan <path>` | Detecta el stack de un proyecto existente | `delixon-cli scan ./mi-proyecto` |
| `doctor` | Verifica el estado del sistema | `delixon-cli doctor` |
| `env <project> get` | Muestra variables de entorno | `delixon-cli env mi-app get` |
| `env <project> set <key> <val>` | Establece variable de entorno | `delixon-cli env mi-app set PORT 3000` |
| `export <project>` | Exporta proyecto como `.delixon` | `delixon-cli export mi-app -o archivo.delixon` |
| `import <file> --path <ruta>` | Importa desde archivo `.delixon` | `delixon-cli import app.delixon --path ./apps` |
| `manifest <project>` | Muestra el manifest del proyecto | `delixon-cli manifest mi-app` |
| `catalog [id]` | Navega catálogo de tecnologías | `delixon-cli catalog` o `delixon-cli catalog rust` |
| `validate <techs...>` | Valida combinación de tecnologías | `delixon-cli validate rust react docker` |
| `health <project>` | Ejecuta health checks del proyecto | `delixon-cli health mi-app` |
| `ports` | Muestra puertos en uso por proyectos | `delixon-cli ports` |
| `new <name> --path <ruta>` | Genera proyecto desde scaffold | `delixon-cli new api --path ./apps --type api --profile standard --techs rust,docker` |
| `add <recipe>` | Aplica una recipe al proyecto | `delixon-cli add testing-vitest --project mi-app --preview` |
| `recipes` | Lista recipes disponibles | `delixon-cli recipes` |
| `status <project>` | Muestra estado Git del proyecto | `delixon-cli status mi-app` |
| `docker up <project>` | Inicia servicios Docker Compose | `delixon-cli docker up mi-app` |
| `docker down <project>` | Detiene servicios Docker | `delixon-cli docker down mi-app` |
| `docker status <project>` | Estado de servicios Docker | `delixon-cli docker status mi-app` |
| `docker logs <project>` | Muestra logs Docker | `delixon-cli docker logs mi-app --lines 100` |
| `run <script>` | Ejecuta script del manifest | `delixon-cli run dev --project mi-app` |
| `snapshot save <project>` | Guarda snapshot del manifest | `delixon-cli snapshot save mi-app` |
| `snapshot list <project>` | Lista snapshots guardados | `delixon-cli snapshot list mi-app` |
| `snapshot diff <project> <v1> <v2>` | Compara dos versiones | `delixon-cli snapshot diff mi-app 1 2` |
| `snapshot rollback <project> <ver>` | Restaura versión anterior | `delixon-cli snapshot rollback mi-app 1` |
| `diff <project>` | Cambios desde último snapshot | `delixon-cli diff mi-app` |
| `note <project> [text]` | Gestiona notas (sin texto = lista) | `delixon-cli note mi-app "nota aquí"` |
| `ps [project]` | Lista procesos en puertos del proyecto | `delixon-cli ps mi-app` |

> **Decisión de diseño:** CLI y GUI son dos interfaces al mismo motor. No son productos separados, no compiten. La CLI es para acciones rápidas, la GUI es para explorar y configurar. Ambas leen/escriben sobre el mismo manifest y la misma persistencia JSON.

**P0 — Cross-platform desde el dia 1 (Windows + Linux + macOS):**
- [x] Tauri compila cross-platform
- [x] Adaptacion de rutas y permisos por SO — logica platform-aware en Rust (cfg(target_os), lsof vs tasklist, etc.)
- [ ] CI/CD con GitHub Actions: build y test en Windows, Ubuntu y macOS en cada PR
- [ ] Documentar diferencias por SO: rutas de datos, terminales disponibles, binarios detectados
- [ ] Probar la CLI en los tres SO

> **Por qué no esperar:** El 50%+ de los developers objetivo usan Linux o macOS. Retrasar cross-platform es retrasar la adopción. Además, Tauri + Rust hacen que el coste de mantener 3 plataformas sea bajo desde el inicio. El verdadero coste de "agregar Linux después" es acumular decisiones Windows-only que luego cuestan mucho revertir.

**P1 — Crear proyectos reales (Capa 2 — Scaffolding):**
- [x] Conectar flujo "crear proyecto" del dashboard y CLI con ScaffoldOrchestrator
- [x] 7 templates funcionales: Node+Express, React+Vite, Python+FastAPI, Python+Django, Fullstack, Rust CLI, Docker Compose
- [x] Cada template genera: estructura, deps, scripts, docker-compose, .env.example, README, Makefile, CI workflows, VS Code config
- [ ] Tests automatizados de generacion: cada template se genera y arranca sin errores
- [x] `delixon create` y `delixon new` desde CLI con parametros de stack

**P1 — Diagnosticar (lo que hace que Delixon sea util el dia 1):**
- [x] `doctor` del sistema: verificar runtimes (Node, Python, Rust, Go, PHP, Ruby), Docker, Git, config, datos
- [x] Health checks por proyecto: directorio, README, Git, .gitignore, deps, runtimes, puertos
- [x] HealthTab en GUI con sugerencias de fix por cada check
- [x] `delixon doctor` y `delixon health <proyecto>` desde CLI

**P1 — Scan de proyectos existentes (duplica el publico objetivo):**
- [x] `delixon scan ./mi-proyecto` → detecta: lenguaje, framework, package manager, ORM, DB, scripts, Docker, CI, testing, linters, TypeScript, fullstack
- [x] Generar manifest desde scan → registrar proyecto → gestionar con Delixon
- [x] Readiness score con breakdown detallado por categoria

**Entregable:** ✅ MVP completado — se pueden CREAR proyectos completos desde GUI (wizard) o CLI (`new`/`create`), IMPORTAR proyectos existentes con `scan`, ver el ESTADO REAL con health/doctor, gestionar Docker/Git/scripts/env. Manifest unifica todo. **Pendiente:** CI/CD multi-SO, tests automatizados de templates, terminal integrada.

### MEDIANO PLAZO (3-6 meses) — "Que sea útil de verdad"

> **Operación diaria completa + evolución de proyectos**
>
> Lo que retiene usuarios: "puedo evolucionar mi proyecto sin miedo y Delixon me dice qué falla".

**P1 — Recipes (lo que hace que Delixon sirva despues del dia 1):**
- [x] Sistema de recipes: `delixon add <recipe>` con preview y aplicacion
- [x] Preview de cambios antes de aplicar (`--preview` flag)
- [x] 6 recipes funcionales: testing-vitest, testing-pytest, docker, ci-github, linting-biome, prisma
- [ ] **3 recipes criticas (prioridad maxima):** Auth (NextAuth/Clerk/JWT), Database+ORM (PostgreSQL+Prisma / SQLAlchemy), Monitoring (health endpoint + logging estructurado) — sin estas 3, la propuesta de recipes se siente incompleta
- [ ] Mas recipes: Pagos (Stripe), Email (Resend/Nodemailer), Admin panel, Observabilidad

**P2 — Versionado de stacks (reduce el miedo):**
- [x] Save del estado del stack antes de cambios (snapshot save)
- [x] Diff visual entre versiones — techs añadidas/removidas, recipes aplicadas (GUI VersioningTab + CLI)
- [x] Rollback de manifest a version anterior (snapshot rollback)
- [x] Historial de evolucion del proyecto (snapshot list)

**P2 — Operacion diaria avanzada:**
- [x] Docker Compose management integrado — up/down/status/logs desde GUI (DockerTab) y CLI
- [ ] Terminal integrada dentro de Delixon (panel embebido)
- [x] Contexto de Git integrado — rama, cambios, remoto, commits (GitTab + CLI status)
- [x] Scripts con alias unificados — `delixon run <script>` ejecuta desde manifest (ScriptsTab + CLI)
- [x] Gestion de procesos — `delixon ps`, kill desde GUI (ProcessesTab + CLI)
- [x] Snapshots de entorno — comparar runtimes y deps entre momentos (snapshots.rs)
- [ ] Gestion de runtimes: instalar/cambiar versiones desde la app
- [ ] Notificaciones de dependencias desactualizadas o vulnerables

**P2 — CLI avanzado:**
- [x] Completar comandos: `add`, `ps`, `run`, `docker logs`, `env`, `snapshot`, `diff`, `note`, `recipes`, `validate`, `new`, `catalog`, `health`, `ports`, `manifest`, `export`, `import`, `status` (28 comandos totales)
- [ ] Autocompletado para bash/zsh/fish/PowerShell
- [x] Output formateado (colores, tablas con colored crate)

### LARGO PLAZO (6-12 meses) — "Que sea indispensable"

> **Equipos + madurez del producto**
>
> Lo que monetiza: equipos pagan, individuos no.

**P3 — Equipos (Capa 5):**
- [ ] Exportacion de configuracion de equipo (`.delixon-team`)
- [ ] Onboarding automatizado: nuevo dev → `delixon setup` → entorno completo en 5 min
- [ ] Secrets vault encriptado (AES-256) para compartir credenciales
- [x] Project notes / contexto rapido — implementado con CRUD, UUID, timestamps (GUI NotesTab + CLI)

**P3 — Madurez:**
- [x] Perfiles de madurez que cambian archivos reales — rapid/standard/production integrados en scaffold
- [ ] Generacion orientada por tipo de producto ("¿Que vas a construir?" → stack recomendado) — mediano plazo
- [ ] Soporte multi-editor completo: Cursor, WebStorm, Neovim, Zed
- [ ] Control de versiones de plantillas y configuraciones
- [x] 7 templates funcionales (Node+Express, React+Vite, FastAPI, Django, Fullstack, Rust CLI, Docker Compose)
- [x] Catalogo de 30+ tecnologias con metadatos completos

### VISION FUTURA — Ideas condicionales (12+ meses)

> **⚠️ Nada de esta seccion se implementa hasta que las capas 0-3 esten blindadas, con CI/CD cross-platform, tests automatizados en los 3 SO, y metricas reales de uso.** Estas son ideas, no compromisos. El producto se juzga por lo que funciona hoy, no por lo que promete para manana.

**Capa 4: Inteligencia**

- [ ] Asistente IA con aprendizaje adaptativo (recuerda preferencias, sugiere stacks)
- [ ] Agentes especializados: SecurityGuard, CodeReviewer, TestBuilder, PerfAnalyzer, DocWriter
- [ ] Pipeline de auditoria completa (seguridad + calidad + tests + performance en un comando)
- [ ] Modo "arquitecto asistente" (describe lo que quieres → stack recomendado con estimacion de coste)

**Capa 5: Equipos y empresa**

- [ ] Catalogos corporativos (tecnologias aprobadas/prohibidas por empresa)
- [ ] Templates privadas de organizacion con politicas y cobertura minima
- [ ] Configuracion de equipo (`.delixon-team`) — sincronizar preferencias
- [ ] Onboarding automatizado — nuevo dev productivo en 5 min
- [ ] Secrets vault encriptado (AES-256)

**Capa 6: Servidor y cloud**

> Delixon no tiene por que limitarse a desktop local. La misma logica de capas 0-3 puede operar en servidores sin GUI.

- [ ] CLI headless para servidores de desarrollo (Linux, sin Tauri, sin GUI)
- [ ] API remota para gestionar entornos desde CI/CD pipelines
- [ ] Gestion de multiples proyectos en produccion (modo servidor)
- [ ] Integracion con herramientas de monitoreo (Grafana, Prometheus)
- [ ] Delixon como servicio en entornos de staging/pre-produccion
- [ ] Delixon agent: proceso que vigila health, puertos y servicios en background

> **Caso de uso servidor:** Un equipo de 10 devs tiene un servidor de desarrollo compartido. Delixon corre en modo headless, cada dev usa `delixon-cli` para gestionar sus entornos remotos. El servidor mantiene aislamiento por proyecto, health checks automaticos, y notifica si algo se rompe. Funciona en **Linux** (principal), **Windows Server** y **macOS** (CI runners).

**Ecosistema y distribucion**

- [ ] Sistema de plugins (la comunidad extiende Delixon)
- [ ] Marketplace de templates y recipes
- [ ] Exportacion automatica de decisiones tecnicas (ADR)
- [ ] Editor visual de plantillas y catalogos
- [ ] Soporte multi-editor completo: Cursor, WebStorm, Neovim, Zed, Sublime
- [ ] DevContainers export (para equipos que lo requieran)
- [ ] Editor visual de plantillas

### Tabla resumen de prioridades

| Prioridad | Que | Estado | Pendiente |
|---|---|---|---|
| **P0** | Manifest + workspace + CLI + cross-platform | ✅ Mayormente completado | Terminal aislada, activacion runtimes, CI/CD multi-SO |
| **P1** | Templates + scaffold + scan + health + doctor | ✅ Completado | Tests automatizados de templates |
| **P1** | Recipes (6 funcionales) | ✅ Completado | Mas recipes: Auth (NextAuth/Clerk/JWT), DB+ORM, Monitoring, Pagos, Email |
| **P2** | Versionado + Docker mgmt + CLI avanzado + Git | ✅ Completado | Terminal integrada, gestion runtimes, notificaciones deps |
| **P3** | Equipos + perfiles madurez + multi-editor + mas templates | Pendiente | Lo que monetiza |
| **P4** | IA + agentes + marketplace + plugins + catalogos corp | Pendiente | Lo que diferencia a largo plazo |

---

## 11. Objetivos por fase

### Fase 1 — Objetivos concretos

| Objetivo | Criterio de éxito |
|----------|-------------------|
| Crear un proyecto desde plantilla | En menos de 2 minutos, proyecto creado y abierto en VSCode |
| Abrir un proyecto existente | En 1 clic, VSCode abre con el entorno correcto cargado |
| Aislamiento de entorno | Las variables de un proyecto no son visibles desde otro |
| Historial aislado | El historial de terminal de cada proyecto es independiente |
| Exportar configuración | Se genera un archivo `.delixon` que reconstruye el entorno en otra máquina |

### Fase 2 — Objetivos concretos

| Objetivo | Criterio de éxito |
|----------|-------------------|
| Motor de scaffolding | Un proyecto nuevo con stack validado en menos de 3 minutos |
| Gestión de dependencias | Delixon detecta y vincula dependencias compartidas automáticamente |
| Dashboard funcional | El usuario ve el estado real de todos sus proyectos en una vista |
| Gestión de runtimes | Instalar/cambiar versiones de Node, Python, etc. desde Delixon |
| Recipes | Agregar auth/database/testing a un proyecto existente sin romper nada |
| Scan de proyectos | Analizar un proyecto existente y sugerir mejoras con score |

### Fase 3 — Objetivos concretos

| Objetivo | Criterio de éxito |
|----------|-------------------|
| Onboarding rápido | Un nuevo desarrollador productivo en menos de 10 minutos |
| Configuración de equipo | Un archivo `.delixon-team` sincroniza configuración entre todos |
| Secrets seguros | Vault encriptado reemplaza "pásame el .env por Slack" |

---

## 12. Logros esperados con métricas

### Para el desarrollador individual

| Antes de Delixon | Con Delixon | Mejora |
|------------------|-------------|--------|
| 2-4 horas configurar entorno nuevo | 5-10 minutos | **95% menos tiempo** |
| Errores por mezcla de entornos: frecuentes | Minimizados por diseño | **Reducción significativa** |
| Comandos ejecutados en proyecto equivocado | Muy improbable (terminal aislada) | **Prácticamente eliminado** |
| Espacio en disco duplicado por dependencias | Dependencias compartidas | **30-60% menos espacio** |
| Retomar proyecto después de semanas | 30+ min | **3 segundos** |
| Debugging "ayer funcionaba, hoy no" | Horas | **Segundos (snapshot diff)** |

### Para un equipo de 5 desarrolladores

| Métrica | Estimación |
|---------|------------|
| Horas ahorradas por onboarding | 6-8 horas por nuevo miembro |
| Horas ahorradas por semana en configuración | 10-25 horas (2-5h por persona) |
| Reducción de bugs por entorno incorrecto | ~80% de ese tipo de bug eliminado |
| Tiempo de "pásame el .env" | Eliminado (vault compartido) |

### Valor como producto

- Herramienta sin un equivalente directo completo en el mercado
- Modelo freemium: gratis para uso individual, pago para equipos y funciones avanzadas
- Base de usuarios natural: todo desarrollador que trabaje en más de un proyecto
- Extensible: marketplace de plantillas y recipes creadas por la comunidad

---

## 13. Comparativa con herramientas existentes

| Herramienta | Env vars | Runtimes | Templates | Terminal | Dashboard | Un clic | Sin Docker |
|---|---|---|---|---|---|---|---|
| **DevContainers** | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ (config larga) | ❌ |
| **direnv** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **nvm / pyenv** | ❌ | ✅ (1 solo) | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Docker Compose** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **mise (ex rtx)** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Scripts manuales** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Delixon** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

Delixon integra en una sola experiencia capacidades que hoy estan fragmentadas en 4-5 herramientas distintas. No es que cada competidor sea malo — es que el developer tiene que ensamblar la solucion a mano.

### Competidor más cercano: mise (antes rtx)

`mise` es lo más cercano y es una herramienta excelente en lo suyo:
- Gestiona múltiples runtimes, carga env vars por directorio, ejecuta tareas, CLI puro
- Activacion automatica por directorio, plugins, ecosistema maduro

**Donde mise no llega (y Delixon si):**
- GUI con dashboard visual para explorar y configurar
- Scaffolding completo: templates, recipes, wizard de creacion
- Health checks y diagnostico del sistema (`doctor`, `health`)
- Versionado de stacks con diff y rollback
- Export/import portable para onboarding de equipo
- Deteccion y scan de proyectos existentes

**Donde mise es mas fuerte hoy:**
- Activacion automatica de runtimes por directorio (Delixon aun no lo implementa)
- Ecosistema de plugins mas maduro
- Mas tiempo en produccion con comunidad establecida

Delixon no compite con mise en gestion de runtimes. Delixon integra gestion de runtimes dentro de una plataforma mas amplia que cubre todo el ciclo de vida del workspace.

---

## 14. Funcionalidades diferenciadoras

Funcionalidades que no están en ningún competidor y resolverían problemas reales:

### A) Detección de conflictos de puertos

Al abrir un segundo proyecto, detecta si hay puertos en conflicto:
```
⚠️ Puerto 3000 en uso por proyecto-X
💡 ¿Usar puerto 3001? ¿Detener proyecto-X primero?
```

### B) Snapshots de entorno

Toma un snapshot cada vez que abres un proyecto. Si algo deja de funcionar:
```
$ delixon diff mi-proyecto
⚠️ Node global cambió: 20.10 → 20.11
⚠️ Variable DB_HOST eliminada del sistema
✅ Python 3.11 sin cambios
```

### C) Secrets vault integrado

Vault local encriptado (AES-256). Los secrets van en `${vault:nombre}`, se comparten encriptados, no por Slack.

### D) Health checks por proyecto

Dashboard muestra:
```
🟢 proyecto-a: build OK, 0 vulnerabilidades, actualizado
🟡 proyecto-b: 3 dependencias desactualizadas
🔴 proyecto-c: 2 vulnerabilidades críticas
```

### E) Contexto de Git por proyecto

```
📂 proyecto-a  main ✅ limpio
📂 proyecto-b  feat/auth ⚠️ 3 archivos sin commitear
📂 proyecto-c  main ⬇️ 5 commits detrás de origin
📂 proyecto-d  fix/bug 🔄 PR #45 abierta
```

### F) Scripts con alias unificados

`delixon run start` funciona sin importar si el stack es Node, Python o Go.

### G) Gestión de procesos en background

```
$ delixon ps mi-proyecto
PID   NAME    STATUS    PORT    UPTIME
1234  api     running   3000    2h 15m
1235  worker  running   -       2h 15m
```

### H) Project notes

Al abrir un proyecto olvidado:
```
📝 Última nota (hace 14 días):
"Estoy refactorizando el auth middleware. Falta testing."
```

---

## 15. Gobernanza y equipos/empresa

### Catálogos corporativos

```yaml
# delixon-org.yaml
organization: "MiEmpresa"

approved_technologies:
  frontend: [nextjs, react]
  backend: [fastapi, nestjs]
  database: [postgresql]

forbidden_technologies:
  - angular
  - mongodb
  - jquery

required_modules:
  - sentry
  - eslint
  - docker

defaults:
  profile: production
  testing: vitest
  ci: github-actions
```

### Templates privadas de organización

Plantillas corporativas estándar con estructura obligatoria, headers de seguridad, cobertura mínima de tests.

### Exportación de decisiones técnicas

```yaml
# decisions.yaml (generado automáticamente)
decisions:
  - date: 2026-03-15
    topic: "Base de datos"
    choice: postgresql
    alternatives: [mysql, mongodb]
    reasoning: "Política de empresa: solo SQL. PostgreSQL mejor soporte JSON."
```

Cuando alguien pregunta "¿por qué usamos X?", la respuesta está documentada.

---

## 16. Landing page y waitlist

### Estado actual de la landing (delixon-web)

La landing page está en producción con:

**Secciones implementadas:**
- [x] Hero — subtítulo rítmico + CTA
- [x] Problem — 4 tarjetas con paneles expandibles speech-bubble, efecto 3D
- [x] Solution — misma mecánica, acento cyan
- [x] HowItWorks — badges de pasos con panel expandible, efecto 3D
- [x] Waitlist — formulario conectado a API backend
- [x] Footer
- [x] Navbar con scroll suave a secciones

**Diseño y UX:**
- Paneles expandibles con `grid-template-rows` (zero-jump, sin deformación)
- Beak SVG con clip-path animado (speech bubble)
- Contenido progresivo (reveal line-by-line con `deli-reveal-up`)
- Efectos 3D (`perspective()`, `preserve-3d`, `deli-panel-emerge`)
- Clases forzadas en `utilities.css` para override de Tailwind
- i18n completo ES/EN

### Backend de waitlist (operativo)

**Stack**: Fastify + PostgreSQL (Docker) + npm workspaces

**Implementado:**
- [x] Registro con posición en cola
- [x] Sistema de referidos (link compartible, sube posiciones)
- [x] Double opt-in (email de confirmación con Resend)
- [x] Perfil opcional: nombre, stack, equipo, OS, fuente
- [x] Admin panel HTML con stats, tabla paginada, filtros, CSV export
- [x] Rate limiting (10/min), CORS
- [x] Docker Compose (PostgreSQL + API + hot-reload)
- [x] SweetAlert2 para errores cuando API no responde

**Datos capturados:**
- Fase 1: email, timestamp, IP
- Fase 2: nombre, stack, tamaño equipo, OS, cómo nos encontró

### Próximos pasos de la landing

**Siguiente iteración:**
- [ ] Botón navbar: cambiar "Descarga" → "Acceso anticipado" (scroll a waitlist)
- [ ] Eliminar link "Planes" del navbar (no existe el producto)
- [ ] Contador de registrados en tiempo real (prueba social: "247 devs en la lista")
- [ ] Sección `UseCases` — ejemplos reales (freelancer, equipo, proyecto olvidado, microservicios)
- [ ] Página `/gracias` post-registro con posición, referido, timeline
- [ ] Footer con estructura por columnas (Producto, Empresa, Comunidad, Legal)

**Marketing y conversión (futuro):**
- [ ] SEO y meta tags (Open Graph, Twitter Cards, imagen OG 1200x630)
- [ ] Analytics (PostHog o Plausible — scroll depth, clics, conversión)
- [ ] A/B testing de textos del Hero (cuando haya tráfico >500/semana)
- [ ] Crear servidor de Discord para comunidad beta
- [ ] Crear cuenta de Twitter/X para updates
- [ ] Página `/privacidad` (obligatorio antes de analytics)
- [ ] Página `/terminos` (obligatorio antes de beta)
- [ ] Sección "¿Por qué no X?" (comparativa sutil vs DevContainers, mise)

### Estrategia pre-lanzamiento — Qué NO mostrar todavía

En fase de waitlist, NO mostrar precios, descarga, ni planes. El mensaje es "apúntate", no "cómpralo".

| Elemento | Acción |
|---|---|
| Botón "Descarga" en navbar | → "Acceso anticipado" (scroll a waitlist) |
| Link "Planes" en navbar | → Eliminar |
| Sección de precios | → NO crear |
| Footer link "Precios" / "Plantillas" / "Changelog" | → NO incluir aún |

**Evolución del navbar por fase:**
- **Waitlist**: `[Acceso anticipado]` · Problema · Solución · Cómo funciona
- **Beta**: `[Descargar beta]` · Características · Plantillas · Cómo funciona · Changelog
- **Lanzamiento**: `[Descargar]` · Características · Plantillas · Precios · Changelog

### Preguntas de producto por resolver

1. **¿GUI + CLI desde Fase 1?** — Muchos devs prefieren CLI desde el día 1 (`delixon open`, `delixon list`)
2. **¿Formato de config: YAML o JSON?** — YAML es más legible (docker-compose, GitHub Actions)
3. **¿Secrets en vault o en proyecto?** — Vault por defecto con fallback a .env local
4. **¿Multi-editor?** — VSCode primero (80% mercado), extensible via plugins
5. **¿Freemium o gratis?** — Decidir antes de beta para ser transparente
6. **¿Marketplace de templates?** — Mencionar como "próximamente" para anticipación
7. **¿Cuántos beta testers?** — 100 "founders" primero, luego expandir a 500
8. **¿Qué dar a cambio del email?** — Email de bienvenida + acceso a Discord + updates mensuales

---

## 17. Opinión sincera y riesgos

### Lo que Delixon tiene a favor

1. **El problema es real y universal**. Todo dev con 2+ proyectos ha sufrido esto.
2. **No hay competidor directo completo**. DevContainers es pesado, mise es CLI puro, direnv solo env vars.
3. **Tauri es la elección correcta**. Ligero, nativo, Rust por debajo.
4. **El archivo `.delixon`** es un killer feature para equipos.
5. **La landing tiene nivel visual alto**. Genera confianza y profesionalismo.

### Lo que preocupa

1. **Gap entre landing y producto**. La landing promete cosas que el código aún no implementa. Los beta testers tienen que encontrar algo funcional.
2. **El scope es enorme**. Workspace + scaffolding + IA + templates + gobernanza. La priorización es crítica.
3. **Calidad del scaffolding**. Si el código generado no compila o tiene errores, se pierde confianza inmediatamente.
4. **La competencia no duerme**. mise gana tracción, DevContainers tiene Microsoft, Cursor integra más features.

### Errores a evitar

1. **No lanzar beta con stubs**. Mejor 5 features perfectas que 20 a medias.
2. **No competir en cantidad**. 10 tecnologías perfectas > 80 a medias.
3. **No meter IA por meterla**. Solo si resuelve problemas concretos.
4. **No prometer multi-plataforma antes de tener Windows sólido**.
5. **No ignorar DX del scaffold**. El código generado tiene que verse profesional.

---

## 18. Resumen ejecutivo

```
DELIXON — El sistema operativo local del developer

MISIÓN:
  Crear, entender, ejecutar, aislar, ampliar, validar y reparar
  proyectos localmente desde una sola app.

IDENTIDAD:
  Sistema operativo local del developer.
  GUI + CLI como interfaces duales al mismo core.
  Un solo producto con capas bien definidas.

COLUMNA VERTEBRAL:
  Project Manifest → formato declarativo que unifica toda la info
  del proyecto (techs, servicios, env vars, health, comandos, madurez).
  Todas las features leen y escriben sobre él.

CAPAS DEL PRODUCTO:
  1. Workspace → Aislamiento, env vars, runtimes, terminal, dashboard
  2. Scaffolding → Motor de stacks, catálogo, templates, recipes, scan
  3. Inteligencia → IA adaptativa, agentes, auditoría (solo si 1+2 están sólidas)

DIFERENCIAL:
  - Unica herramienta que integra todas las capas del ciclo de vida
  - Sin Docker para el dev (solo para servicios de infra)
  - App nativa de 5MB (Tauri, no Electron)
  - Archivo .delixon para onboarding de equipo en 5 min
  - Funciona offline, todo local, zero vendor lock-in
  - Sirve para proyectos NUEVOS y EXISTENTES (scan + import)
  - Doctor + Health = sabe qué falta y cómo arreglarlo

ROADMAP:
  P0 Corto (1-3m) → Manifest + catálogo + reglas + workspace completo + doctor + health
  P1 Corto→Medio → Templates reales + scaffold + recipes + scan de existentes
  P2 Medio (3-6m) → ✅ Versionado + Docker mgmt + CLI 28 cmds + Git. Pendiente: terminal integrada, gestion runtimes
  P3 Largo (6-12m) → Equipos + cross-platform + perfiles madurez + multi-editor
  P4 Futuro (12+m) → IA + agentes + marketplace + plugins (el sueño)

COMPETENCIA:
  mise = CLI puro, sin GUI, sin scaffolding, sin health checks
  DevContainers = Pesado, requiere Docker, sin dashboard, sin scan
  direnv = Solo env vars
  Delixon = Todo integrado en una app nativa moderna

PÚBLICO:
  Dev individual → Productividad personal
  Equipos → Onboarding + consistencia
  Empresas → Gobernanza tecnológica

MODELO:
  Gratis para individual
  Pro para equipos (catálogos, templates privadas, políticas, vault)
```

---

## 19. Analisis de estado y vision final

> Estado real de cada capacidad de Delixon y hacia donde va.

### 19.1 Vision — Que es Delixon

| Aspecto | Estado actual | Opinión final — Cómo debería funcionar |
|---|---|---|
| **Identidad** | ✅ GUI + CLI dual | **"Sistema operativo local del developer"** — gestiona TODO el ciclo de vida local: crear, configurar, aislar, ejecutar, evolucionar, diagnosticar, reparar |
| **Nucleo declarativo** | ✅ Implementado | Manifest `.delixon/manifest.yaml` con schema_version, metadata, editor, validacion y normalizacion obligatoria |
| **Flujo de creacion** | ✅ Wizard completo | Scaffold multi-step en GUI + `new`/`create` en CLI. Elegir tipo → stack → validar → generar → registrar |
| **Scan/import** | ✅ Implementado | `scan` detecta 15+ aspectos del stack. Export/import con formato `.delixon` portable |
| **Health + Doctor** | ✅ Implementado | Doctor del sistema + health por proyecto. GUI (HealthTab) + CLI. Sugerencias de fix |
| **Versionado de stack** | ✅ Implementado | save/list/diff/rollback de manifest. GUI (VersioningTab) + CLI snapshot commands |
| **CLI** | ✅ 28 comandos | Estrategia dual GUI+CLI. Ambas interfaces al mismo `delixon_lib`. Falta: autocompletado shell, instalacion global |
| **Templates** | ✅ 7 funcionales | Node+Express, React+Vite, FastAPI, Django, Fullstack, Rust CLI, Docker Compose. Ampliar con mas recipes |
| **Docker** | ✅ Integrado | up/down/status/logs desde GUI (DockerTab) y CLI. Deteccion de puertos |
| **Perfiles madurez** | ✅ Integrados | rapid/standard/production en scaffold. Cambian archivos reales |

### 19.2 Priorizacion por capas

| Prioridad | Que | Estado |
|---|---|---|
| **P0 — Base** | Manifest + catalogo + reglas + workspace + CLI + GUI | ✅ Completado |
| **P1 — Crear** | 7 templates + scaffold wizard + recipes (6) + scan | ✅ Completado |
| **P1 — Operar** | Health + doctor + Docker + env vars + Git + scripts + procesos | ✅ Completado |
| **P2 — Evolucionar** | Versionado + diff/rollback + perfiles + notas + export/import | ✅ Completado |
| **P2 — Expandir** | CLI 28 cmds + output formateado | ✅ Completado. Pendiente: terminal integrada, autocompletado shell |
| **P3 — Equipos** | `.delixon-team` + onboarding + secrets vault + multi-editor | Pendiente — lo que monetiza |
| **P4 — Inteligencia** (Capa 4) | IA + agentes + auditoria automatizada | Pendiente — lo que diferencia |
| **P5 — Equipos y empresa** (Capa 5) | `.delixon-team` + onboarding + secrets vault + catalogos corporativos + marketplace | Pendiente — lo que monetiza a escala |

### 19.3 Conclusion

Delixon es **el sistema operativo local del developer**: una sola app que crea, entiende, ejecuta, aisla, amplia, valida y repara proyectos.

**Lo critico:** el nucleo declarativo (Project Manifest) es la columna vertebral. Todas las capas leen y escriben sobre el. Sin el, el producto seria una coleccion de botones inconexos.

**La trampa a evitar:** competir en cantidad por la cantidad. La fuerza esta en la **calidad de la experiencia completa** — 30+ tecnologias solidas, 7 templates probados, 28 comandos CLI, 6 recipes, y un flujo GUI+CLI que funcione de extremo a extremo.

---

## 20. Glosario de tecnologías y siglas

### Siglas y acrónimos

| Sigla | Significado | Descripción breve |
|-------|-------------|-------------------|
| **API** | Application Programming Interface | Interfaz que permite a dos programas comunicarse entre sí |
| **AES-256** | Advanced Encryption Standard (256 bits) | Estándar de encriptación simétrica considerado prácticamente irrompible |
| **CI/CD** | Continuous Integration / Continuous Deployment | Automatización de pruebas, builds y despliegue de código |
| **CLI** | Command Line Interface | Herramienta que se usa desde la terminal con comandos de texto |
| **CORS** | Cross-Origin Resource Sharing | Política de seguridad del navegador que controla qué dominios pueden hacer peticiones a un servidor |
| **CRUD** | Create, Read, Update, Delete | Las 4 operaciones básicas sobre datos |
| **CSV** | Comma-Separated Values | Formato de archivo tabular separado por comas, compatible con Excel |
| **CTA** | Call To Action | Botón o enlace que invita al usuario a realizar una acción (ej: "Regístrate") |
| **DX** | Developer Experience | Calidad de la experiencia del programador al usar una herramienta |
| **GUI** | Graphical User Interface | Interfaz visual con botones, ventanas, etc. (opuesto a CLI) |
| **i18n** | Internationalization | Soporte para múltiples idiomas en una aplicación |
| **JSON** | JavaScript Object Notation | Formato ligero de intercambio de datos, legible por humanos y máquinas |
| **MVP** | Minimum Viable Product | La versión más simple de un producto que sirve para validar la idea |
| **OG** | Open Graph | Protocolo de metadatos que controla cómo se ve un enlace al compartirlo en redes sociales |
| **ORM** | Object-Relational Mapping | Capa que permite trabajar con la base de datos usando objetos en lugar de SQL directo |
| **OWASP** | Open Web Application Security Project | Organización que define las vulnerabilidades de seguridad más comunes |
| **PATH** | — | Variable del sistema operativo que lista las carpetas donde buscar ejecutables |
| **PR** | Pull Request | Solicitud para integrar cambios de código en un repositorio |
| **RBAC** | Role-Based Access Control | Control de permisos basado en roles (admin, editor, viewer) |
| **SEO** | Search Engine Optimization | Optimización para que una página aparezca en buscadores (Google) |
| **SQL** | Structured Query Language | Lenguaje para consultar y manipular bases de datos relacionales |
| **SSR** | Server-Side Rendering | Renderizar HTML en el servidor en lugar del navegador |
| **SVG** | Scalable Vector Graphics | Formato de imagen vectorial que escala sin perder calidad |
| **UX** | User Experience | Calidad de la experiencia del usuario final |
| **YAML** | YAML Ain't Markup Language | Formato de configuración legible por humanos, usado en Docker Compose, GitHub Actions, etc. |

### Tecnologías del stack de Delixon (app de escritorio)

| Tecnología | Qué es | Por qué se usa en Delixon |
|------------|--------|---------------------------|
| **Tauri** | Framework para crear apps de escritorio usando web tech + Rust | Core de Delixon: app nativa ligera (~5MB), acceso completo al SO |
| **Rust** | Lenguaje de programación de sistemas, seguro y rápido | Backend nativo de Tauri: manejo de archivos, procesos, env vars, PATH |
| **React** | Librería JavaScript para construir interfaces de usuario | Frontend de la app: dashboard, settings, formularios |
| **TypeScript** | JavaScript con tipos estáticos | Código más seguro y mantenible, autocompletado en el IDE |
| **TailwindCSS** | Framework CSS utility-first | Estilos rápidos sin escribir CSS personalizado |
| **Zustand** | Gestor de estado global para React | Estado de la app (proyecto activo, preferencias) sin boilerplate |
| **React Query** | Librería para manejo de datos asíncronos | Caché y sincronización de datos del backend Rust |
| **shadcn/ui** | Colección de componentes UI basados en Radix | Componentes accesibles y personalizables (modales, selects, tabs) |
| **Radix UI** | Primitivos de UI headless (sin estilos) | Base de shadcn/ui, garantiza accesibilidad (WAI-ARIA) |
| **Serde** | Librería de serialización/deserialización para Rust | Convertir datos entre JSON y structs de Rust |
| **Tokio** | Runtime asíncrono para Rust | Operaciones no bloqueantes (leer archivos, ejecutar procesos) |

### Tecnologías del backend de waitlist (landing page)

| Tecnología | Qué es | Por qué se usa |
|------------|--------|----------------|
| **Fastify** | Framework web para Node.js, enfocado en velocidad | API del waitlist: rápido, tipado, con plugins |
| **PostgreSQL** | Base de datos relacional open source | Almacenar registros del waitlist, perfiles, referidos |
| **Docker** | Plataforma de contenedores | Ejecutar PostgreSQL sin instalarlo en el sistema |
| **Docker Compose** | Herramienta para definir servicios multi-contenedor | Levantar PostgreSQL + API con un solo comando |
| **Resend** | Servicio de envío de emails transaccionales | Emails de confirmación (double opt-in) |
| **SweetAlert2** | Librería de alertas bonitas para el navegador | Mostrar errores y confirmaciones con buen diseño |
| **PostHog** | Plataforma de analytics open source | Tracking de eventos, scroll depth, conversión (futuro) |
| **Plausible** | Analytics ligero y privado | Alternativa a PostHog sin cookies (futuro) |

### Motores internos (Capa 2 — Scaffolding)

| Tecnología / Concepto | Qué es | Rol en el motor |
|------------------------|--------|-----------------|
| **RulesEngine** | Motor de reglas de compatibilidad | Valida que las tecnologías elegidas son compatibles entre sí |
| **ScaffoldOrchestrator** | Orquestador de generación de proyecto | Coordina la creación de archivos, configs, docker-compose |
| **TechInstaller** | Instalador por tecnología | Ejecuta lógica específica: init de ORM, rutas de auth, etc. |
| **Templates** | Plantillas predefinidas de stacks | Combinaciones probadas (T3 Stack, MERN, SaaS Starter) |
| **Recipes** | Módulos agregables a proyectos existentes | Agregar auth, testing, database a un proyecto sin romperlo |
| **Health checks** | Verificaciones de estado por servicio | Comprobar que cada servicio del proyecto funciona |
| **Perfiles de madurez** | rapid / standard / production / enterprise | Nivel de configuración: desde prototipo rápido hasta producción |

### Herramientas de referencia (competidores y alternativas)

| Herramienta | Qué es |
|-------------|--------|
| **DevContainers** | Entornos de desarrollo dentro de contenedores Docker, integrado en VSCode |
| **direnv** | Herramienta que carga/descarga env vars automáticamente al entrar a un directorio |
| **nvm** | Node Version Manager — gestiona múltiples versiones de Node.js |
| **pyenv** | Gestiona múltiples versiones de Python |
| **mise** (antes rtx) | Gestor de runtimes + env vars + tareas, todo en CLI |
| **Electron** | Framework para apps de escritorio usando Chromium + Node.js (pesado, ~200MB) |
| **Cursor** | Editor de código basado en VSCode con IA integrada |
| **Vitest** | Framework de testing rápido para proyectos con Vite |
| **Sentry** | Plataforma de monitoreo de errores en producción |
| **ESLint** | Linter de JavaScript/TypeScript — detecta errores y aplica estilo |
| **Prettier** | Formateador automático de código |
| **GitHub Actions** | CI/CD integrado en GitHub para automatizar builds, tests, deploys |
| **Prisma** | ORM moderno para Node.js/TypeScript con migraciones y tipo seguro |
| **Drizzle** | ORM ligero para TypeScript, más cercano a SQL puro |
| **Redis** | Base de datos en memoria, usada para caché y colas |
| **SQLite** | Base de datos embebida en un solo archivo, sin servidor |
| **Symlinks** | Enlaces simbólicos del SO — Delixon los usa para compartir dependencias sin duplicar |

---

## 21. Dependencias y versiones

> **Política de versiones:** Todas las dependencias usan versiones exactas (sin `^` ni `~`) para evitar actualizaciones involuntarias que rompan el proyecto. Se actualiza manualmente tras verificar compatibilidad.

> **Última actualización:** 2026-03-25

### Frontend (npm — package.json)

| Paquete | Versión | Tipo | Notas |
|---|---|---|---|
| **react** | 19.2.4 | dependencies | React 19 (migrado desde 18) |
| **react-dom** | 19.2.4 | dependencies | Sigue la versión de React |
| **react-router-dom** | 7.13.2 | dependencies | Router v7 (migrado desde v6) |
| **@tanstack/react-query** | 5.95.2 | dependencies | Manejo de datos asíncronos |
| **@tauri-apps/api** | 2.10.1 | dependencies | API core de Tauri v2 |
| **@tauri-apps/plugin-fs** | 2.4.5 | dependencies | Plugin de filesystem |
| **@tauri-apps/plugin-process** | 2.3.1 | dependencies | Plugin de procesos |
| **@tauri-apps/plugin-shell** | 2.3.5 | dependencies | Plugin de shell |
| **clsx** | 2.1.1 | dependencies | Utilidad para clases CSS |
| **tailwind-merge** | 3.5.0 | dependencies | Merge inteligente de clases Tailwind (migrado desde v2) |
| **zustand** | 5.0.12 | dependencies | Estado global (migrado desde v4, API `create<T>()()`) |
| **@eslint/js** | 9.39.4 | devDependencies | Config base de ESLint |
| **@tailwindcss/vite** | 4.2.2 | devDependencies | Plugin Vite para Tailwind 4 (reemplaza postcss plugin) |
| **@tauri-apps/cli** | 2.10.1 | devDependencies | CLI de Tauri |
| **@types/node** | 22.16.4 | devDependencies | Tipos de Node.js |
| **@types/react** | 19.2.14 | devDependencies | Tipos de React 19 |
| **@types/react-dom** | 19.2.3 | devDependencies | Tipos de React DOM 19 |
| **@typescript-eslint/eslint-plugin** | 8.57.2 | devDependencies | Reglas ESLint para TS |
| **@typescript-eslint/parser** | 8.57.2 | devDependencies | Parser ESLint para TS |
| **@vitejs/plugin-react** | 4.7.0 | devDependencies | Plugin React para Vite |
| **eslint** | 9.39.4 | devDependencies | Linter |
| **eslint-plugin-react-hooks** | 5.2.0 | devDependencies | Reglas de hooks |
| **prettier** | 3.8.1 | devDependencies | Formateador de código |
| **tailwindcss** | 4.2.2 | devDependencies | Tailwind CSS v4 (migrado desde v3, config en CSS) |
| **typescript** | 5.9.3 | devDependencies | TypeScript |
| **vite** | 6.4.1 | devDependencies | Bundler |
| **vitest** | 3.2.4 | devDependencies | Testing |

**Eliminados en la migración a Tailwind 4:**
- `autoprefixer` — integrado en Tailwind 4
- `postcss` — reemplazado por `@tailwindcss/vite` plugin
- `tailwind.config.ts` — configuración movida a `@theme {}` en `index.css`
- `postcss.config.js` — no necesario con el plugin de Vite

### Backend (Cargo — Cargo.toml)

| Crate | Versión | Notas |
|---|---|---|
| **tauri** | 2.10.3 | Core de Tauri v2 |
| **tauri-build** | 2.5.6 | Build-time |
| **tauri-plugin-shell** | 2.3.5 | Plugin de shell |
| **tauri-plugin-fs** | 2.4.5 | Plugin de filesystem |
| **tauri-plugin-process** | 2.3.1 | Plugin de procesos |
| **serde** | 1.0.228 | Serialización/deserialización |
| **serde_json** | 1.0.149 | JSON |
| **tokio** | 1.50.0 | Runtime asíncrono |
| **which** | 8.0.2 | Detección de binarios (migrado desde v6) |
| **dirs** | 6.0.0 | Rutas del sistema (migrado desde v5) |
| **thiserror** | 2.0.18 | Manejo de errores (migrado desde v1) |
| **uuid** | 1.22.0 | Generación de UUIDs |
| **chrono** | 0.4.44 | Fechas y tiempos |

### Migraciones mayores completadas (2026-03-25)

| Paquete | Anterior | Actual | Cambios realizados |
|---|---|---|---|
| React | 18.3.1 | 19.2.4 | Imports simplificados, tipos actualizados |
| react-router-dom | 6.30.3 | 7.13.2 | Eliminados future flags (ahora son default) |
| TailwindCSS | 3.4.19 | 4.2.2 | Config movida a `@theme {}` en CSS, plugin Vite |
| Zustand | 4.5.7 | 5.0.12 | API `create<T>()()` con doble invocación |
| tailwind-merge | 2.6.1 | 3.5.0 | API compatible, internos actualizados |
| which (Rust) | 6.0.3 | 8.0.2 | API compatible |
| dirs (Rust) | 5.0.1 | 6.0.0 | API compatible |
| thiserror (Rust) | 1.0.69 | 2.0.18 | API compatible |

---

## 22. Checklist general de implementación

### Landing page (delixon-web) — Completado
- [x] Estructura de carpetas por sección (`layout/`, `hero/`, `problem/`, `solution/`, `how-it-works/`, `waitlist/`)
- [x] Componentes compartidos: `SectionTag`, `SectionTitle`
- [x] Subcomponentes: `ProblemExpanded`, `SolutionExpanded`, `HowItWorksExpanded`, `WaitlistForm`
- [x] i18n completo ES/EN con textos mejorados y contenido expandido
- [x] Paneles expandibles speech-bubble con `grid-template-rows` (zero-jump)
- [x] Beak SVG con `clip-path` animado, reveal progresivo (`deli-reveal-up`), efectos 3D
- [x] Clases forzadas en `utilities.css` para override de Tailwind
- [x] Navbar con scroll suave a secciones
- [x] Build de producción verificado sin errores

### Waitlist backend — Completado
- [x] API Fastify: signup, referral, double opt-in, profile update
- [x] PostgreSQL en Docker (contenedor `delixon-db`, puerto 5480)
- [x] Sistema de referidos (link compartible, boost de posición)
- [x] Perfil opcional (nombre, stack, equipo, OS, fuente)
- [x] Admin panel HTML: stats, breakdowns, tabla paginada, filtro, CSV export
- [x] Rate limiting, CORS, auth admin con secret
- [x] npm workspaces (frontend + server comparten `node_modules`)
- [x] Hot-reload con tsx watch, logs legibles
- [x] Docker Compose (PostgreSQL + API)
- [x] SweetAlert2 para errores de API
- [x] Documentación: guía de backend + Docker + PostgreSQL

### Landing — Siguiente iteración
- [ ] Botón navbar: "Descarga" → "Acceso anticipado" (scroll a waitlist)
- [ ] Eliminar link "Planes" del navbar
- [ ] Contador de registrados (prueba social)
- [ ] Sección `UseCases` con ejemplos reales
- [ ] Página `/gracias` post-registro
- [ ] Footer con estructura por columnas y evolución por fase

### Landing — Futuro
- [ ] SEO y meta tags (Open Graph, Twitter Cards)
- [ ] Analytics (PostHog o Plausible)
- [ ] A/B testing de textos del Hero
- [ ] Páginas legales (`/privacidad`, `/terminos`)
- [ ] Discord y Twitter/X para comunidad

### Producto — Capa 0: Nucleo declarativo ✅
- [x] Project Manifest (`.delixon/manifest.yaml`) — techs, servicios, env vars, comandos, puertos, recipes, health checks
- [x] Todas las capas leen y escriben sobre el manifest
- [x] `schema_version` en el manifest — permite migraciones futuras sin romper manifests existentes
- [x] `metadata` (description, created_at, author) — metadatos basicos para entender el proyecto
- [x] `editor` opcional por proyecto — permite editor distinto al global
- [x] `validate_manifest()` — rechaza manifests con name vacio, puertos invalidos, duplicados, env vars con valores
- [x] `normalize_manifest()` — limpia y deduplica antes de guardar (puertos, techs, recipes, whitespace)
- [x] `save_manifest()` valida y normaliza SIEMPRE antes de escribir — nunca se guarda basura
- [x] Manifests antiguos (sin schema_version) se normalizan automaticamente al cargar
- [x] Export/import incluye manifest completo — el archivo `.delixon` ahora transporta el manifest
- [x] Separacion clara: manifest = schema (que necesita el proyecto), envs/*.json = valores reales (secretos fuera del manifest)

> **Principio de diseno:** El manifest es el contrato central que define que es un proyecto para Delixon. Todas las capas leen y escriben sobre el. Las notas (`notes/*.json`) y los valores de env vars (`envs/*.json`) se mantienen separados por diseno: las notas son efimeras y de alta frecuencia, los valores de env vars son sensibles y varian por maquina.

### Producto — Capa 1: Workspace ✅
- [x] Persistencia de proyectos (CRUD completo)
- [x] Aislamiento de env vars funcional (JSON por proyecto)
- [x] Apertura de proyecto en editor configurado (VS Code, Cursor, Zed, Neovim, Sublime)
- [x] Deteccion de runtimes (Node, Python, Rust, Go, .NET, PHP, Ruby)
- [x] Apertura de terminal con env vars cargadas
- [x] Dashboard con busqueda, filtros, grid de proyectos
- [x] Pagina de detalle con gestion de env vars
- [x] Archivo `.delixon` export/import (formato portable)
- [x] Deteccion de conflictos de puertos (TCP check)
- [x] Settings: editor, tema, idioma, runtimes
- [x] Generacion de `.code-workspace` con extensiones recomendadas
- [ ] Activacion automatica de runtimes al abrir proyecto — nvm/fnm (Node), pyenv (Python), rustup (Rust)
- [ ] Historial de terminal aislado por proyecto

### Producto — Capa 2: Scaffolding ✅
- [x] Catalogo de 30+ tecnologias en YAML con UI de browse/search/filtros
- [x] RulesEngine: dependencias auto, incompatibilidades, puertos, sugerencias
- [x] Scaffold wizard multi-step en GUI (info → stack → preview → generar)
- [x] `delixon-cli new` con parametros de tipo/perfil/techs
- [x] 7 templates funcionales (Node+Express, React+Vite, FastAPI, Django, Fullstack, Rust CLI, Docker Compose)
- [x] 6 recipes con preview (vitest, pytest, docker, ci-github, biome, prisma)
- [x] Full-stack detection (frontend/ + backend/) con readiness score
- [x] Perfiles de madurez (rapid/standard/production)
- [x] Scan de proyectos existentes — detecta 15+ aspectos del stack
- [x] Validacion de combinaciones de tecnologias (`delixon-cli validate`)
- [ ] Recipe: Auth — NextAuth/Clerk/JWT (lo primero que todo proyecto real necesita)
- [ ] Recipe: Database + ORM — PostgreSQL + Prisma (Node) / SQLAlchemy (Python) como recipe aplicable
- [ ] Recipe: Monitoring basico — health endpoint + logging estructurado
- [ ] Mas recipes: Pagos (Stripe), Email (Resend/Nodemailer), Admin panel, Observabilidad
- [ ] Generacion interactiva por tipo de producto ("¿Que vas a construir?")
- [ ] Tests automatizados: cada template se genera y arranca sin errores

### Producto — Capa 3: Operacion diaria ✅
- [x] 28 comandos CLI (clap) — ver `docs/cli/CLI_REFERENCE.md`
- [x] Docker Compose management — up/down/status/logs (GUI DockerTab + CLI)
- [x] Git integration — rama, cambios, remoto, commits (GUI GitTab + CLI)
- [x] Scripts unificados — ejecutar desde manifest (GUI ScriptsTab + CLI)
- [x] Health checks por proyecto con sugerencias de fix (GUI HealthTab + CLI)
- [x] Doctor del sistema — runtimes, Docker, Git, config, datos (GUI + CLI)
- [x] Versionado de stacks — save/list/diff/rollback (GUI VersioningTab + CLI)
- [x] Snapshots de entorno — comparar runtimes y deps entre momentos
- [x] Notas por proyecto — CRUD con UUID y timestamps (GUI NotesTab + CLI)
- [x] Gestion de puertos — ver puertos en uso, conflictos (GUI + CLI)
- [x] Gestion de procesos — listar/kill por puerto (GUI ProcessesTab + CLI)
- [x] Output formateado con colores (colored crate)
- [ ] Terminal integrada dentro de Delixon (panel embebido)
- [ ] Gestion de runtimes: instalar/cambiar versiones desde la app
- [ ] Notificaciones de dependencias desactualizadas o vulnerables
- [ ] Autocompletado para bash/zsh/fish/PowerShell

### Producto — Capa 4: Inteligencia (futuro)
- [ ] Asistente IA con aprendizaje adaptativo
- [ ] Agentes especializados (SecurityGuard, CodeReviewer, TestBuilder, PerfAnalyzer, DocWriter)
- [ ] Pipeline de auditoria completa (`delixon audit`)
- [ ] Modo "arquitecto asistente" — sugiere stack segun tipo de producto
- [ ] Sugerencias contextuales basadas en patrones de uso

### Producto — Capa 5: Equipos y empresa (futuro)
- [ ] Configuracion de equipo (`.delixon-team`)
- [ ] Onboarding automatizado — nuevo dev productivo en 5 min
- [ ] Secrets vault encriptado (AES-256)
- [ ] Catalogos corporativos y templates privadas de organizacion
- [ ] Politicas de stack por equipo/empresa

### Cross-platform (transversal a todas las capas)
- [x] Tauri compila cross-platform (Windows, Linux, macOS)
- [x] Logica platform-aware en Rust (`cfg(target_os)`, lsof vs tasklist, rutas)
- [ ] CI/CD con GitHub Actions: build y test en Windows, Ubuntu y macOS en cada PR
- [ ] Documentar diferencias por SO: rutas de datos, terminales, binarios
- [ ] Probar CLI en los tres SO
- [ ] Instalacion global del CLI (`delixon` desde cualquier terminal en cualquier SO)

### Producto — Capa 6: Servidor y cloud (vision futura)
- [ ] CLI headless para servidores de desarrollo (sin GUI, sin Tauri)
- [ ] API remota para gestionar entornos desde CI/CD pipelines
- [ ] Gestion de multiples proyectos en produccion (modo servidor)
- [ ] Integracion con herramientas de monitoreo (Grafana, Prometheus)
- [ ] Delixon como servicio en entornos de staging/pre-produccion

### Distribucion y ecosistema (futuro)
- [ ] Sistema de plugins
- [ ] Marketplace de templates y recipes
- [ ] Soporte multi-editor completo (Cursor, WebStorm, Neovim, Zed, Sublime)
- [ ] Editor visual de plantillas y catalogos
- [ ] Exportacion de decisiones tecnicas (ADR)

### Errores a evitar (checklist de disciplina)

> Reglas que aplican en cada sprint. Si se incumple alguna, se para y se corrige antes de seguir.

- [ ] **No agregar features sin pulir las existentes** — antes de Capa 4 (IA), las capas 0-3 deben estar blindadas al 100%
- [ ] **Paridad GUI + CLI en cada feature nueva** — si solo funciona en una interfaz, no se considera completada
- [ ] **Windows nunca es ciudadano de segunda** — cada path usa `Path::new()`, cada `lsof` tiene su `tasklist`, tests en los 3 SO
- [ ] **No competir con Docker** — Docker solo para servicios (PostgreSQL, Redis); runtimes siempre nativos. No diluir el diferencial
- [ ] **CI/CD cross-platform antes de lanzar** — build y test automatico en Windows + Linux + macOS en cada PR
- [ ] **Patron preview/diff/confirm en todo comando destructivo** — si escribe en disco, primero muestra que va a hacer. Sin excepciones
- [ ] **Errores con los 4 campos** — que intento, que detecto, por que fallo, que hacer. Un error sin "que hacer" no se shipea
- [ ] **`open` debe ser <2 segundos** — si tarda mas, se investiga y se arregla antes de cualquier otra feature

### Disciplinas futuras (detectadas, pendientes de implementar)

> Estos puntos no requieren accion hoy pero estan documentados para cuando llegue el momento. Si no se anotan ahora, se olvidan.

**⚠️ MANIFEST — No inflar por entusiasmo:**

El manifest es la columna vertebral, pero eso no significa que todo campo nuevo tenga derecho a entrar. Regla para evaluar cualquier campo nuevo:

> *"¿Lo leen al menos 2 modulos distintos O es critico para reconstruir/entender el proyecto?"*

Si la respuesta es no, el campo no entra en el manifest. Va a otro sitio (config local, notes, settings) o no existe. El riesgo real es que cada feature nueva quiera "su campo" y en 6 meses haya 40 campos donde 15 son ruido. La disciplina es decir "no" mas que "si".

**⚠️ ON_OPEN — Zona de cuidado cuando se implemente:**

El concepto de `on_open` (ejecutar comandos al abrir un proyecto) es util pero peligroso si crece mal. Puede convertir `open` de "abrir y preparar contexto" a "ejecutar cosas con efectos laterales impredecibles". Cuando llegue el momento:

- Por defecto **desactivado** (opt-in explicito en manifest)
- Solo comandos que el usuario escribio a mano — nunca generados automaticamente
- Mostrar que va a ejecutar antes de hacerlo (patron preview/confirm)
- Separar claramente: "preparar contexto" (runtime, env vars = siempre) vs "ejecutar scripts" (on_open = solo si el usuario lo pidio)
- Si un comando de on_open tarda >3 segundos, debe poder cancelarse sin afectar el `open`
- Nunca romper la promesa de `open` en <2 segundos por culpa de on_open

### Mantenimiento — Completado
- [x] Migración completa a React 19 (2026-03-25)
- [x] Migración completa a React Router 7 (2026-03-25)
- [x] Migración completa a Tailwind CSS 4 con `@tailwindcss/vite` plugin (2026-03-25)
- [x] Migración completa a Zustand 5 con API `create<T>()()` (2026-03-25)
- [x] Migración de dependencias Rust: which 8, dirs 6, thiserror 2 (2026-03-25)
- [x] Eliminados `tailwind.config.ts`, `postcss.config.js`, `autoprefixer` (obsoletos en TW4)
- [x] Versiones exactas fijadas (sin `^`) para evitar actualizaciones involuntarias
- [x] 0 vulnerabilidades en `npm audit`
- [x] Mock system para desarrollo en navegador (safeInvoke + datos mock)

---

## 23. Vision del producto — Que no deberia faltar

> Escrito desde la perspectiva del equipo de desarrollo de Delixon, pensando como un product manager senior que ademas es developer y usuario final del producto.

### Lo que ya tenemos y esta bien

La arquitectura de las capas 0-3 esta completada: manifest, workspace, scaffolding, operacion diaria, CLI de 28 comandos, GUI con 9 tabs, y todo compartiendo el mismo core Rust. Eso es mas de lo que la mayoria de herramientas similares ofrecen. La base esta construida correctamente, pero la experiencia aun necesita blindarse: falta activacion de runtimes, CI multi-SO, instalacion global del CLI, y tests automatizados de templates.

### Lo que NO puede faltar antes de lanzar

Estas son las cosas que si un developer descarga Delixon y no encuentra, cierra la app y no vuelve:

**1. Cross-platform real (no solo "compila")**

No basta con que Tauri compile en los tres SO. Necesitamos:
- CI/CD que haga build + test en Windows, Ubuntu y macOS en cada PR
- Probar CADA comando CLI en los tres SO — las diferencias de rutas, shells y procesos son las que rompen
- Documentar las diferencias reales: donde se guardan los datos, que terminal se abre, como se detectan runtimes
- El instalador debe funcionar limpio en los tres SO

Si un usuario de Linux o macOS descarga Delixon y algo no funciona, no va a reportar un bug. Va a desinstalarlo.

**2. Instalacion global del CLI**

`cargo run --manifest-path src-tauri/Cargo.toml --bin delixon-cli -- doctor` no es aceptable para un usuario final. Necesitamos:
- `delixon doctor` desde cualquier terminal, en cualquier SO
- Instalador que agregue al PATH automaticamente
- Considerar: brew (macOS), scoop/winget (Windows), apt/snap (Linux)
- Que la GUI instale el CLI automaticamente al instalarse

**3. Activacion de runtimes**

Detectar que un proyecto usa Node 20 y no activarlo es como un GPS que te dice donde estas pero no te da direcciones. Necesitamos:
- Al hacer `delixon open`, que se active la version correcta de Node/Python/Rust
- Integracion con nvm/fnm (Node), pyenv (Python), rustup (Rust)
- Si la version requerida no esta instalada, sugerir instalarla

**4. Al menos 2-3 recipes mas**

6 recipes es un buen inicio pero el developer que busca "agregar auth a mi proyecto" y no lo encuentra va a sentir que la feature esta incompleta. Prioridad:
- Auth (NextAuth/Clerk/JWT) — lo primero que todo proyecto real necesita
- Base de datos con ORM — PostgreSQL + Prisma/SQLAlchemy como recipe
- Monitoring basico — health endpoint + logging estructurado

### Lo que deberia tener a mediano plazo

**5. Terminal integrada**

No para reemplazar la terminal del developer, sino para que desde Delixon pueda ejecutar comandos rapidos sin salir de la app. Es la diferencia entre "gestor de proyectos" y "entorno de desarrollo". Un panel con la terminal del proyecto actual, env vars ya cargadas, runtime activado.

**6. Notificaciones de dependencias**

El developer deja un proyecto 2 meses. Vuelve. Las deps estan desactualizadas. Algunas tienen CVEs. Delixon deberia decirle eso en el dashboard antes de que lo descubra en produccion. Un badge amarillo "3 deps desactualizadas" en la tarjeta del proyecto.

**7. Generacion por tipo de producto**

"¿Que vas a construir?" → SaaS / API / Landing / CLI / Desktop → stack recomendado. Es la feature que convierte a Delixon de "herramienta para devs que saben lo que quieren" a "herramienta que te guia si no sabes por donde empezar". Amplía el publico objetivo.

### La vision de servidor (Capa 6) — Por que importa

Delixon como app local es el producto de hoy. Pero el producto de manana es Delixon en servidores:

**Caso real:** Una startup con 15 developers tiene un servidor de desarrollo compartido. Cada dev tiene 3-4 proyectos. Hoy cada uno configura su entorno a mano, los puertos se pisan, las variables de entorno se contaminan, y cuando alguien nuevo entra tarda 2 dias en tener todo funcionando.

**Con Delixon servidor:**
- El admin instala `delixon-server` (CLI headless, sin GUI)
- Cada dev usa `delixon-cli` para gestionar sus entornos remotos
- El servidor mantiene aislamiento por proyecto automaticamente
- Health checks corren en background y notifican si algo se rompe
- Nuevo dev: `delixon setup --team acme` → entorno listo en 5 minutos
- Funciona en Linux (principal), Windows Server y macOS (CI runners)

Esto es lo que convierte a Delixon de herramienta individual gratuita a producto empresarial que factura. El modelo: gratis para individuos, de pago para equipos y servidores.

### Medir la primera semana obsesivamente

La primera semana de un usuario nuevo determina si Delixon se convierte en habito o se desinstala. Metricas locales (sin telemetria externa, sin enviar datos) que Delixon deberia trackear internamente:

| Metrica | Que mide | Señal de exito |
|---|---|---|
| Tiempo hasta primer `open` | ¿Cuanto tarda en abrir un proyecto? | < 5 min desde instalacion |
| Proyectos registrados en 7 dias | ¿Adopta Delixon para su flujo real? | ≥ 3 proyectos |
| Uso de `doctor`/`health` | ¿Descubre el diagnostico? | ≥ 2 usos en 7 dias |
| Retorno al dia 2, 3, 7 | ¿Vuelve a abrirlo? | Dia 2: si. Dia 7: si |
| Recipes aplicadas | ¿Evoluciona proyectos con Delixon? | ≥ 1 recipe en 7 dias |
| Errores encontrados sin solucion | ¿Se queda atascado? | 0 idealmente |

Estas metricas son locales — un archivo JSON en `~/.local/share/delixon/metrics.json` que solo el usuario puede ver. No se envian a ningun servidor. Sirven para que el equipo de desarrollo sepa QUE medir cuando haga pruebas con beta testers, no para tracking de usuarios.

> Si no medimos la primera semana, estamos adivinando. Y adivinar es caro.

### Lo que NO deberiamos hacer (errores a evitar)

1. **No agregar features sin pulir las existentes.** 28 comandos CLI que funcionan al 95% es peor que 15 que funcionan al 100%. Antes de agregar Capa 4 (IA), las capas 0-3 deben estar blindadas.

2. **No priorizar GUI sobre CLI ni viceversa.** Ambas interfaces son ciudadanos de primera clase. Cada feature nueva debe funcionar en ambas desde el dia 1. Si solo funciona en GUI, los power users no la van a usar. Si solo funciona en CLI, los usuarios visuales no la van a encontrar.

3. **No ignorar Windows.** Muchos productos dev-tools tratan Windows como ciudadano de segunda. Mas del 40% de developers usan Windows. Cada path hardcodeado con `/` en vez de usar `Path::new()`, cada `lsof` sin su equivalente `tasklist`, es un usuario perdido.

4. **No competir con Docker.** Delixon no es Docker. Docker es para servidores y servicios. Delixon es para el entorno del developer. La filosofia "Docker solo para servicios (PostgreSQL, Redis), runtimes nativos" es el diferencial. No diluirlo.

5. **No lanzar sin CI/CD cross-platform.** Si no estamos probando automaticamente en Windows + Linux + macOS en cada PR, vamos a romper cosas sin darnos cuenta. Es la primera infraestructura que hay que montar.

### Prioridades inmediatas (proximo sprint)

| # | Que | Por que | Impacto |
|---|---|---|---|
| 1 | `open` perfecto (<2s, runtime activado) | Es la feature sagrada — crea el habito diario | Critico |
| 2 | Instalacion global CLI | Usabilidad basica — no podemos pedir `cargo run --manifest-path...` | Critico |
| 3 | CI/CD multi-SO | Sin esto vamos a ciegas en 2 de 3 plataformas | Critico |
| 4 | 3 recipes criticas (auth, db+orm, monitoring) | Sin estas, la propuesta de recipes se siente incompleta | Alto |
| 5 | Activacion de runtimes en `open` | Lo que convierte `open` de "abre editor" a "activa entorno completo" | Alto |
| 6 | Tests automatizados templates | Cada template debe generarse y arrancar sin errores en CI | Medio |
| 7 | Metricas locales de primera semana | Sin medir, adivinamos — y adivinar es caro | Medio |

---

*Delixon — Deja de configurar. Empieza a construir.*
