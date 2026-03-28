import * as api from "@/lib/tauri";

interface PathInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  autoFocus?: boolean;
  onKeyDown?: (e: React.KeyboardEvent) => void;
  onBlur?: () => void;
}

/** Input de ruta con botón de explorar carpeta (nativo via Tauri). */
export default function PathInput({
  value,
  onChange,
  placeholder = "C:\\Users\\you\\projects\\my-app",
  autoFocus,
  onKeyDown,
  onBlur,
}: PathInputProps) {
  async function handleBrowse() {
    try {
      const selected = await api.pickFolder();
      if (selected) onChange(selected);
    } catch {
      // No-op si el usuario cancela
    }
  }

  return (
    <div className="flex gap-2">
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="flex-1 px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm font-mono placeholder-gray-600 focus:outline-none focus:border-primary-500"
        autoFocus={autoFocus}
        onKeyDown={onKeyDown}
        onBlur={onBlur}
      />
      <button
        type="button"
        onClick={handleBrowse}
        className="px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-gray-300 text-sm hover:bg-gray-700 hover:text-white transition-colors"
        title="Explorar"
      >
        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
          <path strokeLinecap="round" strokeLinejoin="round" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
      </button>
    </div>
  );
}
