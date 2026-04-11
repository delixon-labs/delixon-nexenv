import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { create } from "zustand";
import "./Toast.css";

/* ── Store global ─────────────────────────── */

type ToastType = "success" | "error" | "warning" | "info";

interface ToastState {
  message: string;
  type: ToastType;
  duration: number;
  key: number;
}

interface ToastStore {
  toast: ToastState | null;
  show: (message: string, type?: ToastType, duration?: number) => void;
}

export const useToastStore = create<ToastStore>()((set) => ({
  toast: null,
  show: (message, type = "info", duration = 3000) =>
    set({ toast: { message, type, duration, key: Date.now() } }),
}));

/** Shorthand: toast.success("mensaje"), toast.error("mensaje"), etc. */
export const toast = {
  success: (msg: string, duration?: number) =>
    useToastStore.getState().show(msg, "success", duration),
  error: (msg: string, duration?: number) =>
    useToastStore.getState().show(msg, "error", duration),
  warning: (msg: string, duration?: number) =>
    useToastStore.getState().show(msg, "warning", duration),
  info: (msg: string, duration?: number) =>
    useToastStore.getState().show(msg, "info", duration),
};

/* ── Componente ───────────────────────────── */

export function ToastContainer() {
  const data = useToastStore((s) => s.toast);
  const [visible, setVisible] = useState(false);
  const [leaving, setLeaving] = useState(false);
  const [paused, setPaused] = useState(false);
  const progressRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!data) return;
    setLeaving(false);
    setPaused(false);
    setVisible(true);
  }, [data]);

  function handleAnimationEnd() {
    setLeaving(true);
    setTimeout(() => setVisible(false), 300);
  }

  if (!visible || !data) return null;

  return createPortal(
    <div
      className={`dlx-toast dlx-toast--${data.type} ${leaving ? "dlx-toast--leave" : "dlx-toast--enter"}`}
      onMouseEnter={() => setPaused(true)}
      onMouseLeave={() => setPaused(false)}
    >
      <div className={`dlx-toast__bg dlx-toast__bg--${data.type}`} />
      <div
        ref={progressRef}
        className={`dlx-toast__progress dlx-toast__progress--${data.type}`}
        style={{
          animationDuration: `${data.duration}ms`,
          animationPlayState: paused ? "paused" : "running",
        }}
        onAnimationEnd={handleAnimationEnd}
      />
      <span className="dlx-toast__text">{data.message}</span>
    </div>,
    document.body
  );
}
