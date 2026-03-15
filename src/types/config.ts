export interface DelixonConfig {
  version: string;
  dataDir: string;
  defaultEditor: string;
  theme: "dark" | "light" | "system";
  language: "es" | "en";
  autoCheckUpdates: boolean;
}

export const DEFAULT_CONFIG: DelixonConfig = {
  version: "0.1.0",
  dataDir: "",
  defaultEditor: "code",
  theme: "dark",
  language: "es",
  autoCheckUpdates: true,
};
