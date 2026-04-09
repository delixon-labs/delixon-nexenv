import { describe, it, expect, vi } from "vitest";

// Mock tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  isTauri: () => false,
}));

describe("Nexenv", () => {
  it("should load without errors", () => {
    expect(true).toBe(true);
  });
});
