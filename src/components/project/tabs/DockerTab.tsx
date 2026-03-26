import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { DockerComposeStatus } from "@/types/docker";

export default function DockerTab({ projectId }: { projectId: string; projectPath: string }) {
  const [status, setStatus] = useState<DockerComposeStatus | null>(null);
  const [logs, setLogs] = useState<string>("");
  const [showLogs, setShowLogs] = useState(false);
  const [loading, setLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const s = await api.dockerStatus(projectId);
      setStatus(s);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  async function handleUp() {
    setActionLoading(true);
    try {
      await api.dockerUp(projectId);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setActionLoading(false);
    }
  }

  async function handleDown() {
    setActionLoading(true);
    try {
      await api.dockerDown(projectId);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setActionLoading(false);
    }
  }

  async function handleLogs() {
    try {
      const l = await api.dockerLogs(projectId, 100);
      setLogs(l);
      setShowLogs(true);
    } catch (err) {
      setError(String(err));
    }
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  if (!status?.hasCompose) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500 text-sm">No se encontro docker-compose en este proyecto.</p>
        <p className="text-gray-600 text-xs mt-1">Agrega un docker-compose.yml para gestionar servicios.</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {error && (
        <div className="px-4 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
      )}

      <div className="flex items-center justify-between">
        <div>
          <span className="text-sm text-gray-400">Archivo: </span>
          <span className="text-sm text-white font-mono">{status.composeFile}</span>
        </div>
        <div className="flex gap-2">
          <button onClick={handleUp} disabled={actionLoading}
            className="px-3 py-1.5 rounded-lg bg-green-500/10 text-green-400 text-sm font-medium hover:bg-green-500/20 disabled:opacity-50 transition-colors">
            {actionLoading ? "..." : "Iniciar"}
          </button>
          <button onClick={handleDown} disabled={actionLoading}
            className="px-3 py-1.5 rounded-lg bg-red-500/10 text-red-400 text-sm font-medium hover:bg-red-500/20 disabled:opacity-50 transition-colors">
            Detener
          </button>
          <button onClick={handleLogs}
            className="px-3 py-1.5 rounded-lg bg-gray-800 text-gray-300 text-sm hover:bg-gray-700 transition-colors">
            Logs
          </button>
          <button onClick={load}
            className="px-3 py-1.5 rounded-lg bg-gray-800 text-gray-300 text-sm hover:bg-gray-700 transition-colors">
            Refrescar
          </button>
        </div>
      </div>

      {status.services.length > 0 ? (
        <div className="rounded-xl bg-gray-900 border border-gray-800 overflow-hidden">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-gray-800">
                <th className="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase">Servicio</th>
                <th className="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase">Estado</th>
                <th className="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase">Puertos</th>
              </tr>
            </thead>
            <tbody>
              {status.services.map((svc) => (
                <tr key={svc.name} className="border-b border-gray-800/50">
                  <td className="px-4 py-2 text-white font-medium">{svc.name}</td>
                  <td className="px-4 py-2">
                    <span className={svc.status.toLowerCase().includes("up") ? "text-green-400" : "text-gray-500"}>
                      {svc.status}
                    </span>
                  </td>
                  <td className="px-4 py-2 text-gray-400 font-mono text-xs">{svc.ports || "-"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ) : (
        <p className="text-sm text-gray-500 py-4">No hay servicios corriendo.</p>
      )}

      {showLogs && (
        <div className="rounded-xl bg-gray-900 border border-gray-800 p-4">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm font-medium text-gray-400">Logs</span>
            <button onClick={() => setShowLogs(false)} className="text-gray-500 hover:text-white text-xs">Cerrar</button>
          </div>
          <pre className="text-xs text-gray-300 font-mono max-h-64 overflow-auto whitespace-pre-wrap">{logs || "Sin logs"}</pre>
        </div>
      )}
    </div>
  );
}
