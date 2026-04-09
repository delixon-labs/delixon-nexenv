import { invoke, isTauri } from "@tauri-apps/api/core";
import type { Project, CreateProjectInput, RuntimeConfig, ProjectStatus } from "@/types/project";
import type { NexenvConfig } from "@/types/config";
import { DEFAULT_CONFIG } from "@/types/config";
import type { HealthReport, DoctorReport, PortConflict, PortInfo } from "@/types/health";
import type { GitStatus, GitCommit } from "@/types/git";
import type { DockerComposeStatus } from "@/types/docker";
import type { ScriptResult } from "@/types/scripts";
import type { ProjectNote } from "@/types/notes";
import type { ProjectProcess } from "@/types/processes";
import type { Recipe, RecipePreview, RecipeApplyResult } from "@/types/recipes";
import type { Technology } from "@/types/catalog";
import type { ScaffoldConfig, ScaffoldPreview, ValidationResult } from "@/types/scaffold";
import type { Snapshot, SnapshotDiff, EnvSnapshot, EnvDiff } from "@/types/versioning";

// --- Mock data para desarrollo en navegador ---

const MOCK_PROJECTS: Project[] = [
  {
    id: "mock-1",
    name: "cliente-a/ecommerce",
    path: "D:/projects/cliente-a/ecommerce",
    description: "E-commerce con Next.js y Stripe",
    runtimes: [{ runtime: "node", version: "18.17" }],
    status: "active",
    createdAt: "2026-01-15T10:00:00Z",
    lastOpenedAt: "2026-03-25T09:30:00Z",
    tags: ["nextjs", "stripe", "postgresql"],
  },
  {
    id: "mock-2",
    name: "cliente-b/dashboard",
    path: "D:/projects/cliente-b/dashboard",
    description: "Dashboard interno con React + Vite",
    runtimes: [{ runtime: "node", version: "20.10" }],
    status: "active",
    createdAt: "2026-02-01T14:00:00Z",
    lastOpenedAt: "2026-03-24T16:00:00Z",
    tags: ["react", "vite", "tailwind"],
  },
  {
    id: "mock-3",
    name: "ml-pipeline",
    path: "D:/projects/ml-pipeline",
    description: "Pipeline de ML con FastAPI",
    runtimes: [{ runtime: "python", version: "3.11" }],
    status: "idle",
    createdAt: "2025-11-20T08:00:00Z",
    tags: ["fastapi", "pytorch", "docker"],
  },
];

const MOCK_ENV_VARS: Record<string, Record<string, string>> = {
  "mock-1": { DATABASE_URL: "postgresql://localhost:5432/ecommerce", STRIPE_KEY: "sk_test_xxx", PORT: "3000" },
  "mock-2": { VITE_API_URL: "http://localhost:8080", PORT: "5173" },
  "mock-3": { DATABASE_URL: "postgresql://localhost:5432/ml", FLASK_ENV: "development" },
};

function loadMockProjects(): Project[] {
  try {
    const stored = localStorage.getItem("nexenv_mock_projects");
    if (stored) return JSON.parse(stored);
  } catch { /* fallback */ }
  return [...MOCK_PROJECTS];
}

function saveMockProjects(projects: Project[]) {
  localStorage.setItem("nexenv_mock_projects", JSON.stringify(projects));
}

let mockProjects = loadMockProjects();

// --- Invoke seguro con fallback mock ---

function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri()) {
    return invoke<T>(cmd, args);
  }
  console.info(`[mock] ${cmd}`, args ?? "");
  return mockInvoke<T>(cmd, args);
}

function mockInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  switch (cmd) {
    case "list_projects":
      return Promise.resolve(mockProjects as T);

    case "get_project": {
      const p = mockProjects.find((p) => p.id === args?.id);
      return p ? Promise.resolve(p as T) : Promise.reject("Proyecto no encontrado");
    }

    case "create_project": {
      const input = args?.input as CreateProjectInput;
      const newProject: Project = {
        id: `mock-${Date.now()}`,
        name: input.name,
        path: input.path,
        description: input.description,
        runtimes: input.runtimes,
        status: "active",
        createdAt: new Date().toISOString(),
        tags: input.tags ?? [],
      };
      mockProjects.push(newProject);
      saveMockProjects(mockProjects);
      return Promise.resolve(newProject as T);
    }

    case "delete_project":
      mockProjects = mockProjects.filter((p) => p.id !== args?.id);
      saveMockProjects(mockProjects);
      return Promise.resolve(undefined as T);

    case "update_project": {
      const idx = mockProjects.findIndex((p) => p.id === args?.id);
      if (idx >= 0) {
        mockProjects[idx] = { ...mockProjects[idx], ...args };
        saveMockProjects(mockProjects);
        return Promise.resolve(mockProjects[idx] as T);
      }
      return Promise.reject("Proyecto no encontrado");
    }

    case "open_project": {
      const idx = mockProjects.findIndex((p) => p.id === args?.id);
      if (idx >= 0) {
        mockProjects[idx].lastOpenedAt = new Date().toISOString();
        saveMockProjects(mockProjects);
      }
      console.info("[mock] Abriendo proyecto en VSCode (simulado)");
      return Promise.resolve(undefined as T);
    }

    case "get_env_vars": {
      const vars = MOCK_ENV_VARS[args?.projectId as string] ?? {};
      return Promise.resolve(vars as T);
    }

    case "set_env_vars":
      if (args?.projectId) MOCK_ENV_VARS[args.projectId as string] = args.vars as Record<string, string>;
      return Promise.resolve(undefined as T);

    case "detect_runtimes":
      return Promise.resolve([
        { name: "Node.js", version: "20.10.0", path: "C:/Program Files/nodejs/node.exe" },
        { name: "Python", version: "3.11.5", path: "C:/Python311/python.exe" },
        { name: "Rust", version: "1.94.0", path: "C:/Users/user/.cargo/bin/rustc.exe" },
      ] as T);

    case "get_manifest":
      return Promise.resolve({ name: "mock", projectType: "api", profile: "standard", runtime: "node", technologies: ["node"], services: [], envVars: { required: [], optional: [] }, commands: { dev: "npm run dev" }, ports: [3000], recipesApplied: [], healthChecks: [] } as T);

    case "regenerate_manifest":
      return Promise.resolve({ name: "mock", projectType: "api", profile: "standard", runtime: "node", technologies: ["node"], services: [], envVars: { required: [], optional: [] }, commands: { dev: "npm run dev" }, ports: [3000], recipesApplied: [], healthChecks: [] } as T);

    case "create_from_template": {
      const tplProject: Project = {
        id: `mock-tpl-${Date.now()}`,
        name: (args?.name as string) || "new-project",
        path: (args?.path as string) || "/tmp/new-project",
        description: `Creado desde plantilla: ${args?.templateId}`,
        runtimes: [],
        status: "active",
        createdAt: new Date().toISOString(),
        tags: [],
      };
      mockProjects.push(tplProject);
      saveMockProjects(mockProjects);
      return Promise.resolve(tplProject as T);
    }

    case "detect_project_stack":
      return Promise.resolve({ runtimes: [], tags: [], packageManager: null, orm: null, auth: null, ci: null, testing: null, docker: null, linter: null, isFullstack: false, hasEnvExample: false, hasReadme: false, hasTypes: false, readinessScore: { total: 0, max: 10, breakdown: [], suggestions: [] } } as T);

    case "scan_and_register": {
      const regProject: Project = {
        id: `mock-reg-${Date.now()}`,
        name: (args?.name as string) || "registered",
        path: (args?.path as string) || "/tmp/registered",
        runtimes: [],
        status: "active",
        createdAt: new Date().toISOString(),
        tags: [],
      };
      mockProjects.push(regProject);
      saveMockProjects(mockProjects);
      return Promise.resolve(regProject as T);
    }

    case "get_config":
      return Promise.resolve(DEFAULT_CONFIG as T);

    case "set_config":
      console.info("[mock] Config guardada (simulado)");
      return Promise.resolve(undefined as T);

    case "export_project":
      return Promise.resolve('{"version":"1","exportedAt":"2026-01-01","project":{"name":"mock","runtimes":[],"tags":[],"envKeys":[]}}' as T);

    case "import_project":
      return Promise.resolve({ id: `mock-import-${Date.now()}`, name: "imported", path: "/tmp/imported", runtimes: [], status: "active", createdAt: new Date().toISOString(), tags: [] } as T);

    case "generate_vscode_workspace":
      return Promise.resolve({ filesCreated: ["proyecto.code-workspace", ".vscode/tasks.json", ".vscode/launch.json", ".vscode/extensions.json"], filesSkipped: [], warnings: [] } as T);

    case "open_terminal":
    case "open_in_editor":
      console.info(`[mock] ${cmd} (simulado en navegador)`);
      return Promise.resolve(undefined as T);

    case "list_installed_editors":
      return Promise.resolve([["code", "VS Code"], ["cursor", "Cursor"]] as T);

    case "check_project_health":
      return Promise.resolve({ projectId: args?.projectId ?? "", projectName: "mock", overall: "ok", checks: [{ name: "Directorio", status: "ok", message: "Existe", fixSuggestion: "" }] } as T);

    case "run_doctor":
      return Promise.resolve({ checks: [{ name: "Git", ok: true, message: "git version 2.43" }], overallOk: true } as T);

    case "detect_port_conflicts":
      return Promise.resolve([] as T);

    case "list_project_ports":
      return Promise.resolve([] as T);

    case "git_status":
      return Promise.resolve({ branch: "main", isClean: true, modifiedFiles: 0, untrackedFiles: 0, ahead: 0, behind: 0, hasRemote: true, lastCommit: null } as T);

    case "git_log":
      return Promise.resolve([] as T);

    case "docker_status":
      return Promise.resolve({ hasCompose: false, services: [], composeFile: "" } as T);

    case "docker_up":
    case "docker_down":
      return Promise.resolve("ok" as T);

    case "docker_logs":
      return Promise.resolve("" as T);

    case "list_project_scripts":
      return Promise.resolve([] as T);

    case "run_project_script":
      return Promise.resolve({ script: "", command: "", exitCode: 0, stdout: "", stderr: "" } as T);

    case "get_notes":
      return Promise.resolve([] as T);

    case "add_note":
      return Promise.resolve({ id: `mock-note-${Date.now()}`, text: args?.text ?? "", createdAt: new Date().toISOString() } as T);

    case "delete_note":
      return Promise.resolve(undefined as T);

    case "list_project_processes":
      return Promise.resolve([] as T);

    case "kill_process":
      return Promise.resolve(undefined as T);

    case "list_recipes":
      return Promise.resolve([] as T);

    case "preview_recipe":
      return Promise.resolve({ recipe: { id: "", name: "", description: "", category: "", filesToCreate: [], depsToInstall: [], devDepsToInstall: [], envVarsToAdd: {}, scriptsToAdd: {} }, filesThatExist: [] } as T);

    case "apply_recipe":
      return Promise.resolve({ recipeId: args?.recipeId ?? "", filesCreated: [], filesSkipped: [], envVarsAdded: [] } as T);

    case "list_catalog":
      return Promise.resolve([] as T);

    case "get_catalog_tech":
      return Promise.resolve(null as T);

    case "list_catalog_categories":
      return Promise.resolve([] as T);

    case "validate_stack":
      return Promise.resolve({ valid: true, issues: [], resolvedDependencies: [], portAssignments: {}, suggestions: [] } as T);

    case "preview_scaffold":
      return Promise.resolve({ files: [], validation: { valid: true, issues: [], resolvedDependencies: [], portAssignments: {}, suggestions: [] } } as T);

    case "generate_scaffold":
      return Promise.resolve({ id: `mock-scaffold-${Date.now()}`, name: args?.name ?? "", path: "", runtimes: [], status: "active", createdAt: new Date().toISOString(), tags: [] } as T);

    case "save_snapshot":
      return Promise.resolve({ version: 1, timestamp: new Date().toISOString(), manifest: {} } as T);

    case "list_snapshots":
      return Promise.resolve([] as T);

    case "diff_snapshots":
      return Promise.resolve({ fromVersion: 0, toVersion: 0, addedTechs: [], removedTechs: [], addedRecipes: [] } as T);

    case "rollback_snapshot":
      return Promise.resolve(undefined as T);

    case "take_env_snapshot":
      return Promise.resolve({ timestamp: new Date().toISOString(), runtimes: [], depsHash: "" } as T);

    case "diff_env_snapshot":
      return Promise.resolve(null as T);

    default:
      return Promise.reject(`[mock] Comando no implementado: ${cmd}`);
  }
}

