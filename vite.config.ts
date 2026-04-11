import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  define: {
    __APP_VERSION__: JSON.stringify(
      (await import("./package.json", { with: { type: "json" } })).default
        .version
    ),
  },
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  // Tauri espera un puerto fijo en desarrollo
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // En Windows, usar polling para detectar cambios de archivos
      usePolling: true,
    },
  },
  // Variables de entorno expuestas al frontend
  envPrefix: ["VITE_", "TAURI_"],
  // Tests: jsdom provee localStorage, document, window (entorno navegador)
  test: {
    environment: "jsdom",
  },
}));
