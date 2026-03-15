# Delixon
## La plataforma que libera a los desarrolladores de la carga invisible

> *Cada desarrollador pierde horas cada semana en trabajo que no es su trabajo.*
> *Delixon se las devuelve.*

---

## El problema

Existe un problema que afecta a cada desarrollador del mundo, todos los días, sin excepción: **el coste invisible de gestionar entornos de desarrollo**.

Un desarrollador profesional no trabaja en un solo proyecto. Trabaja en tres, en cinco, en ocho. Cada uno con su propio lenguaje, su propia versión de runtime, sus propias variables de entorno, sus propias dependencias. Y cada vez que cambia de proyecto, o empieza uno nuevo, o incorpora a alguien al equipo, paga ese coste.

**¿Cuánto cuesta exactamente?**

- Configurar un entorno nuevo desde cero: entre **1 y 3 horas** por proyecto
- Depurar errores causados por entornos mezclados: **45 minutos promedio** por incidente
- Incorporar un nuevo desarrollador a un proyecto existente: **4 a 8 horas** de onboarding técnico
- Instalar dependencias que ya existen en otro proyecto: tiempo y espacio duplicado, **siempre**

Para un desarrollador que trabaja en 4 proyectos simultáneamente, esto suma entre **2 y 5 horas semanales** en trabajo que no aporta ningún valor al producto que está construyendo.

Para un equipo de 10 personas, estamos hablando de **20 a 50 horas semanales** dedicadas a infraestructura de desarrollo local, no a desarrollo.

**Este problema no es pequeño. Es estructural. Y nadie lo ha resuelto correctamente.**

---

## Las soluciones actuales no funcionan

Existen herramientas que abordan partes del problema:

| Herramienta | Qué resuelve | Por qué no es suficiente |
|-------------|--------------|--------------------------|
| `venv` / `nvm` | Aísla un runtime | Solo uno. No integra el resto. No tiene UI. |
| Docker / DevContainers | Aislamiento total | Requiere conocer Docker. Lento. Pesado. Curva alta. |
| `direnv` | Variables de entorno | Solo eso. Sin interfaz. Sin integración con el flujo. |
| Scripts manuales | Flexible | No son mantenibles. No son compartibles. No escalan. |
| `.code-workspace` de VSCode | Configuración del editor | No aísla el entorno real del sistema. |

Ninguna de estas herramientas resuelve el problema completo. El desarrollador tiene que combinarlas, mantenerlas, documentarlas, y enseñarlas a cada nuevo miembro del equipo. El problema no desaparece — se traslada.

---

## La solución: Delixon

**Delixon es una aplicación de escritorio local que gestiona el entorno de desarrollo de cada proyecto de forma completamente aislada e independiente.**

No es un nuevo lenguaje. No es un IDE. No es un gestor de paquetes. Es la **capa de organización inteligente** que se sienta entre el desarrollador y sus proyectos, y se encarga de todo lo que el desarrollador no debería tener que pensar.

El desarrollador abre Delixon, selecciona el proyecto, hace clic en "Abrir".
VSCode se lanza con el workspace correcto. La terminal tiene el entorno correcto. Las variables de entorno correctas. La versión de Node correcta. La versión de Python correcta. El historial de comandos de ese proyecto, y solo de ese proyecto.

**Sin pasos manuales. Sin documentos de configuración. Sin sorpresas.**

---

## Cómo funciona

### Aislamiento completo por proyecto

Cada proyecto registrado en Delixon vive en su propio contexto independiente:

- Variables de entorno propias — no se filtran a otros proyectos
- Historial de terminal propio — los comandos de un proyecto no aparecen en otro
- Versión de runtime propia — Node 18 en uno, Node 20 en otro, sin gestor externo
- Dependencias propias o compartidas inteligentemente — sin duplicados innecesarios

### Gestión inteligente de dependencias

Delixon no reinstala lo que ya existe. Detecta si una dependencia compatible ya está en el sistema y la vincula. Si necesita una versión diferente, la instala de forma aislada solo para ese proyecto. El resultado: **menos espacio en disco, menos tiempo de instalación, mismo aislamiento**.