// --- Proyectos ---

export async function listProjects(): Promise<Project[]> {
  return safeInvoke<Project[]>("list_projects");
}

export async function getProject(id: string): Promise<Project> {
  return safeInvoke<Project>("get_project", { id });
}

export async function createProject(input: CreateProjectInput): Promise<Project> {
  return safeInvoke<Project>("create_project", { input });
}

export async function openProject(id: string): Promise<void> {
  return safeInvoke<void>("open_project", { id });
}

export async function updateProject(
  id: string,
  updates: {
    name?: string;
    description?: string;
    runtimes?: RuntimeConfig[];
    status?: ProjectStatus;
    tags?: string[];
  }
): Promise<Project> {
  return safeInvoke<Project>("update_project", { id, ...updates });
}

export async function deleteProject(id: string): Promise<void> {
  return safeInvoke<void>("delete_project", { id });
}

// --- Variables de entorno ---

export async function getEnvVars(projectId: string): Promise<Record<string, string>> {
  return safeInvoke<Record<string, string>>("get_env_vars", { projectId });
}

export async function setEnvVars(projectId: string, vars: Record<string, string>): Promise<void> {
  return safeInvoke<void>("set_env_vars", { projectId, vars });
}

// --- Runtimes ---

export interface DetectedRuntime {
  name: string;
  version: string;
  path: string;
}

