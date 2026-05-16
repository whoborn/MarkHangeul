import { parseAnnotation } from "./annotation";
import type {
  MarkHangeulAstNode,
  MarkHangeulDocument,
  MarkHangeulNode,
  MarkHangeulScope,
  ParseError,
  TextNode,
} from "./types";

type TargetMatch = {
  text: string;
  startInPrefix: number;
  scope: MarkHangeulScope;
};

const annotatedRangeStart = "((";
const annotatedRangeEnd = "))";

export function parseMarkHangeul(source: string): MarkHangeulDocument {
  const nodes: MarkHangeulAstNode[] = [];
  const errors: ParseError[] = [];
  let cursor = 0;
  let index = 0;
  let markIndex = 0;

  const pushText = (text: string, start: number) => {
    if (!text) return;
    const previous = nodes[nodes.length - 1];
    if (previous?.type === "text" && previous.end === start) {
      previous.text += text;
      previous.end = start + text.length;
      return;
    }
    nodes.push({
      type: "text",
      text,
      start,
      end: start + text.length,
    } satisfies TextNode);
  };

  while (index < source.length) {
    if (source.startsWith(annotatedRangeStart, index)) {
      const closeRange = source.indexOf(annotatedRangeEnd, index + annotatedRangeStart.length);

      if (closeRange === -1) {
        errors.push(
          makeParseError("UNCLOSED_RANGE", "범위 annotation의 닫힘 '))'가 없습니다.", index, 2),
        );
        index += 2;
        continue;
      }

      const annotationOpen = closeRange + annotatedRangeEnd.length;
      if (source[annotationOpen] === "{") {
        const annotationClose = findClosingBrace(source, annotationOpen);
        if (annotationClose === -1) {
          errors.push(
            makeParseError("UNCLOSED_ANNOTATION", "annotation의 닫힘 '}'가 없습니다.", annotationOpen, 1),
          );
          index = annotationOpen + 1;
          continue;
        }

        pushText(source.slice(cursor, index), cursor);

        const rawAnnotation = source.slice(annotationOpen + 1, annotationClose);
        const targetText = source.slice(index + annotatedRangeStart.length, closeRange);
        const parsed = parseAnnotation(rawAnnotation, annotationOpen + 1);
        const markNode = makeMarkNode({
          type: "markhangeul",
          id: makeNodeId(markIndex++),
          text: targetText,
          rawAnnotation,
          scope: "range",
          start: index,
          end: annotationClose + 1,
          annotationStart: annotationOpen,
          annotationEnd: annotationClose + 1,
          errors: parsed.errors,
          attributes: parsed.attributes,
        });

        nodes.push(markNode);
        errors.push(...parsed.errors);
        cursor = annotationClose + 1;
        index = cursor;
        continue;
      }
    }

    if (source[index] === "{") {
      const annotationClose = findClosingBrace(source, index);

      if (annotationClose === -1) {
        errors.push(makeParseError("UNCLOSED_ANNOTATION", "annotation의 닫힘 '}'가 없습니다.", index, 1));
        index += 1;
        continue;
      }

      const prefix = source.slice(cursor, index);
      const target = findImplicitTarget(prefix);

      if (target) {
        const targetStart = cursor + target.startInPrefix;
        pushText(prefix.slice(0, target.startInPrefix), cursor);

        const rawAnnotation = source.slice(index + 1, annotationClose);
        const parsed = parseAnnotation(rawAnnotation, index + 1);
        const markNode = makeMarkNode({
          type: "markhangeul",
          id: makeNodeId(markIndex++),
          text: target.text,
          rawAnnotation,
          scope: target.scope,
          start: targetStart,
          end: annotationClose + 1,
          annotationStart: index,
          annotationEnd: annotationClose + 1,
          errors: parsed.errors,
          attributes: parsed.attributes,
        });

        nodes.push(markNode);
        errors.push(...parsed.errors);
        cursor = annotationClose + 1;
        index = cursor;
        continue;
      }
    }

    index += 1;
  }

  pushText(source.slice(cursor), cursor);

  return {
    type: "document",
    source,
    nodes,
    errors,
  };
}

function findClosingBrace(source: string, openIndex: number): number {
  return source.indexOf("}", openIndex + 1);
}

function findImplicitTarget(prefix: string): TargetMatch | null {
  if (!prefix) return null;

  const chars = Array.from(prefix);
  const lastChar = chars[chars.length - 1];

  if (!lastChar || /\s/.test(lastChar)) return null;

  if (isLatinWordChar(lastChar)) {
    let charIndex = chars.length - 1;

    while (charIndex >= 0 && isLatinWordChar(chars[charIndex])) {
      charIndex -= 1;
    }

    const text = chars.slice(charIndex + 1).join("");
    return {
      text,
      startInPrefix: prefix.length - text.length,
      scope: "word",
    };
  }

  if (isSingleGraphemeScope(lastChar)) {
    return {
      text: lastChar,
      startInPrefix: prefix.length - lastChar.length,
      scope: "grapheme",
    };
  }

  return null;
}

function isLatinWordChar(char: string): boolean {
  return /^[\p{Script=Latin}0-9'_’.-]$/u.test(char);
}

function isSingleGraphemeScope(char: string): boolean {
  return /^(\p{Script=Hangul}|\p{Script=Han}|\p{Script=Hiragana}|\p{Script=Katakana})$/u.test(char);
}

function makeNodeId(index: number): string {
  return `mh-${index}`;
}

function makeMarkNode(node: MarkHangeulNode): MarkHangeulNode {
  return node;
}

function makeParseError(code: string, message: string, index: number, length: number): ParseError {
  return {
    code,
    message,
    index,
    length,
    severity: "error",
  };
}
