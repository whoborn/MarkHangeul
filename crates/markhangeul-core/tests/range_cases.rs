use markhangeul_core::{export_plain_markdown, parse_markhangeul};

#[test]
fn reports_unclosed_range_without_dropping_source_text() {
    let source = "((닫히지 않은 범위){pitch=rise}";
    let document = parse_markhangeul(source);

    assert_eq!(document.errors[0].code, "UNCLOSED_RANGE");
    assert_eq!(export_plain_markdown(&document), source);
}

#[test]
fn reports_unclosed_annotation_without_dropping_source_text() {
    let source = "안녕{↗";
    let document = parse_markhangeul(source);

    assert_eq!(document.errors[0].code, "UNCLOSED_ANNOTATION");
    assert_eq!(export_plain_markdown(&document), source);
}
