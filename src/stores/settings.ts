import { create } from "zustand";
import type { DelixonConfig } from "@/types/config";
import { DEFAULT_CONFIG } from "@/types/config";

interface SettingsState {
  config: DelixonConfig;
  sidebarCollapsed: boolean;

  setConfig: (config: Partial<DelixonConfig>) => void;
  toggleSidebar: () => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  config: DEFAULT_CONFIG,
  sidebarCollapsed: false,

  setConfig: (updates: Partial<DelixonConfig>) => {
    set((state) => ({
      config: { ...state.config, ...updates },
    }));
  },

  toggleSidebar: () => {
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
  },
}));
