import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useSettingsStore } from "@/stores/settings";
import * as api from "@/lib/tauri";
import type { DetectedRuntime } from "@/lib/tauri";

export default function Settings() {
  const { t } = useTranslation();
  const { config, setConfig } = useSettingsStore();
  const loadConfig = useSettingsStore((s) => s.loadConfig);
  const isLoaded = useSettingsStore((s) => s.isLoaded);
  const [runtimes, setRuntimes] = useState<DetectedRuntime[]>([]);
  const [loadingRuntimes, setLoadingRuntimes] = useState(false);

  useEffect(() => {
    if (!isLoaded) loadConfig();
    loadRuntimes();
  }, [isLoaded, loadConfig]);

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
            <select
              value={config.defaultEditor}
              onChange={(e) => setConfig({ defaultEditor: e.target.value })}
              className="px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm focus:outline-none focus:border-primary-500"
            >
              <option value="code">VS Code</option>
              <option value="code-insiders">VS Code Insiders</option>
              <option value="cursor">Cursor</option>
              <option value="zed">Zed</option>
              <option value="subl">Sublime Text</option>
              <option value="atom">Atom</option>
              <option value="nvim">Neovim</option>
            </select>
          </SettingRow>

          {/* Tema */}
          <SettingRow
            label={t("settings.theme")}
            description={t("settings.themeDesc")}
          >
            <div className="flex gap-2">
              {(["dark", "light", "system"] as const).map((theme) => (
                <button
                  key={theme}
                  onClick={() => setConfig({ theme })}
                  className={`px-3 py-1.5 rounded-lg text-sm font-medium border transition-colors ${
                    config.theme === theme
                      ? "bg-primary-500/10 text-primary-500 border-primary-500/30"
                      : "bg-gray-800 text-gray-500 border-gray-700 hover:text-gray-300"
                  }`}
                >
                  {theme === "dark"
                    ? t("settings.themeDark")
                    : theme === "light"
                      ? t("settings.themeLight")
                      : t("settings.themeSystem")}
                </button>
              ))}
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
            <input
              type="text"
              value={config.dataDir || "~/.local/share/delixon"}
              onChange={(e) => setConfig({ dataDir: e.target.value })}
              className="w-full max-w-sm px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-sm font-mono text-gray-300 focus:outline-none focus:border-primary-500/50"
            />
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
            className="px-3 py-1 rounded-md text-xs text-primary-500 hover:bg-primary-500/10 transition-colors disabled:opacity-50"
          >
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
        <div className="rounded-xl bg-gray-900 border border-gray-800 px-4 py-4">
          <p className="text-sm text-white font-medium">{t("settings.version")}</p>
          <p className="text-xs text-gray-500 mt-1">
            {t("settings.tagline")}
          </p>
          <p className="text-xs text-gray-600 mt-2">
            {t("settings.motto")}
          </p>
        </div>
      </section>
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
