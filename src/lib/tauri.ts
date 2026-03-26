import { invoke, isTauri } from "@tauri-apps/api/core";
import type { Project, CreateProjectInput, RuntimeConfig } from "@/types/project";

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

let mockProjects = [...MOCK_PROJECTS];

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
      return Promise.resolve(newProject as T);
    }

    case "delete_project":
      mockProjects = mockProjects.filter((p) => p.id !== args?.id);
      return Promise.resolve(undefined as T);

    case "update_project": {
      const idx = mockProjects.findIndex((p) => p.id === args?.id);
      if (idx >= 0) {
        mockProjects[idx] = { ...mockProjects[idx], ...args };
        return Promise.resolve(mockProjects[idx] as T);
      }
      return Promise.reject("Proyecto no encontrado");
    }

    case "open_project": {
      const idx = mockProjects.findIndex((p) => p.id === args?.id);
      if (idx >= 0) mockProjects[idx].lastOpenedAt = new Date().toISOString();
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

    case "open_terminal":
    case "open_in_editor":
      console.info(`[mock] ${cmd} (simulado en navegador)`);
      return Promise.resolve(undefined as T);

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
    status?: string;
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

// --- Shell / Editor ---

export async function openTerminal(projectId: string): Promise<void> {
  return safeInvoke<void>("open_terminal", { projectId });
}

export async function openInEditor(projectPath: string, editor?: string): Promise<void> {
  return safeInvoke<void>("open_in_editor", { projectPath, editor });
}
