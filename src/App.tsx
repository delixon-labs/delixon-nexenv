import { useEffect } from "react";
import { Routes, Route } from "react-router-dom";
import { useTranslation } from "react-i18next";
import AppLayout from "./components/layout/AppLayout";
import Titlebar from "./components/layout/Titlebar";
import Dashboard from "./pages/Dashboard";
import ProjectDetail from "./pages/ProjectDetail";
import Templates from "./pages/Templates";
import Settings from "./pages/Settings";
import Catalog from "./pages/Catalog";
import Scaffold from "./pages/Scaffold";
import { useSettingsStore } from "./stores/settings";

function App() {
  const { i18n } = useTranslation();
  const loadConfig = useSettingsStore((s) => s.loadConfig);
  const isLoaded = useSettingsStore((s) => s.isLoaded);
  const language = useSettingsStore((s) => s.config.language);
  const fontPack = useSettingsStore((s) => s.config.fontPack);

  useEffect(() => {
    if (!isLoaded) loadConfig();
  }, [isLoaded, loadConfig]);

  // Sync i18n language with config
  useEffect(() => {
    if (language && i18n.language !== language) {
      i18n.changeLanguage(language);
    }
  }, [language, i18n]);

  // Sync font pack with config
  useEffect(() => {
    document.documentElement.setAttribute("data-font-pack", fontPack || "system");
  }, [fontPack]);

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
    <Titlebar />
    <AppLayout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/project/:id" element={<ProjectDetail />} />
        <Route path="/templates" element={<Templates />} />
        <Route path="/catalog" element={<Catalog />} />
        <Route path="/scaffold" element={<Scaffold />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </AppLayout>
    </div>
  );
}

export default App;
