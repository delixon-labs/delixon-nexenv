import { useEffect } from "react";
import Sidebar from "./Sidebar";
import { useProjectsStore } from "@/stores/projects";

interface AppLayoutProps {
  children: React.ReactNode;
}

export default function AppLayout({ children }: AppLayoutProps) {
  const fetchProjects = useProjectsStore((s) => s.fetchProjects);

  useEffect(() => {
    // Refrescar al volver a la ventana o pasar el mouse sobre ella
    // Detecta cambios externos (CLI, otro proceso) sin polling costoso
    function handleVisibility() {
      if (document.visibilityState === "visible") {
        fetchProjects();
      }
    }
    function handleMouseEnter() {
      fetchProjects();
    }
    document.addEventListener("visibilitychange", handleVisibility);
    document.documentElement.addEventListener("mouseenter", handleMouseEnter);

    return () => {
      document.removeEventListener("visibilitychange", handleVisibility);
      document.documentElement.removeEventListener("mouseenter", handleMouseEnter);
    };
  }, [fetchProjects]);

  return (
    <div className="flex h-screen overflow-hidden bg-gray-950 text-gray-100 font-sans">
      <Sidebar />
      <main className="flex-1 overflow-hidden">{children}</main>
    </div>
  );
}
