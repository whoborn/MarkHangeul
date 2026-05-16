import type { MarkHangeulAstNode, MarkHangeulDocument, MarkHangeulNode } from "./types";

export function isMarkHangeulNode(node: MarkHangeulAstNode): node is MarkHangeulNode {
  return node.type === "markhangeul";
}

export function getMarkNodes(document: MarkHangeulDocument): MarkHangeulNode[] {
  return document.nodes.filter(isMarkHangeulNode);
}

export function exportPlainMarkdown(document: MarkHangeulDocument): string {
  return document.nodes.map((node) => node.text).join("");
}

export function exportJsonAst(document: MarkHangeulDocument): string {
  return JSON.stringify(document, null, 2);
}
