import { useRef, useState, useEffect, useCallback } from "react";

interface ScrollRowProps {
  children: React.ReactNode;
  className?: string;
  wrapperClassName?: string;
}

/**
 * Contenedor horizontal con carousel inteligente.
 * - Si todo cabe: sin scroll, sin flechas.
 * - Si no cabe: rueda del raton scrollea horizontal,
 *   flechas ‹ › aparecen en los bordes y desaparecen al llegar al limite.
 * - Cuando la rueda esta sobre el carousel, bloquea el scroll vertical de la pagina.
 */
export default function ScrollRow({ children, className = "", wrapperClassName = "" }: ScrollRowProps) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(false);

  const checkOverflow = useCallback(() => {
    const el = scrollRef.current;
    if (!el) return;
    const hasOverflow = el.scrollWidth > el.clientWidth + 1;
    setCanScrollLeft(hasOverflow && el.scrollLeft > 1);
    setCanScrollRight(hasOverflow && el.scrollLeft < el.scrollWidth - el.clientWidth - 1);
  }, []);

  useEffect(() => {
    checkOverflow();
    window.addEventListener("resize", checkOverflow);
    return () => window.removeEventListener("resize", checkOverflow);
  }, [checkOverflow]);

  // Listener nativo con passive:false para poder bloquear scroll vertical
  useEffect(() => {
    const el = scrollRef.current;
    if (!el) return;

    function onWheel(e: WheelEvent) {
      if (el!.scrollWidth <= el!.clientWidth) return;
      e.preventDefault();
      e.stopPropagation();
      const delta = e.deltaY !== 0 ? e.deltaY : e.deltaX;
      el!.scrollBy({ left: delta, behavior: "smooth" });
      setTimeout(checkOverflow, 100);
    }

    el.addEventListener("wheel", onWheel, { passive: false });
    return () => el.removeEventListener("wheel", onWheel);
  }, [checkOverflow]);

  function scrollByArrow(direction: number) {
    const el = scrollRef.current;
    if (!el) return;
    el.scrollBy({ left: direction * 120, behavior: "smooth" });
    setTimeout(checkOverflow, 200);
  }

  return (
    <div className={`relative ${wrapperClassName}`}>
      {canScrollLeft && (
        <button
          onClick={() => scrollByArrow(-1)}
          className="absolute left-0 top-0 bottom-0 z-20 flex items-center pl-1 pr-4 text-gray-400 hover:text-white transition-colors bg-linear-to-r from-gray-950 via-gray-950/90 to-transparent"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
      )}

      <div
        ref={scrollRef}
        onScroll={checkOverflow}
        className={`flex overflow-x-hidden ${className}`}
      >
        {children}
      </div>

      {canScrollRight && (
        <button
          onClick={() => scrollByArrow(1)}
          className="absolute right-0 top-0 bottom-0 z-20 flex items-center pr-1 pl-4 text-gray-400 hover:text-white transition-colors bg-linear-to-l from-gray-950 via-gray-950/90 to-transparent"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        </button>
      )}
    </div>
  );
}
