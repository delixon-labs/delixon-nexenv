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
9. [Integración con StackPilot](#9-integración-con-stackpilot)
10. [Hoja de ruta y fases](#10-hoja-de-ruta-y-fases)
11. [Objetivos por fase](#11-objetivos-por-fase)
12. [Logros esperados con métricas](#12-logros-esperados-con-métricas)
13. [Comparativa con herramientas existentes](#13-comparativa-con-herramientas-existentes)
14. [Funcionalidades diferenciadoras](#14-funcionalidades-diferenciadoras)
15. [Gobernanza y equipos/empresa](#15-gobernanza-y-equiposempresa)
16. [Landing page y waitlist](#16-landing-page-y-waitlist)
17. [Opinión sincera y riesgos](#17-opinión-sincera-y-riesgos)
18. [Resumen ejecutivo](#18-resumen-ejecutivo)
19. [Análisis de opinión final — Integración](#19-análisis-de-opinión-final--integración-stackpilot--delixon)
20. [Glosario de tecnologías y siglas](#20-glosario-de-tecnologías-y-siglas)
21. [Checklist general de implementación](#21-checklist-general-de-implementación)

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

**Capa 1 (Workspace)** — El core. Aislamiento, env vars, runtimes, terminal, dashboard. Lo que hace que Delixon sea útil TODOS los días.
**Capa 2 (Scaffolding)** — Motor de generación y composición. No solo "abrir un proyecto existente", sino crearlo, escanearlo, evolucionar su stack, y validar su arquitectura.
**Capa 3 (Inteligencia)** — Asistente que aprende, audita, sugiere y automatiza. Solo cuando las capas 1 y 2 estén sólidas.

**Transversal: Núcleo declarativo** — El `project manifest` que unifica toda la información del proyecto (techs, versiones, servicios, env vars, health checks, comandos, nivel de madurez). Todas las capas leen y escriben sobre él. Sin este núcleo, la integración es frágil.

**Capas de implementación** (no confundir con capas del producto):
- **Capa A (base estructural):** catálogo + reglas + manifest + templates + recipes + scan → sin esto lo demás es difícil de mantener
- **Capa B (operación local):** env vars + runtimes + terminal + VSCode + Docker services + health + doctor → aquí Delixon gana valor real diario
- **Capa C (confianza y evolución):** diff/rollback + perfiles madurez + recomendaciones + hardening → aquí se vuelve algo serio

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

## 9. Integración con StackPilot

### Visión central

> **Delixon es el sistema operativo local del developer.**
> **StackPilot es su motor de generación y composición de proyectos.**

No es "Delixon + otra cosa pegada". Es **Delixon con un engine de scaffolding/composición por debajo**. El usuario nunca ve "StackPilot" — ve Delixon haciendo cosas potentes.

```
Delixon = gestión, aislamiento, operación y experiencia diaria
Stack engine = creación, expansión, validación y evolución del stack
```

La fuerza real está en la combinación: no solo genera proyectos (eso lo hacen muchos), y no solo gestiona entornos (eso lo hace direnv). **Delixon crea, entiende, ejecuta, aísla, amplía, valida y repara proyectos localmente desde una sola app.**

### Origen

StackPilot (repo `Xplus-Technologies-open-source/StackPilot`) se desarrolló en paralelo, enfocado en scaffolding inteligente: configuración inicial, planificación del stack, y generación de estructura con buenas prácticas. Delixon absorbe ese motor como capacidad interna.

### Qué aporta cada uno

| Aspecto | Delixon (Capa 1 — Workspace) | StackPilot (Capa 2 — Scaffolding) | Decisión final |
|---|---|---|---|
| Proyecto management (CRUD) | ✅ Funcional (Rust + React) | ✅ Funcional (SQLite + CLI) | **Delixon** — UI nativa Tauri, persistencia JSON |
| Env vars aisladas | ✅ JSON por proyecto | ❌ Solo genera .env.example | **Delixon** — Core de Capa 1 |
| Runtime detection | ✅ Node, Python, Rust, Go | ❌ Asume instalado | **Delixon** — Detecta y activa |
| Abrir en VSCode | ✅ Funcional | ❌ No tiene | **Delixon** |
| Terminal aislada | ✅ Con env vars cargadas | ❌ No tiene | **Delixon** |
| Dashboard UI | ✅ React + Tailwind, 4 páginas | ✅ React + Tailwind, 7 páginas | **Delixon** — Una sola UI unificada |
| Templates | ✅ 7 templates funcionales (UI + backend) | ✅ 20 templates completos | **Completado** — Templates integrados en GUI y scaffold |
| Catálogo tecnológico | ✅ 30+ techs en YAML con UI | ✅ 83 tecnologías en YAML | **Completado** — Catálogo migrado con browse, search, filtros |
| Scaffolding real | ✅ Genera proyectos completos | ✅ Genera proyectos completos | **Completado** — Wizard multi-step en GUI + `new` en CLI |
| Validación de stacks | ✅ RulesEngine integrado | ✅ RulesEngine | **Completado** — Dependencias auto, incompatibilidades, puertos, sugerencias |
| Docker management | ✅ up/down/status/logs | ✅ up/down/status/logs | **Completado** — GUI (DockerTab) + CLI (docker subcommands) |
| CLI | ✅ 28 comandos (clap) | ✅ 23 comandos (origen) | **Completado** — CLI y GUI comparten `delixon_lib`, ambas interfaces al mismo motor |
| Versionado de stacks | ✅ save/diff/rollback + UI | ✅ save/diff/rollback | **Completado** — VersioningTab en GUI + snapshot en CLI |
| Health checks | ✅ Por proyecto con UI | ✅ Por tecnología | **Completado** — HealthTab con sugerencias de fix + CLI health |
| Doctor command | ✅ Sistema completo | ✅ Verifica requisitos | **Completado** — Runtimes, Docker, Git, config, datos |
| Perfiles de madurez | ✅ rapid/standard/production | ✅ rapid/standard/production/enterprise | **Completado** — Integrados en scaffold |
| Full-stack detection | ✅ Deteccion completa (964 lineas) | ✅ frontend/ + backend/ automático | **Completado** — Readiness score, frameworks, ORMs, CI, linters |
| Settings persistentes | ✅ Editor, tema, idioma, runtimes | ✅ Editor, package manager | **Delixon** — Unificar preferencias |
| TechInstaller / Recipes | ✅ 6 recipes funcionales | ✅ Lógica por tecnología | **Completado** — vitest, pytest, docker, ci-github, biome, prisma |
| Config DB | ✅ JSON local | ✅ SQLite | **JSON corto plazo** — Evaluar SQLite si escala a equipos |

### Ideas de StackPilot que NO se integran (o se postergan)

| Concepto | Razón |
|---|---|
| ~~23 comandos CLI completos~~ | ✅ **Completado** — 28 comandos implementados (ver `docs/commander_cli/CLI_REFERENCE.md`) |
| DevContainers generation (.devcontainer/) | Va contra la filosofía "sin Docker para dev". Solo como export opcional futuro |
| Monorepo con Turborepo | Delixon es monolito Tauri. No aplica |
| ~~User tech notes (rating personal)~~ | ✅ **Completado** — Sistema de notas por proyecto (CRUD con UUID y timestamps) |
| SQLite como DB local | JSON funciona para <100 proyectos. Evaluar a futuro |

### El núcleo declarativo común (CRÍTICO)

Sin una capa declarativa compartida, la integración será frágil — "una colección de botones" en vez de un sistema coherente.

**Project Manifest** — formato interno que define cada proyecto:

```yaml
# .delixon/manifest.yaml (generado automáticamente, editable)
name: mi-saas
type: saas-b2b
profile: standard
runtime: node@20
technologies:
  - nextjs@14
  - prisma@5
  - postgresql@16
  - tailwindcss@3
  - nextauth@4
services:
  - type: postgresql
    port: 5432
    docker: true
    health_check: "pg_isready -U postgres"
  - type: redis
    port: 6379
    docker: true
    health_check: "redis-cli ping"
env_vars:
  required: [DATABASE_URL, NEXTAUTH_SECRET, NEXTAUTH_URL]
  optional: [REDIS_URL, STRIPE_KEY]
commands:
  dev: "npm run dev"
  build: "npm run build"
  test: "npm run test"
  lint: "npm run lint"
ports: [3000, 5432, 6379]
recipes_applied: [auth-nextauth, database-prisma, docker-services]
```

**Este manifiesto es la columna vertebral.** Todo lo demás (dashboard, health checks, doctor, scan, recipes, versionado) lee y escribe sobre él. Sin él, cada feature es un silo independiente.

### Flujo ideal de la integración

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

### Lo que NO es la integración

- **No es meter StackPilot como módulo externo** — es absorber sus capacidades como motor interno
- **No es tener dos UIs** — es una sola app (Delixon) con más capacidades
- **No es copiar todo** — es migrar lo valioso (catalogo, reglas, scaffold, recipes, health) y descartar lo que no aplica (monorepo, devcontainers). El CLI ya supera al original con 28 comandos
- **No es priorizar cantidad** — 25 tecnologías sólidas > 83 a medias; 8 templates probados > 20 sin mantener

---

## 10. Hoja de ruta y fases

### Estado actual — Lo que YA funciona

**Delixon app (Capa 1 — Workspace):**
- [x] App de escritorio con Tauri + React
- [x] CRUD de proyectos (crear, abrir, eliminar, actualizar)
- [x] Aislamiento de env vars por proyecto (JSON por proyecto)
- [x] Detección de runtimes: Node.js, Python, Rust, Go
- [x] Apertura de proyecto en VSCode con contexto
- [x] Apertura de terminal con env vars cargadas
- [x] Dashboard con búsqueda, filtros, y grid de proyectos
- [x] Página de detalle de proyecto con gestión de env vars
- [x] Página de templates (7 definidos, sin contenido aún)
- [x] Settings: editor, tema, idioma, detección de runtimes
- [x] Sidebar con navegación y proyectos recientes
- [x] Persistencia local (JSON en `~/.local/share/delixon/`)

**StackPilot (motor de scaffolding, repo separado — pendiente de absorción):**
- [x] 83 tecnologías en catálogo YAML con metadatos completos
- [x] RulesEngine: validación, dependencias automáticas, conflictos, puertos
- [x] ScaffoldOrchestrator: genera docker-compose, .env, README, CI/CD, scripts
- [x] TechInstaller: lógica específica por tecnología
- [x] 20 templates funcionales
- [x] 28 comandos CLI (ver tabla completa en sección P0 CLI)
- [x] Full-stack detection (frontend/ + backend/)
- [x] Versionado de stacks (save/diff/rollback)
- [x] Docker management (up/down/status/logs)
- [x] Health checks por tecnología (command, endpoint, interval)
- [x] Perfiles de madurez (rapid/standard/production/enterprise)
- [x] Doctor command (verificar requisitos del sistema)
- [x] Desktop app con Tauri 2

**Landing page (delixon-web):**
- [x] Landing completa con paneles expandibles, efectos 3D, i18n ES/EN
- [x] Waitlist backend: Fastify + PostgreSQL + Docker
- [x] Admin panel, referidos, double opt-in

### Principio rector

> **"Primero indispensable. Luego potente. Después ambicioso."**
>
> Hoy, "indispensable" para Delixon es: manifest, open, doctor, health, scan, CLI, 3-5 templates perfectos, 2-4 recipes impecables. Todo lo demás puede esperar.

### CORTO PLAZO (1-3 meses) — "Que funcione de verdad"

> **Base declarativa + operación local + CLI + cross-platform desde el día 1.**
>
> Sin la base declarativa, todo lo demás será frágil. Sin operación local útil, nadie lo usa dos veces.
> Sin CLI, pierdes a la mayoría de devs. Sin Linux, pierdes a la mitad del público.

**P0 — Nucleo declarativo (la columna vertebral):**
- [x] Definir formato de `project manifest` (.delixon/manifest.yaml) — techs, servicios, env vars, comandos, puertos, recipes, health checks
- [x] Migrar catalogo YAML de StackPilot → 30+ tecnologias con metadatos completos, UI de browse/search
- [x] Integrar RulesEngine con dependencias auto, incompatibilidades, puertos, sugerencias
- [x] Generar manifest automaticamente al crear o importar un proyecto

**P0 — Completar workspace (Capa 1):**
- [ ] Historial de terminal aislado por proyecto
- [ ] Activacion automatica de runtimes al abrir proyecto (ya detecta, falta activar)
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

**P1 — Crear proyectos reales (integrar motor de StackPilot):**
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
- [ ] Mas recipes: Auth, Pagos, Email, Admin panel, Observabilidad

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

**P3 — Equipos:**
- [ ] Exportación de configuración de equipo (`.delixon-team`)
- [ ] Onboarding automatizado: nuevo dev → `delixon setup` → entorno completo en 5 min
- [ ] Secrets vault encriptado (AES-256) para compartir credenciales
- [ ] Project notes / contexto rápido (retomar proyectos olvidados en 10 segundos)

**P3 — Madurez:**
- [ ] Perfiles de madurez completos que cambien archivos, deps, estructura y validaciones reales
- [ ] Generación orientada por tipo de producto ("¿Qué vas a construir?" → stack recomendado)
- [ ] Soporte multi-editor: Cursor, WebStorm, Neovim, Zed
- [ ] Control de versiones de plantillas y configuraciones
- [ ] 8-10 templates sólidos (expandir desde los 3-5 iniciales)
- [ ] Catálogo completo de 25-30 tecnologías probadas

### VISIÓN FUTURA — "El sueño" (12+ meses, no comprometido)

> Ideas ambiciosas que dependen de validación del mercado y recursos. No se implementarán hasta que las fases anteriores estén sólidas. Esto no es un compromiso, es una dirección.

- [ ] Asistente IA con aprendizaje adaptativo (recuerda preferencias, sugiere stacks)
- [ ] Agentes especializados: SecurityGuard, CodeReviewer, TestBuilder, PerfAnalyzer, DocWriter
- [ ] Pipeline de auditoría completa (seguridad + calidad + tests + performance en un comando)
- [ ] Modo "arquitecto asistente" (describe lo que quieres → stack recomendado con estimación de coste)
- [ ] Catálogos corporativos (tecnologías aprobadas/prohibidas por empresa)
- [ ] Templates privadas de organización con políticas y cobertura mínima
- [ ] Sistema de plugins (la comunidad extiende Delixon)
- [ ] Marketplace de templates y recipes
- [ ] Gestión de múltiples proyectos en producción (modo servidor)
- [ ] Integración con herramientas de monitoreo (Grafana, Prometheus)
- [ ] Exportación automática de decisiones técnicas
- [ ] DevContainers export (para equipos que lo requieran)
- [ ] Editor visual de plantillas

### Tabla resumen de prioridades

| Prioridad | Que | Estado | Pendiente |
|---|---|---|---|
| **P0** | Manifest + workspace + CLI + cross-platform | ✅ Mayormente completado | Terminal aislada, activacion runtimes, CI/CD multi-SO |
| **P1** | Templates + scaffold + scan + health + doctor | ✅ Completado | Tests automatizados de templates |
| **P1** | Recipes (6 funcionales) | ✅ Completado | Mas recipes: Auth, Pagos, Email, Observabilidad |
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
  Delixon = gestión, aislamiento, operación y experiencia diaria
  Stack engine (ex-StackPilot) = creación, expansión, validación del stack
  → Un solo producto. El usuario nunca ve "dos cosas".

COLUMNA VERTEBRAL:
  Project Manifest → formato declarativo que unifica toda la info
  del proyecto (techs, servicios, env vars, health, comandos, madurez).
  Todas las features leen y escriben sobre él.

CAPAS DEL PRODUCTO:
  1. Workspace → Aislamiento, env vars, runtimes, terminal, dashboard
  2. Scaffolding → Motor de stacks, catálogo, templates, recipes, scan
  3. Inteligencia → IA adaptativa, agentes, auditoría (solo si 1+2 están sólidas)

DIFERENCIAL:
  - Única herramienta que integra las 3 capas
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

## 19. Análisis de opinión final — Integración StackPilot + Delixon

> Este análisis evalúa cada aspecto de la integración entre StackPilot y Delixon con una opinión objetiva sobre qué debería tener el producto final, qué priorizar, y qué descartar.

### 19.1 Ideas de StackPilot integradas al plan

| Concepto de StackPilot | Cómo se integró en el plan | Opinión final — Qué debería tener Delixon |
|---|---|---|
| Catálogo de 83 tecnologías en YAML | → "Catálogo tecnológico declarativo" con +80 techs | **Sí, pero empezar con 25-30 sólidas.** 83 es inmantenible sin equipo. Calidad > cantidad. Las 30 más usadas cubren el 90% de stacks reales |
| Validación de compatibilidades | → "Validación inteligente de stacks" | **Sí, con niveles.** No solo compatible/incompatible: `válido` / `válido con advertencia` / `no recomendado` / `incompatible`. Esto orienta, no solo bloquea |
| Templates prearmados (T3, MERN, SaaS) | → "Templates prearmados" (8 listados) | **Sí, pero 8-10 máximo, modulares.** Templates base + recipes encima. No 200 templates rígidos inmantenibles |
| Generación por tipo de producto | → "Generación orientada por tipo" | **Mediano plazo, no MVP.** Requiere que el catálogo y las reglas estén sólidos primero. Es una capa de UX encima, no una prioridad técnica |
| Recipes | → "Recipes: módulos que se añaden" | **Sí, absolutamente prioritario.** Es lo que hace que Delixon sirva DESPUÉS del día 1. Sin recipes solo genera; con recipes evoluciona |
| Perfiles de madurez | → "Production hardening" con 4 perfiles | **Sí, pero que cambien cosas reales** (archivos, deps, validaciones, warnings). No solo una etiqueta visual |
| Análisis de proyecto existente | → "delixon scan" | **Sí, prioridad alta.** Sin esto, Delixon solo sirve para proyectos nuevos. Con scan, sirve para adoptar los existentes. Eso duplica el público objetivo |
| Docker Compose solo para servicios | → Misma filosofía adoptada | **Sí, mantener firme.** Runtimes nativos + Docker solo para postgres, redis, rabbitmq, etc. Es el diferencial vs DevContainers |

### 19.2 Ideas de StackPilot que quedaron fuera — Evaluación

| Concepto de StackPilot | Por qué no entró | Opinión final |
|---|---|---|
| 23 comandos CLI completos | ✅ **28 comandos implementados** | **Completado y superado.** CLI y GUI son interfaces duales al mismo core. Ver `docs/commander_cli/CLI_REFERENCE.md` |
| Versionado de stacks (save/diff/rollback) | No aparecía en el plan | **Incluir en mediano plazo.** Es lo que reduce el miedo a tocar el proyecto. "Puedo revertir" = confianza = uso real |
| TechInstaller (lógica específica) | Mencionado en recipes superficialmente | **Incluir como motor interno de recipes.** Cada recipe necesita un installer que sepa qué hacer por tecnología |
| SQLite como DB local | Delixon usa JSON | **Mantener JSON corto plazo, evaluar SQLite a futuro.** JSON funciona para <100 proyectos. Si escala a equipos, SQLite o similar será necesario |
| DevContainers generation | No mencionado | **No prioritario.** Va contra la filosofía "sin Docker para dev". Solo como export opcional para equipos que lo requieran |
| Makefile + scripts auxiliares | No mencionado | **Incluir como parte del scaffold.** Un `scripts/dev.sh` o `Makefile` básico es best practice. El scaffold debería generarlo |
| Health checks por tecnología | Solo mencionado superficialmente | **Subir a corto plazo.** Sin health checks, el dashboard es decorativo. Con ellos, es útil cada día |
| Doctor command | ✅ **Implementado** | Verifica runtimes, Docker, Git, config, datos. GUI + CLI |
| Full-stack detection (frontend/+backend/) | No mencionado explícitamente | **Incluir en scan.** El scan debe detectar automáticamente la estructura frontend/backend y actuar en consecuencia |
| User tech notes (rating personal) | ✅ **Implementado** como sistema de notas por proyecto (CRUD con timestamps) |

### 19.3 Visión final — Qué debe ser Delixon

| Aspecto | Estado actual | Opinión final — Cómo debería funcionar |
|---|---|---|
| **Identidad** | ✅ GUI + CLI dual | **"Sistema operativo local del developer"** — gestiona TODO el ciclo de vida local: crear, configurar, aislar, ejecutar, evolucionar, diagnosticar, reparar |
| **Nucleo declarativo** | ✅ Implementado | Manifest `.delixon/manifest.yaml` unifica: techs, servicios, env vars, comandos, puertos, recipes, health checks |
| **Flujo de creacion** | ✅ Wizard completo | Scaffold multi-step en GUI + `new`/`create` en CLI. Elegir tipo → stack → validar → generar → registrar |
| **Scan/import** | ✅ Implementado | `scan` detecta 15+ aspectos del stack. Export/import con formato `.delixon` portable |
| **Health + Doctor** | ✅ Implementado | Doctor del sistema + health por proyecto. GUI (HealthTab) + CLI. Sugerencias de fix |
| **Versionado de stack** | ✅ Implementado | save/list/diff/rollback de manifest. GUI (VersioningTab) + CLI snapshot commands |
| **CLI** | ✅ 28 comandos | Estrategia dual GUI+CLI. Ambas interfaces al mismo `delixon_lib`. Falta: autocompletado shell, instalacion global |
| **Templates** | ✅ 7 funcionales | Node+Express, React+Vite, FastAPI, Django, Fullstack, Rust CLI, Docker Compose. Ampliar con mas recipes |
| **Docker** | ✅ Integrado | up/down/status/logs desde GUI (DockerTab) y CLI. Deteccion de puertos |
| **Perfiles madurez** | ✅ Integrados | rapid/standard/production en scaffold. Cambian archivos reales |

### 19.4 Priorización final recomendada

| Prioridad | Que | Estado |
|---|---|---|
| **P0 — Base** | Manifest + catalogo + reglas + workspace + CLI + GUI | ✅ Completado |
| **P1 — Crear** | 7 templates + scaffold wizard + recipes (6) + scan | ✅ Completado |
| **P1 — Operar** | Health + doctor + Docker + env vars + Git + scripts + procesos | ✅ Completado |
| **P2 — Evolucionar** | Versionado + diff/rollback + perfiles + notas + export/import | ✅ Completado |
| **P2 — Expandir** | CLI 28 cmds + output formateado | ✅ Completado. Pendiente: terminal integrada, autocompletado shell |
| **P3 — Equipos** | `.delixon-team` + onboarding + secrets vault + multi-editor | Pendiente — lo que monetiza |
| **P4 — Futuro** | IA + agentes + marketplace + plugins | Pendiente — lo que diferencia |
| **P4 — Sueño** | IA + agentes + marketplace + plugins + catálogos corporativos | Lo que diferencia a largo plazo, pero solo si lo anterior está sólido |

### 19.5 Conclusión

> **La mejor integración no es "Delixon + StackPilot pegados". Es Delixon con un engine de scaffolding/composición por debajo.**

La fuerza real de la fusión está en que Delixon deja de ser solo un gestor de entornos para convertirse en **el sistema operativo local del developer**: una sola app que crea, entiende, ejecuta, aísla, amplía, valida y repara proyectos.

**Lo crítico:** sin el núcleo declarativo (project manifest), la integración será vistosa pero frágil — "una colección de botones" en vez de un sistema coherente. El manifiesto es la columna vertebral sobre la que todo lo demás se construye.

**La trampa a evitar:** no competir en cantidad por la cantidad. Competir en **calidad de la experiencia completa** — 30+ tecnologias solidas, 7 templates probados, 28 comandos CLI, 6 recipes, y un flujo GUI+CLI que funcione de extremo a extremo.

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

### Tecnologías del motor de scaffolding (de StackPilot)

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

*Delixon — Deja de configurar. Empieza a construir.*
