use unicode_segmentation::UnicodeSegmentation;

use crate::ast::Scope;

#[derive(Debug, Clone)]
pub struct TargetMatch {
    pub text: String,
    pub start_in_prefix: usize,
    pub scope: Scope,
}

pub fn next_char_boundary(source: &str, index: usize) -> usize {
    source[index..]
        .chars()
        .next()
        .map(|ch| index + ch.len_utf8())
        .unwrap_or(index + 1)
}

pub fn find_implicit_target(prefix: &str) -> Option<TargetMatch> {
    if prefix.is_empty() {
        return None;
    }

    let graphemes: Vec<(usize, &str)> = prefix.grapheme_indices(true).collect();
    let (last_start, last_grapheme) = graphemes.last().copied()?;

    if last_grapheme.chars().all(char::is_whitespace) {
        return None;
    }

    if is_latin_word_grapheme(last_grapheme) {
        let mut start = last_start;

        for (candidate_start, grapheme) in graphemes.iter().rev() {
            if is_latin_word_grapheme(grapheme) {
                start = *candidate_start;
            } else {
                break;
            }
        }

        return Some(TargetMatch {
            text: prefix[start..].to_string(),
            start_in_prefix: start,
            scope: Scope::Word,
        });
    }

    if is_single_grapheme_scope(last_grapheme) {
        return Some(TargetMatch {
            text: last_grapheme.to_string(),
            start_in_prefix: last_start,
            scope: Scope::Grapheme,
        });
    }

    None
}

fn is_latin_word_grapheme(grapheme: &str) -> bool {
    grapheme.chars().all(|ch| {
        is_latin_char(ch) || ch.is_ascii_digit() || matches!(ch, '\'' | '’' | '.' | '-' | '_')
    })
}

fn is_latin_char(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0041..=0x005A
            | 0x0061..=0x007A
            | 0x00C0..=0x00FF
            | 0x0100..=0x017F
            | 0x0180..=0x024F
            | 0x1E00..=0x1EFF
    )
}

fn is_single_grapheme_scope(grapheme: &str) -> bool {
    grapheme.chars().any(|ch| {
        matches!(
            ch as u32,
            0x1100..=0x11FF
                | 0x3130..=0x318F
                | 0xAC00..=0xD7AF
                | 0x4E00..=0x9FFF
                | 0x3400..=0x4DBF
                | 0x3040..=0x309F
                | 0x30A0..=0x30FF
        )
    })
}
