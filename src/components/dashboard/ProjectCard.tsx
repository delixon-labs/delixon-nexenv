import { useNavigate } from "react-router-dom";
import type { Project } from "@/types/project";
import { useProjectsStore } from "@/stores/projects";
import * as api from "@/lib/tauri";

const RUNTIME_COLORS: Record<string, string> = {
  node: "bg-green-500/10 text-green-400 border-green-500/20",
  python: "bg-yellow-500/10 text-yellow-400 border-yellow-500/20",
  rust: "bg-orange-500/10 text-orange-400 border-orange-500/20",
  go: "bg-cyan-500/10 text-cyan-400 border-cyan-500/20",
  dotnet: "bg-purple-500/10 text-purple-400 border-purple-500/20",
  php: "bg-indigo-500/10 text-indigo-400 border-indigo-500/20",
  ruby: "bg-red-500/10 text-red-400 border-red-500/20",
};

const STATUS_LABELS: Record<string, { label: string; color: string }> = {
  active: { label: "Activo", color: "bg-green-500" },
  idle: { label: "Inactivo", color: "bg-yellow-500" },
  archived: { label: "Archivado", color: "bg-gray-500" },
};

interface ProjectCardProps {
  project: Project;
}

export default function ProjectCard({ project }: ProjectCardProps) {
  const navigate = useNavigate();
  const { openProject } = useProjectsStore();

  const status = STATUS_LABELS[project.status] || STATUS_LABELS.active;

  const timeAgo = formatTimeAgo(project.lastOpenedAt || project.createdAt);

  async function handleOpen(e: React.MouseEvent) {
    e.stopPropagation();
    try {
      await openProject(project.id);
    } catch (err) {
      console.error("Error abriendo proyecto:", err);
    }
  }

  async function handleTerminal(e: React.MouseEvent) {
    e.stopPropagation();
    try {
      await api.openTerminal(project.id);
    } catch (err) {
      console.error("Error abriendo terminal:", err);
    }
  }

  return (
    <div
      onClick={() => navigate(`/project/${project.id}`)}
      className="group relative flex flex-col gap-3 p-5 rounded-xl bg-gray-900 border border-gray-800 hover:border-primary-500/40 hover:bg-gray-900/80 transition-all duration-200 cursor-pointer"
    >
      {/* Header */}
      <div className="flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <h3 className="text-base font-semibold text-white truncate group-hover:text-primary-500 transition-colors">
            {project.name}
          </h3>
          {project.description && (
            <p className="mt-1 text-sm text-gray-500 line-clamp-2">
              {project.description}
            </p>
          )}
        </div>
        <div className="flex items-center gap-1.5 ml-3">
          <span className={`w-2 h-2 rounded-full ${status.color}`} />
          <span className="text-xs text-gray-500">{status.label}</span>
        </div>
      </div>

      {/* Runtimes */}
      {project.runtimes.length > 0 && (
        <div className="flex flex-wrap gap-1.5">
          {project.runtimes.map((rt) => (
            <span
              key={rt.runtime}
              className={`inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium border ${RUNTIME_COLORS[rt.runtime] || "bg-gray-800 text-gray-400 border-gray-700"}`}
            >
              {rt.runtime} {rt.version}
            </span>
          ))}
        </div>
      )}

      {/* Tags */}
      {project.tags.length > 0 && (
        <div className="flex flex-wrap gap-1">
          {project.tags.map((tag) => (
            <span
              key={tag}
              className="px-1.5 py-0.5 rounded text-xs text-gray-500 bg-gray-800"
            >
              {tag}
            </span>
          ))}
        </div>
      )}

      {/* Footer */}
      <div className="flex items-center justify-between pt-2 border-t border-gray-800/50">
        <span className="text-xs text-gray-600 truncate" title={project.path}>
          {project.path}
        </span>
        <span className="text-xs text-gray-600 flex-shrink-0">{timeAgo}</span>
      </div>

      {/* Actions (visible on hover) */}
      <div className="absolute top-3 right-3 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          onClick={handleOpen}
          className="p-1.5 rounded-md bg-primary-500/10 text-primary-500 hover:bg-primary-500/20 transition-colors"
          title="Abrir en VSCode"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
          </svg>
        </button>
        <button
          onClick={handleTerminal}
          className="p-1.5 rounded-md bg-gray-800 text-gray-400 hover:text-white hover:bg-gray-700 transition-colors"
          title="Abrir terminal"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z" />
          </svg>
        </button>
      </div>
    </div>
  );
}

function formatTimeAgo(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffMins < 1) return "Ahora";
  if (diffMins < 60) return `Hace ${diffMins}m`;
  if (diffHours < 24) return `Hace ${diffHours}h`;
  if (diffDays < 7) return `Hace ${diffDays}d`;
  return date.toLocaleDateString("es");
}
