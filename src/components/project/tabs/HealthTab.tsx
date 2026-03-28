import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { HealthReport } from "@/types/health";

export default function HealthTab({ projectId }: { projectId: string; projectPath: string }) {
  const [report, setReport] = useState<HealthReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const r = await api.checkProjectHealth(projectId);
      setReport(r);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  if (loading) return <Spinner />;
  if (error) return <ErrorBox message={error} />;
  if (!report) return null;

  const statusIcon = (s: string) =>
    s === "ok" ? "text-green-400" : s === "warning" ? "text-yellow-400" : "text-red-400";

  const statusLabel = (s: string) =>
    s === "ok" ? "OK" : s === "warning" ? "Advertencia" : "Error";

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <span className={`text-2xl font-bold ${statusIcon(report.overall)}`}>
            {statusLabel(report.overall)}
          </span>
          <span className="text-sm text-gray-500">
            {report.checks.filter((c) => c.status === "ok").length}/{report.checks.length} checks OK
          </span>
        </div>
        <button
          onClick={load}
          className="px-3 py-1.5 rounded-lg bg-dlx-light-2/50 text-dlx-text-light-1 text-sm hover:bg-dlx-light-3/50 transition-colors"
        >
          Refrescar
        </button>
      </div>

      <div className="space-y-2">
        {report.checks.map((check) => (
          <div
            key={check.name}
            className="flex items-start gap-3 px-4 py-3 rounded-lg bg-gray-900 border border-gray-800"
          >
            <span className={`mt-0.5 w-2 h-2 rounded-full flex-shrink-0 ${
              check.status === "ok" ? "bg-green-400" : check.status === "warning" ? "bg-yellow-400" : "bg-red-400"
            }`} />
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2">
                <span className="text-sm font-medium text-white">{check.name}</span>
              </div>
              <p className="text-sm text-gray-400 mt-0.5">{check.message}</p>
              {check.fixSuggestion && check.status !== "ok" && (
                <p className="text-xs text-gray-500 mt-1">
                  Sugerencia: {check.fixSuggestion}
                </p>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

function Spinner() {
  return (
    <div className="flex items-center justify-center py-12">
      <div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" />
    </div>
  );
}

function ErrorBox({ message }: { message: string }) {
  return (
    <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
      {message}
    </div>
  );
}