export async function detectRuntimes(): Promise<DetectedRuntime[]> {
  return safeInvoke<DetectedRuntime[]>("detect_runtimes");
}

// --- Manifest ---

export interface ProjectManifest {
  name: string;
  projectType: string;
  profile: string;
  runtime: string;
  technologies: string[];
  services: { name: string; docker: boolean; port: number; healthCheck: string }[];
  envVars: { required: string[]; optional: string[] };
  commands: Record<string, string>;
  ports: number[];
  recipesApplied: string[];
  healthChecks: { name: string; command: string; endpoint: string }[];
}

export async function getManifest(projectId: string): Promise<ProjectManifest | null> {
  return safeInvoke<ProjectManifest | null>("get_manifest", { projectId });
}

export async function regenerateManifest(projectId: string): Promise<ProjectManifest> {
  return safeInvoke<ProjectManifest>("regenerate_manifest", { projectId });
}

// --- Templates ---

export async function createFromTemplate(
  templateId: string,
  path: string,
  name: string
): Promise<Project> {
  return safeInvoke<Project>("create_from_template", { templateId, path, name });
}

// --- Detection ---

export interface ScoreItem {
  name: string;
  points: number;
  maxPoints: number;
  present: boolean;
}

export interface ReadinessScore {
  total: number;
  max: number;
  breakdown: ScoreItem[];
  suggestions: string[];
}

