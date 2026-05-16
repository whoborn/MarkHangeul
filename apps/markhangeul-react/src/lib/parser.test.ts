import { describe, expect, it } from "vitest";
import { exportPlainMarkdown, getMarkNodes } from "./exporters";
import { parseMarkHangeul } from "./parser";

describe("parseMarkHangeul", () => {
  it("parses symbol annotations for the previous Hangul grapheme", () => {
    const document = parseMarkHangeul("안녕{↗—!}하세요");
    const [node] = getMarkNodes(document);

    expect(node.text).toBe("녕");
    expect(node.scope).toBe("grapheme");
    expect(node.attributes).toMatchObject({
      pitch: "rise",
      duration: "long",
      stress: "strong",
    });
    expect(exportPlainMarkdown(document)).toBe("안녕하세요");
  });

  it("parses symbol annotations for the previous Latin word", () => {
    const document = parseMarkHangeul("Hello{!↗} world{—}");
    const nodes = getMarkNodes(document);

    expect(nodes).toHaveLength(2);
    expect(nodes[0]).toMatchObject({
      text: "Hello",
      scope: "word",
      attributes: {
        stress: "strong",
        pitch: "rise",
      },
    });
    expect(nodes[1].attributes.duration).toBe("long");
  });

  it("parses key-value annotations", () => {
    const document = parseMarkHangeul("녕{pitch=rise,duration=long,stress=strong,lang=ko}");
    const [node] = getMarkNodes(document);

    expect(node.attributes).toMatchObject({
      pitch: "rise",
      duration: "long",
      stress: "strong",
      lang: "ko",
    });
    expect(document.errors).toHaveLength(0);
  });

  it("parses explicit range annotations", () => {
    const document = parseMarkHangeul("((want to)){reduced=true,stress=weak,duration=short}");
    const [node] = getMarkNodes(document);

    expect(node.text).toBe("want to");
    expect(node.scope).toBe("range");
    expect(node.attributes).toMatchObject({
      reduced: true,
      stress: "weak",
      duration: "short",
    });
    expect(exportPlainMarkdown(document)).toBe("want to");
  });

  it("parses Mandarin tone shorthands", () => {
    const document = parseMarkHangeul("妈{T1} 麻{T2} 马{T3} 骂{T4}");
    const nodes = getMarkNodes(document);

    expect(nodes.map((node) => node.attributes.tone)).toEqual(["1", "2", "3", "4"]);
  });

  it("preserves ordinary markdown text", () => {
    const source = "# 제목\n\n**Hello** world";
    const document = parseMarkHangeul(source);

    expect(document.nodes).toEqual([{ type: "text", text: source, start: 0, end: source.length }]);
    expect(exportPlainMarkdown(document)).toBe(source);
  });

  it("reports malformed annotation values", () => {
    const document = parseMarkHangeul("Hello{pitch=curve} 妈{T7}");

    expect(document.errors.map((error) => error.code)).toContain("INVALID_VALUE");
    expect(document.errors.map((error) => error.code)).toContain("UNKNOWN_SYMBOL");
  });

  it("reports unclosed annotations without dropping source text", () => {
    const source = "안녕{↗";
    const document = parseMarkHangeul(source);

    expect(document.errors[0].code).toBe("UNCLOSED_ANNOTATION");
    expect(exportPlainMarkdown(document)).toBe(source);
  });
});
