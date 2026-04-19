import { useState, useEffect } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import logo from "@/assets/logos/Log-bt.png";
import "./Titlebar.css";

function detectOs(): "win" | "mac" | "linux" {
  const p = navigator.platform?.toLowerCase() || "";
  if (p.includes("mac")) return "mac";
  if (p.includes("linux")) return "linux";
  return "win";
}

const tauri = isTauri();

const safeWindow = () => (tauri ? getCurrentWindow() : null);

export default function Titlebar() {
  const [os] = useState(detectOs);
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    if (!tauri) return;
    const w = getCurrentWindow();
    w.isMaximized().then(setMaximized).catch(() => {});
    const unlisten = w.onResized(() => {
      w.isMaximized().then(setMaximized).catch(() => {});
    });
    return () => { unlisten.then((fn) => fn()); };
  }, []);

  const isMac = os === "mac";

  return (
    <div className={`titlebar ${isMac ? "titlebar--macos" : ""}`} data-tauri-drag-region>
      {/* Center: logo + title */}
      <div className="titlebar__center" data-tauri-drag-region>
        <img src={logo} alt="Nexenv" className="titlebar__logo" draggable={false} />
        <span className="titlebar__name" data-tauri-drag-region>Nexenv</span>
      </div>

      {/* Window controls (solo en Tauri, no en navegador) */}
      {!isMac && tauri && (
        <div className="titlebar__controls">
          <button className="titlebar__btn" onClick={() => safeWindow()?.minimize()}>
            <svg width="10" height="1" viewBox="0 0 10 1">
              <rect width="10" height="1" fill="#7a7a9a" />
            </svg>
          </button>
          <button className="titlebar__btn" onClick={() => safeWindow()?.toggleMaximize()}>
            {maximized ? (
              <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                <rect x="2" y="0" width="8" height="8" stroke="#7a7a9a" strokeWidth="1" fill="none" />
                <rect x="0" y="2" width="8" height="8" stroke="#7a7a9a" strokeWidth="1" fill="#22223a" />
              </svg>
            ) : (
              <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                <rect x="0.5" y="0.5" width="9" height="9" stroke="#7a7a9a" strokeWidth="1" />
              </svg>
            )}
          </button>
          <button className="titlebar__btn titlebar__btn--close" onClick={() => safeWindow()?.close()}>
            <svg width="10" height="10" viewBox="0 0 10 10">
              <line x1="1" y1="1" x2="9" y2="9" stroke="#7a7a9a" strokeWidth="1.2" />
              <line x1="9" y1="1" x2="1" y2="9" stroke="#7a7a9a" strokeWidth="1.2" />
            </svg>
          </button>
        </div>
      )}
    </div>
  );
}
