import { useEffect, useState } from "react";
import * as api from "@/lib/tauri";
import type { Technology } from "@/types/catalog";
import { CATEGORY_LABELS } from "@/lib/catalog";
import { getTechIcon, techCatalogClass } from "@/lib/tech-meta";

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
    <div className="p-6 lg:p-8 h-full overflow-y-auto">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-white">Catalogo de tecnologias</h1>
        <p className="text-sm text-gray-500 mt-1">{techs.length} tecnologias disponibles</p>
      </div>

      {loadError && (
        <div className="px-4 py-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm mb-4">
          Error cargando catalogo: {loadError}
        </div>
      )}

      {/* Search */}
      <div className="mb-4">
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Buscar tecnologia..."
          className="w-full max-w-md px-4 py-2 rounded-lg bg-gray-900 border border-gray-800 text-white placeholder-gray-600 text-sm focus:outline-none focus:border-primary-500/50"
        />
      </div>

      {/* Category filter */}
      <div className="flex flex-wrap gap-1.5 mb-6">
        <button
          onClick={() => setActiveCategory(null)}
          className={`min-w-26 px-3 py-1.5 rounded-lg text-xs font-medium text-center whitespace-nowrap transition-colors ${!activeCategory ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-800 text-gray-400 hover:text-white"}`}
        >
          Todas
        </button>
        {categories.map((cat) => (
          <button
            key={cat}
            onClick={() => setActiveCategory(cat)}
            className={`min-w-26 px-3 py-1.5 rounded-lg text-xs font-medium text-center whitespace-nowrap transition-colors ${activeCategory === cat ? "bg-primary-500/10 text-primary-500 border border-primary-500/30" : "bg-gray-800 text-gray-400 hover:text-white"}`}
          >
            {categoryLabels[cat] || cat}
          </button>
        ))}
      </div>

      {/* Grid */}
      {filtered.length === 0 ? (
        <p className="text-gray-500 text-center py-12">No se encontraron tecnologias</p>
      ) : !activeCategory ? (
        /* Vista agrupada por categoría */
        <div className="space-y-8">
          {categories.map((cat) => {
            const catTechs = filtered.filter((t) => t.category === cat);
            if (catTechs.length === 0) return null;
            return (
              <div key={cat}>
                <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3">
                  {categoryLabels[cat] || cat}
                </h3>
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
                  {catTechs.map((tech) => <TechCard key={tech.id} tech={tech} onClick={() => setSelectedTech(tech)} />)}
                </div>
              </div>
            );
          })}
        </div>
      ) : (
        /* Vista plana con filtro activo */
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
          {filtered.map((tech) => <TechCard key={tech.id} tech={tech} onClick={() => setSelectedTech(tech)} />)}
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
                      <span key={r} className="px-2 py-0.5 rounded bg-info/10 text-info-light text-xs">{r}</span>
                    ))}
                  </div>
                </div>
              )}

              {selectedTech.incompatibleWith.length > 0 && (
                <div>
                  <span className="text-xs font-semibold text-gray-500 uppercase">Incompatible con</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {selectedTech.incompatibleWith.map((r) => (
                      <span key={r} className="px-2 py-0.5 rounded bg-error/10 text-error-light text-xs">{r}</span>
                    ))}
                  </div>
                </div>
              )}

              {selectedTech.suggestedWith.length > 0 && (
                <div>
                  <span className="text-xs font-semibold text-gray-500 uppercase">Recomendado con</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {selectedTech.suggestedWith.map((r) => (
                      <span key={r} className="px-2 py-0.5 rounded bg-success/10 text-success-light text-xs">{r}</span>
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

function TechCard({ tech, onClick }: { tech: Technology; onClick: () => void }) {
  const icon = getTechIcon(tech.id, tech.name);
  const cls = techCatalogClass(tech.id);
  /* Variable CSS: --color-cat-{id} registrada en catalog.css */
  const cssVar = `var(--color-cat-${cls.id})`;
  return (
    <button
      onClick={onClick}
      className="text-left rounded-xl border border-gray-800 hover:border-gray-600 transition-colors overflow-hidden"
      style={{ backgroundColor: `color-mix(in srgb, ${cssVar} 6%, transparent)` }}
    >
      <div className="px-4 py-4">
        <div className="flex items-center gap-3 mb-3">
          <span
            className={`w-9 h-9 rounded-lg flex items-center justify-center text-xs font-bold shrink-0 ${cls.text}`}
            style={{ backgroundColor: `color-mix(in srgb, ${cssVar} 15%, transparent)` }}
          >
            {icon}
          </span>
          <div className="min-w-0 flex-1">
            <span className="text-sm font-semibold text-white block truncate">{tech.name}</span>
            {tech.defaultPort > 0 && (
              <span className={`text-xs font-mono ${cls.text}`}>:{tech.defaultPort}</span>
            )}
          </div>
        </div>
        <p className="text-xs text-gray-500 line-clamp-2 mb-3">{tech.description}</p>
        <div className="flex items-center gap-2">
          <span
            className="text-xs px-2 py-0.5 rounded text-gray-400"
            style={{ backgroundColor: `color-mix(in srgb, ${cssVar} 10%, transparent)` }}
          >
            {CATEGORY_LABELS[tech.category] || tech.category}
          </span>
          {tech.defaultVersion && (
            <span className="text-xs text-gray-600">v{tech.defaultVersion}</span>
          )}
        </div>
      </div>
    </button>
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
