use markhangeul_core::{
    Duration, MarkHangeulDocument, MarkHangeulNode, MarkHangeulToken, Pitch, Scope, Stress, Volume,
};
use pulldown_cmark::{html, CowStr, Event, Options, Parser};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_name = markHangeulTypesetMath)]
    fn mark_hangeul_typeset_math();
}

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
    let rendered_html = render_preview_html(&props.document, props.selected_id.as_deref());

    {
        let rendered_html = rendered_html.clone();
        use_effect_with(rendered_html, |_| {
            typeset_math();
            || ()
        });
    }

    let onclick = {
        let on_select = props.on_select.clone();
        Callback::from(move |event: MouseEvent| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(element) = target.dyn_into::<Element>() else {
                return;
            };
            let Ok(Some(mark_element)) = element.closest("[data-mh-id]") else {
                return;
            };
            if let Some(node_id) = mark_element.get_attribute("data-mh-id") {
                on_select.emit(node_id);
            }
        })
    };

    html! {
        <section class="panel preview-panel" aria-labelledby="preview-title">
            <div class="panel-header">
                <div class="panel-title">
                    <span class="panel-icon">{"↗"}</span>
                    <h2 id="preview-title">{"Render"}</h2>
                </div>
                <span class="counter">{mark_count}</span>
            </div>
            <div class="render-surface markdown-body" aria-label="MarkHangeul rendered output" {onclick}>
                {Html::from_html_unchecked(AttrValue::from(rendered_html))}
            </div>
        </section>
    }
}

fn render_preview_html(document: &MarkHangeulDocument, selected_id: Option<&str>) -> String {
    let markdown = render_markhangeul_markdown(document, selected_id);
    markdown_to_html(&markdown)
}

