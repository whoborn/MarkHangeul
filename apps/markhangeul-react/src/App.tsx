import { useMemo, useState } from "react";
import { FileCode2, Play, RotateCcw } from "lucide-react";
import { ErrorPanel } from "./components/ErrorPanel";
import { ExportPanel } from "./components/ExportPanel";
import { Inspector } from "./components/Inspector";
import { MarkHangeulRenderer } from "./components/MarkHangeulRenderer";
import { exportJsonAst, exportPlainMarkdown, getMarkNodes } from "./lib/exporters";
import { parseMarkHangeul } from "./lib/parser";
import { sampleDocuments } from "./lib/samples";

function App() {
  const [source, setSource] = useState(sampleDocuments[0].source);
  const [selectedId, setSelectedId] = useState<string | null>("mh-0");
  const [activeExport, setActiveExport] = useState<"plain" | "json">("plain");

  const document = useMemo(() => parseMarkHangeul(source), [source]);
  const markNodes = useMemo(() => getMarkNodes(document), [document]);
  const plainMarkdown = useMemo(() => exportPlainMarkdown(document), [document]);
  const jsonAst = useMemo(() => exportJsonAst(document), [document]);

  const handleSourceChange = (nextSource: string) => {
    setSource(nextSource);
    const nextDocument = parseMarkHangeul(nextSource);
    const firstMark = getMarkNodes(nextDocument)[0];
    setSelectedId(firstMark?.id ?? null);
  };

  return (
    <main className="app-shell">
      <header className="top-bar">
        <div className="brand-block">
          <FileCode2 aria-hidden="true" size={24} />
          <div>
            <h1>MarkHangeul</h1>
            <p>마크한글</p>
          </div>
        </div>
        <div className="sample-bar" aria-label="samples">
          {sampleDocuments.map((sample) => (
            <button
              className="sample-button"
              key={sample.id}
              type="button"
              onClick={() => handleSourceChange(sample.source)}
            >
              <Play aria-hidden="true" size={15} />
              {sample.label}
            </button>
          ))}
          <button
            className="sample-button"
            type="button"
            onClick={() => handleSourceChange(sampleDocuments[0].source)}
          >
            <RotateCcw aria-hidden="true" size={15} />
            Reset
          </button>
        </div>
      </header>

      <section className="workspace-grid">
        <section className="panel editor-panel" aria-labelledby="source-title">
          <div className="panel-header">
            <div className="panel-title">
              <FileCode2 aria-hidden="true" size={18} />
              <h2 id="source-title">Source</h2>
            </div>
            <span className="counter">{source.length}</span>
          </div>
          <textarea
            className="source-editor"
            spellCheck={false}
            value={source}
            onChange={(event) => handleSourceChange(event.target.value)}
          />
        </section>

        <section className="panel preview-panel" aria-labelledby="preview-title">
          <div className="panel-header">
            <div className="panel-title">
              <Play aria-hidden="true" size={18} />
              <h2 id="preview-title">Render</h2>
            </div>
            <span className="counter">{markNodes.length}</span>
          </div>
          <MarkHangeulRenderer document={document} selectedId={selectedId} onSelect={setSelectedId} />
        </section>
      </section>

      <section className="analysis-grid">
        <Inspector nodes={markNodes} selectedId={selectedId} onSelect={setSelectedId} />
        <ErrorPanel errors={document.errors} />
        <ExportPanel
          activeExport={activeExport}
          jsonAst={jsonAst}
          plainMarkdown={plainMarkdown}
          onExportChange={setActiveExport}
        />
      </section>
    </main>
  );
}

export default App;
