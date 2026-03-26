import { useState } from "react";

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
      <div className="flex gap-1 border-b border-gray-800 mb-6 overflow-x-auto">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`px-4 py-2 text-sm font-medium whitespace-nowrap transition-colors border-b-2 -mb-px ${
              activeTab === tab.id
                ? "text-primary-500 border-primary-500"
                : "text-gray-500 border-transparent hover:text-gray-300 hover:border-gray-700"
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>
      {ActiveComponent && <ActiveComponent projectId={projectId} projectPath={projectPath} />}
    </div>
  );
}
