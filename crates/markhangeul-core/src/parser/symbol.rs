use crate::ast::{Duration, MarkHangeulAttributes, Pitch, Stress, Tone, Volume};
use crate::errors::ParseError;

pub fn parse_symbol_annotation(
    part: &str,
    attributes: &mut MarkHangeulAttributes,
    errors: &mut Vec<ParseError>,
    base_index: usize,
) {
    let mut index = 0;

    while index < part.len() {
        let rest = &part[index..];
        let current = rest.chars().next().expect("index is on a char boundary");

        if current.is_whitespace() || current == ',' {
            index += current.len_utf8();
            continue;
        }

        if let Some((tone, byte_len)) = parse_tone_symbol(rest) {
            attributes.tone = Some(tone);
            index += byte_len;
            continue;
        }

        if rest.starts_with("˘˘") {
            attributes.duration = Some(Duration::ExtraShort);
            index += "˘˘".len();
            continue;
        }

        if rest.starts_with("——") {
            attributes.duration = Some(Duration::ExtraLong);
            index += "——".len();
            continue;
        }

        if rest.starts_with("!!") {
            attributes.stress = Some(Stress::ExtraStrong);
            index += 2;
            continue;
        }

        match current {
            '↗' => attributes.pitch = Some(Pitch::Rise),
            '↘' => attributes.pitch = Some(Pitch::Fall),
            '↑' => attributes.pitch = Some(Pitch::High),
            '↓' => attributes.pitch = Some(Pitch::Low),
            '·' => attributes.pitch = Some(Pitch::Mid),
            '˘' => attributes.duration = Some(Duration::Short),
            '-' => attributes.duration = Some(Duration::Normal),
            '—' => attributes.duration = Some(Duration::Long),
            '!' => attributes.stress = Some(Stress::Strong),
            '?' => attributes.stress = Some(Stress::Weak),
            '°' => attributes.volume = Some(Volume::Soft),
            '•' => attributes.volume = Some(Volume::Normal),
            '●' => attributes.volume = Some(Volume::Loud),
            _ => errors.push(ParseError::error(
                "UNKNOWN_SYMBOL",
                format!("알 수 없는 annotation 기호 '{current}'입니다."),
                base_index + index,
                current.len_utf8(),
            )),
        }

        index += current.len_utf8();
    }
}

fn parse_tone_symbol(rest: &str) -> Option<(Tone, usize)> {
    let first = rest.chars().next()?;
    if first != 'T' && first != 't' {
        return None;
    }

    let mut byte_len = first.len_utf8();
    let mut digits = String::new();

    for ch in rest[byte_len..].chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
            byte_len += ch.len_utf8();
        } else {
            break;
        }
    }

    if digits.is_empty() {
        return None;
    }

    if digits == "0" {
        return Some((Tone::neutral(), byte_len));
    }

    let value = digits.parse::<u8>().ok()?;
    (1..=9)
        .contains(&value)
        .then(|| (Tone::new(value.to_string()), byte_len))
}
