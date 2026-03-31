<!-- cSpell:enable -->
<!-- cSpell:words delixon tauri zustand radix shadcn vite vitest -->
# Delixon — Investor Memo

> *Un clic. Proyecto abierto. Entorno correcto.*

---

## Resumen ejecutivo

Delixon es una aplicación de escritorio local que organiza y aísla entornos de desarrollo por proyecto, permitiendo a los desarrolladores abrir cualquier proyecto con su configuración correcta en un clic.

Resuelve un problema cotidiano y costoso: la pérdida de tiempo causada por conflictos entre versiones, variables de entorno, dependencias y configuraciones al trabajar en varios proyectos con stacks distintos.

**La propuesta de valor es simple: un clic, proyecto abierto, entorno correcto.**

Delixon no reemplaza herramientas como VSCode, Git, Docker, nvm o pyenv. Se sitúa por encima de ellas como una capa de organización que coordina el contexto técnico de cada proyecto para que el desarrollador pueda empezar a trabajar sin fricción.

---

## El problema

Los desarrolladores que trabajan en varios proyectos pierden horas cada semana en tareas que no generan valor directo:

| Tarea sin valor | Impacto real |
|-----------------|--------------|
| Cambiar versiones de Node, Python o Go | 15-30 min por incidente |
| Corregir variables de entorno mezcladas | 30-45 min de depuración |
| Reconstruir configuraciones locales | 1-3 horas por proyecto nuevo |
| Resolver conflictos entre dependencias | Variable, a veces horas |
| Entender cómo ejecutar un proyecto ajeno | 4-8 horas de onboarding |

En equipos, este problema escala rápidamente:

- Ralentiza el onboarding de cada nuevo miembro.
- Introduce errores de entorno difíciles de rastrear.
- Genera dependencia de documentación informal (Slack, wikis desactualizadas).
- Reduce las horas efectivas de desarrollo real.

La mayoría de herramientas actuales resuelven piezas aisladas del problema, pero no la experiencia completa de entrada al proyecto.

| Herramienta | Qué resuelve | Qué deja sin resolver |
|-------------|--------------|----------------------|
| `venv` / `nvm` | Aísla un runtime | Solo uno. No integra el resto. Sin UI. |
| Docker / DevContainers | Aislamiento total | Requiere conocer Docker. Pesado. Curva alta. |
| `direnv` | Variables de entorno | Solo eso. Sin interfaz. Sin integración. |
| Scripts manuales | Flexible | No mantenibles. No compartibles. No escalan. |

---

## La solución

Delixon actúa como punto de entrada operativo para el desarrollador.

Cuando el usuario selecciona un proyecto en Delixon:

- Se abre el workspace correcto.
- Se carga el entorno adecuado.
- Se aplican las variables de entorno necesarias.
- Se respeta la versión requerida de cada runtime.
- Se mantiene el historial y contexto de ese proyecto de forma aislada.

La experiencia objetivo es eliminar el trabajo invisible previo al desarrollo.

En lugar de recordar pasos, leer documentación o preguntar a otro miembro del equipo, el desarrollador entra al proyecto desde un entorno ya preparado.

---

## Producto

La primera versión está enfocada en una experiencia local de escritorio, priorizando simplicidad, rendimiento y control del sistema operativo.

### Propuesta de producto inicial

- Creación de proyectos desde plantillas preconfiguradas.
- Apertura de proyectos existentes con su entorno correcto.
- Aislamiento de variables de entorno por proyecto.
- Historial de terminal separado por proyecto.
- Integración con VSCode.
- Exportación e importación de configuración (archivo `.delixon`).

### Evolución prevista

En fases posteriores, Delixon incorporará:

- Gestión inteligente de dependencias.
- Gestor integrado de runtimes.
- Terminal embebida.
- Funciones colaborativas para equipos.
- Soporte cross-platform completo (Windows, Linux, macOS).
- Futura versión CLI para entornos de servidor.

---

## Stack y decisión técnica

Delixon está construido con:

