export interface NexenvConfig {
  version: string;
  dataDir: string;
  defaultEditor: string;
  theme: "dark" | "light" | "system";
  language: "es" | "en";
  fontPack: "system" | "classic";
  autoCheckUpdates: boolean;
}

export const DEFAULT_CONFIG: NexenvConfig = {
  version: "1.0.0",
  dataDir: "",
  defaultEditor: "code",
  theme: "dark",
  language: "es",
  fontPack: "system",
  autoCheckUpdates: true,
};
