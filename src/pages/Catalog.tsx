import { useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { Technology } from "@/types/catalog";
import { CATEGORY_LABELS } from "@/lib/catalog";

export default function Catalog() {
  const [techs, setTechs] = useState<Technology[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [activeCategory, setActiveCategory] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedTech, setSelectedTech] = useState<Technology | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      try {
        const [t, c] = await Promise.all([
          api.listCatalog(),
          api.listCatalogCategories(),
        ]);
        setTechs(t);
        setCategories(c);
      } catch (err) {
        setLoadError(String(err));
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const filtered = techs.filter((t) => {
    if (activeCategory && t.category !== activeCategory) return false;
    if (!searchQuery) return true;
    const q = searchQuery.toLowerCase();
    return (
      t.name.toLowerCase().includes(q) ||
      t.id.toLowerCase().includes(q) ||
      t.description.toLowerCase().includes(q) ||
      t.tags.some((tag) => tag.toLowerCase().includes(q))
    );
  });

  const categoryLabels = CATEGORY_LABELS;

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="w-8 h-8 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" />
      </div>
    );
  }

  return (
    <div className="p-6 lg:p-8 max-w-7xl">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-white">Catalogo de tecnologias</h1>
        <p className="text-sm text-gray-500 mt-1">{techs.length} tecnologias disponibles</p>
      </div>

      {loadError && (
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm mb-4">
          Error cargando catalogo: {loadError}
        </div>
      )}

      {/* Search + Category filter */}
      <div className="flex flex-col sm:flex-row gap-3 mb-6">
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Buscar tecnologia..."
          className="flex-1 max-w-md px-4 py-2 rounded-lg bg-gray-900 border border-gray-800 text-white placeholder-gray-600 text-sm focus:outline-none focus:border-primary-500/50"
        />
        <div className="flex gap-1 flex-wrap">
          <button
            onClick={() => setActiveCategory(null)}
            className={`px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${!activeCategory ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-800 text-gray-400 hover:text-white"}`}
          >
            Todas
          </button>
          {categories.map((cat) => (
            <button
              key={cat}
              onClick={() => setActiveCategory(cat)}
              className={`px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${activeCategory === cat ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-800 text-gray-400 hover:text-white"}`}
            >
              {categoryLabels[cat] || cat}
            </button>
          ))}
        </div>
      </div>

      {/* Grid */}
      {filtered.length === 0 ? (
        <p className="text-gray-500 text-center py-12">No se encontraron tecnologias</p>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {filtered.map((tech) => (
            <button
              key={tech.id}
              onClick={() => setSelectedTech(tech)}
              className="text-left px-4 py-4 rounded-xl bg-gray-900 border border-gray-800 hover:border-primary-500/30 transition-colors"
            >
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-semibold text-white">{tech.name}</span>
                {tech.defaultPort > 0 && (
                  <span className="text-xs text-gray-600 font-mono">:{tech.defaultPort}</span>
                )}
              </div>
              <p className="text-xs text-gray-500 line-clamp-2 mb-3">{tech.description}</p>
              <div className="flex items-center gap-2">
                <span className="text-xs px-2 py-0.5 rounded bg-gray-800 text-gray-400">{categoryLabels[tech.category] || tech.category}</span>
                {tech.defaultVersion && (
                  <span className="text-xs text-gray-600">v{tech.defaultVersion}</span>
                )}
              </div>
            </button>
          ))}
        </div>
      )}

      {/* Detail modal */}
      {selectedTech && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm" onClick={() => setSelectedTech(null)}>
          <div className="w-full max-w-lg rounded-2xl bg-gray-900 border border-gray-800 shadow-2xl" onClick={(e) => e.stopPropagation()}>
            <div className="flex items-center justify-between px-6 py-4 border-b border-gray-800">
              <div>
                <h2 className="text-lg font-semibold text-white">{selectedTech.name}</h2>
                <span className="text-xs text-gray-500">{selectedTech.id}</span>
              </div>
              <button onClick={() => setSelectedTech(null)} className="p-1 text-gray-500 hover:text-white">
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            <div className="px-6 py-4 space-y-4 max-h-96 overflow-auto">
              <p className="text-sm text-gray-300">{selectedTech.description}</p>

              <div className="grid grid-cols-2 gap-3">
                {selectedTech.defaultVersion && <DetailField label="Version" value={selectedTech.defaultVersion} />}
                {selectedTech.defaultPort > 0 && <DetailField label="Puerto" value={String(selectedTech.defaultPort)} />}
                <DetailField label="Categoria" value={categoryLabels[selectedTech.category] || selectedTech.category} />
                {selectedTech.dockerImage && <DetailField label="Docker" value={selectedTech.dockerImage} />}
              </div>

              {selectedTech.requires.length > 0 && (
                <div>
                  <span className="text-xs font-semibold text-gray-500 uppercase">Requiere</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {selectedTech.requires.map((r) => (
                      <span key={r} className="px-2 py-0.5 rounded bg-blue-500/10 text-blue-400 text-xs">{r}</span>
                    ))}
                  </div>
                </div>
              )}

              {selectedTech.incompatibleWith.length > 0 && (
                <div>
                  <span className="text-xs font-semibold text-gray-500 uppercase">Incompatible con</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {selectedTech.incompatibleWith.map((r) => (
                      <span key={r} className="px-2 py-0.5 rounded bg-red-500/10 text-red-400 text-xs">{r}</span>
                    ))}
                  </div>
                </div>
              )}

              {selectedTech.suggestedWith.length > 0 && (
                <div>
                  <span className="text-xs font-semibold text-gray-500 uppercase">Recomendado con</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {selectedTech.suggestedWith.map((r) => (
                      <span key={r} className="px-2 py-0.5 rounded bg-green-500/10 text-green-400 text-xs">{r}</span>
                    ))}
                  </div>
                </div>
              )}

              {Object.keys(selectedTech.envVars).length > 0 && (
                <div>
                  <span className="text-xs font-semibold text-gray-500 uppercase">Variables de entorno</span>
                  <div className="mt-1 space-y-0.5">
                    {Object.entries(selectedTech.envVars).map(([k, v]) => (
                      <p key={k} className="text-xs font-mono text-gray-400">{k}={v}</p>
                    ))}
                  </div>
                </div>
              )}

              {selectedTech.tags.length > 0 && (
                <div className="flex flex-wrap gap-1">
                  {selectedTech.tags.map((tag) => (
                    <span key={tag} className="px-2 py-0.5 rounded bg-gray-800 text-gray-500 text-xs">{tag}</span>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function DetailField({ label, value }: { label: string; value: string }) {
  return (
    <div className="px-3 py-2 rounded-lg bg-gray-800">
      <p className="text-xs text-gray-500">{label}</p>
      <p className="text-sm text-white font-mono">{value}</p>
    </div>
  );
}
