mod key_value;
mod range;
mod symbol;
mod tokenize;

use crate::ast::{MarkHangeulDocument, MarkHangeulNode, MarkHangeulToken, Scope, TextNode};
use crate::errors::ParseError;

use self::key_value::parse_key_value_annotation;
use self::range::{find_closing_brace, RANGE_CLOSE, RANGE_OPEN};
use self::symbol::parse_symbol_annotation;
use self::tokenize::{find_implicit_target, next_char_boundary};

#[derive(Debug, Clone)]
struct AnnotationParseResult {
    attributes: crate::ast::MarkHangeulAttributes,
    errors: Vec<ParseError>,
}

pub fn parse_markhangeul(source: &str) -> MarkHangeulDocument {
    let mut nodes = Vec::new();
    let mut errors = Vec::new();
    let mut cursor = 0;
    let mut index = 0;
    let mut mark_index = 0;

    while index < source.len() {
        if source[index..].starts_with(RANGE_OPEN) {
            if let Some(close_range) = source[index + RANGE_OPEN.len()..].find(RANGE_CLOSE) {
                let close_range = index + RANGE_OPEN.len() + close_range;
                let annotation_open = close_range + RANGE_CLOSE.len();

                if source[annotation_open..].starts_with('{') {
                    match find_closing_brace(source, annotation_open) {
                        Some(annotation_close) => {
                            push_text(&mut nodes, &source[cursor..index], cursor);

                            let target_text = &source[index + RANGE_OPEN.len()..close_range];
                            let raw_annotation = &source[annotation_open + 1..annotation_close];
                            let parsed = parse_annotation(raw_annotation, annotation_open + 1);
                            let node_errors = parsed.errors.clone();

                            nodes.push(MarkHangeulToken::Markhangeul(MarkHangeulNode {
                                id: make_node_id(mark_index),
                                text: target_text.to_string(),
                                raw_annotation: raw_annotation.to_string(),
                                scope: Scope::Range,
                                attributes: parsed.attributes,
                                start: index,
                                end: annotation_close + 1,
                                annotation_start: annotation_open,
                                annotation_end: annotation_close + 1,
                                errors: node_errors,
                            }));

                            mark_index += 1;
                            errors.extend(parsed.errors);
                            cursor = annotation_close + 1;
                            index = cursor;
                            continue;
                        }
                        None => {
                            errors.push(ParseError::error(
                                "UNCLOSED_ANNOTATION",
                                "annotation의 닫힘 '}'가 없습니다.",
                                annotation_open,
                                1,
                            ));
                            index = next_char_boundary(source, annotation_open);
                            continue;
                        }
                    }
                }
            } else {
                errors.push(ParseError::error(
                    "UNCLOSED_RANGE",
                    "범위 annotation의 닫힘 '))'가 없습니다.",
                    index,
                    RANGE_OPEN.len(),
                ));
                index = next_char_boundary(source, index);
                continue;
            }
        }

        if source[index..].starts_with('{') {
            match find_closing_brace(source, index) {
                Some(annotation_close) => {
                    let prefix = &source[cursor..index];

                    if let Some(target) = find_implicit_target(prefix) {
                        push_text(&mut nodes, &prefix[..target.start_in_prefix], cursor);

                        let raw_annotation = &source[index + 1..annotation_close];
                        let parsed = parse_annotation(raw_annotation, index + 1);
                        let node_errors = parsed.errors.clone();
                        let target_start = cursor + target.start_in_prefix;

                        nodes.push(MarkHangeulToken::Markhangeul(MarkHangeulNode {
                            id: make_node_id(mark_index),
                            text: target.text,
                            raw_annotation: raw_annotation.to_string(),
                            scope: target.scope,
                            attributes: parsed.attributes,
                            start: target_start,
                            end: annotation_close + 1,
                            annotation_start: index,
                            annotation_end: annotation_close + 1,
                            errors: node_errors,
                        }));

                        mark_index += 1;
                        errors.extend(parsed.errors);
                        cursor = annotation_close + 1;
                        index = cursor;
                        continue;
                    }

                    index = next_char_boundary(source, index);
                    continue;
                }
                None => {
                    errors.push(ParseError::error(
                        "UNCLOSED_ANNOTATION",
                        "annotation의 닫힘 '}'가 없습니다.",
                        index,
                        1,
                    ));
                    index = next_char_boundary(source, index);
                    continue;
                }
            }
        }

        index = next_char_boundary(source, index);
    }

    push_text(&mut nodes, &source[cursor..], cursor);
    MarkHangeulDocument::new(source, nodes, errors)
}

fn parse_annotation(raw_annotation: &str, base_index: usize) -> AnnotationParseResult {
    let raw = raw_annotation.trim();
    let mut attributes = crate::ast::MarkHangeulAttributes::default();
    let mut errors = Vec::new();

    if raw.is_empty() {
        errors.push(ParseError::error(
            "EMPTY_ANNOTATION",
            "빈 annotation입니다.",
            base_index,
            raw_annotation.len().max(1),
        ));
        return AnnotationParseResult { attributes, errors };
    }

    let parts: Vec<&str> = raw
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();

    if parts.iter().any(|part| part.contains('=')) {
        for part in parts {
            let local_offset = raw_annotation.find(part).unwrap_or_default();
            if part.contains('=') {
                parse_key_value_annotation(
                    part,
                    &mut attributes,
                    &mut errors,
                    base_index + local_offset,
                );
            } else {
                parse_symbol_annotation(
                    part,
                    &mut attributes,
                    &mut errors,
                    base_index + local_offset,
                );
            }
        }
    } else {
        let offset = raw_annotation.find(raw).unwrap_or_default();
        parse_symbol_annotation(raw, &mut attributes, &mut errors, base_index + offset);
    }

    AnnotationParseResult { attributes, errors }
}

fn push_text(nodes: &mut Vec<MarkHangeulToken>, text: &str, start: usize) {
    if text.is_empty() {
        return;
    }

    if let Some(MarkHangeulToken::Text(previous)) = nodes.last_mut() {
        if previous.end == start {
            previous.text.push_str(text);
            previous.end = start + text.len();
            return;
        }
    }

    nodes.push(MarkHangeulToken::Text(TextNode {
        text: text.to_string(),
        start,
        end: start + text.len(),
    }));
}

fn make_node_id(index: usize) -> String {
    format!("mh-{index}")
}
