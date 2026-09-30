pub const RANGE_OPEN: &str = "((";
pub const RANGE_CLOSE: &str = "))";

pub fn find_closing_brace(source: &str, open_index: usize) -> Option<usize> {
    let mut quote = None;
    let mut escaped = false;
    for (offset, ch) in source[open_index + 1..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
        } else if ch == '}' {
            return Some(open_index + 1 + offset);
        } else if ch == '\n' || ch == '{' {
            return None;
        }
    }
    None
}

pub fn annotation_parts(raw: &str) -> Vec<(usize, &str)> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut quote = None;
    let mut escaped = false;
    for (index, ch) in raw.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
        }
        if ch == ',' {
            parts.push((start, &raw[start..index]));
            start = index + 1;
        }
    }
    parts.push((start, &raw[start..]));
    parts
        .into_iter()
        .filter_map(|(offset, part)| {
            let trimmed = part.trim();
            (!trimmed.is_empty())
                .then_some((offset + part.len() - part.trim_start().len(), trimmed))
        })
        .collect()
}
