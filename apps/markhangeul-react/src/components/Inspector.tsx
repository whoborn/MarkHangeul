import { Braces, CircleAlert } from "lucide-react";
import { describeScope } from "./MarkHangeulRenderer";
import type { MarkHangeulNode } from "../lib/types";

type InspectorProps = {
  nodes: MarkHangeulNode[];
  selectedId: string | null;
  onSelect: (nodeId: string) => void;
};

export function Inspector({ nodes, selectedId, onSelect }: InspectorProps) {
  const selectedNode = nodes.find((node) => node.id === selectedId) ?? nodes[0] ?? null;

  return (
    <section className="panel inspector-panel" aria-labelledby="inspector-title">
      <div className="panel-header">
        <div className="panel-title">
          <Braces aria-hidden="true" size={18} />
          <h2 id="inspector-title">Inspector</h2>
        </div>
        <span className="counter">{nodes.length}</span>
      </div>

      <div className="inspector-grid">
        <div className="node-list" aria-label="annotations">
          {nodes.length === 0 ? <div className="empty-state">No annotations</div> : null}
          {nodes.map((node) => (
            <button
              className="node-row"
              data-selected={selectedId === node.id ? "true" : "false"}
              key={node.id}
              onClick={() => onSelect(node.id)}
              type="button"
            >
              <span className="node-target">{node.text}</span>
              <span className="node-meta">{describeScope(node.scope)}</span>
              {node.errors.length > 0 ? <CircleAlert aria-hidden="true" size={15} /> : null}
            </button>
          ))}
        </div>

        <div className="attribute-view">
          {selectedNode ? (
            <>
              <dl className="detail-list">
                <div>
                  <dt>target</dt>
                  <dd>{selectedNode.text}</dd>
                </div>
                <div>
                  <dt>scope</dt>
                  <dd>{selectedNode.scope}</dd>
                </div>
                <div>
                  <dt>raw</dt>
                  <dd>{selectedNode.rawAnnotation}</dd>
                </div>
              </dl>
              <pre className="code-block">{JSON.stringify(selectedNode.attributes, null, 2)}</pre>
            </>
          ) : (
            <div className="empty-state">No selection</div>
          )}
        </div>
      </div>
    </section>
  );
}
