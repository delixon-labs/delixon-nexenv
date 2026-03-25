import { useState } from "react";
import type { CreateProjectInput, Runtime, RuntimeConfig } from "@/types/project";
import { useProjectsStore } from "@/stores/projects";

const AVAILABLE_RUNTIMES: { value: Runtime; label: string }[] = [
  { value: "node", label: "Node.js" },
  { value: "python", label: "Python" },
  { value: "rust", label: "Rust" },
  { value: "go", label: "Go" },
  { value: "dotnet", label: ".NET" },
  { value: "php", label: "PHP" },
  { value: "ruby", label: "Ruby" },
];

interface CreateProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function CreateProjectModal({
  isOpen,
  onClose,
}: CreateProjectModalProps) {
  const { addProject } = useProjectsStore();

  const [name, setName] = useState("");
  const [path, setPath] = useState("");
  const [description, setDescription] = useState("");
  const [selectedRuntimes, setSelectedRuntimes] = useState<Runtime[]>([]);
  const [tags, setTags] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  function toggleRuntime(rt: Runtime) {
    setSelectedRuntimes((prev) =>
      prev.includes(rt) ? prev.filter((r) => r !== rt) : [...prev, rt]
    );
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);

    if (!name.trim()) {
      setError("El nombre es obligatorio");
      return;
    }
    if (!path.trim()) {
      setError("La ruta del proyecto es obligatoria");
      return;
    }

    setIsSubmitting(true);
    try {
      const runtimes: RuntimeConfig[] = selectedRuntimes.map((rt) => ({
        runtime: rt,
        version: "",
      }));

      const input: CreateProjectInput = {
        name: name.trim(),
        path: path.trim(),
        description: description.trim() || undefined,
        runtimes,
        tags: tags
          .split(",")
          .map((t) => t.trim())
          .filter(Boolean),
      };

      await addProject(input);
      resetForm();
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setIsSubmitting(false);
    }
  }

  function resetForm() {
    setName("");
    setPath("");
    setDescription("");
    setSelectedRuntimes([]);
    setTags("");
    setError(null);
  }

  function handleClose() {
    resetForm();
    onClose();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onClick={handleClose}
      />

      {/* Modal */}
      <div className="relative w-full max-w-lg mx-4 bg-gray-900 rounded-2xl border border-gray-800 shadow-2xl">
        <form onSubmit={handleSubmit}>
          {/* Header */}
          <div className="flex items-center justify-between px-6 py-4 border-b border-gray-800">
            <h2 className="text-lg font-semibold text-white">
              Nuevo Proyecto
            </h2>
            <button
              type="button"
              onClick={handleClose}
              className="p-1 rounded-md text-gray-500 hover:text-white hover:bg-gray-800 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          {/* Body */}
          <div className="px-6 py-4 space-y-4">
            {error && (
              <div className="px-3 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
                {error}
              </div>
            )}

            {/* Nombre */}
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Nombre del proyecto
              </label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="mi-proyecto"
                className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:border-primary-500 focus:ring-1 focus:ring-primary-500 text-sm"
                autoFocus
              />
            </div>

            {/* Ruta */}
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Ruta del proyecto
              </label>
              <input
                type="text"
                value={path}
                onChange={(e) => setPath(e.target.value)}
                placeholder="/home/user/projects/mi-proyecto"
                className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:border-primary-500 focus:ring-1 focus:ring-primary-500 text-sm font-mono"
              />
            </div>

            {/* Descripcion */}
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Descripcion
                <span className="text-gray-600 ml-1">(opcional)</span>
              </label>
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Breve descripcion del proyecto..."
                rows={2}
                className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:border-primary-500 focus:ring-1 focus:ring-primary-500 text-sm resize-none"
              />
            </div>

            {/* Runtimes */}
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Runtimes
              </label>
              <div className="flex flex-wrap gap-2">
                {AVAILABLE_RUNTIMES.map(({ value, label }) => (
                  <button
                    key={value}
                    type="button"
                    onClick={() => toggleRuntime(value)}
                    className={`px-3 py-1 rounded-lg text-sm font-medium border transition-colors ${
                      selectedRuntimes.includes(value)
                        ? "bg-primary-500/10 text-primary-500 border-primary-500/30"
                        : "bg-gray-800 text-gray-500 border-gray-700 hover:text-gray-300 hover:border-gray-600"
                    }`}
                  >
                    {label}
                  </button>
                ))}
              </div>
            </div>

            {/* Tags */}
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Tags
                <span className="text-gray-600 ml-1">(separados por coma)</span>
              </label>
              <input
                type="text"
                value={tags}
                onChange={(e) => setTags(e.target.value)}
                placeholder="web, api, personal"
                className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:border-primary-500 focus:ring-1 focus:ring-primary-500 text-sm"
              />
            </div>
          </div>

          {/* Footer */}
          <div className="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-800">
            <button
              type="button"
              onClick={handleClose}
              className="px-4 py-2 rounded-lg text-sm font-medium text-gray-400 hover:text-white hover:bg-gray-800 transition-colors"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={isSubmitting}
              className="px-4 py-2 rounded-lg text-sm font-medium bg-primary-500 text-white hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isSubmitting ? "Creando..." : "Crear proyecto"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
