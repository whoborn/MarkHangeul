use markhangeul_core::{
    Duration, MarkHangeulDocument, MarkHangeulNode, MarkHangeulToken, Pitch, Scope, Stress, Tone,
    Volume,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PreviewPanelProps {
    pub document: MarkHangeulDocument,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
}

#[function_component(PreviewPanel)]
pub fn preview_panel(props: &PreviewPanelProps) -> Html {
    let mark_count = props
        .document
        .nodes
        .iter()
        .filter(|node| matches!(node, MarkHangeulToken::Markhangeul(_)))
        .count();

    html! {
        <section class="panel preview-panel" aria-labelledby="preview-title">
            <div class="panel-header">
                <div class="panel-title">
                    <span class="panel-icon">{"↗"}</span>
                    <h2 id="preview-title">{"Render"}</h2>
                </div>
                <span class="counter">{mark_count}</span>
            </div>
            <div class="render-surface" aria-label="MarkHangeul rendered output">
                {for props.document.nodes.iter().map(|node| render_node(node, &props.selected_id, &props.on_select))}
            </div>
        </section>
    }
}

fn render_node(
    node: &MarkHangeulToken,
    selected_id: &Option<String>,
    on_select: &Callback<String>,
) -> Html {
    match node {
        MarkHangeulToken::Text(text) => html! {<>{text.text.clone()}</>},
        MarkHangeulToken::Markhangeul(mark) => render_mark_node(mark, selected_id, on_select),
    }
}

fn render_mark_node(
    node: &MarkHangeulNode,
    selected_id: &Option<String>,
    on_select: &Callback<String>,
) -> Html {
    let selected = selected_id.as_deref() == Some(node.id.as_str());
    let class = mark_class(node);
    let data_selected = selected.to_string();
    let data_error = (!node.errors.is_empty()).to_string();
    let data_scope = scope_name(node.scope);
    let node_id = node.id.clone();
    let onclick = {
        let on_select = on_select.clone();
        Callback::from(move |_| on_select.emit(node_id.clone()))
    };
    let chars: Vec<char> = node.text.chars().collect();
    let char_count = chars.len();

    html! {
        <span
            class={class}
            data-selected={data_selected}
            data-scope={data_scope}
            data-error={data_error}
            role="button"
            tabindex="0"
            aria-label={format!("{} annotation {}", node.text, node.raw_annotation)}
            {onclick}
        >
            {tone_path(node.attributes.tone)}
            {for chars.into_iter().enumerate().map(|(index, ch)| {
                if ch == '\n' {
                    html! {<br />}
                } else {
                    html! {
                        <span class="mh-char" style={format!("--pitch-y: {}px;", pitch_offset(node, index, char_count))}>
                            {ch}
                        </span>
                    }
                }
            })}
        </span>
    }
}

fn mark_class(node: &MarkHangeulNode) -> String {
    let mut classes = vec!["mh-mark".to_string()];

    if let Some(pitch) = node.attributes.pitch {
        classes.push(
            match pitch {
                Pitch::Low => "mh-pitch-low",
                Pitch::Mid => "mh-pitch-mid",
                Pitch::High => "mh-pitch-high",
                Pitch::Rise => "mh-pitch-rise",
                Pitch::Fall => "mh-pitch-fall",
            }
            .to_string(),
        );
    }

    if let Some(duration) = node.attributes.duration {
        classes.push(
            match duration {
                Duration::Short => "mh-duration-short",
                Duration::Normal => "mh-duration-normal",
                Duration::Long => "mh-duration-long",
                Duration::ExtraLong => "mh-duration-extra-long",
            }
            .to_string(),
        );
    }

    if let Some(stress) = node.attributes.stress {
        classes.push(
            match stress {
                Stress::Weak => "mh-stress-weak",
                Stress::Normal => "mh-stress-normal",
                Stress::Strong => "mh-stress-strong",
                Stress::ExtraStrong => "mh-stress-extra-strong",
            }
            .to_string(),
        );
    }

    if let Some(volume) = node.attributes.volume {
        classes.push(
            match volume {
                Volume::Soft => "mh-volume-soft",
                Volume::Normal => "mh-volume-normal",
                Volume::Loud => "mh-volume-loud",
            }
            .to_string(),
        );
    }

    if let Some(tone) = node.attributes.tone {
        classes.push(
            match tone {
                Tone::One => "mh-tone-1",
                Tone::Two => "mh-tone-2",
                Tone::Three => "mh-tone-3",
                Tone::Four => "mh-tone-4",
                Tone::Neutral => "mh-tone-neutral",
            }
            .to_string(),
        );
    }

    classes.join(" ")
}

fn tone_path(tone: Option<Tone>) -> Html {
    let Some(tone) = tone else {
        return html! {};
    };

    let path = match tone {
        Tone::One => "M 4 6 L 96 6",
        Tone::Two => "M 4 18 C 32 16 58 10 96 4",
        Tone::Three => "M 4 8 C 24 20 58 20 96 5",
        Tone::Four => "M 4 4 C 32 7 64 14 96 20",
        Tone::Neutral => "M 8 13 L 92 13",
    };

    html! {
        <svg class="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true">
            <path d={path} />
        </svg>
    }
}

fn pitch_offset(node: &MarkHangeulNode, index: usize, char_count: usize) -> f32 {
    let denominator = char_count.saturating_sub(1).max(1) as f32;
    let progress = index as f32 / denominator;

    if let Some(tone) = node.attributes.tone {
        return match tone {
            Tone::One => -5.0,
            Tone::Two => interpolate(5.0, -6.0, progress),
            Tone::Three => {
                if progress < 0.5 {
                    interpolate(0.0, 7.0, progress * 2.0)
                } else {
                    interpolate(7.0, -4.0, (progress - 0.5) * 2.0)
                }
            }
            Tone::Four => interpolate(-6.0, 6.0, progress),
            Tone::Neutral => 1.0,
        };
    }

    match node.attributes.pitch {
        Some(Pitch::Low) => 5.0,
        Some(Pitch::Mid) => 0.0,
        Some(Pitch::High) => -6.0,
        Some(Pitch::Rise) => interpolate(4.0, -6.0, progress),
        Some(Pitch::Fall) => interpolate(-5.0, 5.0, progress),
        None => 0.0,
    }
}

fn interpolate(from: f32, to: f32, progress: f32) -> f32 {
    from + (to - from) * progress
}

fn scope_name(scope: Scope) -> &'static str {
    match scope {
        Scope::Grapheme => "grapheme",
        Scope::Word => "word",
        Scope::Range => "range",
    }
}
