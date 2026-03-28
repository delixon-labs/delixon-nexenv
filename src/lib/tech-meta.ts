/** Iconos / siglas para cada tecnologia del catalogo.
 *  Los IDs deben coincidir EXACTAMENTE con los del backend (YAML).
 *  Los colores vienen de las clases CSS en src/styles/tech/.
 */

const TECH_ICONS: Record<string, string> = {
  // Runtimes
  "nodejs":          "⬢",
  "python":          "🐍",
  "go":              "Go",
  "rust":            "🦀",
  "ruby":            "Rb",
  "php":             "PHP",
  "dotnet":          ".N",
  "java":            "Jv",
  "deno":            "Dn",
  "bun":             "Bn",

  // Frontend
  "react":           "⚛",
  "nextjs":          "N",
  "vue":             "V",
  "nuxt":            "Nx",
  "svelte":          "S",
  "angular":         "Ng",
  "astro":           "As",
  "solid":           "So",

  // Backend
  "express":         "Ex",
  "fastify":         "Fy",
  "fastapi":         "⚡",
  "django":          "Dj",
  "flask":           "Fl",
  "nestjs":          "Ns",
  "spring":          "Sp",
  "laravel":         "Lv",
  "rails":           "Ra",

  // Bases de datos
  "postgresql":      "🐘",
  "mysql":           "🐬",
  "mongodb":         "🍃",
  "redis":           "Re",
  "sqlite":          "Sq",
  "mariadb":         "Ma",
  "cassandra":       "Ca",

  // ORMs
  "prisma":          "Pr",
  "drizzle":         "Dr",
  "sqlalchemy":      "SA",
  "typeorm":         "TO",
  "sequelize":       "Sq",

  // Auth
  "nextauth":        "NA",
  "clerk":           "Ck",
  "auth0":           "A0",
  "firebase":        "Fb",

  // Estilos
  "tailwindcss":     "Tw",
  "shadcn-ui":       "Sh",
  "bootstrap":       "Bs",
  "sass":            "Sc",

  // DevOps / Tooling
  "docker":          "🐳",
  "kubernetes":      "K8",
  "github-actions":  "GH",
  "typescript":      "TS",
  "vitest":          "Vi",
  "jest":            "Je",
  "eslint":          "Es",
  "prettier":        "Pt",
  "webpack":         "Wp",
  "vite":            "Vt",
  "nginx":           "Ng",
  "terraform":       "Tf",

  // Testing
  "playwright":      "Pw",
  "cypress":         "Cy",
  "mocha":           "Mo",
  "pytest":          "Py",
};

/* ── Aliases: mapea IDs alternativos al ID canonico de CSS ── */
const TECH_ALIASES: Record<string, string> = {
  "node":       "nodejs",
  "postgres":   "postgresql",
  "mongo":      "mongodb",
  "net":        "dotnet",
  "csharp":     "dotnet",
  "tailwind":   "tailwindcss",
  "shadcn":     "shadcn-ui",
  "github":     "github-actions",
  "ts":         "typescript",
  "js":         "nodejs",
  "next":       "nextjs",
  "nest":       "nestjs",
  "pg":         "postgresql",
  "sql-alchemy":"sqlalchemy",
  "type-orm":   "typeorm",
};

/* ── Set de IDs que tienen clase CSS definida en brand.css / catalog.css ── */
const KNOWN_TECH_IDS = new Set(Object.keys(TECH_ICONS));

/** Resuelve un ID (posible alias) al ID canonico de CSS */
function resolveId(techId: string): string {
  const alias = TECH_ALIASES[techId];
  if (alias) return alias;
  if (KNOWN_TECH_IDS.has(techId)) return techId;
  return "unknown";
}

/**
 * Devuelve el icono/sigla de una tecnologia.
 * Si no existe, genera iniciales del nombre (nunca "?").
 */
export function getTechIcon(techId: string, techName?: string): string {
  const canonical = resolveId(techId);
  const icon = TECH_ICONS[canonical];
  if (icon) return icon;

  const source = techName || techId;
  const words = source.replace(/[.-]/g, " ").split(/\s+/);
  return words.length >= 2
    ? (words[0][0] + words[1][0]).toUpperCase()
    : source.slice(0, 2).toUpperCase();
}

/**
 * Devuelve las clases CSS de color de marca para una tecnologia.
 * Usa las clases de brand.css (text-tech-{id}, bg-tech-{id}).
 * Si el ID no tiene clase conocida, usa tech-unknown.
 */
export function techBrandClass(techId: string) {
  const id = resolveId(techId);
  return {
    id,
    text: `text-tech-${id}`,
    bg:   `bg-tech-${id}`,
    border: `border-tech-${id}`,
  };
}

/**
 * Devuelve las clases CSS de color de catalogo para una tecnologia.
 * Usa las clases de catalog.css (text-cat-{id}, bg-cat-{id}).
 * Si el ID no tiene clase conocida, usa cat-unknown.
 */
export function techCatalogClass(techId: string) {
  const id = resolveId(techId);
  return {
    id,
    text: `text-cat-${id}`,
    bg:   `bg-cat-${id}`,
    border: `border-cat-${id}`,
  };
}

