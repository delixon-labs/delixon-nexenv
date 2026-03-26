import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { Snapshot, SnapshotDiff } from "@/types/versioning";

export default function VersioningTab({ projectId }: { projectId: string; projectPath: string }) {
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [diff, setDiff] = useState<SnapshotDiff | null>(null);
  const [confirmRollback, setConfirmRollback] = useState<number | null>(null);
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

  async function handleRollback(version: number) {
    try {
      await api.rollbackSnapshot(projectId, version);
      setConfirmRollback(null);
      await load();
    } catch (err) {
      setError(String(err));
    }
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
          className="px-4 py-1.5 rounded-lg bg-primary-500 text-white text-sm font-medium hover:bg-primary-600 disabled:opacity-50 transition-colors"
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
                      className="px-2 py-1 rounded bg-gray-800 text-gray-400 text-xs hover:bg-gray-700 transition-colors"
                    >
                      Diff vs v{prevSnap.version}
                    </button>
                  )}
                  {confirmRollback === snap.version ? (
                    <div className="flex items-center gap-1">
                      <span className="text-xs text-yellow-400">Restaurar?</span>
                      <button onClick={() => handleRollback(snap.version)} className="px-2 py-0.5 rounded bg-yellow-500 text-black text-xs">Si</button>
                      <button onClick={() => setConfirmRollback(null)} className="px-2 py-0.5 rounded bg-gray-700 text-gray-300 text-xs">No</button>
                    </div>
                  ) : (
                    <button
                      onClick={() => setConfirmRollback(snap.version)}
                      className="px-2 py-1 rounded bg-yellow-500/10 text-yellow-400 text-xs hover:bg-yellow-500/20 transition-colors"
                    >
                      Restaurar
                    </button>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      )}

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
