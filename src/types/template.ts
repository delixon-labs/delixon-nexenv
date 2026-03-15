import type { Runtime } from "./project";

export interface Template {
  id: string;
  name: string;
  description: string;
  runtimes: Runtime[];
  tags: string[];
  isOfficial: boolean;
  author?: string;
  version: string;
  previewImage?: string;
}
