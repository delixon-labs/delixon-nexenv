import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { Snapshot, SnapshotDiff, RollbackPreview } from "@/types/versioning";
import PreviewConfirmModal, { type PreviewSection } from "@/components/ui/PreviewConfirmModal";
import { toast } from "@/components/ui/Toast";

interface RollbackState {
  version: number;
  preview: RollbackPreview | null;
  loading: boolean;
  applying: boolean;
}

export default function VersioningTab({ projectId }: { projectId: string; projectPath: string }) {
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [diff, setDiff] = useState<SnapshotDiff | null>(null);
  const [rollback, setRollback] = useState<RollbackState | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const s = await api.listSnapshots(projectId);
      setSnapshots(s);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  async function handleSave() {
    setSaving(true);
    try {
      await api.saveSnapshot(projectId);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setSaving(false);
    }
  }

  async function handleDiff(v1: number, v2: number) {
    try {
      const d = await api.diffSnapshots(projectId, v1, v2);
      setDiff(d);
    } catch (err) {
      setError(String(err));
    }
  }

  async function openRollbackPreview(version: number) {
    setRollback({ version, preview: null, loading: true, applying: false });
    try {
      const preview = await api.previewRollback(projectId, version);
      setRollback({ version, preview, loading: false, applying: false });
    } catch (err) {
      toast.error(err);
      setRollback(null);
    }
  }

  async function applyRollback() {
    if (!rollback) return;
    setRollback({ ...rollback, applying: true });
    try {
      await api.rollbackSnapshot(projectId, rollback.version);
      toast.success(`Rollback a v${rollback.version} aplicado`);
      setRollback(null);
      await load();
    } catch (err) {
      toast.error(err);
      setRollback({ ...rollback, applying: false });
    }
  }

  function rollbackSections(p: RollbackPreview): PreviewSection[] {
    const out: PreviewSection[] = [];
    if (p.addedTechs.length) out.push({ label: "Tecnologias que se anaden", items: p.addedTechs, tone: "added" });
    if (p.removedTechs.length) out.push({ label: "Tecnologias que se quitan", items: p.removedTechs, tone: "removed" });
    if (p.addedRecipes.length) out.push({ label: "Recipes que se anaden", items: p.addedRecipes, tone: "added" });
    if (p.removedRecipes.length) out.push({ label: "Recipes que se quitan", items: p.removedRecipes, tone: "removed" });
    if (p.profileChanged) out.push({ label: "Profile", items: [`${p.profileChanged[0]} → ${p.profileChanged[1]}`], tone: "changed" });
    if (p.editorChanged) out.push({ label: "Editor", items: [`${p.editorChanged[0] ?? "(ninguno)"} → ${p.editorChanged[1] ?? "(ninguno)"}`], tone: "changed" });
    if (p.nameChanged) out.push({ label: "Nombre", items: [`${p.nameChanged[0]} → ${p.nameChanged[1]}`], tone: "changed" });
    if (p.runtimeChanged) out.push({ label: "Runtime", items: [`${p.runtimeChanged[0]} → ${p.runtimeChanged[1]}`], tone: "changed" });
    return out;
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  return (
    <div className="space-y-6">
      {error && (
        <div className="px-4 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
      )}

      <div className="flex items-center justify-between">
        <span className="text-sm text-gray-400">{snapshots.length} snapshot(s)</span>
        <button
          onClick={handleSave}
          disabled={saving}
          className="px-4 py-1.5 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
        >
          {saving ? "Guardando..." : "Tomar snapshot"}
        </button>
      </div>

      {snapshots.length === 0 ? (
        <p className="text-sm text-gray-500 text-center py-8">No hay snapshots. Toma uno para versionar el manifest.</p>
      ) : (
        <div className="space-y-2">
          {snapshots.slice().reverse().map((snap, idx) => {
            const prevSnap = snapshots[snapshots.length - 1 - idx - 1];
            return (
              <div key={snap.version} className="flex items-center justify-between px-4 py-3 rounded-lg bg-gray-900 border border-gray-800">
                <div>
                  <span className="text-sm font-medium text-primary-500">v{snap.version}</span>
                  <span className="text-xs text-gray-500 ml-3">{new Date(snap.timestamp).toLocaleString("es")}</span>
                  <span className="text-xs text-gray-600 ml-2">{snap.manifest.technologies?.length ?? 0} techs</span>
                </div>
                <div className="flex gap-2">
                  {prevSnap && (
                    <button
                      onClick={() => handleDiff(prevSnap.version, snap.version)}
                      className="px-2 py-1 rounded bg-info/10 text-info-light text-xs hover:bg-info/20 transition-colors"
                    >
                      Diff vs v{prevSnap.version}
                    </button>
                  )}
                  <button
                    onClick={() => openRollbackPreview(snap.version)}
                    className="px-2 py-1 rounded bg-warning/10 text-warning-light text-xs hover:bg-warning/20 transition-colors"
                  >
                    Restaurar
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      )}

      <PreviewConfirmModal
        open={!!rollback}
        title={`Rollback a snapshot v${rollback?.version ?? ""}`}
        subtitle={
          rollback?.preview
            ? `Tomado el ${new Date(rollback.preview.targetTimestamp).toLocaleString("es")}. Sobrescribe el manifest actual.`
            : rollback?.loading
              ? "Calculando cambios..."
              : ""
        }
        sections={rollback?.preview ? rollbackSections(rollback.preview) : []}
        warning={
          rollback?.preview && !rollback.preview.currentManifestExists
            ? "El proyecto no tiene manifest actualmente. El rollback creara uno nuevo desde el snapshot."
            : undefined
        }
        confirmLabel={`Restaurar v${rollback?.version ?? ""}`}
        destructive
        busy={!!rollback?.applying || !!rollback?.loading}
        onConfirm={applyRollback}
        onCancel={() => setRollback(null)}
      />

      {diff && (
        <div className="rounded-xl bg-gray-900 border border-gray-800 p-4">
          <div className="flex items-center justify-between mb-3">
            <span className="text-sm font-medium text-white">Diff v{diff.fromVersion} → v{diff.toVersion}</span>
            <button onClick={() => setDiff(null)} className="text-xs text-gray-500 hover:text-white">Cerrar</button>
          </div>
          {diff.addedTechs.length === 0 && diff.removedTechs.length === 0 && diff.addedRecipes.length === 0 ? (
            <p className="text-sm text-gray-500">Sin cambios</p>
          ) : (
            <div className="space-y-1">
              {diff.addedTechs.map((t) => (
                <p key={t} className="text-sm"><span className="text-green-400">+ {t}</span></p>
              ))}
              {diff.removedTechs.map((t) => (
                <p key={t} className="text-sm"><span className="text-red-400">- {t}</span></p>
              ))}
              {diff.addedRecipes.map((r) => (
                <p key={r} className="text-sm"><span className="text-blue-400">+ recipe: {r}</span></p>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
