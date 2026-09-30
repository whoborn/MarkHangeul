use crate::ast::{Duration, MarkHangeulAttributes, Pitch, Stress, Tone, Volume};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderClasses {
    pub pitch: Option<&'static str>,
    pub duration: Option<&'static str>,
    pub stress: Option<&'static str>,
    pub volume: Option<&'static str>,
    pub tone: Option<&'static str>,
}

pub fn render_classes(attributes: &MarkHangeulAttributes) -> RenderClasses {
    RenderClasses {
        pitch: attributes.pitch.map(pitch_class),
        duration: attributes.duration.map(duration_class),
        stress: attributes.stress.map(stress_class),
        volume: attributes.volume.map(volume_class),
        tone: attributes.tone.as_ref().map(tone_class),
    }
}

fn pitch_class(value: Pitch) -> &'static str {
    match value {
        Pitch::Low => "mh-pitch-low",
        Pitch::Mid => "mh-pitch-mid",
        Pitch::High => "mh-pitch-high",
        Pitch::Rise => "mh-pitch-rise",
        Pitch::Fall => "mh-pitch-fall",
    }
}

fn duration_class(value: Duration) -> &'static str {
    match value {
        Duration::ExtraShort => "mh-duration-extra-short",
        Duration::Short => "mh-duration-short",
        Duration::SlightShort => "mh-duration-slight-short",
        Duration::Normal => "mh-duration-normal",
        Duration::SlightLong => "mh-duration-slight-long",
        Duration::Long => "mh-duration-long",
        Duration::ExtraLong => "mh-duration-extra-long",
    }
}

fn stress_class(value: Stress) -> &'static str {
    match value {
        Stress::Weak => "mh-stress-weak",
        Stress::Normal => "mh-stress-normal",
        Stress::Strong => "mh-stress-strong",
        Stress::ExtraStrong => "mh-stress-extra-strong",
    }
}

fn volume_class(value: Volume) -> &'static str {
    match value {
        Volume::Soft => "mh-volume-soft",
        Volume::Normal => "mh-volume-normal",
        Volume::Loud => "mh-volume-loud",
    }
}

fn tone_class(value: &Tone) -> &'static str {
    if value.as_str() == "neutral" {
        "mh-tone-neutral"
    } else {
        "mh-tone"
    }
}

/// Explicit contours are authoritative. Numeric categories have no universal pitch.
pub fn resolve_tone(a: &MarkHangeulAttributes) -> Result<Option<Vec<u8>>, String> {
    if let Some(contour) = &a.tone_contour {
        if !(2..=16).contains(&contour.len())
            || !contour.bytes().all(|b| (b'1'..=b'5').contains(&b))
        {
            return Err("toneContour는 1~5 숫자 2~16개여야 합니다.".into());
        }
        return Ok(Some(contour.bytes().map(|b| b - b'0').collect()));
    }
    let Some(tone) = &a.tone else {
        return Ok(None);
    };
    let value = tone.as_str();
    let named = match value {
        "neutral" => Some("33"),
        "high" | "high-level" | "level-high" => Some("55"),
        "mid" | "mid-level" | "level-mid" => Some("33"),
        "low" | "low-level" | "level-low" => Some("11"),
        "rise" | "rising" | "high-rising" => Some("25"),
        "fall" | "falling" | "high-falling" => Some("51"),
        "dip" | "dipping" | "fall-rise" | "falling-rising" => Some("314"),
        _ => None,
    };
    if let Some(contour) = named {
        return Ok(Some(contour.bytes().map(|b| b - b'0').collect()));
    }
    let system = a
        .tone_system
        .as_deref()
        .or(a.lang.as_deref())
        .unwrap_or("mandarin")
        .to_ascii_lowercase();
    let contours: &[&str] = match system.as_str() {
        "mandarin" | "zh" | "zh-cn" | "zh-tw" | "cmn" => &["55", "35", "214", "51"],
        "yue" | "cantonese" | "yue-hk" | "zh-hk" => &["55", "35", "33", "21", "13", "22"],
        // Demonstration inventory, deliberately not attributed to a real language.
        "generic-8" => &["55", "35", "214", "51", "33", "22", "53", "24"],
        _ => {
            return Err(format!(
                "성조 체계 '{system}'의 기본값이 없습니다. toneContour를 명시하세요."
            ))
        }
    };
    let contour = value
        .parse::<usize>()
        .ok()
        .and_then(|n| n.checked_sub(1))
        .and_then(|i| contours.get(i))
        .ok_or_else(|| {
            format!("성조 '{value}'는 '{system}'에 없습니다. toneContour를 명시하세요.")
        })?;
    Ok(Some(contour.bytes().map(|b| b - b'0').collect()))
}

pub fn contour_level(profile: &[u8], progress: f32) -> f32 {
    if profile.is_empty() {
        return 3.0;
    }
    let p = progress.clamp(0.0, 1.0) * profile.len().saturating_sub(1) as f32;
    let left = p.floor() as usize;
    let right = (left + 1).min(profile.len() - 1);
    profile[left] as f32 + (profile[right] as f32 - profile[left] as f32) * (p - left as f32)
}
