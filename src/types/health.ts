export type HealthStatusLevel = "ok" | "warning" | "error";

export interface HealthCheck {
  name: string;
  status: HealthStatusLevel;
  message: string;
  fixSuggestion: string;
}

export interface HealthReport {
  projectId: string;
  projectName: string;
  overall: HealthStatusLevel;
  checks: HealthCheck[];
}

export interface DoctorCheck {
  name: string;
  ok: boolean;
  message: string;
}

export interface DoctorReport {
  checks: DoctorCheck[];
  overallOk: boolean;
}

export interface PortConflict {
  port: number;
  projects: string[];
  inUse: boolean;
}

export interface PortInfo {
  port: number;
  project: string;
  inUse: boolean;
}
