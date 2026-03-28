import { useNavigate } from "react-router-dom";
import type { Project } from "@/types/project";
import { useProjectsStore } from "@/stores/projects";
import * as api from "@/lib/tauri";
import { techBrandClass } from "@/lib/tech-meta";

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
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span
            className={`w-2 h-2 rounded-full shrink-0 ${status.color}`}
            title={status.label}
          />
          <h3 className="text-base font-semibold text-white truncate group-hover:text-primary-500 transition-colors">
            {project.name}
          </h3>
        </div>
        {project.description && (
          <p className="mt-1 text-sm text-gray-500 line-clamp-2 pl-4">
            {project.description}
          </p>
        )}
      </div>

      {/* Runtimes */}
      {project.runtimes.length > 0 && (
        <div className="flex flex-wrap gap-x-3 gap-y-1">
          {project.runtimes.map((rt) => {
            const cls = techBrandClass(rt.runtime);
            return (
              <span
                key={rt.runtime}
                className={`text-xs font-medium ${cls.text}`}
              >
                {rt.runtime}{rt.version ? ` ${rt.version}` : ""}
              </span>
            );
          })}
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
        <span className="text-xs text-gray-600 shrink-0">{timeAgo}</span>
      </div>

      {/* Actions (visible on hover) */}
      <div className="absolute top-3 right-3 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          onClick={handleOpen}
          className="p-1.5 rounded-md bg-info/10 text-info-light hover:bg-info/20 transition-colors"
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
