export type Runtime = "node" | "python" | "rust" | "go" | "dotnet" | "php" | "ruby";

export type ProjectStatus = "active" | "idle" | "archived";

export interface RuntimeConfig {
  runtime: Runtime;
  version: string;
}

export interface Project {
  id: string;
  name: string;
  path: string;
  description?: string;
  runtimes: RuntimeConfig[];
  status: ProjectStatus;
  createdAt: string;
  lastOpenedAt?: string;
  templateId?: string;
  tags: string[];
}

export interface CreateProjectInput {
  name: string;
  path: string;
  description?: string;
  templateId?: string;
  runtimes: RuntimeConfig[];
  tags?: string[];
}
