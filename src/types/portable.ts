import type { RuntimeConfig } from "@/types/project";

export interface ImportPreview {
  name: string;
  description: string | null;
  runtimes: RuntimeConfig[];
  tags: string[];
  templateId: string | null;
  envKeys: string[];
  hasManifest: boolean;
  targetPath: string;
  targetExists: boolean;
  targetHasFiles: boolean;
  conflictWithExisting: boolean;
}
