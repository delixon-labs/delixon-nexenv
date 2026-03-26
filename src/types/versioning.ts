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
