use markhangeul_core::{
    export_json_ast, export_plain_markdown, parse_markhangeul, Duration, MarkHangeulToken, Pitch,
    Scope, Stress, Tone,
};

fn mark_nodes(source: &str) -> Vec<markhangeul_core::MarkHangeulNode> {
    parse_markhangeul(source)
        .nodes
        .into_iter()
        .filter_map(|node| match node {
            MarkHangeulToken::Markhangeul(mark) => Some(mark),
            MarkHangeulToken::Text(_) => None,
        })
        .collect()
}

#[test]
fn parses_symbol_annotation_for_hangul_grapheme() {
    let document = parse_markhangeul("안녕{↗—!}하세요");
    let nodes = mark_nodes("안녕{↗—!}하세요");
    let node = &nodes[0];

    assert_eq!(node.text, "녕");
    assert_eq!(node.scope, Scope::Grapheme);
    assert_eq!(node.attributes.pitch, Some(Pitch::Rise));
    assert_eq!(node.attributes.duration, Some(Duration::Long));
    assert_eq!(node.attributes.stress, Some(Stress::Strong));
    assert_eq!(export_plain_markdown(&document), "안녕하세요");
}

#[test]
fn parses_symbol_annotation_for_latin_word() {
    let nodes = mark_nodes("Hello{!↗} world{—}");

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].text, "Hello");
    assert_eq!(nodes[0].scope, Scope::Word);
    assert_eq!(nodes[0].attributes.stress, Some(Stress::Strong));
    assert_eq!(nodes[0].attributes.pitch, Some(Pitch::Rise));
    assert_eq!(nodes[1].attributes.duration, Some(Duration::Long));
}

#[test]
fn parses_key_value_annotation() {
    let nodes = mark_nodes("녕{pitch=rise,duration=long,stress=strong,lang=ko}");
    let node = &nodes[0];

    assert_eq!(node.attributes.pitch, Some(Pitch::Rise));
    assert_eq!(node.attributes.duration, Some(Duration::Long));
    assert_eq!(node.attributes.stress, Some(Stress::Strong));
    assert_eq!(node.attributes.lang.as_deref(), Some("ko"));
}

#[test]
fn parses_explicit_range_annotation() {
    let document = parse_markhangeul("((want to)){reduced=true,stress=weak,duration=short}");
    let nodes = mark_nodes("((want to)){reduced=true,stress=weak,duration=short}");
    let node = &nodes[0];

    assert_eq!(node.text, "want to");
    assert_eq!(node.scope, Scope::Range);
    assert_eq!(node.attributes.reduced, Some(true));
    assert_eq!(node.attributes.stress, Some(Stress::Weak));
    assert_eq!(node.attributes.duration, Some(Duration::Short));
    assert_eq!(export_plain_markdown(&document), "want to");
}

#[test]
fn parses_mandarin_tones() {
    let nodes = mark_nodes("妈{T1} 麻{T2} 马{T3} 骂{T4}");
    let tones: Vec<_> = nodes.iter().map(|node| node.attributes.tone).collect();

    assert_eq!(
        tones,
        vec![
            Some(Tone::One),
            Some(Tone::Two),
            Some(Tone::Three),
            Some(Tone::Four)
        ]
    );
}

#[test]
fn preserves_plain_markdown() {
    let source = "# 제목\n\n**Hello** world";
    let document = parse_markhangeul(source);

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(export_plain_markdown(&document), source);
}

#[test]
fn reports_invalid_values_and_unknown_symbols() {
    let document = parse_markhangeul("Hello{pitch=curve} 妈{T7}");
    let codes: Vec<_> = document
        .errors
        .iter()
        .map(|error| error.code.as_str())
        .collect();

    assert!(codes.contains(&"INVALID_VALUE"));
    assert!(codes.contains(&"UNKNOWN_SYMBOL"));
}

#[test]
fn exports_json_ast() {
    let document = parse_markhangeul("녕{↗}");
    let json = export_json_ast(&document);

    assert!(json.contains("\"type\""));
    assert!(json.contains("\"markhangeul\""));
    assert!(json.contains("\"pitch\""));
}
