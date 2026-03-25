# Delixon

> Deja de configurar. Empieza a construir.
>
> *Un clic. Proyecto abierto. Entorno correcto.*

Delixon es una aplicación de escritorio local que gestiona el entorno de desarrollo de cada proyecto de forma completamente aislada e independiente.

## ¿Qué hace Delixon?

- **Aísla proyectos** — cada proyecto tiene su propio entorno, variables, historial de terminal y versiones de runtimes
- **Gestiona dependencias inteligentemente** — detecta lo que ya tienes instalado y lo vincula en lugar de duplicarlo
- **Plantillas con mejores prácticas** — arranca cualquier proyecto en minutos con la estructura correcta desde el primer momento
- **Apertura instantánea** — un clic, y tu proyecto está listo: editor, terminal, entorno, todo correcto

## Stack

- **Frontend**: React 18 + TypeScript + TailwindCSS
- **Backend**: Rust + Tauri 2.x
- **Plataforma**: Windows (v1), Linux y macOS (futuro)

## Documentación

- [Plan del producto](docs/PLAN.md)
- [Estructura del repositorio](docs/REPO_STRUCTURE.md)
- [Arquitectura técnica](docs/architecture.md)
- [Guía de contribución](docs/contributing.md)

## Desarrollo local

```bash
# Requisitos: Node 20+, Rust 1.77+
git clone https://github.com/deli-technology/delixon.git
cd delixon
npm install
cp .env.example .env.local
npm run tauri dev
```

---

*deli-technology · [github.com/deli-technology/delixon](https://github.com/deli-technology/delixon)*
