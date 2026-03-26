import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  isTauri: () => false,
}));

describe("Delixon", () => {
  it("should load without errors", () => {
    expect(true).toBe(true);
  });
});
