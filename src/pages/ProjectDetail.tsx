import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import * as api from "@/lib/tauri";
import type { Project } from "@/types/project";
import { useProjectsStore } from "@/stores/projects";

export default function ProjectDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
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

  useEffect(() => {
    if (!id) return;
    loadProject();
  }, [id]);

  async function loadProject() {
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
    try {
      await api.openProject(project.id);
    } catch (err) {
      setError(String(err));
    }
  }

  async function handleOpenTerminal() {
    if (!project) return;
    try {
      await api.openTerminal(project.id);
    } catch (err) {
      setError(String(err));
    }
  }

  async function handleDelete() {
    if (!id) return;
    try {
      await removeProject(id);
      navigate("/");
    } catch (err) {
      setError(String(err));
    }
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
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
          {error}
        </div>
        <button
          onClick={() => navigate("/")}
          className="mt-4 text-sm text-primary-500 hover:underline"
        >
          Volver al dashboard
        </button>
      </div>
    );
  }

  if (!project) return null;

  return (
    <div className="p-6 lg:p-8 max-w-4xl">
      {/* Back */}
      <button
        onClick={() => navigate("/")}
        className="flex items-center gap-1 text-sm text-gray-500 hover:text-white transition-colors mb-6"
      >
        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
          <path strokeLinecap="round" strokeLinejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
        </svg>
        Volver
      </button>

      {/* Header */}
      <div className="flex items-start justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold text-white">{project.name}</h1>
          {project.description && (
            <p className="text-gray-500 mt-1">{project.description}</p>
          )}
          <p className="text-xs text-gray-600 font-mono mt-2">{project.path}</p>
        </div>
        <div className="flex gap-2">
          <button
            onClick={handleOpenInEditor}
            className="flex items-center gap-2 px-3 py-2 rounded-lg bg-primary-500 text-white text-sm font-medium hover:bg-primary-600 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
            </svg>
            Abrir en VSCode
          </button>
          <button
            onClick={handleOpenTerminal}
            className="flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-800 text-gray-300 text-sm font-medium hover:bg-gray-700 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z" />
            </svg>
            Terminal
          </button>
        </div>
      </div>

      {error && (
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm mb-6">
          {error}
        </div>
      )}

      {/* Info Cards */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <InfoCard label="Estado" value={project.status} />
        <InfoCard
          label="Creado"
          value={new Date(project.createdAt).toLocaleDateString("es")}
        />
        <InfoCard
          label="Ultimo acceso"
          value={
            project.lastOpenedAt
              ? new Date(project.lastOpenedAt).toLocaleDateString("es")
              : "Nunca"
          }
        />
        <InfoCard
          label="Runtimes"
          value={
            project.runtimes.length > 0
              ? project.runtimes.map((r) => r.runtime).join(", ")
              : "Ninguno"
          }
        />
      </div>

      {/* Runtimes */}
      {project.runtimes.length > 0 && (
        <section className="mb-8">
          <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">
            Runtimes del proyecto
          </h2>
          <div className="flex flex-wrap gap-2">
            {project.runtimes.map((rt) => (
              <span
                key={rt.runtime}
                className="px-3 py-1.5 rounded-lg bg-gray-900 border border-gray-800 text-sm text-gray-300"
              >
                {rt.runtime}
                {rt.version && (
                  <span className="text-gray-600 ml-1">v{rt.version}</span>
                )}
              </span>
            ))}
          </div>
        </section>
      )}

      {/* Tags */}
      {project.tags.length > 0 && (
        <section className="mb-8">
          <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">
            Tags
          </h2>
          <div className="flex flex-wrap gap-2">
            {project.tags.map((tag) => (
              <span
                key={tag}
                className="px-2 py-1 rounded-md bg-gray-900 text-sm text-gray-500"
              >
                {tag}
              </span>
            ))}
          </div>
        </section>
      )}

      {/* Environment Variables */}
      <section className="mb-8">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider">
            Variables de entorno
          </h2>
          <div className="flex items-center gap-2">
            {envSaved && (
              <span className="text-xs text-green-400">Guardado</span>
            )}
            <button
              onClick={handleSaveEnvVars}
              disabled={envSaving}
              className="px-3 py-1 rounded-md bg-primary-500/10 text-primary-500 text-xs font-medium hover:bg-primary-500/20 disabled:opacity-50 transition-colors"
            >
              {envSaving ? "Guardando..." : "Guardar"}
            </button>
          </div>
        </div>

        <div className="rounded-xl bg-gray-900 border border-gray-800 overflow-hidden">
          {/* Existing vars */}
          {Object.entries(envVars).length > 0 ? (
            <div className="divide-y divide-gray-800">
              {Object.entries(envVars).map(([key, value]) => (
                <div key={key} className="flex items-center gap-2 px-4 py-2">
                  <span className="text-sm font-mono text-primary-500 w-40 flex-shrink-0 truncate">
                    {key}
                  </span>
                  <input
                    type="text"
                    value={value}
                    onChange={(e) => handleEnvValueChange(key, e.target.value)}
                    className="flex-1 px-2 py-1 rounded bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 focus:outline-none focus:border-primary-500/50"
                  />
                  <button
                    onClick={() => handleRemoveEnvVar(key)}
                    className="p-1 rounded text-gray-600 hover:text-red-400 transition-colors"
                  >
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              ))}
            </div>
          ) : (
            <div className="px-4 py-6 text-center text-sm text-gray-600">
              No hay variables de entorno configuradas
            </div>
          )}

          {/* Add new */}
          <div className="flex items-center gap-2 px-4 py-3 border-t border-gray-800 bg-gray-900/50">
            <input
              type="text"
              value={newKey}
              onChange={(e) => setNewKey(e.target.value)}
              placeholder="CLAVE"
              className="w-40 px-2 py-1 rounded bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 placeholder-gray-600 focus:outline-none focus:border-primary-500/50"
              onKeyDown={(e) => e.key === "Enter" && handleAddEnvVar()}
            />
            <input
              type="text"
              value={newValue}
              onChange={(e) => setNewValue(e.target.value)}
              placeholder="valor"
              className="flex-1 px-2 py-1 rounded bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 placeholder-gray-600 focus:outline-none focus:border-primary-500/50"
              onKeyDown={(e) => e.key === "Enter" && handleAddEnvVar()}
            />
            <button
              onClick={handleAddEnvVar}
              className="px-3 py-1 rounded-md bg-gray-800 text-gray-400 text-sm hover:text-white hover:bg-gray-700 transition-colors"
            >
              Agregar
            </button>
          </div>
        </div>
      </section>

      {/* Danger Zone */}
      <section className="rounded-xl border border-red-500/20 bg-red-500/5 p-6">
        <h2 className="text-sm font-semibold text-red-400 uppercase tracking-wider mb-2">
          Zona de peligro
        </h2>
        <p className="text-sm text-gray-500 mb-4">
          Eliminar el proyecto del registro de Delixon. Los archivos del disco
          no se borran.
        </p>
        {!confirmDelete ? (
          <button
            onClick={() => setConfirmDelete(true)}
            className="px-4 py-2 rounded-lg border border-red-500/30 text-red-400 text-sm font-medium hover:bg-red-500/10 transition-colors"
          >
            Eliminar proyecto
          </button>
        ) : (
          <div className="flex items-center gap-3">
            <span className="text-sm text-red-400">Confirmar eliminacion?</span>
            <button
              onClick={handleDelete}
              className="px-4 py-2 rounded-lg bg-red-500 text-white text-sm font-medium hover:bg-red-600 transition-colors"
            >
              Si, eliminar
            </button>
            <button
              onClick={() => setConfirmDelete(false)}
              className="px-4 py-2 rounded-lg text-sm text-gray-500 hover:text-white transition-colors"
            >
              Cancelar
            </button>
          </div>
        )}
      </section>
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
