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
7. [Las tres capas de Delixon](#7-las-tres-capas-de-delixon)
8. [Estructura del proyecto](#8-estructura-del-proyecto)
9. [Hoja de ruta y fases](#9-hoja-de-ruta-y-fases)
10. [Objetivos por fase](#10-objetivos-por-fase)
11. [Logros esperados con métricas](#11-logros-esperados-con-métricas)
12. [Comparativa con herramientas existentes](#12-comparativa-con-herramientas-existentes)
13. [Funcionalidades diferenciadoras](#13-funcionalidades-diferenciadoras)
14. [Gobernanza y equipos/empresa](#14-gobernanza-y-equiposempresa)
15. [Landing page y waitlist](#15-landing-page-y-waitlist)
16. [Opinión sincera y riesgos](#16-opinión-sincera-y-riesgos)
17. [Resumen ejecutivo](#17-resumen-ejecutivo)

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

Delixon es una **aplicación de escritorio local** que actúa como capa de organización e inteligencia entre el desarrollador y sus proyectos.

**No reemplaza ninguna herramienta.** El desarrollador sigue usando VSCode, su terminal preferida, Git, Docker, npm, pip — todo lo que ya conoce y domina. Delixon se encarga de que cada proyecto viva en su propio mundo perfectamente configurado, listo para trabajar desde el primer segundo.

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

## 7. Las tres capas de Delixon

Delixon no es solo un gestor de workspaces. Su verdadera visión es convertirse en una **plataforma integral para el ciclo de vida completo del desarrollo**.

```
┌─────────────────────────────────────────────────────────────┐
│                    CAPA 3: INTELIGENCIA                      │
│  Asistente IA · Auditoría automática · Agentes especiali-   │
│  zados · Aprendizaje adaptativo · Sugerencias contextuales  │
├─────────────────────────────────────────────────────────────┤
│                    CAPA 2: SCAFFOLDING                        │
│  Motor de stacks · Catálogo tecnológico · Templates ·        │
│  Validación de compatibilidades · Generación de proyectos    │
├─────────────────────────────────────────────────────────────┤
│                    CAPA 1: WORKSPACE                          │
│  Aislamiento · Env vars · Runtimes · Terminal · Dashboard    │
│  · Apertura instantánea · Historial por proyecto             │
└─────────────────────────────────────────────────────────────┘
```

**Capa 1 (Workspace)** — El core. Aislamiento, env vars, runtimes, terminal, dashboard.
**Capa 2 (Scaffolding)** — Motor de generación de proyectos integrado. No solo "abrir un proyecto existente", sino crearlo desde cero.
**Capa 3 (Inteligencia)** — Asistente que aprende, audita, sugiere y automatiza.

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

### 7.2 Capa 3: Inteligencia

#### Asistente IA integrado

1. **Aprende de cada interacción**: patrones de uso, comandos frecuentes, preferencias de stack
2. **Se adapta al desarrollador**: si siempre eliges TypeScript + Tailwind, lo sugiere primero
3. **Memoria persistente**: recuerda decisiones pasadas, errores resueltos, preferencias
4. **Clasificación automática**: interpreta lo que el dev pide y ejecuta la acción correcta

```
Developer: "necesito una API rápida para un MVP"

Delixon AI:
  Basado en tus preferencias anteriores:
  → FastAPI + PostgreSQL + Docker
  → Perfil: rapid (MVP)
  → Estimado: 2 minutos de scaffolding

  ¿Generar?
```

#### Agentes especializados por dominio

| Agente | Dominio | Qué hace |
|---|---|---|
| **SecurityGuard** | Seguridad | Auditoría OWASP, detección de secrets expuestos, CVE scanning |
| **CodeReviewer** | Calidad | Code smells, principios SOLID/DRY/KISS, complejidad ciclomática |
| **TestBuilder** | Testing | Genera tests unitarios/integración/e2e, analiza cobertura |
| **PerfAnalyzer** | Rendimiento | Core Web Vitals, bundle size, queries lentas, N+1 detection |
| **DocWriter** | Documentación | README, API docs, changelogs, guías de contribución |
| **InfraOps** | DevOps | Docker optimization, CI/CD pipelines, deployment configs |
| **DataOptimizer** | Base de datos | Schema review, query optimization, índices faltantes |
| **APIDesigner** | Diseño de API | REST/GraphQL best practices, versionado, OpenAPI spec |

#### Pipeline de auditoría completa

```
$ delixon audit mi-proyecto

  🔒 SecurityGuard:
     ⚠️ CORS permite * (restringir en producción)

  📝 CodeReviewer:
     ⚠️ 3 funciones con complejidad ciclomática > 10

  🧪 TestBuilder:
     ❌ Cobertura: 12% (mínimo recomendado: 60%)
     📋 Tests generados automáticamente: 15 archivos

  📊 PerfAnalyzer:
     ⚠️ 2 queries N+1 detectadas en routes/users.ts

  Score general: 6.2/10
```

### 7.3 Integración de las tres capas

```
1. CREAR (Capa 2)  → $ delixon create --type saas-b2b
2. REGISTRAR (Capa 1) → Auto-registrado con aislamiento
3. TRABAJAR (Capa 1)  → $ delixon open → editor + terminal + entorno en 2s
4. EVOLUCIONAR (Capa 2) → $ delixon add auth / add payments
5. AUDITAR (Capa 3)  → $ delixon audit → seguridad, calidad, tests
6. APRENDER (Capa 3)  → Sugiere mejoras, anticipa problemas
7. COMPARTIR (Capa 1+2) → $ delixon export → archivo .delixon completo
```

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
│   │   │   ├── template.rs
│   │   │   └── config.rs
│   │   └── utils/                # Utilidades del sistema
│   │       ├── fs.rs             # Operaciones de archivo
│   │       ├── process.rs        # Gestión de procesos
│   │       └── platform.rs       # Detección de SO
│   └── tauri.conf.json
│
├── src/                          # Frontend React
│   ├── components/
│   │   ├── ui/                   # Componentes base (shadcn)
│   │   ├── project-card/
│   │   ├── project-editor/
│   │   ├── template-gallery/
│   │   ├── dependency-viewer/
│   │   └── terminal-panel/
│   ├── stores/
│   ├── hooks/
│   └── lib/
│
├── templates/                    # Plantillas de proyectos
│   ├── node-express/
│   ├── react-vite/
│   ├── python-fastapi/
│   ├── python-django/
│   ├── fullstack-react-python/
│   ├── rust-cli/
│   └── docker-compose/
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

## 9. Hoja de ruta y fases

### Fase 1 — MVP Windows (3-4 meses)
> Objetivo: tener algo funcional y usable en el día a día
> Prioridad absoluta: implementar el core de Capa 1 (Workspace)

- [ ] App de escritorio básica con Tauri + React
- [ ] Registro y gestión de proyectos (crear, abrir, eliminar) — persistencia real
- [ ] Aislamiento de variables de entorno por proyecto — la promesa #1
- [ ] Historial de terminal aislado por proyecto
- [ ] Integración con VSCode (abrir proyecto con workspace correcto)
- [ ] 3 plantillas funcionales que funcionen PERFECTO: Node, React, Python
- [ ] Detección + activación de runtimes instalados al abrir proyecto
- [ ] Exportar/importar configuración de proyecto (archivo `.delixon`)
- [ ] UI mínima del dashboard: lista de proyectos, estado, botón "Abrir"
- [ ] Detección de conflictos de puertos

**Entregable**: Un MVP donde puedas registrar proyectos, abrirlos con un clic, con env vars aisladas, y exportar la config.

### Fase 2 — Madurez + Scaffolding (3-6 meses)
> Objetivo: crear proyectos desde Delixon, no solo registrarlos

- [ ] Motor de scaffolding con catálogo de 30+ tecnologías
- [ ] 10 templates completos y probados
- [ ] Validación inteligente de compatibilidades entre tecnologías
- [ ] Gestión de runtimes integrado (instalar/cambiar versiones de Node, Python, Go, Rust)
- [ ] Recipes: agregar módulos a proyectos existentes (`delixon add auth`, `delixon add database`)
- [ ] Dashboard con estado de salud de proyectos (health checks)
- [ ] Contexto de Git integrado (rama, cambios pendientes, PRs abiertas)
- [ ] Análisis de proyecto existente (`delixon scan`)
- [ ] Editor visual de plantillas personalizado
- [ ] Terminal integrada dentro de Delixon
- [ ] Notificaciones de dependencias desactualizadas o con vulnerabilidades
- [ ] Docker Compose para servicios de infraestructura (BD, Redis, colas)
- [ ] Snapshots de entorno (debugging de "ayer funcionaba, hoy no")
- [ ] Scripts de proyecto con alias (`delixon run start` funciona sin importar el stack)
- [ ] Gestión de procesos en background por proyecto (`delixon ps`, `delixon logs`)

### Fase 3 — Equipos (2-3 meses)
> Objetivo: que varios desarrolladores trabajen con la misma configuración

- [ ] Exportación de configuración de equipo (`.delixon-team`)
- [ ] Onboarding automatizado para nuevos miembros
- [ ] Control de versiones de plantillas y configuraciones
- [ ] Integración con repositorios de plantillas compartidas
- [ ] Secrets vault encriptado (AES-256) para compartir credenciales
- [ ] Project notes / contexto rápido para retomar trabajo

### Fase 4 — Cross-platform (2-3 meses)
> Objetivo: mismo comportamiento en Linux y macOS

- [ ] Soporte completo en Ubuntu/Debian
- [ ] Soporte completo en macOS
- [ ] Adaptación de rutas, permisos y comportamientos por SO
- [ ] CI/CD para builds en los tres sistemas

### Fase 5 — Inteligencia + Ecosistema (6-12 meses)
> Objetivo: hacer de Delixon algo indispensable

- [ ] Versión CLI (headless) para servidores sin interfaz gráfica
- [ ] Asistente IA con aprendizaje adaptativo
- [ ] Agentes especializados (seguridad, calidad, testing, performance)
- [ ] Pipeline de auditoría completa en un comando
- [ ] Catálogos corporativos y templates privadas
- [ ] Modo "arquitecto asistente" (IA que sugiere stacks)
- [ ] Generación orientada por tipo de producto
- [ ] Production hardening (perfiles de madurez)
- [ ] Sistema de plugins (comunidad extiende el producto)
- [ ] Marketplace de templates y recipes
- [ ] Soporte multi-editor: Cursor, WebStorm, Neovim, Zed
- [ ] Gestión de múltiples proyectos en producción
- [ ] Integración con herramientas de monitoreo

---

## 10. Objetivos por fase

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

## 11. Logros esperados con métricas

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

## 12. Comparativa con herramientas existentes

| Herramienta | Env vars | Runtimes | Templates | Terminal | Dashboard | Un clic | Sin Docker |
|---|---|---|---|---|---|---|---|
| **DevContainers** | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ (config larga) | ❌ |
| **direnv** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **nvm / pyenv** | ❌ | ✅ (1 solo) | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Docker Compose** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **mise (ex rtx)** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Scripts manuales** | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Delixon** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

**Delixon es la única herramienta que integra las 7 capacidades** en una sola interfaz sin requerir Docker.

### Competidor más cercano: mise (antes rtx)

`mise` es lo más cercano:
- Gestiona múltiples runtimes, carga env vars por directorio, ejecuta tareas, CLI puro

**Dónde Delixon supera a mise:**
- GUI con dashboard visual
- Plantillas con estructura completa (no solo runtimes)
- Historial de terminal aislado
- Apertura integrada con editor
- Gestión de dependencias compartidas
- Onboarding de equipo exportable
- Health checks y estado de proyectos
- Motor de scaffolding y recipes
- Agentes IA especializados

**Delixon no es "un mise con GUI". Es una plataforma completa de gestión de workspace que incluye lo que mise hace + mucho más.**

---

## 13. Funcionalidades diferenciadoras

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

## 14. Gobernanza y equipos/empresa

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

## 15. Landing page y waitlist

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

## 16. Opinión sincera y riesgos

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

## 17. Resumen ejecutivo

```
DELIXON — Visión completa

MISIÓN:
  Eliminar toda fricción entre "quiero trabajar en mi proyecto"
  y "estoy trabajando en mi proyecto".

CAPAS:
  1. Workspace → Aislamiento, env vars, runtimes, terminal, dashboard
  2. Scaffolding → Motor de stacks, +80 tecnologías, templates, recipes
  3. Inteligencia → IA adaptativa, agentes especializados, auditoría

DIFERENCIAL:
  - Única herramienta que integra las 3 capas
  - Sin Docker para el dev (solo para servicios de infra)
  - App nativa de 5MB (Tauri, no Electron)
  - Archivo .delixon para onboarding de equipo en 5 minutos
  - Funciona offline, todo local, zero vendor lock-in

ROADMAP:
  Corto (1-4m) → Core workspace funcional + 3 templates + landing con waitlist
  Medio (3-6m) → Motor scaffolding + recipes + scan proyectos + health checks
  Largo (6-12m) → IA + agentes + equipos + marketplace + cross-platform

COMPETENCIA:
  mise = CLI puro, sin GUI, sin scaffolding, sin IA
  DevContainers = Pesado, requiere Docker, sin dashboard
  direnv = Solo env vars
  Delixon = Todo integrado en una app nativa moderna

PÚBLICO:
  Dev individual → Productividad personal
  Equipos → Onboarding + consistencia
  Empresas → Gobernanza tecnológica

MODELO:
  Gratis para individual
  Pro para equipos (catálogos, templates privadas, políticas)
```

---

## 18. Checklist general de implementación

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

### Producto (delixon app) — Fase 1
- [ ] Persistencia de proyectos (CRUD real)
- [ ] Aislamiento de env vars funcional
- [ ] Apertura de proyecto con VSCode
- [ ] Detección + activación de runtimes al abrir
- [ ] Historial de terminal aislado
- [ ] UI mínima del dashboard
- [ ] Archivo `.delixon` export/import
- [ ] 3 plantillas funcionales (Node, React, Python)
- [ ] Detección de conflictos de puertos

### Producto — Fase 2
- [ ] Motor de scaffolding con catálogo de 30+ tecnologías
- [ ] 10 templates completos y probados
- [ ] Validación de compatibilidades entre tecnologías
- [ ] Gestión de runtimes (instalar/cambiar versiones)
- [ ] Recipes: agregar módulos a proyectos existentes
- [ ] Dashboard con health checks
- [ ] Contexto de Git integrado
- [ ] Análisis de proyecto existente (`delixon scan`)
- [ ] Terminal integrada
- [ ] Snapshots de entorno
- [ ] Scripts con alias unificados
- [ ] Gestión de procesos en background

### Producto — Fase 3
- [ ] Exportación de configuración de equipo (`.delixon-team`)
- [ ] Onboarding automatizado
- [ ] Secrets vault encriptado (AES-256)
- [ ] Project notes / contexto rápido

### Producto — Fase 4
- [ ] Soporte Ubuntu/Debian
- [ ] Soporte macOS
- [ ] CI/CD para builds en tres SO

### Producto — Fase 5
- [ ] CLI headless para servidores
- [ ] Asistente IA con aprendizaje adaptativo
- [ ] Agentes especializados (seguridad, calidad, testing, performance)
- [ ] Pipeline de auditoría completa
- [ ] Catálogos corporativos y templates privadas
- [ ] Modo "arquitecto asistente"
- [ ] Production hardening (perfiles de madurez)
- [ ] Sistema de plugins
- [ ] Marketplace de templates y recipes
- [ ] Soporte multi-editor (Cursor, WebStorm, Neovim, Zed)

---

*Delixon — Deja de configurar. Empieza a construir.*
