import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { ProjectProcess } from "@/types/processes";

export default function ProcessesTab({ projectId }: { projectId: string; projectPath: string }) {
  const [processes, setProcesses] = useState<ProjectProcess[]>([]);
  const [loading, setLoading] = useState(true);
  const [confirmKill, setConfirmKill] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const p = await api.listProjectProcesses(projectId);
      setProcesses(p);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  async function handleKill(pid: number) {
    try {
      await api.killProcess(pid);
      setConfirmKill(null);
      await load();
    } catch (err) {
      setError(String(err));
    }
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  return (
    <div className="space-y-4">
      {error && (
        <div className="px-4 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
      )}

      <div className="flex items-center justify-between">
        <span className="text-sm text-gray-400">
          {processes.length === 0
            ? "No hay procesos en los puertos del proyecto"
            : `${processes.length} proceso(s) detectado(s)`}
        </span>
        <button onClick={load} className="px-3 py-1 rounded-lg bg-gray-800 text-gray-400 text-xs hover:bg-gray-700 transition-colors">
          Refrescar
        </button>
      </div>

      {processes.length > 0 && (
        <div className="rounded-xl bg-gray-900 border border-gray-800 overflow-hidden">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-gray-800">
                <th className="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase">PID</th>
                <th className="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase">Proceso</th>
                <th className="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase">Puerto</th>
                <th className="px-4 py-2 text-right text-xs font-semibold text-gray-500 uppercase">Accion</th>
              </tr>
            </thead>
            <tbody>
              {processes.map((proc) => (
                <tr key={proc.pid} className="border-b border-gray-800/50">
                  <td className="px-4 py-2 font-mono text-gray-300">{proc.pid}</td>
                  <td className="px-4 py-2 text-white">{proc.name}</td>
                  <td className="px-4 py-2 text-gray-400 font-mono">{proc.port ? `:${proc.port}` : "-"}</td>
                  <td className="px-4 py-2 text-right">
                    {confirmKill === proc.pid ? (
                      <div className="flex items-center justify-end gap-2">
                        <span className="text-xs text-red-400">Confirmar?</span>
                        <button onClick={() => handleKill(proc.pid)} className="px-2 py-1 rounded bg-red-500 text-white text-xs">Si</button>
                        <button onClick={() => setConfirmKill(null)} className="px-2 py-1 rounded bg-gray-700 text-gray-300 text-xs">No</button>
                      </div>
                    ) : (
                      <button
                        onClick={() => setConfirmKill(proc.pid)}
                        className="px-2 py-1 rounded bg-red-500/10 text-red-400 text-xs hover:bg-red-500/20 transition-colors"
                      >
                        Kill
                      </button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
