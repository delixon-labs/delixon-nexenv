import { create } from "zustand";
import type { Project, CreateProjectInput } from "@/types/project";
import * as api from "@/lib/tauri";

interface ProjectsState {
  projects: Project[];
  isLoading: boolean;
  error: string | null;
  searchQuery: string;

  fetchProjects: () => Promise<void>;
  addProject: (input: CreateProjectInput) => Promise<Project>;
  removeProject: (id: string) => Promise<void>;
  openProject: (id: string) => Promise<void>;
  setSearchQuery: (query: string) => void;
}

export const useProjectsStore = create<ProjectsState>()((set, get) => ({
  projects: [],
  isLoading: false,
  error: null,
  searchQuery: "",

  fetchProjects: async () => {
    set({ isLoading: true, error: null });
    try {
      const projects = await api.listProjects();
      set({ projects, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  addProject: async (input: CreateProjectInput) => {
    const project = await api.createProject(input);
    set((state) => ({ projects: [...state.projects, project] }));
    return project;
  },

  removeProject: async (id: string) => {
    await api.deleteProject(id);
    set((state) => ({
      projects: state.projects.filter((p) => p.id !== id),
    }));
  },

  openProject: async (id: string) => {
    await api.openProject(id);
    // Refrescar para actualizar last_opened_at
    await get().fetchProjects();
  },

  setSearchQuery: (query: string) => {
    set({ searchQuery: query });
  },
}));