- **Backend:** Tauri 2 + Rust
- **Frontend:** React 18 + TypeScript + TailwindCSS

Esta elección responde a criterios de producto, no solo de ingeniería.

| Criterio | Delixon (Tauri/Rust) | Alternativa típica (Electron) |
|----------|----------------------|-------------------------------|
| Tamaño del instalador | ~2-5 MB | ~80-150 MB |
| Uso de RAM | ~50 MB | ~300-500 MB |
| Velocidad de arranque | < 1 segundo | 3-8 segundos |
| Acceso al SO | Nativo (Rust) | Mediado (Node.js) |
| Seguridad | Modelo de permisos estricto | Media |

Para un producto destinado a desarrolladores, el rendimiento y el peso de la aplicación no son detalles secundarios. Son parte de la propuesta de valor.

---

## Por qué ahora

La fragmentación del ecosistema de desarrollo ha aumentado de forma estructural en los últimos años:

- Más lenguajes y frameworks conviviendo en un mismo equipo.
- Más versiones activas por stack.
- Más proyectos paralelos por desarrollador.
- Más necesidad de onboarding rápido en equipos pequeños.
- Más complejidad local incluso en entornos aparentemente simples.

Las herramientas existentes ayudan, pero obligan al desarrollador a orquestarlas manualmente. Delixon nace para simplificar precisamente esa capa de coordinación que nadie ha abordado de forma completa.

---

## Ventaja de producto

La tesis central de Delixon es que el problema no es solo técnico, sino organizativo.

No se trata únicamente de instalar dependencias o cambiar versiones. El problema real es que cada proyecto necesita su propio contexto operativo, y ese contexto hoy se reconstruye manualmente demasiadas veces.

Delixon busca convertirse en la capa desde la que el desarrollador entra a trabajar. Esa posición es estratégica porque aumenta:

- **Frecuencia de uso** — se abre cada vez que el desarrollador trabaja.
- **Retención** — el contexto acumulado hace difícil abandonar la herramienta.
- **Dependencia funcional** — los equipos sincronizan sus entornos a través de Delixon.
- **Potencial de monetización** — a nivel individual y de equipo.

---

## Estado actual del proyecto

Delixon se encuentra en fase de arquitectura avanzada y scaffolding completo.

### Ya construido

- Repositorio principal configurado en GitHub (`delixon-technology/delixon`).
- Organización GitHub creada con estructura de equipos.
- Stack técnico instalado, compilando y generando instaladores nativos.
- Arquitectura backend modular en Rust/Tauri (6 módulos).
- Arquitectura frontend estructurada en React/TypeScript (páginas, componentes, tipos).
- 7 carpetas de plantillas creadas.
- Pipeline de CI/CD básica en GitHub Actions.
- Documentación completa del producto, visión, repositorio y pitch.

### Pendiente de implementación

- Lógica funcional real de los módulos backend.
- Stores de estado y hooks del frontend.
- Interfaz visual operativa.
- Contenido real de las plantillas.
- Tests unitarios y de integración.
- Primeras integraciones funcionales de entorno.

En términos prácticos: la base está definida y construida. El trabajo principal ahora es convertir esa estructura en producto usable.

---

## Roadmap

### Fase 1 — MVP Windows `[3-4 meses]`

Objetivo: entregar una versión funcional que permita usar Delixon en casos reales.

- Crear proyecto desde plantilla.
- Abrir proyecto existente con entorno correcto.
- Aislamiento de variables de entorno.
- Integración con VSCode.
- Exportar/importar configuración.
- Instalador Windows (.msi / .exe).
- Beta privada con usuarios reales.

### Fase 2 — Madurez `[2-3 meses tras MVP]`

Objetivo: aumentar utilidad y activar monetización inicial.

- Gestión inteligente de dependencias.
- Dashboard de estado de proyectos.
- Gestor de runtimes integrado.
- Terminal integrada.
- **Activación del Plan Pro.**

### Fase 3 — Equipos `[2-3 meses tras Fase 2]`

