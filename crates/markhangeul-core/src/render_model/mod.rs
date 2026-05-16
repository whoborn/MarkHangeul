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
        tone: attributes.tone.map(tone_class),
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
        Duration::Short => "mh-duration-short",
        Duration::Normal => "mh-duration-normal",
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

fn tone_class(value: Tone) -> &'static str {
    match value {
        Tone::One => "mh-tone-1",
        Tone::Two => "mh-tone-2",
        Tone::Three => "mh-tone-3",
        Tone::Four => "mh-tone-4",
        Tone::Neutral => "mh-tone-neutral",
    }
}
