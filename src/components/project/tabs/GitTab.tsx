import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { GitStatus, GitCommit } from "@/types/git";

export default function GitTab({ projectId }: { projectId: string; projectPath: string }) {
  const [status, setStatus] = useState<GitStatus | null>(null);
  const [commits, setCommits] = useState<GitCommit[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [gs, log] = await Promise.all([
        api.gitStatus(projectId),
        api.gitLog(projectId, 10),
      ]);
      setStatus(gs);
      setCommits(log);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;
  if (error) return <div className="px-4 py-3 rounded-lg bg-yellow-500/10 border border-yellow-500/20 text-yellow-400 text-sm">{error}</div>;
  if (!status) return null;

  return (
    <div className="space-y-6">
      {/* Branch & Status */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <InfoCard label="Rama" value={status.branch} />
        <InfoCard
          label="Estado"
          value={status.isClean ? "Limpio" : `${status.modifiedFiles} mod, ${status.untrackedFiles} sin trackear`}
          color={status.isClean ? "text-green-400" : "text-yellow-400"}
        />
        <InfoCard
          label="Remote"
          value={status.hasRemote ? `+${status.ahead} -${status.behind}` : "Sin remote"}
          color={status.hasRemote && status.ahead === 0 && status.behind === 0 ? "text-green-400" : "text-gray-400"}
        />
        <InfoCard
          label="Ultimo commit"
          value={status.lastCommit?.message ?? "Sin commits"}
        />
      </div>

      {/* Commit Log */}
      {commits.length > 0 && (
        <div>
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider">
              Commits recientes
            </h3>
            <button
              onClick={load}
              className="px-3 py-1 rounded-lg bg-gray-800 text-gray-400 text-xs hover:bg-gray-700 transition-colors"
            >
              Refrescar
            </button>
          </div>
          <div className="space-y-1">
            {commits.map((commit) => (
              <div key={commit.hash} className="flex items-center gap-3 px-4 py-2 rounded-lg bg-gray-900 border border-gray-800">
                <span className="text-xs font-mono text-primary-500 flex-shrink-0">
                  {commit.hash.slice(0, 7)}
                </span>
                <span className="text-sm text-white truncate flex-1">{commit.message}</span>
                <span className="text-xs text-gray-500 flex-shrink-0">{commit.author}</span>
                <span className="text-xs text-gray-600 flex-shrink-0">{commit.date.slice(0, 10)}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function InfoCard({ label, value, color }: { label: string; value: string; color?: string }) {
  return (
    <div className="px-4 py-3 rounded-xl bg-gray-900 border border-gray-800">
      <p className="text-xs text-gray-500 mb-1">{label}</p>
      <p className={`text-sm font-medium truncate ${color ?? "text-white"}`}>{value}</p>
    </div>
  );
}
