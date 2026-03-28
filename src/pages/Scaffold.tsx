import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import * as api from "@/lib/tauri";
import PathInput from "@/components/ui/PathInput";
import type { Technology } from "@/types/catalog";
import type { ValidationResult, ScaffoldPreview } from "@/types/scaffold";
import { CATEGORY_LABELS } from "@/lib/catalog";

type Step = "info" | "stack" | "preview" | "result";

const PROJECT_TYPES = [
  { value: "api", label: "API REST" },
  { value: "frontend", label: "Frontend" },
  { value: "fullstack", label: "Full Stack" },
  { value: "cli", label: "CLI" },
  { value: "desktop", label: "Desktop" },
  { value: "monorepo", label: "Monorepo" },
];

const PROFILES = [
  { value: "rapid", label: "Rapido", desc: "Minimo setup, ideal para prototipos" },
  { value: "standard", label: "Estandar", desc: "Balance entre velocidad y buenas practicas" },
  { value: "production", label: "Produccion", desc: "CI/CD, Docker, testing, seguridad" },
];

export default function Scaffold() {
  const navigate = useNavigate();

  const [step, setStep] = useState<Step>("info");
  const [name, setName] = useState("");
  const [path, setPath] = useState("");
  const [projectType, setProjectType] = useState("api");
  const [profile, setProfile] = useState("standard");

  const [allTechs, setAllTechs] = useState<Technology[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [selectedTechs, setSelectedTechs] = useState<string[]>([]);
  const [validation, setValidation] = useState<ValidationResult | null>(null);

  const [preview, setPreview] = useState<ScaffoldPreview | null>(null);
  const [generating, setGenerating] = useState(false);
  const [resultProjectId, setResultProjectId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      try {
        const [t, c] = await Promise.all([
          api.listCatalog(),
          api.listCatalogCategories(),
        ]);
        setAllTechs(t);
        setCategories(c);
      } catch (err) {
        setError(String(err));
      }
    }
    load();
  }, []);

  useEffect(() => {
    if (selectedTechs.length > 0) {
      api.validateStack(selectedTechs).then(setValidation).catch(() => {});
    } else {
      setValidation(null);
    }
  }, [selectedTechs]);

  function toggleTech(id: string) {
    setSelectedTechs((prev) =>
      prev.includes(id) ? prev.filter((t) => t !== id) : [...prev, id]
    );
  }

  async function handlePreview() {
    setError(null);
    try {
      const p = await api.previewScaffold({
        name,
        projectType,
        profile,
        technologies: selectedTechs,
        path,
      });
      setPreview(p);
      setStep("preview");
    } catch (err) {
      setError(String(err));
    }
  }

  async function handleGenerate() {
    setGenerating(true);
    setError(null);
    try {
      const project = await api.generateScaffold({
        name,
        projectType,
        profile,
        technologies: selectedTechs,
        path,
      });
      setResultProjectId(project.id);
      setStep("result");
    } catch (err) {
      setError(String(err));
    } finally {
      setGenerating(false);
    }
  }

  const categoryLabels = CATEGORY_LABELS;

  const canAdvanceToStack = name.trim() && path.trim();
  const canAdvanceToPreview = selectedTechs.length > 0 && (validation?.valid !== false);

  return (
    <div className="p-6 lg:p-8 max-w-4xl h-full overflow-y-auto">
      <h1 className="text-2xl font-bold text-white mb-2">Crear proyecto</h1>
      <p className="text-sm text-gray-500 mb-6">Genera un proyecto con scaffold personalizado</p>

      {error && (
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm mb-6">{error}</div>
      )}

      {/* Step indicators */}
      <div className="flex gap-2 mb-8">
        {(["info", "stack", "preview", "result"] as Step[]).map((s, i) => {
          const labels = ["Informacion", "Stack", "Preview", "Resultado"];
          const isActive = s === step;
          const isPast = ["info", "stack", "preview", "result"].indexOf(step) > i;
          return (
            <div key={s} className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium ${isActive ? "bg-primary-500/10 text-primary-500" : isPast ? "bg-gray-800 text-green-400" : "bg-gray-900 text-gray-600"}`}>
              <span className="w-5 h-5 rounded-full border flex items-center justify-center text-xs" style={{ borderColor: isActive ? "var(--color-primary-500)" : isPast ? "#4ade80" : "#374151" }}>
                {isPast ? "✓" : i + 1}
              </span>
              {labels[i]}
            </div>
          );
        })}
      </div>

      {/* Step 1: Info */}
      {step === "info" && (
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">Nombre del proyecto</label>
            <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="mi-proyecto"
              className="w-full px-3 py-2 rounded-lg bg-gray-900 border border-gray-800 text-white text-sm placeholder-gray-600 focus:outline-hidden focus:border-primary-500" autoFocus />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">Ruta</label>
            <PathInput
              value={path}
              onChange={setPath}
              placeholder="C:\\Users\\you\\projects\\mi-proyecto"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">Tipo de proyecto</label>
            <div className="grid grid-cols-3 gap-2">
              {PROJECT_TYPES.map((pt) => (
                <button key={pt.value} onClick={() => setProjectType(pt.value)}
                  className={`px-3 py-2 rounded-lg text-sm font-medium transition-colors ${projectType === pt.value ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-900 text-gray-400 border border-gray-800 hover:text-white"}`}>
                  {pt.label}
                </button>
              ))}
            </div>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">Perfil</label>
            <div className="space-y-2">
              {PROFILES.map((p) => (
                <button key={p.value} onClick={() => setProfile(p.value)}
                  className={`w-full text-left px-4 py-3 rounded-lg transition-colors ${profile === p.value ? "bg-primary-500/10 border border-primary-500/30" : "bg-gray-900 border border-gray-800 hover:border-gray-700"}`}>
                  <span className={`text-sm font-medium ${profile === p.value ? "text-primary-500" : "text-white"}`}>{p.label}</span>
                  <p className="text-xs text-gray-500 mt-0.5">{p.desc}</p>
                </button>
              ))}
            </div>
          </div>
          <div className="flex justify-end pt-4">
            <button onClick={() => setStep("stack")} disabled={!canAdvanceToStack}
              className="px-6 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors">
              Siguiente
            </button>
          </div>
        </div>
      )}

      {/* Step 2: Stack selection */}
      {step === "stack" && (
        <div className="space-y-6">
          {/* Validation feedback */}
          {validation && !validation.valid && (
            <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20">
              {validation.issues.filter((i) => i.level === "error").map((issue, idx) => (
                <p key={idx} className="text-sm text-red-400">{issue.message}</p>
              ))}
            </div>
          )}
          {validation && validation.issues.filter((i) => i.level === "warning").length > 0 && (
            <div className="px-4 py-3 rounded-lg bg-yellow-500/10 border border-yellow-500/20">
              {validation.issues.filter((i) => i.level === "warning").map((issue, idx) => (
                <p key={idx} className="text-sm text-yellow-400">{issue.message}</p>
              ))}
            </div>
          )}
          {validation && validation.suggestions.length > 0 && (
            <div className="px-3 py-2 rounded-lg bg-blue-500/10 border border-blue-500/20">
              <span className="text-xs text-blue-400">Sugerencias: {validation.suggestions.join(", ")}</span>
            </div>
          )}

          {/* Selected count */}
          <div className="flex items-center justify-between">
            <span className="text-sm text-gray-400">{selectedTechs.length} tecnologia(s) seleccionada(s)</span>
            {selectedTechs.length > 0 && (
              <div className="flex flex-wrap gap-1">
                {selectedTechs.map((id) => (
                  <span key={id} className="px-2 py-0.5 rounded bg-primary-500/10 text-primary-500 text-xs">{id}</span>
                ))}
              </div>
            )}
          </div>

          {/* Tech grid by category */}
          {categories.map((cat) => {
            const catTechs = allTechs.filter((t) => t.category === cat);
            if (catTechs.length === 0) return null;
            return (
              <div key={cat}>
                <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">
                  {categoryLabels[cat] || cat}
                </h3>
                <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-2">
                  {catTechs.map((tech) => {
                    const isSelected = selectedTechs.includes(tech.id);
                    return (
                      <button key={tech.id} onClick={() => toggleTech(tech.id)}
                        className={`text-left px-3 py-2 rounded-lg text-sm transition-colors ${isSelected ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-900 text-gray-300 border border-gray-800 hover:border-gray-700"}`}>
                        <span className="font-medium">{tech.name}</span>
                        {tech.defaultPort > 0 && <span className="text-xs text-gray-600 ml-1">:{tech.defaultPort}</span>}
                      </button>
                    );
                  })}
                </div>
              </div>
            );
          })}

          <div className="flex justify-between pt-4">
            <button onClick={() => setStep("info")} className="px-4 py-2 rounded-lg bg-dlx-light-3 text-dlx-text-light-1 border border-dlx-text-dark-3 text-sm font-medium hover:bg-dlx-text-dark-3 transition-colors">Atras</button>
            <button onClick={handlePreview} disabled={!canAdvanceToPreview}
              className="px-6 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors">
              Preview
            </button>
          </div>
        </div>
      )}

      {/* Step 3: Preview */}
      {step === "preview" && preview && (
        <div className="space-y-6">
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
              <p className="text-xs text-gray-500">Nombre</p>
              <p className="text-sm font-medium text-white">{name}</p>
            </div>
            <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
              <p className="text-xs text-gray-500">Tipo</p>
              <p className="text-sm font-medium text-white">{projectType}</p>
            </div>
            <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
              <p className="text-xs text-gray-500">Perfil</p>
              <p className="text-sm font-medium text-white">{profile}</p>
            </div>
            <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
              <p className="text-xs text-gray-500">Archivos</p>
              <p className="text-sm font-medium text-white">{preview.files.length}</p>
            </div>
          </div>

          {/* Files to create */}
          <div>
            <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Archivos a generar</h3>
            <div className="space-y-1">
              {preview.files.map((f) => (
                <div key={f.path} className="flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-900 border border-gray-800">
                  <span className="text-green-400 text-xs">+</span>
                  <span className="text-sm text-white font-mono">{f.path}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Validation */}
          {preview.validation.issues.length > 0 && (
            <div>
              <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Validacion</h3>
              {preview.validation.issues.map((issue, idx) => (
                <p key={idx} className={`text-sm ${issue.level === "error" ? "text-red-400" : issue.level === "warning" ? "text-yellow-400" : "text-blue-400"}`}>
                  {issue.message}
                </p>
              ))}
            </div>
          )}

          {/* Port assignments */}
          {Object.keys(preview.validation.portAssignments).length > 0 && (
            <div>
              <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Puertos asignados</h3>
              <div className="flex flex-wrap gap-3">
                {Object.entries(preview.validation.portAssignments).map(([tech, port]) => (
                  <span key={tech} className="text-sm text-gray-300">{tech}: <span className="text-primary-500 font-mono">:{port}</span></span>
                ))}
              </div>
            </div>
          )}

          <div className="flex justify-between pt-4">
            <button onClick={() => setStep("stack")} className="px-4 py-2 rounded-lg bg-dlx-light-3 text-dlx-text-light-1 border border-dlx-text-dark-3 text-sm font-medium hover:bg-dlx-text-dark-3 transition-colors">Atras</button>
            <button onClick={handleGenerate} disabled={generating}
              className="px-6 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors">
              {generating ? "Generando..." : "Generar proyecto"}
            </button>
          </div>
        </div>
      )}

      {/* Step 4: Result */}
      {step === "result" && (
        <div className="text-center py-12">
          <div className="w-16 h-16 rounded-2xl bg-green-500/10 border border-green-500/20 flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
            </svg>
          </div>
          <h2 className="text-xl font-bold text-white mb-2">Proyecto generado</h2>
          <p className="text-sm text-gray-500 mb-1">{name}</p>
          <p className="text-xs text-gray-600 font-mono mb-6">{path}</p>
          <div className="flex gap-3 justify-center">
            {resultProjectId && (
              <button onClick={() => navigate(`/project/${resultProjectId}`)}
                className="px-4 py-2 rounded-lg bg-info/10 text-info-light text-sm font-medium hover:bg-info/20 transition-colors">
                Ver proyecto
              </button>
            )}
            <button onClick={() => navigate("/")}
              className="px-4 py-2 rounded-lg bg-dlx-light-3 text-dlx-text-light-1 border border-dlx-text-dark-3 text-sm font-medium hover:bg-dlx-text-dark-3 transition-colors">
              Ir al dashboard
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
