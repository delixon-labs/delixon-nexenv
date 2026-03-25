import { invoke } from "@tauri-apps/api/core";
import type { Project, CreateProjectInput, RuntimeConfig } from "@/types/project";

// --- Proyectos ---

export async function listProjects(): Promise<Project[]> {
  return invoke<Project[]>("list_projects");
}

export async function getProject(id: string): Promise<Project> {
  return invoke<Project>("get_project", { id });
}

export async function createProject(input: CreateProjectInput): Promise<Project> {
  return invoke<Project>("create_project", { input });
}

export async function openProject(id: string): Promise<void> {
  return invoke<void>("open_project", { id });
}

export async function updateProject(
  id: string,
  updates: {
    name?: string;
    description?: string;
    runtimes?: RuntimeConfig[];
    status?: string;
    tags?: string[];
  }
): Promise<Project> {
  return invoke<Project>("update_project", { id, ...updates });
}

export async function deleteProject(id: string): Promise<void> {
  return invoke<void>("delete_project", { id });
}

// --- Variables de entorno ---

export async function getEnvVars(projectId: string): Promise<Record<string, string>> {
  return invoke<Record<string, string>>("get_env_vars", { projectId });
}

export async function setEnvVars(projectId: string, vars: Record<string, string>): Promise<void> {
  return invoke<void>("set_env_vars", { projectId, vars });
}

// --- Runtimes ---

export interface DetectedRuntime {
  name: string;
  version: string;
  path: string;
}

export async function detectRuntimes(): Promise<DetectedRuntime[]> {
  return invoke<DetectedRuntime[]>("detect_runtimes");
}

// --- Shell / Editor ---

export async function openTerminal(projectId: string): Promise<void> {
  return invoke<void>("open_terminal", { projectId });
}

export async function openInEditor(projectPath: string, editor?: string): Promise<void> {
  return invoke<void>("open_in_editor", { projectPath, editor });
}