fn render_markhangeul_markdown(
    document: &MarkHangeulDocument,
    selected_id: Option<&str>,
) -> String {
    let mut markdown = String::new();

    for node in &document.nodes {
        match node {
            MarkHangeulToken::Text(text) => markdown.push_str(&text.text),
            MarkHangeulToken::Markhangeul(mark) => push_mark_html(&mut markdown, mark, selected_id),
        }
    }

    markdown
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_SUPERSCRIPT);
    options.insert(Options::ENABLE_SUBSCRIPT);

    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        Event::InlineMath(math) => Event::Html(CowStr::from(format!(
            r#"<span class="math math-inline">\({}\)</span>"#,
            escape_html(&math)
        ))),
        Event::DisplayMath(math) => Event::Html(CowStr::from(format!(
            r#"<div class="math math-display">\[{}\]</div>"#,
            escape_html(&math)
        ))),
        other => other,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn push_mark_html(output: &mut String, node: &MarkHangeulNode, selected_id: Option<&str>) {
    let selected = selected_id == Some(node.id.as_str());
    output.push_str("<span class=\"");
    output.push_str(&escape_attr(&mark_class(node)));
    output.push_str("\" data-mh-id=\"");
    output.push_str(&escape_attr(&node.id));
    output.push_str("\" data-selected=\"");
    output.push_str(if selected { "true" } else { "false" });
    output.push_str("\" data-scope=\"");
    output.push_str(scope_name(node.scope));
    if let Some(tone) = &node.attributes.tone {
        output.push_str("\" data-tone=\"");
        output.push_str(&escape_attr(tone.as_str()));
    }
    output.push_str("\" data-sound-shape=\"");
    output.push_str(if show_sound_shape(node) {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-guide-color=\"");
    output.push_str(if guide_color(node) { "true" } else { "false" });
    output.push_str("\" data-error=\"");
    output.push_str(if node.errors.is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str("\" role=\"button\" tabindex=\"0\" aria-label=\"");
    output.push_str(&escape_attr(&format!(
        "{} annotation {}",
        node.text, node.raw_annotation
    )));
    output.push_str("\">");

    if let Some(sound_shape_path) = sound_shape_path_html(node) {
        output.push_str(&sound_shape_path);
    }

    let chars: Vec<char> = node.text.chars().collect();
    let char_count = chars.len();

    for (index, ch) in chars.iter().enumerate() {
        if *ch == '\n' {
            output.push_str("<br>");
            continue;
        }

        output.push_str("<span class=\"mh-char\" style=\"--pitch-y: ");
        output.push_str(&pitch_offset(node, index, char_count).to_string());
        output.push_str("px;\">");
        push_escaped_html_char(output, *ch);
        output.push_str("</span>");
    }

    output.push_str("</span>");
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
                Duration::ExtraShort => "mh-duration-extra-short",
                Duration::Short => "mh-duration-short",
                Duration::SlightShort => "mh-duration-slight-short",
                Duration::Normal => "mh-duration-normal",
                Duration::SlightLong => "mh-duration-slight-long",
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

    if let Some(tone) = &node.attributes.tone {
        classes.push("mh-tone".to_string());
        if tone.as_str() == "neutral" {
            classes.push("mh-tone-neutral".to_string());
        } else {
            classes.push(format!("mh-tone-{}", class_safe_token(tone.as_str())));
        }
        if let Some(profile) = tone_profile(node) {
            classes.push(format!("mh-tone-shape-{}", tone_shape_name(&profile)));
        }
    }

    if show_sound_shape(node) {
        classes.push("mh-sound-shape-on".to_string());
    }

    if guide_color(node) {
        classes.push("mh-guide-color-on".to_string());
    }

    classes.join(" ")
}

fn sound_shape_path_html(node: &MarkHangeulNode) -> Option<String> {
    if !show_sound_shape(node) {
        return None;
    }

    let (path, guide_type) = if let Some(profile) = tone_profile(node) {
        (sound_shape_path_from_profile(&profile), "tone")
    } else if let Some(profile) = pitch_profile(node) {
        (sound_shape_path_from_profile(&profile), "pitch")
    } else {
        (
            duration_sound_shape_path(node.attributes.duration?)?,
            "duration",
        )
    };

    Some(format!(
        r#"<svg class="sound-shape-path sound-shape-{}" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path d="{}"></path></svg>"#,
        guide_type,
        escape_attr(&path)
    ))
}

fn pitch_offset(node: &MarkHangeulNode, index: usize, char_count: usize) -> f32 {
    let denominator = char_count.saturating_sub(1).max(1) as f32;
    let progress = index as f32 / denominator;

    if let Some(profile) = tone_profile(node) {
        return tone_offset_from_profile(&profile, progress);
    }

    match node.attributes.pitch {
        Some(Pitch::Low) => 5.0,
        Some(Pitch::Mid) => 0.0,
        Some(Pitch::High) => -6.0,
        Some(Pitch::Rise) => -6.0,
        Some(Pitch::Fall) => 6.0,
        None => 0.0,
    }
}

fn tone_profile(node: &MarkHangeulNode) -> Option<Vec<u8>> {
    let tone = node.attributes.tone.as_ref()?;

    if let Some(contour) = &node.attributes.tone_contour {
        if let Some(profile) = contour_profile(contour) {
            return Some(profile);
        }
    }

    let value = tone.as_str().to_ascii_lowercase();
    if value == "neutral" {
        return Some(vec![3, 3]);
    }

    named_tone_profile(&value).or_else(|| {
        let number = value.parse::<u8>().ok()?;
        numbered_tone_profile(number, node)
    })
}

fn pitch_profile(node: &MarkHangeulNode) -> Option<Vec<u8>> {
    match node.attributes.pitch? {
        Pitch::Low => Some(vec![2, 2]),
        Pitch::Mid => Some(vec![3, 3]),
        Pitch::High => Some(vec![5, 5]),
        Pitch::Rise => Some(vec![2, 5]),
        Pitch::Fall => Some(vec![5, 2]),
    }
}

fn contour_profile(contour: &str) -> Option<Vec<u8>> {
    let profile: Vec<u8> = contour
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .filter(|level| (1..=5).contains(level))
        .map(|level| level as u8)
        .collect();

    (profile.len() >= 2).then_some(profile)
}

fn numbered_tone_profile(number: u8, node: &MarkHangeulNode) -> Option<Vec<u8>> {
    let system = format!(
        "{} {}",
        node.attributes.lang.as_deref().unwrap_or_default(),
        node.attributes.tone_system.as_deref().unwrap_or_default()
    )
    .to_ascii_lowercase();

    if system.contains("yue")
        || system.contains("cantonese")
        || system.contains("hong-kong")
        || system.contains("hong kong")
        || system.contains("hk")
    {
        return cantonese_tone_profile(number);
    }

    generic_tone_profile(number)
}

fn cantonese_tone_profile(number: u8) -> Option<Vec<u8>> {
    match number {
        1 => Some(vec![5, 5]),
        2 => Some(vec![2, 5]),
        3 => Some(vec![3, 3]),
        4 => Some(vec![2, 1]),
        5 => Some(vec![2, 3]),
        6 => Some(vec![2, 2]),
        _ => generic_tone_profile(number),
    }
}

fn generic_tone_profile(number: u8) -> Option<Vec<u8>> {
    match number {
        1 => Some(vec![5, 5]),
        2 => Some(vec![3, 5]),
        3 => Some(vec![2, 1, 4]),
        4 => Some(vec![5, 1]),
        5 => Some(vec![3, 3]),
        6 => Some(vec![2, 2]),
        7 => Some(vec![5, 3]),
        8 => Some(vec![2, 4]),
        9 => Some(vec![1, 1]),
        _ => None,
    }
}

fn named_tone_profile(value: &str) -> Option<Vec<u8>> {
    match value {
        "high" | "high-level" | "level-high" => Some(vec![5, 5]),
        "mid" | "mid-level" | "level-mid" => Some(vec![3, 3]),
        "low" | "low-level" | "level-low" => Some(vec![1, 1]),
        "rise" | "rising" | "high-rising" => Some(vec![2, 5]),
        "fall" | "falling" | "high-falling" => Some(vec![5, 1]),
        "dip" | "dipping" | "fall-rise" | "falling-rising" => Some(vec![3, 1, 4]),
        "checked-high" => Some(vec![5, 3]),
        "checked-low" => Some(vec![2, 1]),
        _ => None,
    }
}

fn sound_shape_path_from_profile(profile: &[u8]) -> String {
    let points = profile_points(profile);
    let Some((x, y)) = points.first() else {
        return String::new();
    };

    let mut path = format!("M {x:.1} {y:.1}");
    if points.len() == 1 {
        return path;
    }

    for index in 0..points.len() - 1 {
        let previous = if index == 0 {
            points[index]
        } else {
            points[index - 1]
        };
        let current = points[index];
        let next = points[index + 1];
        let following = if index + 2 < points.len() {
            points[index + 2]
        } else {
            next
        };

        let control_a = (
            current.0 + (next.0 - previous.0) / 6.0,
            current.1 + (next.1 - previous.1) / 6.0,
        );
        let control_b = (
            next.0 - (following.0 - current.0) / 6.0,
            next.1 - (following.1 - current.1) / 6.0,
        );

        path.push_str(&format!(
            " C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
            control_a.0, control_a.1, control_b.0, control_b.1, next.0, next.1
        ));
    }

    path
}

fn duration_sound_shape_path(duration: Duration) -> Option<String> {
    let width = match duration {
        Duration::ExtraShort => 28.0,
        Duration::Short => 40.0,
        Duration::SlightShort => 52.0,
        Duration::Normal => 64.0,
        Duration::SlightLong => 76.0,
        Duration::Long => 88.0,
        Duration::ExtraLong => 96.0,
    };
    let start = (100.0 - width) / 2.0;
    let end = start + width;
    Some(format!("M {start:.1} 13.0 L {end:.1} 13.0"))
}

fn tone_offset_from_profile(profile: &[u8], progress: f32) -> f32 {
    if profile.is_empty() {
        return 0.0;
    }

    if profile.len() == 1 {
        return level_to_offset(profile[0]);
    }

    let scaled = progress.clamp(0.0, 1.0) * (profile.len() - 1) as f32;
    let left = scaled.floor() as usize;
    let right = (left + 1).min(profile.len() - 1);
    let local = scaled - left as f32;
    interpolate(
        level_to_offset(profile[left]),
        level_to_offset(profile[right]),
        local,
    )
}

fn profile_points(profile: &[u8]) -> Vec<(f32, f32)> {
    let denominator = profile.len().saturating_sub(1).max(1) as f32;
    profile
        .iter()
        .enumerate()
        .map(|(index, level)| {
            let x = 4.0 + (index as f32 / denominator) * 92.0;
            let y = level_to_svg_y(*level);
            (x, y)
        })
        .collect()
}

fn level_to_offset(level: u8) -> f32 {
    7.5 - level.clamp(1, 5) as f32 * 3.0
}

fn level_to_svg_y(level: u8) -> f32 {
    23.0 - level.clamp(1, 5) as f32 * 4.0
}

fn class_safe_token(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn tone_shape_name(profile: &[u8]) -> &'static str {
    if profile.is_empty() {
        return "level";
    }

    let first = profile[0];
    let last = *profile.last().expect("profile is not empty");
    let min = *profile.iter().min().expect("profile is not empty");
    let max = *profile.iter().max().expect("profile is not empty");

    if max.saturating_sub(min) <= 1 {
        return if max <= 2 {
            "low-level"
        } else if min >= 4 {
            "high-level"
        } else {
            "level"
        };
    }

    if profile.len() >= 3
        && profile[1..profile.len() - 1]
            .iter()
            .any(|level| *level < first && *level < last)
    {
        return "dip";
    }

    if last > first {
        "rise"
    } else if last < first {
        "fall"
    } else {
        "level"
    }
}

fn show_sound_shape(node: &MarkHangeulNode) -> bool {
    if let Some(show) = node.attributes.sound_shape {
        return show;
    }

    if node.attributes.tone.is_some() || node.attributes.guide_color == Some(true) {
        return true;
    }

    !is_symbol_only_annotation(&node.raw_annotation)
}

fn guide_color(node: &MarkHangeulNode) -> bool {
    node.attributes.guide_color.unwrap_or(false)
}

fn is_symbol_only_annotation(raw_annotation: &str) -> bool {
    raw_annotation
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .all(|part| !part.contains('='))
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

fn escape_html(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        push_escaped_html_char(&mut output, ch);
    }
    output
}

fn escape_attr(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        match ch {
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => push_escaped_html_char(&mut output, ch),
        }
    }
    output
}

fn push_escaped_html_char(output: &mut String, ch: char) {
    match ch {
        '&' => output.push_str("&amp;"),
        '<' => output.push_str("&lt;"),
        '>' => output.push_str("&gt;"),
        _ => output.push(ch),
    }
}

#[cfg(target_arch = "wasm32")]
fn typeset_math() {
    mark_hangeul_typeset_math();
}

#[cfg(not(target_arch = "wasm32"))]
fn typeset_math() {}

#[cfg(test)]
mod tests {
    use markhangeul_core::parse_markhangeul;

    use super::render_preview_html;

    #[test]
    fn renders_markdown_blocks_around_markhangeul() {
        let document = parse_markhangeul("# Title\n\n**Hello{!↗}**\n\n- item");
        let html = render_preview_html(&document, Some("mh-0"));

        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<strong>"));
        assert!(html.contains("class=\"mh-mark"));
        assert!(html.contains("<li>item</li>"));
    }

    #[test]
    fn renders_tables_and_math() {
        let document =
            parse_markhangeul("| a | b |\n| - | - |\n| $x^2$ | 녕{↗} |\n\n$$\\sum_i x_i$$");
        let html = render_preview_html(&document, None);

        assert!(html.contains("<table>"));
        assert!(html.contains("math math-inline"));
        assert!(html.contains("math math-display"));
        assert!(html.contains("class=\"mh-mark"));
    }

    #[test]
    fn shows_sound_shape_by_default_and_hides_when_requested() {
        let shown = parse_markhangeul("마{T2}");
        let hidden = parse_markhangeul("마{T2,soundShape=false}");

        assert!(render_preview_html(&shown, None).contains("sound-shape-path"));
        assert!(!render_preview_html(&hidden, None).contains("sound-shape-path"));
    }

    #[test]
    fn hides_sound_shape_for_plain_symbol_annotations_by_default() {
        let hidden = parse_markhangeul("마{↗}");
        let shown = parse_markhangeul("마{↗,soundShape=true}");

        assert!(!render_preview_html(&hidden, None).contains("sound-shape-path"));
        assert!(render_preview_html(&shown, None).contains("sound-shape-pitch"));
    }

    #[test]
    fn keeps_guide_color_opt_in_without_coloring_letters() {
        let default = parse_markhangeul("아{duration=long}");
        let colored_guide = parse_markhangeul("아{duration=long,guideColor=true}");

        assert!(!render_preview_html(&default, None).contains("mh-guide-color-on"));
        let html = render_preview_html(&colored_guide, None);
        assert!(html.contains("mh-guide-color-on"));
        assert!(html.contains("sound-shape-duration"));
        assert!(!html.contains("color: var(--"));
    }

    #[test]
    fn renders_tone_shape_with_default_sound_shape_line() {
        let document = parse_markhangeul("마{T2}");
        let html = render_preview_html(&document, None);

        assert!(html.contains("mh-tone-shape-rise"));
        assert!(html.contains("sound-shape-tone"));
    }

    #[test]
    fn renders_tone_sound_shape_as_curved_path() {
        let document = parse_markhangeul("마{T3}");
        let html = render_preview_html(&document, None);

        assert!(html.contains(" C "));
        assert!(!html.contains(" L "));
    }

    #[test]
    fn places_rise_above_and_fall_below() {
        let rise = render_preview_html(&parse_markhangeul("마{↗}"), None);
        let fall = render_preview_html(&parse_markhangeul("마{↘}"), None);

        assert!(rise.contains("--pitch-y: -6px"));
        assert!(fall.contains("--pitch-y: 6px"));
    }
}
