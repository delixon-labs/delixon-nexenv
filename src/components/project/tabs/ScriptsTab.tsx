import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { ScriptResult } from "@/types/scripts";

export default function ScriptsTab({ projectId }: { projectId: string; projectPath: string }) {
  const [scripts, setScripts] = useState<[string, string][]>([]);
  const [loading, setLoading] = useState(true);
  const [running, setRunning] = useState<string | null>(null);
  const [result, setResult] = useState<ScriptResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const s = await api.listProjectScripts(projectId);
      setScripts(s);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  async function handleRun(scriptName: string) {
    setRunning(scriptName);
    setResult(null);
    setError(null);
    try {
      const r = await api.runProjectScript(projectId, scriptName);
      setResult(r);
    } catch (err) {
      setError(String(err));
    } finally {
      setRunning(null);
    }
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  if (scripts.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500 text-sm">No hay scripts definidos en el manifest.</p>
        <p className="text-gray-600 text-xs mt-1">Genera un manifest con nexenv manifest &lt;proyecto&gt;</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {error && (
        <div className="px-4 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
      )}

      <div className="space-y-2">
        {scripts.map(([name, command]) => (
          <div key={name} className="flex items-center justify-between px-4 py-3 rounded-lg bg-gray-900 border border-gray-800">
            <div>
              <span className="text-sm font-medium text-primary-500">{name}</span>
              <span className="text-sm text-gray-500 ml-3 font-mono">{command}</span>
            </div>
            <button
              onClick={() => handleRun(name)}
              disabled={running !== null}
              className="px-3 py-1 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
            >
              {running === name ? "Ejecutando..." : "Ejecutar"}
            </button>
          </div>
        ))}
      </div>

      {result && (
        <div className="rounded-xl bg-gray-900 border border-gray-800 p-4">
          <div className="flex items-center gap-3 mb-2">
            <span className="text-sm font-medium text-white">{result.script}</span>
            <span className={`text-xs px-2 py-0.5 rounded ${result.exitCode === 0 ? "bg-green-500/10 text-green-400" : "bg-red-500/10 text-red-400"}`}>
              exit: {result.exitCode}
            </span>
          </div>
          {result.stdout && (
            <pre className="text-xs text-gray-300 font-mono max-h-48 overflow-auto whitespace-pre-wrap bg-gray-950 rounded p-3 mb-2">{result.stdout}</pre>
          )}
          {result.stderr && (
            <pre className="text-xs text-red-300 font-mono max-h-32 overflow-auto whitespace-pre-wrap bg-gray-950 rounded p-3">{result.stderr}</pre>
          )}
        </div>
      )}
    </div>
  );
}
