use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedRange {
    pub start: usize,
    pub end: usize,
}

// Only ordinary Markdown text is eligible. Destinations, HTML, code, math,
// reference definitions and image descriptions remain byte-for-byte intact.
pub fn markdown_protected_ranges(source: &str) -> Vec<ProtectedRange> {
    let mut editable = Vec::new();
    let mut blocked = 0usize;
    for (event, range) in Parser::new_ext(source, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_) | Tag::Image { .. } | Tag::HtmlBlock) => blocked += 1,
            Event::End(TagEnd::CodeBlock | TagEnd::Image | TagEnd::HtmlBlock) => {
                blocked = blocked.saturating_sub(1)
            }
            Event::Text(_) if blocked == 0 => editable.push(range),
            _ => {}
        }
    }
    editable.sort_by_key(|r| r.start);
    let mut ranges = Vec::new();
    let mut end = 0;
    for range in editable {
        if range.start > end {
            ranges.push(ProtectedRange {
                start: end,
                end: range.start,
            });
        }
        end = end.max(range.end);
    }
    if end < source.len() {
        ranges.push(ProtectedRange {
            start: end,
            end: source.len(),
        });
    }
    ranges
}
