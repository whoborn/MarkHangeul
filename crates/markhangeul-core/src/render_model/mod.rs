pub mod tone_systems;

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

/// A resolved view: source attributes stay untouched for lossless editing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPronunciation {
    pub contour: Option<Vec<u8>>,
    pub phonation: crate::ast::Phonation,
    pub checked: bool,
    pub system: Option<&'static str>,
    pub category: Option<&'static str>,
}

pub fn resolve_pronunciation(a: &MarkHangeulAttributes) -> Result<ResolvedPronunciation, String> {
    use tone_systems::{tone_preset, tone_system};
    let system_id = a
        .tone_system
        .as_deref()
        .or(a.lang.as_deref())
        .unwrap_or("mandarin");
    let system = tone_system(system_id);
    let value = a.tone.as_ref().map(|t| t.as_str());
    let preset = system.and_then(|s| value.and_then(|v| tone_preset(s, v)));
    if let (Some(s), Some(p), Some(checked)) = (system, preset, a.checked) {
        if s.id == "yue" && checked && !matches!(p.key, "1" | "3" | "6") {
            return Err("Jyutping 입성은 1·3·6 범주를 사용합니다. 이 번호에는 checked=true를 지정할 수 없습니다.".into());
        }
        if s.id.starts_with("vietnamese-hanoi") && checked != p.checked {
            return Err(
                "하노이 입성 D1·D2와 개음절/공명음 종결 A·B·C 범주의 checked 값이 충돌합니다."
                    .into(),
            );
        }
    }
    let contour = if let Some(contour) = &a.tone_contour {
        if !(2..=16).contains(&contour.len())
            || !contour.bytes().all(|b| (b'1'..=b'5').contains(&b))
        {
            return Err("toneContour는 1~5 숫자 2~16개여야 합니다.".into());
        }
        Some(contour.as_str())
    } else if let Some(preset) = preset {
        Some(preset.contour)
    } else if let Some(value) = value {
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
        Some(named.ok_or_else(|| format!("성조 '{value}'를 '{system_id}'에서 해석할 수 없습니다. 지원되는 범주 이름 또는 toneContour를 지정하세요. 베트남어는 지역 체계와 A1~D2 코드를 사용합니다."))?)
    } else {
        None
    };
    Ok(ResolvedPronunciation {
        contour: contour.map(|c| c.bytes().map(|b| b - b'0').collect()),
        phonation: a
            .phonation
            .or(preset.map(|p| p.phonation))
            .unwrap_or_default(),
        checked: a.checked.or(preset.map(|p| p.checked)).unwrap_or(false),
        system: system
            .filter(|_| a.tone.is_some() || a.tone_system.is_some() || a.lang.is_some())
            .map(|s| s.id),
        category: preset.map(|p| p.label),
    })
}

pub fn resolve_tone(a: &MarkHangeulAttributes) -> Result<Option<Vec<u8>>, String> {
    resolve_pronunciation(a).map(|r| r.contour)
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
