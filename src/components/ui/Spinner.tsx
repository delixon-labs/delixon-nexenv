import { clsx } from "clsx";

/**
 * Spinner circular — para inputs, botones y bloques async.
 * Sizes: sm (16px), md (24px), lg (32px)
 */
export function Spinner({
  size = "md",
  className,
}: {
  size?: "sm" | "md" | "lg";
  className?: string;
}) {
  const s = size === "sm" ? "w-4 h-4" : size === "lg" ? "w-8 h-8" : "w-6 h-6";
  const stroke = size === "sm" ? "2.5" : "2";

  return (
    <svg
      className={clsx("animate-spin", s, className)}
      viewBox="0 0 24 24"
      fill="none"
    >
      <circle
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        strokeWidth={stroke}
        className="opacity-20"
      />
      <path
        d="M12 2a10 10 0 0 1 10 10"
        stroke="currentColor"
        strokeWidth={stroke}
        strokeLinecap="round"
      />
    </svg>
  );
}

/**
 * Barra de progreso de cuadrados — para carga de pagina completa.
 * Efecto tipo "bloques que se iluminan en secuencia".
 */
export function BlockProgress({ className }: { className?: string }) {
  return (
    <div className={clsx("flex items-center gap-1.5", className)}>
      {[0, 1, 2, 3, 4].map((i) => (
        <div
          key={i}
          className="w-2.5 h-2.5 rounded-xs bg-primary-500"
          style={{
            animation: "block-pulse 1.2s ease-in-out infinite",
            animationDelay: `${i * 0.15}s`,
          }}
        />
      ))}
    </div>
  );
}
