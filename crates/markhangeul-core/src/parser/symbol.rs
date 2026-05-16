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

        if rest
            .get(..2)
            .is_some_and(|value| value.eq_ignore_ascii_case("T0"))
        {
            attributes.tone = Some(Tone::Neutral);
            index += 2;
            continue;
        }

        if rest
            .get(..2)
            .is_some_and(|value| value.eq_ignore_ascii_case("T1"))
        {
            attributes.tone = Some(Tone::One);
            index += 2;
            continue;
        }

        if rest
            .get(..2)
            .is_some_and(|value| value.eq_ignore_ascii_case("T2"))
        {
            attributes.tone = Some(Tone::Two);
            index += 2;
            continue;
        }

        if rest
            .get(..2)
            .is_some_and(|value| value.eq_ignore_ascii_case("T3"))
        {
            attributes.tone = Some(Tone::Three);
            index += 2;
            continue;
        }

        if rest
            .get(..2)
            .is_some_and(|value| value.eq_ignore_ascii_case("T4"))
        {
            attributes.tone = Some(Tone::Four);
            index += 2;
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
