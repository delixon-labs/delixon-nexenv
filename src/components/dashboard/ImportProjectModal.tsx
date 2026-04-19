import { useState } from "react";
import * as api from "@/lib/tauri";
import PathInput from "@/components/ui/PathInput";
import { Spinner } from "@/components/ui/Spinner";
import { toast } from "@/components/ui/Toast";
import PreviewConfirmModal, { type PreviewSection } from "@/components/ui/PreviewConfirmModal";
import type { ImportPreview } from "@/types/portable";

interface ImportProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
  fileContent: string;
  onSuccess: () => void;
}

export default function ImportProjectModal({
  isOpen,
  onClose,
  fileContent,
  onSuccess,
}: ImportProjectModalProps) {
  const [path, setPath] = useState("");
  const [loadingPreview, setLoadingPreview] = useState(false);
  const [preview, setPreview] = useState<ImportPreview | null>(null);
  const [applying, setApplying] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);

    if (!path.trim()) {
      setError("La ruta es obligatoria");
      return;
    }

    setLoadingPreview(true);
    try {
      const p = await api.previewImport(fileContent, path.trim());
      setPreview(p);
    } catch (err) {
      toast.error(err);
      setError(typeof err === "string" ? err : null);
    } finally {
      setLoadingPreview(false);
    }
  }

  async function applyImport() {
    if (!preview) return;
    setApplying(true);
    try {
      await api.importProject(fileContent, preview.targetPath);
      toast.success("Proyecto importado correctamente");
      setPath("");
      setPreview(null);
      onSuccess();
      onClose();
    } catch (err) {
      toast.error(err);
      setApplying(false);
    }
  }

  function importSections(p: ImportPreview): PreviewSection[] {
    const out: PreviewSection[] = [];
    out.push({
      label: "Proyecto a importar",
      items: [
        `Nombre: ${p.name}`,
        ...(p.description ? [`Descripcion: ${p.description}`] : []),
        `Tags: ${p.tags.length ? p.tags.join(", ") : "(ninguno)"}`,
        `Manifest incluido: ${p.hasManifest ? "si" : "no"}`,
      ],
      tone: "neutral",
    });
    if (p.runtimes.length) {
      out.push({
        label: "Runtimes",
        items: p.runtimes.map((r) => `${r.runtime} ${r.version}`),
        tone: "added",
      });
    }
    if (p.envKeys.length) {
      out.push({
        label: "Variables de entorno (solo nombres, sin valores)",
        items: p.envKeys,
        tone: "neutral",
      });
    }
    out.push({
      label: "Destino",
      items: [
        p.targetPath,
        `La carpeta ${p.targetExists ? "existe" : "se creara"}`,
        ...(p.targetHasFiles ? ["La carpeta contiene archivos"] : []),
      ],
      tone: p.targetHasFiles || p.conflictWithExisting ? "warning" : "neutral",
    });
    return out;
  }

  function importWarning(p: ImportPreview): string | undefined {
    if (p.conflictWithExisting) {
      return "Ya existe un proyecto registrado en esta ruta. El import sera rechazado por el backend.";
    }
    if (p.targetHasFiles) {
      return "La carpeta destino contiene archivos. El proyecto se registrara, pero los archivos existentes no se modifican.";
    }
    return undefined;
  }

  function handleClose() {
    setPath("");
    setError(null);
    setPreview(null);
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
            <h2 className="text-lg font-semibold text-white">
              Importar proyecto
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

          <div className="px-6 py-4 space-y-4">
            {error && (
              <div className="px-3 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
                {error}
              </div>
            )}

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Ruta donde registrar el proyecto
              </label>
              <PathInput
                value={path}
                onChange={setPath}
                placeholder="C:\\Users\\you\\projects\\mi-proyecto"
                autoFocus
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
              disabled={loadingPreview}
              className="inline-flex items-center justify-center min-w-28 px-4 py-2 rounded-lg text-sm font-medium bg-success/10 text-success-light hover:bg-success/20 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {loadingPreview ? <Spinner size="sm" className="text-success-light" /> : "Vista previa"}
            </button>
          </div>
        </form>
      </div>

      <PreviewConfirmModal
        open={!!preview}
        title="Confirmar import del proyecto"
        subtitle="Revisa los detalles antes de aplicar."
        sections={preview ? importSections(preview) : []}
        warning={preview ? importWarning(preview) : undefined}
        confirmLabel="Importar"
        destructive={!!preview?.conflictWithExisting}
        busy={applying}
        onConfirm={applyImport}
        onCancel={() => setPreview(null)}
      />
    </div>
  );
}
