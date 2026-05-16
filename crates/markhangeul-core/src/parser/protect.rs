#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedRange {
    pub start: usize,
    pub end: usize,
}

pub fn markdown_protected_ranges(source: &str) -> Vec<ProtectedRange> {
    let mut ranges = fenced_code_ranges(source);
    ranges.extend(inline_code_ranges(source, &ranges));
    ranges.extend(math_ranges(source, &ranges));
    ranges.sort_by_key(|range| (range.start, range.end));
    merge_ranges(ranges)
}

fn fenced_code_ranges(source: &str) -> Vec<ProtectedRange> {
    let mut ranges = Vec::new();
    let mut line_start = 0;
    let mut open_fence: Option<(usize, char, usize)> = None;

    while line_start < source.len() {
        let line_end = source[line_start..]
            .find('\n')
            .map(|offset| line_start + offset + 1)
            .unwrap_or(source.len());
        let line = &source[line_start..line_end];
        let content = line.trim_end_matches(['\n', '\r']);
        let indent = content.chars().take_while(|ch| *ch == ' ').count();
        let trimmed = if indent <= 3 {
            content[indent..].trim_end()
        } else {
            ""
        };

        if let Some((start, marker, length)) = open_fence {
            if is_matching_fence(trimmed, marker, length) {
                ranges.push(ProtectedRange {
                    start,
                    end: line_end,
                });
                open_fence = None;
            }
        } else if let Some((marker, length)) = opening_fence(trimmed) {
            open_fence = Some((line_start, marker, length));
        }

        line_start = line_end;
    }

    if let Some((start, _, _)) = open_fence {
        ranges.push(ProtectedRange {
            start,
            end: source.len(),
        });
    }

    ranges
}

fn opening_fence(line: &str) -> Option<(char, usize)> {
    let marker = line.chars().next()?;
    if marker != '`' && marker != '~' {
        return None;
    }

    let length = line.chars().take_while(|ch| *ch == marker).count();
    (length >= 3).then_some((marker, length))
}

fn is_matching_fence(line: &str, marker: char, min_length: usize) -> bool {
    let length = line.chars().take_while(|ch| *ch == marker).count();
    length >= min_length && line[length..].trim().is_empty()
}

fn inline_code_ranges(source: &str, protected_ranges: &[ProtectedRange]) -> Vec<ProtectedRange> {
    let mut ranges = Vec::new();
    let mut index = 0;

    while index < source.len() {
        if let Some(protected) = containing_range(index, protected_ranges) {
            index = protected.end;
            continue;
        }

        let Some(ch) = source[index..].chars().next() else {
            break;
        };

        if ch != '`' {
            index += ch.len_utf8();
            continue;
        }

        let fence_len = count_repeated(source, index, '`');
        let close = find_repeated(source, index + fence_len, '`', fence_len);

        if let Some(close) = close {
            ranges.push(ProtectedRange {
                start: index,
                end: close + fence_len,
            });
            index = close + fence_len;
        } else {
            index += fence_len;
        }
    }

    ranges
}

fn math_ranges(source: &str, protected_ranges: &[ProtectedRange]) -> Vec<ProtectedRange> {
    let mut ranges = Vec::new();
    let mut index = 0;

    while index < source.len() {
        if let Some(protected) = containing_range(index, protected_ranges) {
            index = protected.end;
            continue;
        }

        let Some(ch) = source[index..].chars().next() else {
            break;
        };

        if ch != '$' || is_escaped(source, index) {
            index += ch.len_utf8();
            continue;
        }

        let delimiter_len = if source[index..].starts_with("$$") {
            2
        } else {
            1
        };
        if delimiter_len == 1 && !can_open_inline_math(source, index) {
            index += 1;
            continue;
        }

        if let Some(close) = find_math_close(source, index + delimiter_len, delimiter_len) {
            ranges.push(ProtectedRange {
                start: index,
                end: close + delimiter_len,
            });
            index = close + delimiter_len;
        } else {
            index += delimiter_len;
        }
    }

    ranges
}

fn find_math_close(source: &str, start: usize, delimiter_len: usize) -> Option<usize> {
    let delimiter = if delimiter_len == 2 { "$$" } else { "$" };
    let mut index = start;

    while index < source.len() {
        if source[index..].starts_with(delimiter) && !is_escaped(source, index) {
            if delimiter_len == 2 || can_close_inline_math(source, index) {
                return Some(index);
            }
        }

        let ch = source[index..].chars().next()?;
        if delimiter_len == 1 && ch == '\n' {
            return None;
        }
        index += ch.len_utf8();
    }

    None
}

fn can_open_inline_math(source: &str, index: usize) -> bool {
    source[index + 1..]
        .chars()
        .next()
        .is_some_and(|ch| !ch.is_whitespace())
}

fn can_close_inline_math(source: &str, index: usize) -> bool {
    source[..index]
        .chars()
        .next_back()
        .is_some_and(|ch| !ch.is_whitespace())
}

fn is_escaped(source: &str, index: usize) -> bool {
    let mut slash_count = 0;
    for ch in source[..index].chars().rev() {
        if ch == '\\' {
            slash_count += 1;
        } else {
            break;
        }
    }
    slash_count % 2 == 1
}

fn containing_range(index: usize, ranges: &[ProtectedRange]) -> Option<ProtectedRange> {
    ranges
        .iter()
        .copied()
        .find(|range| range.start <= index && index < range.end)
}

fn count_repeated(source: &str, index: usize, marker: char) -> usize {
    source[index..]
        .chars()
        .take_while(|ch| *ch == marker)
        .map(char::len_utf8)
        .sum()
}

fn find_repeated(source: &str, start: usize, marker: char, byte_len: usize) -> Option<usize> {
    let mut index = start;

    while index < source.len() {
        let ch = source[index..].chars().next()?;
        if ch == marker && count_repeated(source, index, marker) == byte_len {
            return Some(index);
        }
        index += ch.len_utf8();
    }

    None
}

fn merge_ranges(ranges: Vec<ProtectedRange>) -> Vec<ProtectedRange> {
    let mut merged: Vec<ProtectedRange> = Vec::new();

    for range in ranges {
        if let Some(previous) = merged.last_mut() {
            if range.start <= previous.end {
                previous.end = previous.end.max(range.end);
                continue;
            }
        }
        merged.push(range);
    }

    merged
}
