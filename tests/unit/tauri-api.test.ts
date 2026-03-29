import { describe, it, expect, vi } from "vitest";

// Mock the tauri core module
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  isTauri: () => false,
}));

import * as api from "@/lib/tauri";

describe("Tauri API (mock mode)", () => {
  describe("Projects", () => {
    it("listProjects returns mock projects", async () => {
      const projects = await api.listProjects();
      expect(Array.isArray(projects)).toBe(true);
      expect(projects.length).toBeGreaterThan(0);
      expect(projects[0]).toHaveProperty("id");
      expect(projects[0]).toHaveProperty("name");
      expect(projects[0]).toHaveProperty("path");
    });

    it("getProject returns a project by id", async () => {
      const project = await api.getProject("mock-1");
      expect(project.id).toBe("mock-1");
      expect(project.name).toBe("cliente-a/ecommerce");
    });

    it("getProject rejects for nonexistent id", async () => {
      await expect(api.getProject("nonexistent")).rejects.toBeDefined();
    });

    it("createProject creates and returns project", async () => {
      const input = {
        name: "test-project",
        path: "/tmp/test",
        runtimes: [{ runtime: "node" as const, version: "20" }],
      };
      const project = await api.createProject(input);
      expect(project.name).toBe("test-project");
      expect(project.status).toBe("active");
      expect(project.id).toContain("mock-");
    });

    it("deleteProject removes a project", async () => {
      const before = await api.listProjects();
      const count = before.length;
      await api.deleteProject(before[0].id);
      const after = await api.listProjects();
      expect(after.length).toBe(count - 1);
    });
  });

  describe("Env Vars", () => {
    it("getEnvVars returns vars for mock project", async () => {
      const vars = await api.getEnvVars("mock-1");
      expect(typeof vars).toBe("object");
    });

    it("getEnvVars returns empty for unknown project", async () => {
      const vars = await api.getEnvVars("unknown");
      expect(Object.keys(vars).length).toBe(0);
    });
  });

  describe("Runtimes", () => {
    it("detectRuntimes returns array", async () => {
      const runtimes = await api.detectRuntimes();
      expect(Array.isArray(runtimes)).toBe(true);
      expect(runtimes.length).toBeGreaterThan(0);
      expect(runtimes[0]).toHaveProperty("name");
      expect(runtimes[0]).toHaveProperty("version");
      expect(runtimes[0]).toHaveProperty("path");
    });
  });

  describe("Config", () => {
    it("getConfig returns default config", async () => {
      const config = await api.getConfig();
      expect(config.defaultEditor).toBe("code");
      expect(config.theme).toBe("dark");
      expect(config.language).toBe("es");
    });

    it("setConfig does not throw", async () => {
      const config = await api.getConfig();
      await expect(api.setConfig(config)).resolves.toBeUndefined();
    });
  });

  describe("Detection", () => {
    it("detectProjectStack returns empty for mock", async () => {
      const stack = await api.detectProjectStack("/tmp/fake");
      expect(stack).toHaveProperty("runtimes");
      expect(stack).toHaveProperty("tags");
    });
  });

  describe("Templates", () => {
    it("createFromTemplate returns project", async () => {
      const project = await api.createFromTemplate("node-express", "/tmp/tpl", "my-app");
      expect(project.name).toBe("my-app");
      expect(project.status).toBe("active");
    });
  });

  describe("Portable", () => {
    it("exportProject returns JSON string", async () => {
      const json = await api.exportProject("mock-1");
      expect(typeof json).toBe("string");
      const parsed = JSON.parse(json);
      expect(parsed).toHaveProperty("version");
    });

    it("importProject returns project", async () => {
      const json = '{"version":"1","exportedAt":"2026-01-01","project":{"name":"imported","runtimes":[],"tags":[],"envKeys":[]}}';
      const project = await api.importProject(json, "/tmp/import");
      expect(project.name).toBe("imported");
    });
  });

  describe("Shell/Editor (mock)", () => {
    it("openTerminal does not throw", async () => {
      await expect(api.openTerminal("mock-1")).resolves.toBeUndefined();
    });

    it("openInEditor does not throw", async () => {
      await expect(api.openInEditor("/tmp")).resolves.toBeUndefined();
    });

    it("generateVscodeWorkspace returns result", async () => {
      const result = await api.generateVscodeWorkspace("mock-1");
      expect(result).toBeDefined();
      expect(result.filesCreated).toBeInstanceOf(Array);
    });
  });
});