export interface DockerInfo {
  hasDockerfile: boolean;
  hasCompose: boolean;
}

export interface DetectedStack {
  runtimes: { runtime: string; version: string }[];
  tags: string[];
  packageManager: string | null;
  orm: string | null;
  auth: string | null;
  ci: string | null;
  testing: string | null;
  docker: DockerInfo | null;
  linter: string | null;
  isFullstack: boolean;
  hasEnvExample: boolean;
  hasReadme: boolean;
  hasTypes: boolean;
  readinessScore: ReadinessScore;
}

export async function detectProjectStack(path: string): Promise<DetectedStack> {
  return safeInvoke<DetectedStack>("detect_project_stack", { path });
}

export async function scanAndRegister(path: string, name: string): Promise<Project> {
  return safeInvoke<Project>("scan_and_register", { path, name });
}

// --- Config ---

export async function getConfig(): Promise<NexenvConfig> {
  return safeInvoke<NexenvConfig>("get_config");
}

export async function setConfig(config: NexenvConfig): Promise<void> {
  return safeInvoke<void>("set_config", { config });
}

// --- Portable (export/import) ---

export async function exportProject(projectId: string): Promise<string> {
  return safeInvoke<string>("export_project", { projectId });
}

export async function importProject(json: string, targetPath: string): Promise<Project> {
  return safeInvoke<Project>("import_project", { json, targetPath });
}

// --- VSCode Workspace ---

export interface VscodeGenerationResult {
  filesCreated: string[];
  filesSkipped: string[];
  warnings: string[];
}

export async function generateVscodeWorkspace(projectId: string): Promise<VscodeGenerationResult> {
  return safeInvoke<VscodeGenerationResult>("generate_vscode_workspace", { projectId });
}

// --- Shell / Editor ---

export async function openTerminal(projectId: string): Promise<void> {
  return safeInvoke<void>("open_terminal", { projectId });
}

export async function openInEditor(projectPath: string, editor?: string): Promise<void> {
  return safeInvoke<void>("open_in_editor", { projectPath, editor });
}

export interface InstalledEditor {
  cmd: string;
  label: string;
}

export async function listInstalledEditors(): Promise<InstalledEditor[]> {
  const tuples = await safeInvoke<[string, string][]>("list_installed_editors");
  return tuples.map(([cmd, label]) => ({ cmd, label }));
}

// --- Health ---

export async function checkProjectHealth(projectId: string): Promise<HealthReport> {
  return safeInvoke<HealthReport>("check_project_health", { projectId });
}

export async function runDoctor(): Promise<DoctorReport> {
  return safeInvoke<DoctorReport>("run_doctor");
}

export async function detectPortConflicts(): Promise<PortConflict[]> {
  return safeInvoke<PortConflict[]>("detect_port_conflicts");
}

export async function listProjectPorts(): Promise<PortInfo[]> {
  return safeInvoke<PortInfo[]>("list_project_ports");
}

// --- Git ---

export async function gitStatus(projectId: string): Promise<GitStatus> {
  return safeInvoke<GitStatus>("git_status", { projectId });
}

export async function gitLog(projectId: string, count: number): Promise<GitCommit[]> {
  return safeInvoke<GitCommit[]>("git_log", { projectId, count });
}

// --- Docker ---

export async function dockerStatus(projectId: string): Promise<DockerComposeStatus> {
  return safeInvoke<DockerComposeStatus>("docker_status", { projectId });
}

export async function dockerUp(projectId: string): Promise<string> {
  return safeInvoke<string>("docker_up", { projectId });
}

export async function dockerDown(projectId: string): Promise<string> {
  return safeInvoke<string>("docker_down", { projectId });
}

export async function dockerLogs(projectId: string, lines: number): Promise<string> {
  return safeInvoke<string>("docker_logs", { projectId, lines });
}

// --- Scripts ---

export async function listProjectScripts(projectId: string): Promise<[string, string][]> {
  return safeInvoke<[string, string][]>("list_project_scripts", { projectId });
}