Objetivo: multiplicar el valor en entornos colaborativos.

- Onboarding automatizado.
- Configuraciones compartidas.
- Repositorio privado de plantillas.
- **Activación del Plan Team.**

### Fase 4 — Cross-platform `[2-3 meses tras Fase 3]`

Objetivo: ampliar mercado a Linux y macOS.

### Fase 5 — Servidores `[futuro]`

Objetivo: explorar expansión hacia entornos headless y casos de uso DevOps.

---

## Modelo de negocio

| Plan | Precio | Para quién |
|------|--------|------------|
| Gratuito | $0 | Desarrollador individual |
| Pro | $9/mes | Desarrollador que quiere más |
| Team | $19/mes/usuario (mín. 3) | Equipos |
| Marketplace | Reparto 70/30 | Creadores de plantillas |

### Proyección conservadora (año 1-2)

| Escenario | Usuarios gratuitos | Conversión Pro | MRR estimado |
|-----------|-------------------|----------------|--------------|
| Conservador | 5.000 | 3% → 150 | ~$1.350 |
| Moderado | 20.000 | 5% → 1.000 | ~$9.000 |
| Optimista | 50.000 | 7% → 3.500 | ~$31.500 |

### Lógica de monetización

El canal natural de adopción es bottom-up:

1. Un desarrollador individual descubre valor.
2. Lo introduce en su flujo diario.
3. Lo recomienda a su equipo.
4. El equipo adopta funciones compartidas.
5. La empresa amplía uso y licencias.

---

## Mercado y oportunidad

- **27 millones de desarrolladores** activos en el mundo (Stack Overflow Developer Survey 2024).
- **El 85% trabaja en más de un proyecto simultáneamente.**
- Mercado de herramientas de productividad para desarrolladores: **+$10B anuales**, crecimiento a doble dígito.

La oportunidad no está solo en el tamaño del mercado, sino en la **recurrencia del problema**:

- Ocurre semanalmente.
- Afecta a usuarios técnicos que valoran las buenas herramientas.
- Genera dolor claro tanto en uso individual como en equipos.

Si Delixon consigue convertirse en herramienta de entrada diaria al proyecto, puede capturar una posición con alto valor estratégico dentro del flujo de trabajo del desarrollador.

---

## Riesgos principales

Como todo proyecto en esta fase, Delixon tiene riesgos reales:

| Riesgo | Estrategia de mitigación |
|--------|--------------------------|
| Ejecución técnica del MVP | Stack validado, arquitectura definida, primer build ya funcional |
| Definir mal el alcance inicial | Foco extremo en el problema central, iterar con feedback |
| Abarcar demasiado antes de validar | MVP mínimo → beta → iterar. No más. |
| Adopción si la propuesta no se comunica bien | Frase clara, demo visual, beta con usuarios reales |
| Competidor grande entra al espacio | Ventaja de foco: Delixon resuelve un problema concreto, no es un feature de otra plataforma |

---

## Qué buscamos

Actualmente buscamos una combinación de:

- **Inversión pre-seed** para acelerar el desarrollo del MVP y cubrir los primeros 12 meses.
- **Mentores** con experiencia en herramientas para desarrolladores.
- **Early adopters técnicos** para la beta privada.
- **Acompañamiento estratégico** en go-to-market y pricing.

---

## Cierre

Delixon parte de una observación sencilla:

> Los desarrolladores pierden demasiadas horas en preparar el contexto de trabajo antes de empezar a construir.

Ese tiempo perdido no es anecdótico. Es recurrente, costoso y ampliamente aceptado como "normal" pese a no aportar valor real.

El objetivo no es levantar dinero para buscar un modelo de negocio. El modelo existe, el problema existe, los usuarios existen. El objetivo es acelerar el tiempo hasta tener algo en manos de esos usuarios.

**Un clic. Proyecto abierto. Entorno correcto.**

---

*delirestevez · delixon-technology · `github.com/delixon-technology/delixon`*
