import { Clipboard, FileText } from "lucide-react";

type ExportPanelProps = {
  plainMarkdown: string;
  jsonAst: string;
  activeExport: "plain" | "json";
  onExportChange: (target: "plain" | "json") => void;
};

export function ExportPanel({
  plainMarkdown,
  jsonAst,
  activeExport,
  onExportChange,
}: ExportPanelProps) {
  const exportValue = activeExport === "plain" ? plainMarkdown : jsonAst;

  return (
    <section className="panel export-panel" aria-labelledby="export-title">
      <div className="panel-header">
        <div className="panel-title">
          <FileText aria-hidden="true" size={18} />
          <h2 id="export-title">Export</h2>
        </div>
        <div className="segmented-control" role="tablist" aria-label="export format">
          <button
            aria-selected={activeExport === "plain"}
            role="tab"
            type="button"
            onClick={() => onExportChange("plain")}
          >
            Plain
          </button>
          <button
            aria-selected={activeExport === "json"}
            role="tab"
            type="button"
            onClick={() => onExportChange("json")}
          >
            JSON
          </button>
        </div>
      </div>
      <div className="export-toolbar">
        <button
          className="icon-button text-command"
          type="button"
          onClick={() => void navigator.clipboard?.writeText(exportValue)}
        >
          <Clipboard aria-hidden="true" size={16} />
          Copy
        </button>
      </div>
      <pre className="code-block export-code">{exportValue}</pre>
    </section>
  );
}
