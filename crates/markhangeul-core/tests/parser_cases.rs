use markhangeul_core::{
    export_json_ast, export_plain_markdown, parse_markhangeul, Duration, MarkHangeulToken, Pitch,
    Scope, Stress, Tone,
};

fn mark_nodes(source: &str) -> Vec<markhangeul_core::MarkHangeulNode> {
    parse_markhangeul(source)
        .nodes
        .into_iter()
        .filter_map(|node| match node {
            MarkHangeulToken::Markhangeul(mark) => Some(*mark),
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
    let nodes = mark_nodes("녕{pitch=rise,duration=slight-long,stress=strong,lang=ko}");
    let node = &nodes[0];

    assert_eq!(node.attributes.pitch, Some(Pitch::Rise));
    assert_eq!(node.attributes.duration, Some(Duration::SlightLong));
    assert_eq!(node.attributes.stress, Some(Stress::Strong));
    assert_eq!(node.attributes.lang.as_deref(), Some("ko"));
}

#[test]
fn parses_granular_duration_levels() {
    let nodes = mark_nodes(
        "a{duration=extra-short} b{duration=short} c{duration=slight-short} d{duration=normal} e{duration=slight-long} f{duration=long} g{duration=extra-long}",
    );
    let durations: Vec<_> = nodes.iter().map(|node| node.attributes.duration).collect();

    assert_eq!(
        durations,
        vec![
            Some(Duration::ExtraShort),
            Some(Duration::Short),
            Some(Duration::SlightShort),
            Some(Duration::Normal),
            Some(Duration::SlightLong),
            Some(Duration::Long),
            Some(Duration::ExtraLong),
        ]
    );
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
    let tones: Vec<_> = nodes
        .iter()
        .map(|node| node.attributes.tone.as_ref().map(Tone::as_str))
        .collect();

    assert_eq!(tones, vec![Some("1"), Some("2"), Some("3"), Some("4")]);
}

#[test]
fn parses_generic_tone_systems_and_contours() {
    let nodes = mark_nodes(
        "粤{lang=yue,tone=6,toneContour=22,soundShape=true,guideColor=true} checked{tone=8} a{T8}",
    );

    assert_eq!(
        nodes[0].attributes.tone.as_ref().map(Tone::as_str),
        Some("6")
    );
    assert_eq!(nodes[0].attributes.lang.as_deref(), Some("yue"));
    assert_eq!(nodes[0].attributes.tone_contour.as_deref(), Some("22"));
    assert_eq!(nodes[0].attributes.sound_shape, Some(true));
    assert_eq!(nodes[0].attributes.guide_color, Some(true));
    assert_eq!(
        nodes[1].attributes.tone.as_ref().map(Tone::as_str),
        Some("8")
    );
    assert_eq!(
        nodes[2].attributes.tone.as_ref().map(Tone::as_str),
        Some("8")
    );
}

#[test]
fn keeps_show_color_as_guide_color_alias() {
    let nodes = mark_nodes("아{duration=long,showColor=true}");

    assert_eq!(nodes[0].attributes.guide_color, Some(true));
}

#[test]
fn parses_sound_shape_hiding_aliases() {
    let nodes = mark_nodes("마{T2,hideGuide=true}");

    assert_eq!(nodes[0].attributes.sound_shape, Some(false));
}

#[test]
fn preserves_plain_markdown() {
    let source = "# 제목\n\n**Hello** world";
    let document = parse_markhangeul(source);

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(export_plain_markdown(&document), source);
}

#[test]
fn ignores_annotations_inside_markdown_code() {
    let source = "`녕{↗}`\n\n```md\nHello{!↗}\n```";
    let document = parse_markhangeul(source);

    assert_eq!(document.errors.len(), 0);
    assert_eq!(mark_nodes(source).len(), 0);
    assert_eq!(export_plain_markdown(&document), source);
}

#[test]
fn ignores_latex_math_braces() {
    let source = "수식: $\\frac{1}{2}$\n\n$$\\int_0^1 x^2 dx = \\frac{1}{3}$$\n\n안녕{↗}";
    let document = parse_markhangeul(source);
    let nodes = mark_nodes(source);

    assert_eq!(document.errors.len(), 0);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].text, "녕");
    assert_eq!(
        export_plain_markdown(&document),
        "수식: $\\frac{1}{2}$\n\n$$\\int_0^1 x^2 dx = \\frac{1}{3}$$\n\n안녕"
    );
}

#[test]
fn reports_invalid_values_and_unknown_symbols() {
    let document = parse_markhangeul("Hello{pitch=curve} 妈{tone=}");
    let codes: Vec<_> = document
        .errors
        .iter()
        .map(|error| error.code.as_str())
        .collect();

    assert!(codes.contains(&"INVALID_VALUE"));
    assert!(codes.contains(&"MALFORMED_PAIR"));
}

#[test]
fn exports_json_ast() {
    let document = parse_markhangeul("녕{↗}");
    let json = export_json_ast(&document);

    assert!(json.contains("\"type\""));
    assert!(json.contains("\"markhangeul\""));
    assert!(json.contains("\"pitch\""));
}
