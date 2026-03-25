import { Routes, Route } from "react-router-dom";
import AppLayout from "./components/layout/AppLayout";
import Dashboard from "./pages/Dashboard";
import ProjectDetail from "./pages/ProjectDetail";
import Templates from "./pages/Templates";
import Settings from "./pages/Settings";

function App() {
  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 font-sans">
      <AppLayout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/project/:id" element={<ProjectDetail />} />
          <Route path="/templates" element={<Templates />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </AppLayout>
    </div>
  );
}

export default App;
