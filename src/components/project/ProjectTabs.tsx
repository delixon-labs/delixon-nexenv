import { useState } from "react";
import ScrollRow from "@/components/ui/ScrollRow";

export interface TabDefinition {
  id: string;
  label: string;
  component: React.ComponentType<{ projectId: string; projectPath: string }>;
}

interface Props {
  tabs: TabDefinition[];
  projectId: string;
  projectPath: string;
}

export default function ProjectTabs({ tabs, projectId, projectPath }: Props) {
  const [activeTab, setActiveTab] = useState(tabs[0]?.id ?? "");

  const ActiveComponent = tabs.find((t) => t.id === activeTab)?.component;

  return (
    <div>
      <div className="relative mb-6">
        <hr className="absolute bottom-0 left-0 right-0 border-gray-800" />
        <ScrollRow className="gap-1 pt-1.5 relative z-10">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`px-4 py-2 text-sm font-medium whitespace-nowrap transition-colors border-b-2 ${
                activeTab === tab.id
                  ? "text-primary-500 border-primary-500"
                  : "text-gray-500 border-transparent hover:text-gray-300 hover:border-gray-700"
              }`}
            >
              {tab.label}
            </button>
          ))}
        </ScrollRow>
      </div>
      {ActiveComponent && <ActiveComponent projectId={projectId} projectPath={projectPath} />}
    </div>
  );
}
