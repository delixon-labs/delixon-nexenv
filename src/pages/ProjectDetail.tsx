import { useCallback, useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import * as api from "@/lib/tauri";
import type { Project, Runtime, RuntimeConfig } from "@/types/project";
import { useProjectsStore } from "@/stores/projects";
import ProjectTabs from "@/components/project/ProjectTabs";
import type { TabDefinition } from "@/components/project/ProjectTabs";
import HealthTab from "@/components/project/tabs/HealthTab";
import GitTab from "@/components/project/tabs/GitTab";
import DockerTab from "@/components/project/tabs/DockerTab";
import ScriptsTab from "@/components/project/tabs/ScriptsTab";
import ProcessesTab from "@/components/project/tabs/ProcessesTab";
import NotesTab from "@/components/project/tabs/NotesTab";
import ManifestTab from "@/components/project/tabs/ManifestTab";
import VersioningTab from "@/components/project/tabs/VersioningTab";
import RecipesTab from "@/components/project/tabs/RecipesTab";

const AVAILABLE_RUNTIMES: { value: Runtime; label: string }[] = [
  { value: "node", label: "Node.js" },
  { value: "python", label: "Python" },
  { value: "rust", label: "Rust" },
  { value: "go", label: "Go" },
  { value: "dotnet", label: ".NET" },
  { value: "php", label: "PHP" },
  { value: "ruby", label: "Ruby" },
];

const PROJECT_TABS: TabDefinition[] = [
  { id: "health", label: "Health", component: HealthTab },
  { id: "git", label: "Git", component: GitTab },
  { id: "scripts", label: "Scripts", component: ScriptsTab },
  { id: "docker", label: "Docker", component: DockerTab },
  { id: "manifest", label: "Manifest", component: ManifestTab },
  { id: "recipes", label: "Recipes", component: RecipesTab },
  { id: "versioning", label: "Versiones", component: VersioningTab },
  { id: "processes", label: "Procesos", component: ProcessesTab },
  { id: "notes", label: "Notas", component: NotesTab },
];

export default function ProjectDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { t } = useTranslation();
  const { removeProject } = useProjectsStore();

  const [project, setProject] = useState<Project | null>(null);
  const [envVars, setEnvVars] = useState<Record<string, string>>({});
  const [newKey, setNewKey] = useState("");
  const [newValue, setNewValue] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [envSaving, setEnvSaving] = useState(false);
  const [envSaved, setEnvSaved] = useState(false);
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [workspaceSaved, setWorkspaceSaved] = useState(false);

  const [isEditing, setIsEditing] = useState(false);
  const [editName, setEditName] = useState("");
  const [editDescription, setEditDescription] = useState("");
  const [editRuntimes, setEditRuntimes] = useState<Runtime[]>([]);
  const [editTags, setEditTags] = useState("");
  const [editSaving, setEditSaving] = useState(false);

  const [activeSection, setActiveSection] = useState<"general" | "tabs">("tabs");

  const loadProject = useCallback(async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const [proj, vars] = await Promise.all([
        api.getProject(id),
        api.getEnvVars(id),
      ]);
      setProject(proj);
      setEnvVars(vars);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, [id]);

  useEffect(() => {
    if (!id) return;
    loadProject();
  }, [id, loadProject]);

  function startEditing() {
    if (!project) return;
    setEditName(project.name);
    setEditDescription(project.description || "");
    setEditRuntimes(project.runtimes.map((r) => r.runtime as Runtime));
    setEditTags(project.tags.join(", "));
    setIsEditing(true);
  }

  async function handleSaveEdit() {
    if (!id || !project) return;
    setEditSaving(true);
    try {
      const runtimes: RuntimeConfig[] = editRuntimes.map((rt) => ({
        runtime: rt,
        version: project.runtimes.find((r) => r.runtime === rt)?.version || "",
      }));
      const tags = editTags.split(",").map((t) => t.trim()).filter(Boolean);
      const updated = await api.updateProject(id, {
        name: editName.trim(),
        description: editDescription.trim() || undefined,
        runtimes,
        tags,
      });
      setProject(updated);
      setIsEditing(false);
    } catch (err) {
      setError(String(err));
    } finally {
      setEditSaving(false);
    }
  }

  function toggleEditRuntime(rt: Runtime) {
    setEditRuntimes((prev) =>
      prev.includes(rt) ? prev.filter((r) => r !== rt) : [...prev, rt]
    );
  }

  async function handleSaveEnvVars() {
    if (!id) return;
    setEnvSaving(true);
    try {
      await api.setEnvVars(id, envVars);
      setEnvSaved(true);
      setTimeout(() => setEnvSaved(false), 2000);
    } catch (err) {
      setError(String(err));
    } finally {
      setEnvSaving(false);
    }
  }

  function handleAddEnvVar() {
    if (!newKey.trim()) return;
    setEnvVars((prev) => ({ ...prev, [newKey.trim()]: newValue }));
    setNewKey("");
    setNewValue("");
  }

  function handleRemoveEnvVar(key: string) {
    setEnvVars((prev) => {
      const next = { ...prev };
      delete next[key];
      return next;
    });
  }

  function handleEnvValueChange(key: string, value: string) {
    setEnvVars((prev) => ({ ...prev, [key]: value }));
  }

  async function handleOpenInEditor() {
    if (!project) return;
    try { await api.openProject(project.id); } catch (err) { setError(String(err)); }
  }

  async function handleOpenTerminal() {
    if (!project) return;
    try { await api.openTerminal(project.id); } catch (err) { setError(String(err)); }
  }

  async function handleExport() {
    if (!project) return;
    try {
      const json = await api.exportProject(project.id);
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `${project.name}.delixon`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (err) { setError(String(err)); }
  }

  async function handleGenerateWorkspace() {
    if (!project) return;
    try {
      await api.generateVscodeWorkspace(project.id);
      setWorkspaceSaved(true);
      setTimeout(() => setWorkspaceSaved(false), 2000);
    } catch (err) { setError(String(err)); }
  }

  async function handleDelete() {
    if (!id) return;
    try { await removeProject(id); navigate("/"); } catch (err) { setError(String(err)); }
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="w-8 h-8 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" />
      </div>
    );
  }

  if (error && !project) {
    return (
      <div className="p-8">
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
        <button onClick={() => navigate("/")} className="mt-4 text-sm text-primary-500 hover:underline">Volver al dashboard</button>
      </div>
    );
  }

  if (!project) return null;

  return (
    <div className="p-6 lg:p-8 max-w-5xl h-full overflow-y-auto">
      {/* Back */}
      <button onClick={() => navigate("/")} className="flex items-center gap-1 text-sm text-gray-500 hover:text-white transition-colors mb-6">
        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
          <path strokeLinecap="round" strokeLinejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
        </svg>
        Volver
      </button>

      {/* Header */}
      <div className="flex items-start justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-white">{project.name}</h1>
          {project.description && <p className="text-gray-500 mt-1">{project.description}</p>}
          <p className="text-xs text-gray-600 font-mono mt-2">{project.path}</p>
        </div>
        <div className="flex gap-2 flex-wrap justify-end">
          <button onClick={handleOpenInEditor} className="flex items-center gap-2 px-3 py-2 rounded-lg bg-primary-500 text-white text-sm font-medium hover:bg-primary-600 transition-colors">
            Editor
          </button>
          <button onClick={handleOpenTerminal} className="flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-800 text-gray-300 text-sm font-medium hover:bg-gray-700 transition-colors">
            Terminal
          </button>
          <button onClick={handleGenerateWorkspace} className="px-3 py-2 rounded-lg bg-gray-800 text-gray-300 text-sm font-medium hover:bg-gray-700 transition-colors">
            {workspaceSaved ? "Generado" : "Workspace"}
          </button>
          <button onClick={handleExport} className="px-3 py-2 rounded-lg bg-gray-800 text-gray-300 text-sm font-medium hover:bg-gray-700 transition-colors">
            Exportar
          </button>
        </div>
      </div>

      {error && (
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm mb-6">{error}</div>
      )}

      {/* Section switcher */}
      <div className="flex gap-2 mb-6">
        <button
          onClick={() => setActiveSection("tabs")}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${activeSection === "tabs" ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-800 text-gray-400 hover:text-white"}`}
        >
          Panel
        </button>
        <button
          onClick={() => setActiveSection("general")}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${activeSection === "general" ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-800 text-gray-400 hover:text-white"}`}
        >
          General
        </button>
      </div>

      {/* Tabs section */}
      {activeSection === "tabs" && (
        <ProjectTabs tabs={PROJECT_TABS} projectId={project.id} projectPath={project.path} />
      )}

      {/* General section */}
      {activeSection === "general" && (
        <>
          {/* Info Cards */}
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
            <InfoCard label="Estado" value={project.status} />
            <InfoCard label="Creado" value={new Date(project.createdAt).toLocaleDateString("es")} />
            <InfoCard label="Ultimo acceso" value={project.lastOpenedAt ? new Date(project.lastOpenedAt).toLocaleDateString("es") : "Nunca"} />
            <InfoCard label="Runtimes" value={project.runtimes.length > 0 ? project.runtimes.map((r) => r.runtime).join(", ") : "Ninguno"} />
          </div>

          {/* Runtimes */}
          {project.runtimes.length > 0 && (
            <section className="mb-8">
              <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">Runtimes</h2>
              <div className="flex flex-wrap gap-2">
                {project.runtimes.map((rt) => (
                  <span key={rt.runtime} className="px-3 py-1.5 rounded-lg bg-gray-900 border border-gray-800 text-sm text-gray-300">
                    {rt.runtime}{rt.version && <span className="text-gray-600 ml-1">v{rt.version}</span>}
                  </span>
                ))}
              </div>
            </section>
          )}

          {/* Tags */}
          {project.tags.length > 0 && (
            <section className="mb-8">
              <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">Tags</h2>
              <div className="flex flex-wrap gap-2">
                {project.tags.map((tag) => (
                  <span key={tag} className="px-2 py-1 rounded-md bg-gray-900 text-sm text-gray-500">{tag}</span>
                ))}
              </div>
            </section>
          )}

          {/* Edit button */}
          <section className="mb-8">
            <button onClick={startEditing} className="px-4 py-2 rounded-lg bg-gray-800 text-gray-300 text-sm font-medium hover:bg-gray-700 transition-colors">
              Editar proyecto
            </button>
          </section>

          {/* Edit Form */}
          {isEditing && (
            <section className="mb-8 rounded-xl bg-gray-900 border border-primary-500/30 p-6">
              <h2 className="text-sm font-semibold text-primary-500 uppercase tracking-wider mb-4">Editar proyecto</h2>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-1">Nombre</label>
                  <input type="text" value={editName} onChange={(e) => setEditName(e.target.value)} className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm focus:outline-none focus:border-primary-500" />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-1">Descripcion</label>
                  <textarea value={editDescription} onChange={(e) => setEditDescription(e.target.value)} rows={2} className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm resize-none focus:outline-none focus:border-primary-500" />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">Runtimes</label>
                  <div className="flex flex-wrap gap-2">
                    {AVAILABLE_RUNTIMES.map(({ value, label }) => (
                      <button key={value} type="button" onClick={() => toggleEditRuntime(value)}
                        className={`px-3 py-1 rounded-lg text-sm font-medium border transition-colors ${editRuntimes.includes(value) ? "bg-primary-500/10 text-primary-500 border-primary-500/30" : "bg-gray-800 text-gray-500 border-gray-700 hover:text-gray-300"}`}>
                        {label}
                      </button>
                    ))}
                  </div>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-1">Tags <span className="text-gray-600">(separados por coma)</span></label>
                  <input type="text" value={editTags} onChange={(e) => setEditTags(e.target.value)} className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm focus:outline-none focus:border-primary-500" />
                </div>
                <div className="flex gap-3 pt-2">
                  <button onClick={handleSaveEdit} disabled={editSaving} className="px-4 py-2 rounded-lg bg-primary-500 text-white text-sm font-medium hover:bg-primary-600 disabled:opacity-50 transition-colors">
                    {editSaving ? "Guardando..." : "Guardar"}
                  </button>
                  <button onClick={() => setIsEditing(false)} className="px-4 py-2 rounded-lg text-sm text-gray-400 hover:text-white transition-colors">Cancelar</button>
                </div>
              </div>
            </section>
          )}

          {/* Environment Variables */}
          <section className="mb-8">
            <div className="flex items-center justify-between mb-3">
              <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider">Variables de entorno</h2>
              <div className="flex items-center gap-2">
                {envSaved && <span className="text-xs text-green-400">Guardado</span>}
                <button onClick={handleSaveEnvVars} disabled={envSaving} className="px-3 py-1 rounded-md bg-primary-500/10 text-primary-500 text-xs font-medium hover:bg-primary-500/20 disabled:opacity-50 transition-colors">
                  {envSaving ? "Guardando..." : "Guardar"}
                </button>
              </div>
            </div>
            <div className="rounded-xl bg-gray-900 border border-gray-800 overflow-hidden">
              {Object.entries(envVars).length > 0 ? (
                <div className="divide-y divide-gray-800">
                  {Object.entries(envVars).map(([key, value]) => (
                    <div key={key} className="flex items-center gap-2 px-4 py-2">
                      <span className="text-sm font-mono text-primary-500 w-40 flex-shrink-0 truncate">{key}</span>
                      <input type="text" value={value} onChange={(e) => handleEnvValueChange(key, e.target.value)}
                        className="flex-1 px-2 py-1 rounded bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 focus:outline-none focus:border-primary-500/50" />
                      <button onClick={() => handleRemoveEnvVar(key)} className="p-1 rounded text-gray-600 hover:text-red-400 transition-colors">
                        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}><path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" /></svg>
                      </button>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="px-4 py-6 text-center text-sm text-gray-600">No hay variables de entorno</div>
              )}
              <div className="flex items-center gap-2 px-4 py-3 border-t border-gray-800 bg-gray-900/50">
                <input type="text" value={newKey} onChange={(e) => setNewKey(e.target.value)} placeholder="CLAVE"
                  className="w-40 px-2 py-1 rounded bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 placeholder-gray-600 focus:outline-none focus:border-primary-500/50"
                  onKeyDown={(e) => e.key === "Enter" && handleAddEnvVar()} />
                <input type="text" value={newValue} onChange={(e) => setNewValue(e.target.value)} placeholder="valor"
                  className="flex-1 px-2 py-1 rounded bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 placeholder-gray-600 focus:outline-none focus:border-primary-500/50"
                  onKeyDown={(e) => e.key === "Enter" && handleAddEnvVar()} />
                <button onClick={handleAddEnvVar} className="px-3 py-1 rounded-md bg-gray-800 text-gray-400 text-sm hover:text-white hover:bg-gray-700 transition-colors">Agregar</button>
              </div>
            </div>
          </section>

          {/* Unlink */}
          <section className="rounded-xl border border-gray-700/50 bg-gray-900 p-6">
            <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">{t("project.unlink")}</h2>
            <p className="text-sm text-gray-500 mb-4">{t("project.unlinkDesc")}</p>
            {!confirmDelete ? (
              <button onClick={() => setConfirmDelete(true)} className="px-4 py-2 rounded-lg border border-gray-600 text-gray-400 text-sm font-medium hover:bg-gray-800 transition-colors">
                {t("project.unlinkButton")}
              </button>
            ) : (
              <div className="flex items-center gap-3">
                <span className="text-sm text-amber-400">{t("project.unlinkConfirm")}</span>
                <button onClick={handleDelete} className="px-4 py-2 rounded-lg bg-amber-500 text-white text-sm font-medium hover:bg-amber-600 transition-colors">{t("project.unlinkYes")}</button>
                <button onClick={() => setConfirmDelete(false)} className="px-4 py-2 rounded-lg text-sm text-gray-500 hover:text-white transition-colors">{t("common.cancel")}</button>
              </div>
            )}
          </section>
        </>
      )}
    </div>
  );
}

function InfoCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
      <p className="text-xs text-gray-500 mb-1">{label}</p>
      <p className="text-sm font-medium text-white capitalize">{value}</p>
    </div>
  );
}
