import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { ProjectManifest } from "@/lib/tauri";

export default function ManifestTab({ projectId }: { projectId: string; projectPath: string }) {
  const [manifest, setManifest] = useState<ProjectManifest | null>(null);
  const [loading, setLoading] = useState(true);
  const [regenerating, setRegenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const m = await api.getManifest(projectId);
      setManifest(m);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  async function handleRegenerate() {
    setRegenerating(true);
    try {
      const m = await api.regenerateManifest(projectId);
      setManifest(m);
    } catch (err) {
      setError(String(err));
    } finally {
      setRegenerating(false);
    }
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  if (!manifest) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500 text-sm mb-3">No hay manifest para este proyecto.</p>
        <button
          onClick={handleRegenerate}
          disabled={regenerating}
          className="px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
        >
          {regenerating ? "Generando..." : "Generar manifest"}
        </button>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {error && (
        <div className="px-4 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
      )}

      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider">Manifest del proyecto</h3>
        <button
          onClick={handleRegenerate}
          disabled={regenerating}
          className="px-3 py-1.5 rounded-lg bg-dlx-light-2/50 text-dlx-text-light-1 text-sm hover:bg-dlx-light-3/50 disabled:opacity-50 transition-colors"
        >
          {regenerating ? "Regenerando..." : "Regenerar"}
        </button>
      </div>

      {/* Info grid */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <Field label="Tipo" value={manifest.projectType || "-"} />
        <Field label="Perfil" value={manifest.profile || "-"} />
        <Field label="Runtime" value={manifest.runtime || "-"} />
        <Field label="Puertos" value={manifest.ports.length > 0 ? manifest.ports.join(", ") : "-"} />
      </div>

      {/* Technologies */}
      {manifest.technologies.length > 0 && (
        <Section title="Tecnologias">
          <div className="flex flex-wrap gap-2">
            {manifest.technologies.map((t) => (
              <span key={t} className="px-2.5 py-1 rounded-lg bg-gray-800 text-sm text-primary-500 border border-gray-700">{t}</span>
            ))}
          </div>
        </Section>
      )}

      {/* Commands */}
      {Object.keys(manifest.commands).length > 0 && (
        <Section title="Comandos">
          <div className="space-y-1">
            {Object.entries(manifest.commands).map(([key, val]) => (
              <div key={key} className="flex items-center gap-3 px-3 py-2 rounded-lg bg-gray-900">
                <span className="text-sm font-medium text-primary-500 w-16">{key}</span>
                <span className="text-sm text-gray-300 font-mono">{val}</span>
              </div>
            ))}
          </div>
        </Section>
      )}

      {/* Services */}
      {manifest.services.length > 0 && (
        <Section title="Servicios">
          <div className="space-y-1">
            {manifest.services.map((svc) => (
              <div key={svc.name} className="flex items-center gap-3 px-3 py-2 rounded-lg bg-gray-900">
                <span className="text-sm text-white">{svc.name}</span>
                <span className="text-xs text-gray-500">:{svc.port}</span>
                {svc.docker && <span className="text-xs px-1.5 py-0.5 rounded bg-blue-500/10 text-blue-400">Docker</span>}
              </div>
            ))}
          </div>
        </Section>
      )}

      {/* Env vars */}
      {(manifest.envVars.required.length > 0 || manifest.envVars.optional.length > 0) && (
        <Section title="Variables de entorno">
          {manifest.envVars.required.length > 0 && (
            <div className="mb-2">
              <span className="text-xs text-red-400 uppercase">Requeridas: </span>
              <span className="text-sm text-gray-300">{manifest.envVars.required.join(", ")}</span>
            </div>
          )}
          {manifest.envVars.optional.length > 0 && (
            <div>
              <span className="text-xs text-gray-500 uppercase">Opcionales: </span>
              <span className="text-sm text-gray-400">{manifest.envVars.optional.join(", ")}</span>
            </div>
          )}
        </Section>
      )}

      {/* Recipes applied */}
      {manifest.recipesApplied.length > 0 && (
        <Section title="Recipes aplicadas">
          <div className="flex flex-wrap gap-2">
            {manifest.recipesApplied.map((r) => (
              <span key={r} className="px-2 py-1 rounded bg-green-500/10 text-green-400 text-xs">{r}</span>
            ))}
          </div>
        </Section>
      )}
    </div>
  );
}

function Field({ label, value }: { label: string; value: string }) {
  return (
    <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
      <p className="text-xs text-gray-500 mb-1">{label}</p>
      <p className="text-sm font-medium text-white">{value}</p>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">{title}</h4>
      {children}
    </div>
  );
}
