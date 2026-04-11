import { useRef, useState } from "react";
import { createPortal } from "react-dom";
import "./Tooltip.css";

interface TooltipProps {
  text: string;
  position?: "top" | "bottom" | "left" | "right";
  children: React.ReactNode;
}

export default function Tooltip({
  text,
  position = "top",
  children,
}: TooltipProps) {
  const triggerRef = useRef<HTMLSpanElement>(null);
  const [visible, setVisible] = useState(false);
  const [coords, setCoords] = useState({ top: 0, left: 0 });

  function show() {
    const el = triggerRef.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const gap = 8;

    let top = 0;
    let left = 0;

    switch (position) {
      case "top":
        top = rect.top - gap;
        left = rect.left + rect.width / 2;
        break;
      case "bottom":
        top = rect.bottom + gap;
        left = rect.left + rect.width / 2;
        break;
      case "left":
        top = rect.top + rect.height / 2;
        left = rect.left - gap;
        break;
      case "right":
        top = rect.top + rect.height / 2;
        left = rect.right + gap;
        break;
    }

    setCoords({ top, left });
    setVisible(true);
  }

  function hide() {
    setVisible(false);
  }

  return (
    <>
      <span
        ref={triggerRef}
        onMouseEnter={show}
        onMouseLeave={hide}
        className="inline-flex"
      >
        {children}
      </span>
      {visible &&
        createPortal(
          <span
            className={`dlx-tooltip__bubble dlx-tooltip__bubble--${position} dlx-tooltip__bubble--visible`}
            role="tooltip"
            style={{ top: coords.top, left: coords.left }}
          >
            {text}
          </span>,
          document.body
        )}
    </>
  );
}
