use crate::ast::{AttributeValue, Duration, MarkHangeulAttributes, Pitch, Stress, Tone, Volume};
use crate::errors::ParseError;

pub fn parse_key_value_annotation(
    part: &str,
    attributes: &mut MarkHangeulAttributes,
    errors: &mut Vec<ParseError>,
    base_index: usize,
) {
    let Some(separator_index) = part.find('=') else {
        errors.push(ParseError::error(
            "MALFORMED_PAIR",
            "key=value 형식이 아닙니다.",
            base_index,
            part.len(),
        ));
        return;
    };

    let raw_key = part[..separator_index].trim();
    let raw_value = part[separator_index + 1..].trim();

    if raw_key.is_empty() || raw_value.is_empty() {
        errors.push(ParseError::error(
            "MALFORMED_PAIR",
            "key 또는 value가 비어 있습니다.",
            base_index,
            part.len(),
        ));
        return;
    }

    let key = normalize_key(raw_key);
    let value = raw_value.trim_matches(['"', '\'']);
    let normalized_value = value.to_ascii_lowercase();

    match key.as_str() {
        "pitch" => assign_or_error(
            parse_pitch(&normalized_value),
            |value| attributes.pitch = Some(value),
            "pitch",
            value,
            errors,
            base_index + separator_index + 1,
            raw_value.len(),
        ),
        "duration" => assign_or_error(
            parse_duration(&normalized_value),
            |value| attributes.duration = Some(value),
            "duration",
            value,
            errors,
            base_index + separator_index + 1,
            raw_value.len(),
        ),
        "stress" => assign_or_error(
            parse_stress(&normalized_value),
            |value| attributes.stress = Some(value),
            "stress",
            value,
            errors,
            base_index + separator_index + 1,
            raw_value.len(),
        ),
        "volume" => assign_or_error(
            parse_volume(&normalized_value),
            |value| attributes.volume = Some(value),
            "volume",
            value,
            errors,
            base_index + separator_index + 1,
            raw_value.len(),
        ),
        "tone" => assign_or_error(
            parse_tone(&normalized_value),
            |value| attributes.tone = Some(value),
            "tone",
            value,
            errors,
            base_index + separator_index + 1,
            raw_value.len(),
        ),
        "ipa" => attributes.ipa = Some(value.to_string()),
        "phoneme" => attributes.phoneme = Some(value.to_string()),
        "lang" => attributes.lang = Some(value.to_string()),
        "note" => attributes.note = Some(value.to_string()),
        "mora" => attributes.mora = Some(value.to_string()),
        "syllable_role" => attributes.syllable_role = Some(value.to_string()),
        "nasal" => attributes.nasal = Some(parse_bool(value)),
        "aspiration" => attributes.aspiration = Some(parse_bool(value)),
        "fortis" => attributes.fortis = Some(parse_bool(value)),
        "lenis" => attributes.lenis = Some(parse_bool(value)),
        "palatalization" => attributes.palatalization = Some(parse_bool(value)),
        "retroflexion" => attributes.retroflexion = Some(parse_bool(value)),
        "liaison" => attributes.liaison = Some(parse_bool(value)),
        "reduced" => attributes.reduced = Some(parse_bool(value)),
        "assimilation" => attributes.assimilation = Some(parse_bool(value)),
        "deletion" => attributes.deletion = Some(parse_bool(value)),
        _ => {
            attributes
                .extras
                .insert(raw_key.to_string(), parse_scalar(value));
        }
    }
}

fn assign_or_error<T>(
    parsed: Option<T>,
    assign: impl FnOnce(T),
    key: &str,
    value: &str,
    errors: &mut Vec<ParseError>,
    index: usize,
    length: usize,
) {
    if let Some(parsed) = parsed {
        assign(parsed);
    } else {
        errors.push(ParseError::error(
            "INVALID_VALUE",
            format!("'{key}'에 사용할 수 없는 값 '{value}'입니다."),
            index,
            length,
        ));
    }
}

fn normalize_key(key: &str) -> String {
    match key.trim().replace('-', "_").as_str() {
        "syllablerole" => "syllable_role".to_string(),
        normalized => normalized.to_string(),
    }
}

fn parse_pitch(value: &str) -> Option<Pitch> {
    match value {
        "low" | "down" => Some(Pitch::Low),
        "mid" | "middle" => Some(Pitch::Mid),
        "high" | "up" => Some(Pitch::High),
        "rise" | "rising" => Some(Pitch::Rise),
        "fall" | "falling" => Some(Pitch::Fall),
        _ => None,
    }
}

fn parse_duration(value: &str) -> Option<Duration> {
    match value {
        "short" | "brief" => Some(Duration::Short),
        "normal" | "mid" => Some(Duration::Normal),
        "long" => Some(Duration::Long),
        "extra-long" | "extra_long" | "extralong" | "verylong" => Some(Duration::ExtraLong),
        _ => None,
    }
}

fn parse_stress(value: &str) -> Option<Stress> {
    match value {
        "weak" | "light" => Some(Stress::Weak),
        "normal" => Some(Stress::Normal),
        "strong" | "stressed" => Some(Stress::Strong),
        "extra-strong" | "extra_strong" | "extrastrong" => Some(Stress::ExtraStrong),
        _ => None,
    }
}

fn parse_volume(value: &str) -> Option<Volume> {
    match value {
        "soft" | "quiet" | "weak" => Some(Volume::Soft),
        "normal" => Some(Volume::Normal),
        "loud" | "strong" => Some(Volume::Loud),
        _ => None,
    }
}

fn parse_tone(value: &str) -> Option<Tone> {
    match value {
        "0" | "t0" | "neutral" | "none" => Some(Tone::Neutral),
        "1" | "t1" => Some(Tone::One),
        "2" | "t2" => Some(Tone::Two),
        "3" | "t3" => Some(Tone::Three),
        "4" | "t4" => Some(Tone::Four),
        _ => None,
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "true" | "1" | "yes" | "y" | "on"
    )
}

fn parse_scalar(value: &str) -> AttributeValue {
    match value.to_ascii_lowercase().as_str() {
        "true" => AttributeValue::Bool(true),
        "false" => AttributeValue::Bool(false),
        _ => value
            .parse::<i64>()
            .map(AttributeValue::Number)
            .unwrap_or_else(|_| AttributeValue::String(value.to_string())),
    }
}
