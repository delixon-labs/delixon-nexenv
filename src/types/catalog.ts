export interface TechVersion {
  version: string;
  lts: boolean;
}

export interface TechHealthCheck {
  endpoint: string;
  command: string;
  interval: string;
  timeout: string;
  retries: number;
}

export interface Technology {
  id: string;
  name: string;
  category: string;
  description: string;
  website: string;
  versions: TechVersion[];
  defaultVersion: string;
  defaultPort: number;
  requires: string[];
  incompatibleWith: string[];
  suggestedWith: string[];
  officialScaffold: string | null;
  dockerImage: string;
  healthCheck: TechHealthCheck | null;
  envVars: Record<string, string>;
  configFiles: string[];
  tags: string[];
}
