import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { ProjectNote } from "@/types/notes";

export default function NotesTab({ projectId }: { projectId: string; projectPath: string }) {
  const [notes, setNotes] = useState<ProjectNote[]>([]);
  const [loading, setLoading] = useState(true);
  const [newText, setNewText] = useState("");
  const [saving, setSaving] = useState(false);
  const [confirmDelete, setConfirmDelete] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const n = await api.getNotes(projectId);
      setNotes(n);
    } catch {
      // silently fail
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => { load(); }, [load]);

  async function handleAdd() {
    if (!newText.trim()) return;
    setSaving(true);
    try {
      await api.addNote(projectId, newText.trim());
      setNewText("");
      await load();
    } catch {
      // silently fail
    } finally {
      setSaving(false);
    }
  }

  async function handleDelete(noteId: string) {
    try {
      await api.deleteNote(projectId, noteId);
      setConfirmDelete(null);
      await load();
    } catch {
      // silently fail
    }
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  return (
    <div className="space-y-4">
      {/* Add note */}
      <div className="flex gap-2">
        <textarea
          value={newText}
          onChange={(e) => setNewText(e.target.value)}
          placeholder="Escribe una nota..."
          rows={2}
          className="flex-1 px-3 py-2 rounded-lg bg-gray-900 border border-gray-800 text-white text-sm resize-none placeholder-gray-600 focus:outline-hidden focus:border-primary-500/50"
          onKeyDown={(e) => { if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) handleAdd(); }}
        />
        <button
          onClick={handleAdd}
          disabled={saving || !newText.trim()}
          className="px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors self-end"
        >
          {saving ? "..." : "Agregar"}
        </button>
      </div>

      {/* Notes list */}
      {notes.length === 0 ? (
        <p className="text-sm text-gray-500 text-center py-8">No hay notas. Agrega una para recordar contexto.</p>
      ) : (
        <div className="space-y-2">
          {notes.slice().reverse().map((note) => (
            <div key={note.id} className="px-4 py-3 rounded-lg bg-gray-900 border border-gray-800">
              <div className="flex items-start justify-between gap-2">
                <p className="text-sm text-gray-200 whitespace-pre-wrap flex-1">{note.text}</p>
                {confirmDelete === note.id ? (
                  <div className="flex items-center gap-1 shrink-0">
                    <button onClick={() => handleDelete(note.id)} className="px-2 py-0.5 rounded bg-error text-white text-xs">Si</button>
                    <button onClick={() => setConfirmDelete(null)} className="px-2 py-0.5 rounded bg-gray-700 text-gray-300 text-xs">No</button>
                  </div>
                ) : (
                  <button
                    onClick={() => setConfirmDelete(note.id)}
                    className="p-1 rounded text-gray-600 hover:text-error-light transition-colors shrink-0"
                  >
                    <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                )}
              </div>
              <p className="text-xs text-gray-600 mt-1">{new Date(note.createdAt).toLocaleString("es")}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
