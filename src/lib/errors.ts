export interface UiError {
  intento: string;
  detecto: string;
  fallo: string;
  hacer: string;
}

export function isUiError(value: unknown): value is UiError {
  if (typeof value !== "object" || value === null) return false;
  const v = value as Record<string, unknown>;
  return (
    typeof v.intento === "string" &&
    typeof v.detecto === "string" &&
    typeof v.fallo === "string" &&
    typeof v.hacer === "string"
  );
}

export function parseUiError(err: unknown): UiError | null {
  if (isUiError(err)) return err;
  return null;
}

export function formatUiError(err: UiError): string {
  const parts = [`Intento: ${err.intento}`];
  if (err.detecto) parts.push(`Detecto: ${err.detecto}`);
  if (err.fallo) parts.push(`Fallo: ${err.fallo}`);
  if (err.hacer) parts.push(`Que hacer: ${err.hacer}`);
  return parts.join("\n");
}

export function toErrorMessage(err: unknown, fallback = "Error desconocido"): string {
  const ui = parseUiError(err);
  if (ui) return formatUiError(ui);
  if (typeof err === "string") return err;
  if (err instanceof Error) return err.message;
  return fallback;
}
