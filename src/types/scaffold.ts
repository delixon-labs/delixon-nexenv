export interface ScaffoldConfig {
  name: string;
  projectType: string;
  profile: string;
  technologies: string[];
  path: string;
}

export interface PreviewFile {
  path: string;
  contentPreview: string;
}

export interface ValidationIssue {
  level: "error" | "warning" | "info";
  message: string;
  techId: string;
}

export interface ValidationResult {
  valid: boolean;
  issues: ValidationIssue[];
  resolvedDependencies: string[];
  portAssignments: Record<string, number>;
  suggestions: string[];
}

export interface ScaffoldPreview {
  files: PreviewFile[];
  validation: ValidationResult;
}
