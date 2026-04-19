import type { ProjectManifest } from "@/lib/tauri";

export interface Snapshot {
  version: number;
  timestamp: string;
  manifest: ProjectManifest;
}

export interface SnapshotDiff {
  fromVersion: number;
  toVersion: number;
  addedTechs: string[];
  removedTechs: string[];
  addedRecipes: string[];
}

export interface RuntimeSnapshot {
  name: string;
  version: string;
}

export interface EnvSnapshot {
  timestamp: string;
  runtimes: RuntimeSnapshot[];
  depsHash: string;
}

export interface RuntimeChange {
  name: string;
  oldVersion: string;
  newVersion: string;
}

export interface EnvDiff {
  changedRuntimes: RuntimeChange[];
  depsChanged: boolean;
}

export interface RollbackPreview {
  targetVersion: number;
  targetTimestamp: string;
  currentManifestExists: boolean;
  addedTechs: string[];
  removedTechs: string[];
  addedRecipes: string[];
  removedRecipes: string[];
  profileChanged: [string, string] | null;
  editorChanged: [string | null, string | null] | null;
  nameChanged: [string, string] | null;
  runtimeChanged: [string, string] | null;
}
