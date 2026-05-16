import type { CSSProperties } from "react";
import type {
  MarkHangeulAstNode,
  MarkHangeulDocument,
  MarkHangeulNode,
  MarkHangeulScope,
  ToneValue,
} from "../lib/types";

type MarkHangeulStyle = CSSProperties & Record<`--${string}`, string>;

type MarkHangeulRendererProps = {
  document: MarkHangeulDocument;
  selectedId: string | null;
  onSelect: (nodeId: string) => void;
};

export function MarkHangeulRenderer({ document, selectedId, onSelect }: MarkHangeulRendererProps) {
  return (
    <div className="render-surface" aria-label="MarkHangeul rendered output">
      {document.nodes.map((node, index) => renderNode(node, index, selectedId, onSelect))}
    </div>
  );
}

function renderNode(
  node: MarkHangeulAstNode,
  index: number,
  selectedId: string | null,
  onSelect: (nodeId: string) => void,
) {
  if (node.type === "text") {
    return <TextRun key={`text-${node.start}-${index}`} text={node.text} />;
  }

  return (
    <PronunciationRun
      key={node.id}
      node={node}
      selected={selectedId === node.id}
      onSelect={onSelect}
    />
  );
}

function TextRun({ text }: { text: string }) {
  return <>{text}</>;
}

function PronunciationRun({
  node,
  selected,
  onSelect,
}: {
  node: MarkHangeulNode;
  selected: boolean;
  onSelect: (nodeId: string) => void;
}) {
  const style = getRunStyle(node);
  const chars = Array.from(node.text);
  const hasTone = Boolean(node.attributes.tone);

  return (
    <span
      className="mh-mark"
      data-selected={selected ? "true" : "false"}
      data-scope={node.scope}
      data-error={node.errors.length > 0 ? "true" : "false"}
      data-volume={node.attributes.volume ?? "normal"}
      role="button"
      tabIndex={0}
      aria-label={`${node.text} annotation ${node.rawAnnotation}`}
      style={style}
      onClick={() => onSelect(node.id)}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") onSelect(node.id);
      }}
    >
      {hasTone ? <TonePath tone={node.attributes.tone!} /> : null}
      {chars.map((char, charIndex) => {
        if (char === "\n") return <br key={`${node.id}-br-${charIndex}`} />;
        return (
          <span
            key={`${node.id}-${charIndex}`}
            className="mh-char"
            style={getCharStyle(node, charIndex, chars.length)}
          >
            {char}
          </span>
        );
      })}
    </span>
  );
}

function TonePath({ tone }: { tone: ToneValue }) {
  const path = {
    "1": "M 4 6 L 96 6",
    "2": "M 4 18 C 32 16 58 10 96 4",
    "3": "M 4 8 C 24 20 58 20 96 5",
    "4": "M 4 4 C 32 7 64 14 96 20",
    neutral: "M 8 13 L 92 13",
  }[tone];

  return (
    <svg className="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true">
      <path d={path} />
    </svg>
  );
}

function getRunStyle(node: MarkHangeulNode): CSSProperties {
  const { duration, stress, volume } = node.attributes;
  const style: MarkHangeulStyle = {
    "--duration-pad": "0.02em",
    "--duration-scale-x": "1",
    "--stress-scale": "1",
  };

  if (duration === "short") {
    style["--duration-pad"] = "0";
    style["--duration-scale-x"] = "0.9";
  }

  if (duration === "long") {
    style["--duration-pad"] = "0.12em";
    style["--duration-scale-x"] = "1.12";
  }

  if (duration === "extra-long") {
    style["--duration-pad"] = "0.24em";
    style["--duration-scale-x"] = "1.24";
  }

  if (stress === "weak") {
    style["--stress-scale"] = "0.94";
    style.fontWeight = 410;
  }

  if (stress === "strong") {
    style["--stress-scale"] = "1.08";
    style.fontWeight = 720;
  }

  if (stress === "extra-strong") {
    style["--stress-scale"] = "1.15";
    style.fontWeight = 860;
  }

  if (volume === "soft") {
    style.opacity = 0.64;
  }

  return style;
}

function getCharStyle(node: MarkHangeulNode, charIndex: number, charCount: number): CSSProperties {
  const y = getPitchOffset(node, charIndex, charCount);
  const style: MarkHangeulStyle = {
    "--pitch-y": `${y}px`,
  };

  return style;
}

function getPitchOffset(node: MarkHangeulNode, charIndex: number, charCount: number): number {
  const denominator = Math.max(charCount - 1, 1);
  const progress = charIndex / denominator;
  const pitch = node.attributes.pitch;
  const tone = node.attributes.tone;

  if (tone) {
    switch (tone) {
      case "1":
        return -5;
      case "2":
        return interpolate(5, -6, progress);
      case "3":
        return progress < 0.5
          ? interpolate(0, 7, progress * 2)
          : interpolate(7, -4, (progress - 0.5) * 2);
      case "4":
        return interpolate(-6, 6, progress);
      case "neutral":
        return 1;
      default:
        return 0;
    }
  }

  switch (pitch) {
    case "low":
      return 5;
    case "mid":
      return 0;
    case "high":
      return -6;
    case "rise":
      return interpolate(4, -6, progress);
    case "fall":
      return interpolate(-5, 5, progress);
    default:
      return 0;
  }
}

function interpolate(from: number, to: number, progress: number): number {
  return from + (to - from) * progress;
}

export function describeScope(scope: MarkHangeulScope) {
  return {
    grapheme: "글자",
    word: "단어",
    range: "범위",
  }[scope];
}
