import { useState } from "react";
import type { Template } from "@/types/template";
import UseTemplateModal from "@/components/templates/UseTemplateModal";

const BUILT_IN_TEMPLATES: Template[] = [
  {
    id: "node-express",
    name: "Node.js + Express",
    description:
      "API REST con Express, estructura MVC, ESLint, Prettier y scripts de desarrollo listos.",
    runtimes: ["node"],
    tags: ["backend", "api", "rest"],
    isOfficial: true,
    version: "1.0.0",
  },
  {
    id: "react-vite",
    name: "React + Vite",
    description:
      "Aplicacion React con Vite, TypeScript, TailwindCSS, ESLint y hot reload configurado.",
    runtimes: ["node"],
    tags: ["frontend", "spa", "react"],
    isOfficial: true,
    version: "1.0.0",
  },
  {
    id: "python-fastapi",
    name: "Python + FastAPI",
    description:
      "API moderna con FastAPI, Pydantic, uvicorn, estructura de carpetas profesional.",
    runtimes: ["python"],
    tags: ["backend", "api", "python"],
    isOfficial: true,
    version: "1.0.0",
  },
  {
    id: "python-django",
    name: "Python + Django",
    description:
      "Proyecto Django con configuracion de produccion, static files y admin habilitado.",
    runtimes: ["python"],
    tags: ["backend", "fullstack", "python"],
    isOfficial: true,
    version: "1.0.0",
  },
  {
    id: "fullstack-react-python",
    name: "Full Stack (React + Python)",
    description:
      "Frontend React + Backend FastAPI en un solo proyecto. Monorepo con scripts unificados.",
    runtimes: ["node", "python"],
    tags: ["fullstack", "monorepo"],
    isOfficial: true,
    version: "1.0.0",
  },
  {
    id: "rust-cli",
    name: "Rust CLI",
    description:
      "Herramienta CLI en Rust con clap, manejo de errores con anyhow y estructura modular.",
    runtimes: ["rust"],
    tags: ["cli", "rust", "tool"],
    isOfficial: true,
    version: "1.0.0",
  },
  {
    id: "docker-compose",
    name: "Docker Compose",
    description:
      "Stack multi-servicio con Docker Compose, redes, volumenes y .env configurados.",
    runtimes: [],
    tags: ["docker", "devops", "infra"],
    isOfficial: true,
    version: "1.0.0",
  },
];

const RUNTIME_COLORS: Record<string, string> = {
  node: "bg-green-500/10 text-green-400",
  python: "bg-yellow-500/10 text-yellow-400",
  rust: "bg-orange-500/10 text-orange-400",
  go: "bg-cyan-500/10 text-cyan-400",
};

export default function Templates() {
  const [selectedTemplate, setSelectedTemplate] = useState<Template | null>(null);

  return (
    <div className="p-6 lg:p-8 h-full overflow-y-auto">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-white">Plantillas</h1>
        <p className="text-sm text-gray-500 mt-1">
          Plantillas oficiales con mejores practicas. Haz clic en "Usar" para
          crear un nuevo proyecto.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-4">
        {BUILT_IN_TEMPLATES.map((template) => (
          <div
            key={template.id}
            className="flex flex-col p-5 rounded-xl bg-gray-900 border border-gray-800 hover:border-gray-700 transition-colors"
          >
            <div className="flex items-start justify-between mb-3">
              <h3 className="text-base font-semibold text-white">
                {template.name}
              </h3>
              {template.isOfficial && (
                <span className="px-2 py-0.5 rounded-md bg-primary-500/10 text-primary-500 text-xs font-medium">
                  Oficial
                </span>
              )}
            </div>

            <p className="text-sm text-gray-500 mb-4 flex-1">
              {template.description}
            </p>

            {/* Runtimes */}
            <div className="flex flex-wrap gap-1.5 mb-3">
              {template.runtimes.map((rt) => (
                <span
                  key={rt}
                  className={`px-2 py-0.5 rounded-md text-xs font-medium ${RUNTIME_COLORS[rt] || "bg-gray-800 text-gray-400"}`}
                >
                  {rt}
                </span>
              ))}
              {template.runtimes.length === 0 && (
                <span className="px-2 py-0.5 rounded-md text-xs bg-gray-800 text-gray-500">
                  Sin runtime
                </span>
              )}
            </div>

            {/* Tags + Button */}
            <div className="flex items-center justify-between mt-auto">
              <div className="flex flex-wrap gap-1">
                {template.tags.map((tag) => (
                  <span
                    key={tag}
                    className="px-1.5 py-0.5 rounded text-xs text-gray-600"
                  >
                    #{tag}
                  </span>
                ))}
              </div>
              <button
                onClick={() => setSelectedTemplate(template)}
                className="px-3 py-1.5 rounded-lg text-xs font-medium bg-success/10 text-success-light hover:bg-success/20 transition-colors"
              >
                Usar
              </button>
            </div>
          </div>
        ))}
      </div>

      {selectedTemplate && (
        <UseTemplateModal
          isOpen={true}
          onClose={() => setSelectedTemplate(null)}
          templateId={selectedTemplate.id}
          templateName={selectedTemplate.name}
        />
      )}
    </div>
  );
}
