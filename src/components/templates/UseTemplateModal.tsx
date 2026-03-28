import { useState } from "react";
import { useNavigate } from "react-router-dom";
import * as api from "@/lib/tauri";
import PathInput from "@/components/ui/PathInput";

interface UseTemplateModalProps {
  isOpen: boolean;
  onClose: () => void;
  templateId: string;
  templateName: string;
}

export default function UseTemplateModal({
  isOpen,
  onClose,
  templateId,
  templateName,
}: UseTemplateModalProps) {
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [path, setPath] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);

    if (!name.trim()) {
      setError("El nombre es obligatorio");
      return;
    }
    if (!path.trim()) {
      setError("La ruta es obligatoria");
      return;
    }

    setIsSubmitting(true);
    try {
      const project = await api.createFromTemplate(templateId, path.trim(), name.trim());
      onClose();
      navigate(`/project/${project.id}`);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsSubmitting(false);
    }
  }

  function handleClose() {
    setName("");
    setPath("");
    setError(null);
    onClose();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onClick={handleClose}
      />
      <div className="relative w-full max-w-md mx-4 bg-gray-900 rounded-2xl border border-gray-800 shadow-2xl">
        <form onSubmit={handleSubmit}>
          <div className="flex items-center justify-between px-6 py-4 border-b border-gray-800">
            <div>
              <h2 className="text-lg font-semibold text-white">
                Usar plantilla
              </h2>
              <p className="text-xs text-gray-500">{templateName}</p>
            </div>
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

          <div className="px-6 py-4 space-y-4">
            {error && (
              <div className="px-3 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
                {error}
              </div>
            )}

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Nombre del proyecto
              </label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="mi-proyecto"
                className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-hidden focus:border-primary-500 focus:ring-1 focus:ring-primary-500 text-sm"
                autoFocus
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Ruta donde crear el proyecto
              </label>
              <PathInput
                value={path}
                onChange={setPath}
                placeholder="C:\\Users\\you\\projects\\mi-proyecto"
              />
            </div>
          </div>

          <div className="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-800">
            <button
              type="button"
              onClick={handleClose}
              className="px-4 py-2 rounded-lg text-sm font-medium bg-dlx-light-3 text-dlx-text-light-1 border border-dlx-text-dark-3 hover:bg-dlx-text-dark-3 transition-colors"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={isSubmitting}
              className="px-4 py-2 rounded-lg text-sm font-medium bg-success/10 text-success-light hover:bg-success/20 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isSubmitting ? "Creando..." : "Crear desde plantilla"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
