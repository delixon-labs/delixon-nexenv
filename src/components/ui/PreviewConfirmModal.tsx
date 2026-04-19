import { useEffect } from "react";
import { createPortal } from "react-dom";
import "./PreviewConfirmModal.css";

export interface PreviewSection {
  label: string;
  items: string[];
  tone?: "neutral" | "added" | "removed" | "changed" | "warning";
}

interface Props {
  open: boolean;
  title: string;
  subtitle?: string;
  sections: PreviewSection[];
  warning?: string;
  confirmLabel?: string;
  destructive?: boolean;
  busy?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export default function PreviewConfirmModal({
  open,
  title,
  subtitle,
  sections,
  warning,
  confirmLabel = "Confirmar",
  destructive = false,
  busy = false,
  onConfirm,
  onCancel,
}: Props) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape" && !busy) onCancel();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, busy, onCancel]);

  if (!open) return null;

  const visibleSections = sections.filter((s) => s.items.length > 0);
  const empty = visibleSections.length === 0;

  return createPortal(
    <div className="dlx-pcm__backdrop" onClick={() => !busy && onCancel()}>
      <div className="dlx-pcm__panel" onClick={(e) => e.stopPropagation()}>
        <header className="dlx-pcm__header">
          <h3 className="dlx-pcm__title">{title}</h3>
          {subtitle && <p className="dlx-pcm__subtitle">{subtitle}</p>}
        </header>

        <div className="dlx-pcm__body">
          {empty && (
            <p className="dlx-pcm__empty">
              No hay cambios para mostrar. La accion no modificara nada.
            </p>
          )}
          {visibleSections.map((s, i) => (
            <section key={i} className={`dlx-pcm__section dlx-pcm__section--${s.tone ?? "neutral"}`}>
              <h4 className="dlx-pcm__section-label">{s.label}</h4>
              <ul className="dlx-pcm__list">
                {s.items.map((item, j) => (
                  <li key={j}>{item}</li>
                ))}
              </ul>
            </section>
          ))}
          {warning && (
            <div className="dlx-pcm__warning">
              <strong>Advertencia:</strong> {warning}
            </div>
          )}
        </div>

        <footer className="dlx-pcm__footer">
          <button
            type="button"
            className="dlx-pcm__btn dlx-pcm__btn--cancel"
            onClick={onCancel}
            disabled={busy}
          >
            Cancelar
          </button>
          <button
            type="button"
            className={`dlx-pcm__btn ${destructive ? "dlx-pcm__btn--destructive" : "dlx-pcm__btn--primary"}`}
            onClick={onConfirm}
            disabled={busy}
          >
            {busy ? "Aplicando..." : confirmLabel}
          </button>
        </footer>
      </div>
    </div>,
    document.body
  );
}
