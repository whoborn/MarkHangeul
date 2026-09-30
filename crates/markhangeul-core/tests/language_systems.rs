use markhangeul_core::render_model::{resolve_pronunciation, tone_systems::TONE_SYSTEMS};
use markhangeul_core::{export_json_ast, parse_markhangeul, MarkHangeulToken, Phonation};

fn attributes(input: &str) -> markhangeul_core::MarkHangeulAttributes {
    let d = parse_markhangeul(input);
    assert!(d.errors.is_empty(), "{input}: {:?}", d.errors);
    match d.nodes.into_iter().next().unwrap() {
        MarkHangeulToken::Markhangeul(n) => n.attributes,
        _ => panic!(),
    }
}

#[test]
fn every_registered_category_resolves_with_unique_full_signature() {
    for system in TONE_SYSTEMS {
        let mut signatures = std::collections::HashSet::new();
        for tone in system.tones {
            let a = attributes(&format!("마{{toneSystem={},tone={}}}", system.id, tone.key));
            let r = resolve_pronunciation(&a).unwrap();
            assert!(
                signatures.insert(format!("{:?}-{:?}-{}", r.contour, r.phonation, r.checked)),
                "{} {}",
                system.id,
                tone.key
            );
            assert_eq!(r.category, Some(tone.label));
        }
    }
}

#[test]
fn thai_named_tones_take_precedence_over_generic_pitch_names() {
    let a = attributes("마{lang=th-TH,tone=high}");
    assert_eq!(resolve_pronunciation(&a).unwrap().contour, Some(vec![4, 5]));
    let a = attributes("마{tone=high}");
    assert_eq!(resolve_pronunciation(&a).unwrap().contour, Some(vec![5, 5]));
}

#[test]
fn hanoi_six_plus_two_does_not_invent_tone_numbers_or_merge_phonation() {
    let a = attributes("마{toneSystem=vi-hanoi,tone=sắc}");
    let b = attributes("마{toneSystem=vi-hanoi,tone=ngã}");
    let a = resolve_pronunciation(&a).unwrap();
    let b = resolve_pronunciation(&b).unwrap();
    assert_eq!(a.contour, b.contour);
    assert_eq!(a.phonation, Phonation::Modal);
    assert_eq!(b.phonation, Phonation::Glottalized);
    assert!(
        resolve_pronunciation(&attributes("맛{toneSystem=vi-hanoi-8,tone=D1}"))
            .unwrap()
            .checked
    );
    for input in [
        "마{toneSystem=vi-hanoi,tone=D1}",
        "마{toneSystem=vi-hanoi-8,tone=8}",
        "마{lang=vi,tone=b1}",
        "마{toneSystem=vi-hanoi-8,tone=d1,checked=false}",
        "마{toneSystem=vi-hanoi,tone=b1,checked=true}",
    ] {
        assert!(!parse_markhangeul(input).errors.is_empty(), "{input}");
    }
}

#[test]
fn explicit_overrides_and_extensions_roundtrip_without_mutating_source_defaults() {
    let a = attributes("마{toneSystem=vi-hanoi,tone=c2,toneContour=313,phonation=creaky}");
    let r = resolve_pronunciation(&a).unwrap();
    assert_eq!(r.contour, Some(vec![3, 1, 3]));
    assert_eq!(r.phonation, Phonation::Creaky);
    let a =
        attributes("맛{lang=custom-region,tone=8,toneContour=24,phonation=breathy,checked=true}");
    assert!(resolve_pronunciation(&a).unwrap().checked);
    let d = parse_markhangeul("맛{toneSystem=vi-hanoi-8,tone=d1}");
    let decoded: markhangeul_core::MarkHangeulDocument =
        serde_json::from_str(&export_json_ast(&d)).unwrap();
    assert_eq!(d, decoded);
    assert!(!export_json_ast(&d).contains("\"checked\"")); // inferred view, not silently persisted
}

#[test]
fn invalid_phonation_and_checked_values_report_errors() {
    for input in [
        "마{phonation=whisper}",
        "마{checked=maybe}",
        "맛{lang=yue,tone=2,checked=true}",
    ] {
        assert!(!parse_markhangeul(input).errors.is_empty());
    }
}
