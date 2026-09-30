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
    if matches!(
        key.as_str(),
        "sound_shape"
            | "hide_sound_shape"
            | "guide_color"
            | "nasal"
            | "aspiration"
            | "fortis"
            | "lenis"
            | "palatalization"
            | "retroflexion"
            | "liaison"
            | "reduced"
            | "assimilation"
            | "deletion"
    ) && !matches!(
        normalized_value.as_str(),
        "true" | "false" | "1" | "0" | "yes" | "no" | "y" | "n" | "on" | "off"
    ) {
        errors.push(ParseError::error(
            "INVALID_VALUE",
            format!("'{key}'는 boolean 값이어야 합니다."),
            base_index + separator_index + 1,
            raw_value.len(),
        ));
        return;
    }

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
        "tone_system" => attributes.tone_system = Some(value.to_string()),
        "tone_contour" => attributes.tone_contour = Some(value.to_string()),
        "sound_shape" => attributes.sound_shape = Some(parse_bool(value)),
        "hide_sound_shape" => attributes.sound_shape = Some(!parse_bool(value)),
        "guide_color" => attributes.guide_color = Some(parse_bool(value)),
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
    match key.trim().to_ascii_lowercase().replace('-', "_").as_str() {
        "syllablerole" => "syllable_role".to_string(),
        "tonesystem" => "tone_system".to_string(),
        "tonecontour" => "tone_contour".to_string(),
        "soundshape" | "guide" | "showguide" | "shapeguide" | "contourguide" => {
            "sound_shape".to_string()
        }
        "hidesoundshape" | "hide_soundshape" | "hide_sound_shape" | "hideshape" | "hide_shape"
        | "hideguide" | "hide_guide" | "noguide" | "no_guide" | "hidecontourguide"
        | "hide_contour_guide" => "hide_sound_shape".to_string(),
        "guidecolor" | "guide_color" | "showcolor" | "show_color" | "durationcolor"
        | "duration_color" | "lengthcolor" | "length_color" | "colorguide" | "color_guide"
        | "visualcolor" | "visual_color" => "guide_color".to_string(),
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
        "extra-short" | "extra_short" | "extrashort" | "very-short" | "very_short"
        | "veryshort" | "ultra-short" | "ultra_short" | "xs" | "아주짧게" => {
            Some(Duration::ExtraShort)
        }
        "short" | "brief" | "s" | "보통짧게" => Some(Duration::Short),
        "slight-short" | "slight_short" | "slightshort" | "semi-short" | "semishort"
        | "little-short" | "littleshort" | "조금짧게" => Some(Duration::SlightShort),
        "normal" | "mid" => Some(Duration::Normal),
        "slight-long" | "slight_long" | "slightlong" | "semi-long" | "semilong" | "little-long"
        | "littlelong" | "조금길게" => Some(Duration::SlightLong),
        "long" | "l" | "보통길게" => Some(Duration::Long),
        "extra-long" | "extra_long" | "extralong" | "very-long" | "very_long" | "verylong"
        | "ultra-long" | "ultra_long" | "xl" | "아주길게" => Some(Duration::ExtraLong),
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
        "0" | "t0" | "neutral" | "none" => Some(Tone::neutral()),
        _ if value
            .strip_prefix('t')
            .is_some_and(|digits| parse_tone_number(digits).is_some()) =>
        {
            Some(Tone::new(value.trim_start_matches('t').to_string()))
        }
        _ if parse_tone_number(value).is_some() => Some(Tone::new(value.to_string())),
        _ if is_named_tone(value) => Some(Tone::new(value.to_string())),
        _ => None,
    }
}

fn parse_tone_number(value: &str) -> Option<u8> {
    let parsed = value.parse::<u8>().ok()?;
    (1..=9).contains(&parsed).then_some(parsed)
}

fn is_named_tone(value: &str) -> bool {
    value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | ':'))
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
