use markhangeul_core::render_model::resolve_tone;
use markhangeul_core::{export_plain_markdown, parse_markhangeul, MarkHangeulToken};

#[test]
fn protects_markdown_contexts_and_escapes() {
    for source in [
        "    마{T2}\n",
        "> ```md\n> 마{T2}\n> ```",
        "![마{T2}](img.png)",
        "[링크](https://example.com/마{T2})",
        "[id]: https://example.com/마{T2}\n",
        "<span title=\"마{T2}\">text</span>",
        "마\\{T2}",
        "`마{T2}`",
        "$마{T2}$",
    ] {
        let d = parse_markhangeul(source);
        assert_eq!(export_plain_markdown(&d), source, "{source}");
        assert!(
            d.nodes
                .iter()
                .all(|n| matches!(n, MarkHangeulToken::Text(_))),
            "{source}"
        );
    }
}

#[test]
fn preserves_link_labels_emphasis_and_graphemes() {
    for (input, expected) in [
        ("_hello{T2}_", "_hello_"),
        ("[마{T2}](https://example.com)", "[마](https://example.com)"),
        ("마{T2}", "마"),
        ("cafe\u{301}{T2}", "cafe\u{301}"),
    ] {
        assert_eq!(export_plain_markdown(&parse_markhangeul(input)), expected);
    }
}

#[test]
fn rejects_invalid_profiles_and_booleans_without_losing_source() {
    for input in [
        "마{toneContour=2x4}",
        "마{toneContour=6}",
        "마{tone=8}",
        "마{tone=2,toneSystem=unknown-region}",
        "마{soundShape=maybe}",
        "마{tone=custom}",
    ] {
        let d = parse_markhangeul(input);
        assert!(!d.errors.is_empty(), "{input}");
        assert_eq!(export_plain_markdown(&d), input);
    }
}

#[test]
fn resolves_four_six_eight_distinct_profiles_and_explicit_override() {
    for (system, count) in [("mandarin", 4), ("yue", 6), ("generic-8", 8)] {
        let mut profiles = std::collections::HashSet::new();
        for tone in 1..=count {
            let d = parse_markhangeul(&format!("마{{toneSystem={system},tone={tone}}}"));
            assert!(d.errors.is_empty());
            let MarkHangeulToken::Markhangeul(n) = &d.nodes[0] else {
                panic!()
            };
            assert!(profiles.insert(resolve_tone(&n.attributes).unwrap().unwrap()));
        }
        assert_eq!(profiles.len(), count);
    }
    let d = parse_markhangeul("마{lang=yue,toneSystem=mandarin,tone=2} 마{toneContour=151}");
    let nodes: Vec<_> = d
        .nodes
        .iter()
        .filter_map(|n| {
            if let MarkHangeulToken::Markhangeul(n) = n {
                Some(n)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(
        resolve_tone(&nodes[0].attributes).unwrap(),
        Some(vec![3, 5])
    );
    assert_eq!(
        resolve_tone(&nodes[1].attributes).unwrap(),
        Some(vec![1, 5, 1])
    );
}

#[test]
fn quoted_commas_and_braces_are_one_value() {
    let d = parse_markhangeul("마{T2,note=\"a,b}c\"}");
    assert!(d.errors.is_empty(), "{:?}", d.errors);
    let MarkHangeulToken::Markhangeul(n) = &d.nodes[0] else {
        panic!()
    };
    assert_eq!(n.attributes.note.as_deref(), Some("a,b}c"));
}