### Plantillas con mejores prácticas incluidas

Crear un proyecto nuevo en Delixon no es crear una carpeta vacía. Es seleccionar una plantilla — Node, React, Python, FastAPI, Full Stack, Docker — y obtener en segundos:

- Estructura de carpetas estándar y probada
- Linter y formatter configurados
- Git inicializado con hooks listos
- Scripts de desarrollo, build y test funcionando
- Archivos de entorno y `.gitignore` correctos

El desarrollador empieza a escribir código en minutos, no en horas.

### Portabilidad garantizada

Cada proyecto gestionado por Delixon genera un archivo `.delixon` con toda su configuración. Llevar el proyecto a otra máquina significa clonar el repo y abrir ese archivo en Delixon — que reconstruye el entorno exacto de forma automática.

---

## Ejemplo real: el primer día de un desarrollador nuevo

**Sin Delixon:**
```
09:00 — Llega el nuevo desarrollador
09:15 — Clona el repo
09:30 — "¿Qué versión de Node necesito?" → busca en la wiki, no está actualizada
10:00 — Instala dependencias → error de versión incompatible
10:30 — Configura variables de entorno → falta una, el proyecto no arranca
11:00 — "¿Dónde está el .env.example?" → no existe, alguien se lo manda por Slack
12:00 — Por fin el proyecto arranca. Han pasado 3 horas.
```

**Con Delixon:**
```
09:00 — Llega el nuevo desarrollador
09:05 — Instala Delixon
09:10 — Importa el archivo .delixon del proyecto
09:12 — Delixon clona, instala, configura
09:15 — Está trabajando. Han pasado 15 minutos.
```

---

## Mercado

### Tamaño del mercado

- **27 millones de desarrolladores de software** activos en el mundo (Stack Overflow Developer Survey 2024)
- **El 85% trabaja en más de un proyecto simultáneamente** — todos son usuarios potenciales de Delixon
- El mercado de herramientas de productividad para desarrolladores supera los **$10 billones anuales** y crece a doble dígito

### Usuario objetivo

**Perfil principal — Desarrollador individual:**
- Trabaja en 2 o más proyectos con tecnologías distintas
- Usa Windows, Linux o macOS (en ese orden de prioridad para el lanzamiento)
- Valora su tiempo y la calidad de su entorno de trabajo
- Está dispuesto a pagar por herramientas que le ahorren tiempo real

**Perfil secundario — Equipo de desarrollo:**
- Equipos de 2 a 20 personas
- Con rotación de miembros o incorporación frecuente de nuevos desarrolladores
- Que sufre inconsistencias de entorno entre miembros ("en mi máquina funciona")

**Perfil futuro — DevOps / Servidores:**
- Servidores con múltiples proyectos en producción
- Necesidad de aislamiento de dependencias sin contenedores completos

---

## Modelo de negocio

### Freemium

**Plan Gratuito — Individual:**
- Proyectos ilimitados
- Todas las funciones de aislamiento de entorno
- Plantillas básicas incluidas
- Uso personal, sin límite de tiempo

**Plan Pro — $9/mes por usuario:**
- Plantillas avanzadas y personalizadas
- Dashboard con métricas de proyectos
- Gestor de runtimes integrado (instalar/cambiar versiones desde Delixon)
- Terminal integrada en la app
- Exportación avanzada de configuraciones
- Soporte prioritario

**Plan Team — $19/mes por usuario (mínimo 3):**
- Todo lo de Pro
- Configuraciones de equipo sincronizadas
- Onboarding automatizado para nuevos miembros
- Repositorio privado de plantillas del equipo
- Panel de administración

**Marketplace de plantillas (futuro):**
- Desarrolladores y empresas pueden publicar plantillas
- Plantillas premium: modelo de reparto de ingresos 70/30

### Proyección conservadora (año 1-2)

