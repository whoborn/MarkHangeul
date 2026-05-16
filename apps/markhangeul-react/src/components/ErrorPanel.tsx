import { TriangleAlert } from "lucide-react";
import type { ParseError } from "../lib/types";

export function ErrorPanel({ errors }: { errors: ParseError[] }) {
  return (
    <section className="panel error-panel" aria-labelledby="error-title" data-empty={errors.length === 0}>
      <div className="panel-header">
        <div className="panel-title">
          <TriangleAlert aria-hidden="true" size={18} />
          <h2 id="error-title">Errors</h2>
        </div>
        <span className="counter">{errors.length}</span>
      </div>
      <div className="error-list">
        {errors.length === 0 ? <div className="empty-state">No errors</div> : null}
        {errors.map((error, index) => (
          <div className="error-row" key={`${error.code}-${error.index}-${index}`}>
            <strong>{error.code}</strong>
            <span>{error.message}</span>
            <code>@{error.index}</code>
          </div>
        ))}
      </div>
    </section>
  );
}