export async function runProjectScript(projectId: string, scriptName: string): Promise<ScriptResult> {
  return safeInvoke<ScriptResult>("run_project_script", { projectId, scriptName });
}

// --- Notes ---

export async function getNotes(projectId: string): Promise<ProjectNote[]> {
  return safeInvoke<ProjectNote[]>("get_notes", { projectId });
}

export async function addNote(projectId: string, text: string): Promise<ProjectNote> {
  return safeInvoke<ProjectNote>("add_note", { projectId, text });
}

export async function deleteNote(projectId: string, noteId: string): Promise<void> {
  return safeInvoke<void>("delete_note", { projectId, noteId });
}

// --- Processes ---

export async function listProjectProcesses(projectId: string): Promise<ProjectProcess[]> {
  return safeInvoke<ProjectProcess[]>("list_project_processes", { projectId });
}

export async function killProcess(pid: number, projectId: string): Promise<void> {
  return safeInvoke<void>("kill_process", { pid, projectId });
}

// --- Recipes ---

export async function listRecipes(): Promise<Recipe[]> {
  return safeInvoke<Recipe[]>("list_recipes");
}

export async function previewRecipe(projectId: string, recipeId: string): Promise<RecipePreview> {
  return safeInvoke<RecipePreview>("preview_recipe", { projectId, recipeId });
}

export async function applyRecipe(projectId: string, recipeId: string): Promise<RecipeApplyResult> {
  return safeInvoke<RecipeApplyResult>("apply_recipe", { projectId, recipeId });
}

// --- Catalog ---

export async function listCatalog(): Promise<Technology[]> {
  return safeInvoke<Technology[]>("list_catalog");
}

export async function getCatalogTech(id: string): Promise<Technology | null> {
  return safeInvoke<Technology | null>("get_catalog_tech", { id });
}

export async function listCatalogCategories(): Promise<string[]> {
  return safeInvoke<string[]>("list_catalog_categories");
}

// --- Rules ---

export async function validateStack(technologies: string[]): Promise<ValidationResult> {
  return safeInvoke<ValidationResult>("validate_stack", { technologies });
}

// --- Scaffold ---

export async function previewScaffold(config: ScaffoldConfig): Promise<ScaffoldPreview> {
  return safeInvoke<ScaffoldPreview>("preview_scaffold", { config });
}

export async function generateScaffold(config: ScaffoldConfig): Promise<Project> {
  return safeInvoke<Project>("generate_scaffold", { config });
}

// --- Versioning ---

export async function saveSnapshot(projectId: string): Promise<Snapshot> {
  return safeInvoke<Snapshot>("save_snapshot", { projectId });
}

export async function listSnapshots(projectId: string): Promise<Snapshot[]> {
  return safeInvoke<Snapshot[]>("list_snapshots", { projectId });
}

export async function diffSnapshots(projectId: string, v1: number, v2: number): Promise<SnapshotDiff> {
  return safeInvoke<SnapshotDiff>("diff_snapshots", { projectId, v1, v2 });
}

export async function rollbackSnapshot(projectId: string, version: number): Promise<void> {
  return safeInvoke<void>("rollback_snapshot", { projectId, version });
}

// --- Env Snapshots ---

export async function takeEnvSnapshot(projectId: string): Promise<EnvSnapshot> {
  return safeInvoke<EnvSnapshot>("take_env_snapshot", { projectId });
}

export async function diffEnvSnapshot(projectId: string): Promise<EnvDiff | null> {
  return safeInvoke<EnvDiff | null>("diff_env_snapshot", { projectId });
}

// --- Dialog (folder picker) ---

export async function pickFolder(): Promise<string | null> {
  if (isTauri()) {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({ directory: true, multiple: false });
    return selected as string | null;
  }
  // Mock: prompt en navegador
  return prompt("Ruta del directorio:");
}
