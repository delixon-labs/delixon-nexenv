import { useState } from "react";
import { useTranslation } from "react-i18next";
import * as api from "@/lib/tauri";
import type { DetectedStack } from "@/lib/tauri";
import PathInput from "@/components/ui/PathInput";
import { Spinner } from "@/components/ui/Spinner";

interface Props {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

export default function RegisterProjectModal({ isOpen, onClose, onSuccess }: Props) {
  const { t } = useTranslation();
  const [step, setStep] = useState<"path" | "scan" | "confirm">("path");
  const [path, setPath] = useState("");
  const [name, setName] = useState("");
  const [scanning, setScanning] = useState(false);
  const [registering, setRegistering] = useState(false);
  const [stack, setStack] = useState<DetectedStack | null>(null);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  async function handleScan() {
    if (!path.trim()) return;
    setScanning(true);
    setError(null);
    try {
      const detected = await api.detectProjectStack(path.trim());
      setStack(detected);
      if (!name.trim()) {
        const parts = path.trim().replace(/\\/g, "/").split("/");
        setName(parts[parts.length - 1] || "proyecto");
      }
      setStep("confirm");
    } catch (err) {
      setError(String(err));
    } finally {
      setScanning(false);
    }
  }

  async function handleRegister() {
    if (!path.trim() || !name.trim()) return;
    setRegistering(true);
    setError(null);
    try {
      await api.scanAndRegister(path.trim(), name.trim());
      onSuccess();
      handleClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setRegistering(false);
    }
  }

  function handleClose() {
    setStep("path");
    setPath("");
    setName("");
    setStack(null);
    setError(null);
    onClose();
  }

  const score = stack?.readinessScore;
  const scoreColor =
    score && score.total >= 8
      ? "text-green-400"
      : score && score.total >= 5
        ? "text-yellow-400"
        : "text-red-400";

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="w-full max-w-lg rounded-2xl bg-gray-900 border border-gray-800 shadow-2xl">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-800">
          <h2 className="text-lg font-semibold text-white">{t("register.title")}</h2>
          <button onClick={handleClose} className="p-1 text-gray-500 hover:text-white transition-colors">
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="px-6 py-5 space-y-4">
          {error && (
            <div className="px-3 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
              {error}
            </div>
          )}

          {/* Step 1: Path input */}
          {step === "path" && (
            <>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-1">
                  {t("register.pathLabel")}
                </label>
                <PathInput
                  value={path}
                  onChange={setPath}
                  placeholder={t("register.pathPlaceholder")}
                  onKeyDown={(e) => e.key === "Enter" && handleScan()}
                  autoFocus
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-1">
                  {t("register.nameLabel")} <span className="text-gray-600">({t("register.nameAutoDetect")})</span>
                </label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="mi-proyecto"
                  className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm placeholder-gray-600 focus:outline-hidden focus:border-primary-500"
                />
              </div>
            </>
          )}

          {/* Step 2: Scan results */}
          {step === "confirm" && stack && (
            <>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-1">{t("register.nameLabel")}</label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white text-sm focus:outline-hidden focus:border-primary-500"
                />
              </div>

              {/* Detected info */}
              <div className="space-y-2">
                {stack.runtimes.length > 0 && (
                  <DetailRow label="Runtimes" value={stack.runtimes.map((r) => r.runtime).join(", ")} />
                )}
                {stack.packageManager && <DetailRow label="Package manager" value={stack.packageManager} />}
                {stack.orm && <DetailRow label="ORM" value={stack.orm} />}
                {stack.auth && <DetailRow label="Auth" value={stack.auth} />}
                {stack.ci && <DetailRow label="CI/CD" value={stack.ci} />}
                {stack.testing && <DetailRow label="Testing" value={stack.testing} />}
                {stack.linter && <DetailRow label="Linter" value={stack.linter} />}
                {stack.docker && (
                  <DetailRow
                    label="Docker"
                    value={[
                      stack.docker.hasDockerfile && "Dockerfile",
                      stack.docker.hasCompose && "Compose",
                    ].filter(Boolean).join(" + ")}
                  />
                )}
                {stack.isFullstack && <DetailRow label="Estructura" value="Fullstack" />}
                {stack.tags.length > 0 && (
                  <div className="flex flex-wrap gap-1 pt-1">
                    {stack.tags.map((tag) => (
                      <span key={tag} className="px-2 py-0.5 rounded bg-gray-800 text-xs text-gray-400">
                        {tag}
                      </span>
                    ))}
                  </div>
                )}
              </div>

              {/* Readiness Score */}
              {score && (
                <div className="rounded-lg bg-gray-800/50 border border-gray-700 p-4">
                  <div className="flex items-center justify-between mb-3">
                    <span className="text-sm font-medium text-gray-300">{t("register.readiness")}</span>
                    <span className={`text-lg font-bold ${scoreColor}`}>
                      {score.total}/{score.max}
                    </span>
                  </div>
                  <div className="grid grid-cols-3 gap-2">
                    {score.breakdown.map((item) => (
                      <div key={item.name} className="flex items-center gap-1.5">
                        <span className={`w-1.5 h-1.5 rounded-full ${item.present ? "bg-green-400" : "bg-gray-600"}`} />
                        <span className="text-xs text-gray-400">{item.name}</span>
                      </div>
                    ))}
                  </div>
                  {score.suggestions.length > 0 && (
                    <p className="text-xs text-gray-500 mt-2">
                      {t("register.suggestedRecipes")}: {score.suggestions.join(", ")}
                    </p>
                  )}
                </div>
              )}
            </>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 px-6 py-4 border-t border-gray-800">
          <button
            onClick={handleClose}
            className="px-4 py-2 rounded-lg text-sm font-medium bg-dlx-light-3 text-dlx-text-light-1 border border-dlx-text-dark-3 hover:bg-dlx-text-dark-3 transition-colors"
          >
            {t("common.cancel")}
          </button>
          {step === "path" && (
            <button
              onClick={handleScan}
              disabled={scanning || !path.trim()}
              className="inline-flex items-center justify-center gap-2 min-w-28 px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
            >
              {scanning ? <Spinner size="sm" className="text-success-light" /> : t("register.scan")}
            </button>
          )}
          {step === "confirm" && (
            <>
              <button
                onClick={() => setStep("path")}
                className="inline-flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-gray-300 text-sm font-medium hover:bg-gray-700 hover:text-white transition-colors"
              >
                <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}><path strokeLinecap="round" strokeLinejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" /></svg>
                {t("common.back")}
              </button>
              <button
                onClick={handleRegister}
                disabled={registering || !name.trim()}
                className="inline-flex items-center justify-center gap-2 min-w-28 px-4 py-2 rounded-lg bg-success/10 text-success-light text-sm font-medium hover:bg-success/20 disabled:opacity-50 transition-colors"
              >
                {registering ? <Spinner size="sm" className="text-success-light" /> : t("register.register")}
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function DetailRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center gap-2">
      <span className="text-xs text-gray-500 w-28 shrink-0">{label}</span>
      <span className="text-sm text-white">{value}</span>
    </div>
  );
}
