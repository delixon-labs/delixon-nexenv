import { useEffect, useRef, useState } from "react";
import { useProjectsStore } from "@/stores/projects";
import ProjectCard from "@/components/dashboard/ProjectCard";
import CreateProjectModal from "@/components/dashboard/CreateProjectModal";
import ImportProjectModal from "@/components/dashboard/ImportProjectModal";
import RegisterProjectModal from "@/components/dashboard/RegisterProjectModal";

export default function Dashboard() {
  const { projects, isLoading, error, searchQuery, fetchProjects, setSearchQuery } =
    useProjectsStore();
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showRegisterModal, setShowRegisterModal] = useState(false);
  const [importError, setImportError] = useState<string | null>(null);
  const [importFileContent, setImportFileContent] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  async function handleImportFile(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    setImportError(null);

    try {
      const json = await file.text();
      setImportFileContent(json);
    } catch (err) {
      setImportError(String(err));
    } finally {
      if (fileInputRef.current) fileInputRef.current.value = "";
    }
  }

  useEffect(() => {
    fetchProjects();
  }, [fetchProjects]);

  const filteredProjects = projects.filter((p) => {
    if (!searchQuery) return true;
    const q = searchQuery.toLowerCase();
    return (
      p.name.toLowerCase().includes(q) ||
      p.description?.toLowerCase().includes(q) ||
      p.tags.some((t) => t.toLowerCase().includes(q)) ||
      p.runtimes.some((r) => r.runtime.toLowerCase().includes(q))
    );
  });

  // Ordenar por ultimo acceso (mas reciente primero)
  const sortedProjects = [...filteredProjects].sort((a, b) => {
    const aDate = a.lastOpenedAt || a.createdAt;
    const bDate = b.lastOpenedAt || b.createdAt;
    return bDate.localeCompare(aDate);
  });

  return (
    <div className="p-6 lg:p-8 h-full overflow-y-auto">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-white">Proyectos</h1>
          <p className="text-sm text-gray-500 mt-1">
            {projects.length === 0
              ? "Registra tu primer proyecto para empezar"
              : `${projects.length} proyecto${projects.length !== 1 ? "s" : ""} registrado${projects.length !== 1 ? "s" : ""}`}
          </p>
        </div>
        <div className="flex gap-2">
          <button
            onClick={() => setShowRegisterModal(true)}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-info/10 text-info-light text-sm font-medium hover:bg-info/20 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
            </svg>
            Registrar existente
          </button>
          <button
            onClick={() => fileInputRef.current?.click()}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-info/10 text-info-light text-sm font-medium hover:bg-info/20 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5" />
            </svg>
            Importar
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".nexenv"
            onChange={handleImportFile}
            className="hidden"
          />
          <button
            onClick={() => setShowCreateModal(true)}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
            Nuevo proyecto
          </button>
        </div>
      </div>

      {importError && (
        <div className="mb-4 px-3 py-2 rounded-lg bg-error/10 border border-error/20 text-error-light text-sm">
          Error importando: {importError}
        </div>
      )}

      {/* Search */}
      {projects.length > 0 && (
        <div className="mb-6">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Buscar por nombre, tag, runtime..."
            className="w-full max-w-md px-4 py-2 rounded-lg bg-gray-900 border border-gray-800 text-white placeholder-gray-600 focus:outline-hidden focus:border-primary-500/50 text-sm"
          />
        </div>
      )}

      {/* Loading */}
      {isLoading && (
        <div className="flex items-center justify-center py-20">
          <div className="w-8 h-8 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" />
        </div>
      )}

      {/* Error */}
      {error && (
        <div className="px-4 py-3 rounded-lg bg-error/10 border border-error/20 text-error-light text-sm mb-6">
          {error}
        </div>
      )}

      {/* Empty state */}
      {!isLoading && !error && projects.length === 0 && (
        <div className="flex flex-col items-center justify-center py-20 text-center">
          <div className="w-16 h-16 rounded-2xl bg-gray-900 border border-gray-800 flex items-center justify-center mb-4">
            <svg className="w-8 h-8 text-gray-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
            </svg>
          </div>
          <h3 className="text-lg font-semibold text-white mb-1">
            Sin proyectos
          </h3>
          <p className="text-sm text-gray-500 mb-6 max-w-sm">
            Registra un proyecto existente o crea uno nuevo desde una plantilla.
            Nexenv gestionara su entorno, variables y configuracion.
          </p>
          <button
            onClick={() => setShowCreateModal(true)}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
            Crear primer proyecto
          </button>
        </div>
      )}

      {/* No results */}
      {!isLoading && projects.length > 0 && sortedProjects.length === 0 && (
        <div className="text-center py-12">
          <p className="text-gray-500">
            No se encontraron proyectos para "{searchQuery}"
          </p>
        </div>
      )}

      {/* Project grid */}
      {sortedProjects.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-4">
          {sortedProjects.map((project) => (
            <ProjectCard key={project.id} project={project} />
          ))}
        </div>
      )}

      {/* Create modal */}
      <CreateProjectModal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
      />

      {/* Register modal */}
      <RegisterProjectModal
        isOpen={showRegisterModal}
        onClose={() => setShowRegisterModal(false)}
        onSuccess={() => fetchProjects()}
      />

      {/* Import modal */}
      {importFileContent && (
        <ImportProjectModal
          isOpen={true}
          onClose={() => setImportFileContent(null)}
          fileContent={importFileContent}
          onSuccess={() => fetchProjects()}
        />
      )}
    </div>
  );
}
