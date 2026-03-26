export interface RecipeFile {
  path: string;
  content: string;
}

export interface Recipe {
  id: string;
  name: string;
  description: string;
  category: string;
  filesToCreate: RecipeFile[];
  depsToInstall: string[];
  devDepsToInstall: string[];
  envVarsToAdd: Record<string, string>;
  scriptsToAdd: Record<string, string>;
}

export interface RecipePreview {
  recipe: Recipe;
  filesThatExist: string[];
}

export interface RecipeApplyResult {
  recipeId: string;
  filesCreated: string[];
  filesSkipped: string[];
  envVarsAdded: string[];
}
