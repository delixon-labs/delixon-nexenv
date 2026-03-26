import { create } from "zustand";
import type { DelixonConfig } from "@/types/config";
import { DEFAULT_CONFIG } from "@/types/config";
import * as api from "@/lib/tauri";

interface SettingsState {
  config: DelixonConfig;
  sidebarCollapsed: boolean;
  isLoaded: boolean;

  loadConfig: () => Promise<void>;
  setConfig: (updates: Partial<DelixonConfig>) => void;
  toggleSidebar: () => void;
}

let saveTimeout: ReturnType<typeof setTimeout> | null = null;

export const useSettingsStore = create<SettingsState>()((set) => ({
  config: DEFAULT_CONFIG,
  sidebarCollapsed: false,
  isLoaded: false,

  loadConfig: async () => {
    try {
      const config = await api.getConfig();
      set({ config, isLoaded: true });
    } catch {
      set({ isLoaded: true });
    }
  },

  setConfig: (updates: Partial<DelixonConfig>) => {
    set((state) => {
      const newConfig = { ...state.config, ...updates };

      // Auto-save con debounce de 500ms
      if (saveTimeout) clearTimeout(saveTimeout);
      saveTimeout = setTimeout(() => {
        api.setConfig(newConfig).catch(() => {});
      }, 500);

      return { config: newConfig };
    });
  },

  toggleSidebar: () => {
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
  },
}));
