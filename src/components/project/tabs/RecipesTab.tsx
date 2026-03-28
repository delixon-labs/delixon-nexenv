import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { Recipe, RecipePreview, RecipeApplyResult } from "@/types/recipes";

export default function RecipesTab({ projectId }: { projectId: string; projectPath: string }) {
  const [recipes, setRecipes] = useState<Recipe[]>([]);
  const [loading, setLoading] = useState(true);
  const [preview, setPreview] = useState<RecipePreview | null>(null);
  const [applying, setApplying] = useState(false);
  const [result, setResult] = useState<RecipeApplyResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const r = await api.listRecipes();
      setRecipes(r);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { load(); }, [load]);

  async function handlePreview(recipeId: string) {
    setError(null);
    setResult(null);
    try {
      const p = await api.previewRecipe(projectId, recipeId);
      setPreview(p);
    } catch (err) {
      setError(String(err));
    }
  }

  async function handleApply(recipeId: string) {
    setApplying(true);
    setError(null);
    try {
      const r = await api.applyRecipe(projectId, recipeId);
      setResult(r);
      setPreview(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setApplying(false);
    }
  }

  if (loading) return <div className="flex justify-center py-12"><div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" /></div>;

  const categories = [...new Set(recipes.map((r) => r.category))];

  return (
    <div className="space-y-6">
      {error && (
        <div className="px-4 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">{error}</div>
      )}

      {result && (
        <div className="px-4 py-3 rounded-lg bg-green-500/10 border border-green-500/20 text-sm">
          <p className="text-green-400 font-medium mb-1">Recipe aplicada: {result.recipeId}</p>
          {result.filesCreated.length > 0 && (
            <p className="text-green-300 text-xs">Creados: {result.filesCreated.join(", ")}</p>
          )}
          {result.filesSkipped.length > 0 && (
            <p className="text-yellow-400 text-xs">Ya existian: {result.filesSkipped.join(", ")}</p>
          )}
          {result.envVarsAdded.length > 0 && (
            <p className="text-blue-400 text-xs">Env vars: {result.envVarsAdded.join(", ")}</p>
          )}
        </div>
      )}

      {categories.map((cat) => (
        <div key={cat}>
          <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">{cat}</h3>
          <div className="space-y-2">
            {recipes.filter((r) => r.category === cat).map((recipe) => (
              <div key={recipe.id} className="flex items-center justify-between px-4 py-3 rounded-lg bg-gray-900 border border-gray-800">
                <div>
                  <span className="text-sm font-medium text-white">{recipe.name}</span>
                  <p className="text-xs text-gray-500 mt-0.5">{recipe.description}</p>
                </div>
                <div className="flex gap-2 flex-shrink-0">
                  <button
                    onClick={() => handlePreview(recipe.id)}
                    className="px-3 py-1 rounded-lg bg-info/10 text-info-light text-xs hover:bg-info/20 transition-colors"
                  >
                    Preview
                  </button>
                  <button
                    onClick={() => handleApply(recipe.id)}
                    disabled={applying}
                    className="px-3 py-1 rounded-lg bg-success/10 text-success-light text-xs font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
                  >
                    {applying ? "..." : "Aplicar"}
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}

      {/* Preview modal */}
      {preview && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
          <div className="w-full max-w-lg rounded-2xl bg-gray-900 border border-gray-800 shadow-2xl">
            <div className="flex items-center justify-between px-6 py-4 border-b border-gray-800">
              <h2 className="text-lg font-semibold text-white">{preview.recipe.name}</h2>
              <button onClick={() => setPreview(null)} className="p-1 text-gray-500 hover:text-white">
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            <div className="px-6 py-4 space-y-3 max-h-96 overflow-auto">
              <p className="text-sm text-gray-400">{preview.recipe.description}</p>

              {preview.recipe.filesToCreate.length > 0 && (
                <div>
                  <h4 className="text-xs font-semibold text-gray-500 uppercase mb-1">Archivos</h4>
                  {preview.recipe.filesToCreate.map((f) => {
                    const exists = preview.filesThatExist.includes(f.path);
                    return (
                      <div key={f.path} className="flex items-center gap-2 text-sm py-0.5">
                        <span className={exists ? "text-yellow-400" : "text-green-400"}>{exists ? "~" : "+"}</span>
                        <span className="text-gray-300 font-mono text-xs">{f.path}</span>
                        {exists && <span className="text-yellow-500 text-xs">(ya existe)</span>}
                      </div>
                    );
                  })}
                </div>
              )}

              {preview.recipe.depsToInstall.length > 0 && (
                <div>
                  <h4 className="text-xs font-semibold text-gray-500 uppercase mb-1">Dependencias</h4>
                  <p className="text-sm text-gray-300">{preview.recipe.depsToInstall.join(", ")}</p>
                </div>
              )}

              {preview.recipe.devDepsToInstall.length > 0 && (
                <div>
                  <h4 className="text-xs font-semibold text-gray-500 uppercase mb-1">Dev Dependencies</h4>
                  <p className="text-sm text-gray-300">{preview.recipe.devDepsToInstall.join(", ")}</p>
                </div>
              )}

              {Object.keys(preview.recipe.envVarsToAdd).length > 0 && (
                <div>
                  <h4 className="text-xs font-semibold text-gray-500 uppercase mb-1">Variables de entorno</h4>
                  {Object.entries(preview.recipe.envVarsToAdd).map(([k, v]) => (
                    <p key={k} className="text-xs text-gray-400 font-mono">{k}={v}</p>
                  ))}
                </div>
              )}
            </div>
            <div className="flex justify-end gap-3 px-6 py-4 border-t border-gray-800">
              <button onClick={() => setPreview(null)} className="px-4 py-2 rounded-lg text-sm font-medium bg-dlx-light-3 text-dlx-text-light-1 border border-dlx-text-dark-3 hover:bg-dlx-text-dark-3 transition-colors">Cerrar</button>
              <button
                onClick={() => handleApply(preview.recipe.id)}
                disabled={applying}
                className="px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
              >
                {applying ? "Aplicando..." : "Aplicar recipe"}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
