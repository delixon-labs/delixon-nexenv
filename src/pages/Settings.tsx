import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useSettingsStore } from "@/stores/settings";
import { useProjectsStore } from "@/stores/projects";
import * as api from "@/lib/tauri";
import type { DetectedRuntime, InstalledEditor } from "@/lib/tauri";
import type { Project } from "@/types/project";
import { Spinner } from "@/components/ui/Spinner";
import { toast } from "@/components/ui/Toast";
import PathInput from "@/components/ui/PathInput";
import PreviewConfirmModal, { type PreviewSection } from "@/components/ui/PreviewConfirmModal";

function defaultDataDir(): string {
  const p = navigator.platform?.toLowerCase() || "";
  if (p.includes("win")) return "$LOCALAPPDATA/nexenv";
  if (p.includes("mac")) return "~/Library/Application Support/nexenv";
  return "~/.local/share/nexenv";
}

export default function Settings() {
  const { t } = useTranslation();
  const { config, setConfig } = useSettingsStore();
  const loadConfig = useSettingsStore((s) => s.loadConfig);
  const isLoaded = useSettingsStore((s) => s.isLoaded);
  const [runtimes, setRuntimes] = useState<DetectedRuntime[]>([]);
  const [loadingRuntimes, setLoadingRuntimes] = useState(false);
  const [installedEditors, setInstalledEditors] = useState<InstalledEditor[]>([]);
  const [orphans, setOrphans] = useState<Project[]>([]);
  const [orphansLoading, setOrphansLoading] = useState(false);
  const [orphansCleaning, setOrphansCleaning] = useState(false);
  const [orphansModalOpen, setOrphansModalOpen] = useState(false);
  const reloadProjects = useProjectsStore((s) => s.loadProjects);

  useEffect(() => {
    if (!isLoaded) loadConfig();
    loadRuntimes();
    api.listInstalledEditors().then(setInstalledEditors).catch(() => {});
    loadOrphans();
  }, [isLoaded, loadConfig]);

  async function loadOrphans() {
    setOrphansLoading(true);
    try {
      const list = await api.listOrphanProjects();
      setOrphans(list);
    } catch {
      // No-op en dev browser
    } finally {
      setOrphansLoading(false);
    }
  }

  async function applyCleanup() {
    setOrphansCleaning(true);
    try {
      const removed = await api.cleanupOrphanProjects();
      toast.success(`${removed} proyecto(s) huerfanos eliminados`);
      setOrphans([]);
      setOrphansModalOpen(false);
      await reloadProjects?.();
    } catch (err) {
      toast.error(err);
      setOrphansCleaning(false);
    }
  }

  async function loadRuntimes() {
    setLoadingRuntimes(true);
    try {
      const detected = await api.detectRuntimes();
      setRuntimes(detected);
    } catch {
      // No-op en dev browser
    } finally {
      setLoadingRuntimes(false);
    }
  }

  return (
    <div className="p-6 lg:p-8 max-w-3xl h-full overflow-y-auto">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-white">{t("settings.title")}</h1>
        <p className="text-sm text-gray-500 mt-1">
          {t("settings.subtitle")}
        </p>
      </div>

      {/* General */}
      <section className="mb-8">
        <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-4">
          {t("settings.general")}
        </h2>
        <div className="space-y-4">
          {/* Editor */}
          <SettingRow
            label={t("settings.editor")}
            description={t("settings.editorDesc")}
          >
            <div className="relative w-52">
              {installedEditors.length === 0 ? (
                <div className="flex items-center justify-center w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700">
                  <Spinner size="sm" className="text-gray-500" />
                </div>
              ) : (
                <select
                  value={config.defaultEditor}
                  onChange={(e) => setConfig({ defaultEditor: e.target.value })}
                  className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm focus:outline-hidden focus:border-primary-500"
                >
                  {installedEditors.map((ed) => (
                    <option key={ed.cmd} value={ed.cmd}>{ed.label}</option>
                  ))}
                </select>
              )}
            </div>
          </SettingRow>

          {/* Tema — solo dark por ahora, light/system deshabilitados */}
          <SettingRow
            label={t("settings.theme")}
            description={t("settings.themeDesc")}
          >
            <div className="flex gap-2">
              <button
                className="px-3 py-1.5 rounded-lg text-sm font-medium border transition-colors bg-primary-500/10 text-primary-500 border-primary-500/30"
              >
                {t("settings.themeDark")}
              </button>
            </div>
          </SettingRow>

          {/* Idioma */}
          <SettingRow label={t("settings.language")} description={t("settings.languageDesc")}>
            <div className="flex gap-2">
              {(["es", "en"] as const).map((lang) => (
                <button
                  key={lang}
                  onClick={() => setConfig({ language: lang })}
                  className={`px-3 py-1.5 rounded-lg text-sm font-medium border transition-colors ${
                    config.language === lang
                      ? "bg-primary-500/10 text-primary-500 border-primary-500/30"
                      : "bg-gray-800 text-gray-500 border-gray-700 hover:text-gray-300"
                  }`}
                >
                  {lang === "es" ? "Español" : "English"}
                </button>
              ))}
            </div>
          </SettingRow>

          {/* Font Pack */}
          <SettingRow label={t("settings.fontPack")} description={t("settings.fontPackDesc")}>
            <div className="flex gap-2">
              {(["system", "classic"] as const).map((pack) => (
                <button
                  key={pack}
                  onClick={() => setConfig({ fontPack: pack })}
                  className={`px-3 py-1.5 rounded-lg text-sm font-medium border transition-colors ${
                    config.fontPack === pack
                      ? "bg-primary-500/10 text-primary-500 border-primary-500/30"
                      : "bg-gray-800 text-gray-500 border-gray-700 hover:text-gray-300"
                  }`}
                >
                  <span className={pack === "classic" ? "font-serif" : ""}>
                    {pack === "system" ? "System (Inter)" : "Classic (Georgia)"}
                  </span>
                </button>
              ))}
            </div>
          </SettingRow>

          {/* Data dir */}
          <SettingRow
            label={t("settings.dataDir")}
            description={t("settings.dataDirDesc")}
          >
            <div className="max-w-sm">
              <PathInput
                value={config.dataDir || defaultDataDir()}
                onChange={(v) => setConfig({ dataDir: v })}
                placeholder="$LOCALAPPDATA/nexenv"
              />
            </div>
          </SettingRow>
        </div>
      </section>

      {/* Runtimes detectados */}
      <section className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider">
            {t("settings.runtimes")}
          </h2>
          <button
            onClick={loadRuntimes}
            disabled={loadingRuntimes}
            className="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs text-dlx-text-light-1 bg-dlx-light-2/50 hover:bg-dlx-light-3/50 transition-colors disabled:opacity-50"
          >
            {loadingRuntimes && <Spinner size="sm" className="text-dlx-text-light-1" />}
            {loadingRuntimes ? t("settings.detecting") : t("settings.redetect")}
          </button>
        </div>

        {runtimes.length > 0 ? (
          <div className="rounded-xl bg-gray-900 border border-gray-800 divide-y divide-gray-800">
            {runtimes.map((rt) => (
              <div
                key={rt.name}
                className="flex items-center justify-between px-4 py-3"
              >
                <div>
                  <span className="text-sm font-medium text-white capitalize">
                    {rt.name}
                  </span>
                  <span className="text-sm text-gray-500 ml-2">
                    {rt.version}
                  </span>
                </div>
                <span className="text-xs text-gray-600 font-mono truncate max-w-xs">
                  {rt.path}
                </span>
              </div>
            ))}
          </div>
        ) : loadingRuntimes ? (
          <div className="flex items-center justify-center py-8">
            <div className="w-6 h-6 border-2 border-primary-500/30 border-t-primary-500 rounded-full animate-spin" />
          </div>
        ) : (
          <div className="px-4 py-8 rounded-xl bg-gray-900 border border-gray-800 text-center text-sm text-gray-600">
            {t("settings.noRuntimes")}
          </div>
        )}
      </section>

      {/* About */}
      <section>
        <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-4">
          {t("settings.about")}
        </h2>
        <div className="rounded-xl bg-gray-900 border border-gray-800 px-4 py-5">
          <p className="text-sm text-white font-semibold">{t("settings.version")}</p>
          <p className="text-xs text-gray-400 mt-1">
            {t("settings.tagline")}
          </p>
          <p className="text-xs text-gray-500 mt-0.5 italic">
            {t("settings.motto")}
          </p>

          <div className="mt-4 pt-3 border-t border-gray-800" style={{ fontFamily: "var(--font-heading, 'Inter', system-ui, sans-serif)" }}>
            <p className="text-[11px]">
              <span style={{ color: "#7a7a9a" }}>{t("settings.aboutProduct")}{" "}</span>
              <a
                href="https://delixon.dev"
                target="_blank"
                rel="noopener noreferrer"
                className="transition-colors hover:brightness-125"
                style={{ color: "#8b5cf6", fontFamily: "'Courier New', Courier, monospace" }}
              >
                Delixon Labs
              </a>
              <span style={{ color: "#8b5cf6" }}>{" · "}</span>
              <a
                href="https://delixon.dev/nexenv"
                target="_blank"
                rel="noopener noreferrer"
                className="transition-colors hover:brightness-125"
                style={{ color: "#8b5cf6", fontFamily: "'Courier New', Courier, monospace" }}
              >
                delixon.dev/nexenv
              </a>
            </p>
            <p className="text-[11px] mt-1.5">
              <span style={{ color: "#7a7a9a" }}>{"© 2026 "}</span>
              <a
                href="https://xplustechnologies.com"
                target="_blank"
                rel="noopener noreferrer"
                className="transition-colors hover:brightness-125"
                style={{ color: "#2563eb", fontFamily: "'Courier New', Courier, monospace" }}
              >
                XPlus Technologies LLC
              </a>
            </p>
          </div>
        </div>
      </section>

      {/* Mantenimiento */}
      <section className="mb-8">
        <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-4">
          Mantenimiento
        </h2>
        <div className="space-y-4">
          <SettingRow
            label="Proyectos huerfanos"
            description={
              orphansLoading
                ? "Buscando..."
                : orphans.length === 0
                  ? "No hay proyectos registrados cuya carpeta haya desaparecido."
                  : `${orphans.length} proyecto(s) registrados apuntan a rutas que ya no existen en disco.`
            }
          >
            <button
              type="button"
              onClick={() => setOrphansModalOpen(true)}
              disabled={orphans.length === 0 || orphansLoading}
              className="px-4 py-2 rounded-lg text-sm font-medium bg-warning/10 text-warning-light border border-warning/30 hover:bg-warning/20 disabled:opacity-40 disabled:cursor-not-allowed transition-colors whitespace-nowrap"
            >
              Limpiar {orphans.length > 0 ? `(${orphans.length})` : ""}
            </button>
          </SettingRow>
        </div>
      </section>

      <PreviewConfirmModal
        open={orphansModalOpen}
        title={`Limpiar ${orphans.length} proyecto(s) huerfanos`}
        subtitle="Se eliminan solo del registro de Nexenv. No se toca ningun archivo en disco (ya no existen)."
        sections={[
          {
            label: "Proyectos a desregistrar",
            items: orphans.map((p) => `${p.name} — ${p.path}`),
            tone: "removed",
          } as PreviewSection,
        ]}
        confirmLabel="Eliminar todos"
        destructive
        busy={orphansCleaning}
        onConfirm={applyCleanup}
        onCancel={() => setOrphansModalOpen(false)}
      />
    </div>
  );
}

function SettingRow({
  label,
  description,
  children,
}: {
  label: string;
  description: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col gap-2 p-4 rounded-xl bg-gray-900 border border-gray-800">
      <div>
        <p className="text-sm font-medium text-white">{label}</p>
        <p className="text-xs text-gray-500">{description}</p>
      </div>
      <div>{children}</div>
    </div>
  );
}