| Escenario | Usuarios gratuitos | Conversión a Pro | Ingresos mensuales |
|-----------|-------------------|------------------|--------------------|
| Conservador | 5.000 | 3% (150 usuarios) | ~$1.350 |
| Moderado | 20.000 | 5% (1.000 usuarios) | ~$9.000 |
| Optimista | 50.000 | 7% (3.500 usuarios) | ~$31.500 |

El canal de crecimiento principal es **orgánico**: desarrolladores que usan Delixon recomiendan Delixon. El producto resuelve un problema tan visible que quien lo usa lo comenta.

---

## Ventaja competitiva

### Por qué Delixon puede ganar

**1. El momento es ahora.** El ecosistema de desarrollo se ha fragmentado enormemente en los últimos 5 años. Más lenguajes, más frameworks, más versiones, más proyectos. El problema que Delixon resuelve es más agudo que nunca y sigue creciendo.

**2. Nadie lo ha resuelto de esta forma.** Las soluciones existentes son parciales o requieren conocimiento técnico avanzado para configurarlas. Delixon apunta al desarrollador que quiere trabajar, no al que quiere gestionar infraestructura.

**3. El stack tecnológico es una ventaja real.** Construido con Tauri + Rust, Delixon es nativo, rápido y ligero (~5 MB de instalador, ~50 MB de RAM). No es otra app Electron pesada. Eso importa a los desarrolladores.

**4. Diseñado para crecer.** La arquitectura de Delixon está pensada desde el inicio para ser cross-platform (Windows → Linux → macOS), para escalar a equipos, y para expandirse a casos de uso en servidores. El MVP es solo la primera capa de un producto mucho mayor.

**5. Efecto de red en plantillas.** Cada plantilla que un usuario o empresa crea para Delixon hace el producto más valioso para todos los demás. Es un foso competitivo que crece solo.

---

## Estado actual

Delixon se encuentra en fase de **diseño y arquitectura**. La documentación técnica, la estructura del repositorio y el plan de producto están completamente definidos.

**Lo que existe hoy:**
- Documentación completa del producto y arquitectura
- Repositorio configurado en GitHub (`deli-labs/delixon`)
- Stack tecnológico validado (Tauri 2.x + React 18 + TypeScript)
- Plan de fases detallado con criterios de éxito medibles
- Plantillas iniciales definidas (Node, React, Python, FastAPI, Full Stack)

**Lo que viene:**
- Fase 1 MVP Windows: 3-4 meses de desarrollo
- Beta privada con usuarios reales seleccionados
- Lanzamiento público v1.0 en Windows
- Expansión a Linux y macOS

---

## El equipo

**dRaydel** — Fundador y desarrollador principal

Desarrollador con experiencia en múltiples proyectos simultáneos y tecnologías diversas. El problema que Delixon resuelve es un problema que el fundador ha vivido en primera persona durante años. Esa perspectiva es la base del diseño del producto: cada decisión viene de entender el dolor real del desarrollador, no de suposiciones.

El proyecto está actualmente en desarrollo individual con visión clara de incorporar colaboradores técnicos en cuanto el MVP esté validado.

---

## Lo que buscamos

Delixon busca en este momento:

- **Inversión pre-seed** para acelerar el desarrollo del MVP y cubrir los primeros 12 meses
- **Mentores técnicos o de producto** con experiencia en herramientas para desarrolladores
- **Early adopters** — desarrolladores que quieran ser los primeros en probar Delixon y dar feedback real

El objetivo no es levantar dinero para buscar un modelo de negocio. El modelo existe, el problema existe, los usuarios existen. El objetivo es acelerar el tiempo hasta tener algo en manos de esos usuarios.

---

## Por qué ahora

Cada semana que pasa, millones de desarrolladores en el mundo pierden horas en configuraciones manuales, entornos mezclados, y onboardings interminables. Cada semana sin Delixon es tiempo que no se puede recuperar.

La oportunidad está clara. La tecnología está disponible. El problema está sin resolver correctamente.

**Delixon va a resolverlo.**

---

> *dRaydel · deli-labs · delixon*
> `https://github.com/deli-labs/delixon`
