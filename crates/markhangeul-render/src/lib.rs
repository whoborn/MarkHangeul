use markhangeul_core::{
    Duration, MarkHangeulDocument, MarkHangeulNode, MarkHangeulToken, Pitch, Scope,
};
use pulldown_cmark::{html, CowStr, Event, Options, Parser};

pub fn render_preview_html(document: &MarkHangeulDocument, selected_id: Option<&str>) -> String {
    let mut markdown = String::new();
    let mut replacements = Vec::new();
    let mut prefix = "MHPLACEHOLDER".to_string();
    while document.source.contains(&prefix) {
        prefix.push('X');
    }
    for node in &document.nodes {
        match node {
            MarkHangeulToken::Text(text) => markdown.push_str(&text.text),
            MarkHangeulToken::Markhangeul(mark) => {
                let key = format!("{prefix}N{}END", replacements.len());
                let mut rendered = String::new();
                push_mark_html(&mut rendered, mark, selected_id);
                replacements.push((key.clone(), rendered));
                markdown.push_str(&key);
            }
        }
    }
    let mut events = Vec::new();
    for event in Parser::new_ext(&markdown, Options::all()) {
        match event {
            Event::Text(text) => {
                let mut rest = text.as_ref();
                while let Some((pos, key, rendered)) = replacements
                    .iter()
                    .filter_map(|(key, rendered)| rest.find(key).map(|pos| (pos, key, rendered)))
                    .min_by_key(|entry| entry.0)
                {
                    events.push(Event::Text(CowStr::from(rest[..pos].to_string())));
                    events.push(Event::InlineHtml(CowStr::from(rendered.clone())));
                    rest = &rest[pos + key.len()..];
                }
                events.push(Event::Text(CowStr::from(rest.to_string())));
            }
            Event::Html(raw) | Event::InlineHtml(raw) => events.push(Event::Text(raw)),
            Event::Start(pulldown_cmark::Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let dest_url = if safe_url(&dest_url) {
                    dest_url
                } else {
                    CowStr::from("")
                };
                events.push(Event::Start(pulldown_cmark::Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                }));
            }
            Event::Start(pulldown_cmark::Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let dest_url = if safe_url(&dest_url) {
                    dest_url
                } else {
                    CowStr::from("")
                };
                events.push(Event::Start(pulldown_cmark::Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }));
            }
            Event::InlineMath(math) => events.push(Event::InlineHtml(CowStr::from(format!(
                r#"<span class="math math-inline">\({}\)</span>"#,
                escape_html(&math)
            )))),
            Event::DisplayMath(math) => events.push(Event::Html(CowStr::from(format!(
                r#"<div class="math math-display">\[{}\]</div>"#,
                escape_html(&math)
            )))),
            other => events.push(other),
        }
    }
    let mut output = String::new();
    html::push_html(&mut output, events.into_iter());
    output
}

fn safe_url(url: &str) -> bool {
    let normalized: String = url
        .chars()
        .filter(|ch| !ch.is_control() && !ch.is_whitespace())
        .collect();
    let scheme = normalized.split(['/', '?', '#']).next().unwrap_or_default();
    !scheme.contains(':')
        || ["https:", "http:", "mailto:"]
            .iter()
            .any(|s| normalized.to_ascii_lowercase().starts_with(s))
}

pub fn export_html(document: &MarkHangeulDocument) -> String {
    format!("<!doctype html><html lang=\"ko\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>MarkHangeul</title><style>{} {} .markdown-body {{padding:2rem;line-height:1.85}} </style><body><main class=\"markdown-body\">{}</main></body></html>",
        include_str!("../../../assets/styles/markhangeul.css"),
        include_str!("../../../assets/styles/playground.css"), render_preview_html(document, None).replace("role=\"button\" tabindex=\"0\"", "role=\"img\""))
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
    if let Ok(resolved) = markhangeul_core::render_model::resolve_pronunciation(&node.attributes) {
        output.push_str("\" data-phonation=\"");
        output.push_str(resolved.phonation.as_str());
        output.push_str("\" data-checked=\"");
        output.push_str(if resolved.checked { "true" } else { "false" });
    }
    output.push_str("\" role=\"button\" tabindex=\"0\" aria-label=\"");
    let description = markhangeul_core::render_model::resolve_pronunciation(&node.attributes)
        .map(|r| {
            format!(
                "{}; {}; phonation={}; checked={}",
                node.raw_annotation,
                r.category.unwrap_or("사용자 표기"),
                r.phonation.as_str(),
                r.checked
            )
        })
        .unwrap_or_else(|_| node.raw_annotation.clone());
    output.push_str(&escape_attr(&format!(
        "{} annotation {}",
        node.text, description
    )));
    output.push_str("\">");

    if let Some(sound_shape_path) = sound_shape_path_html(node) {
        output.push_str(&sound_shape_path);
    }

    use unicode_segmentation::UnicodeSegmentation;
    let profile = tone_profile(node).or_else(|| pitch_profile(node));
    let graphemes: Vec<_> = node.text.graphemes(true).collect();
    for (index, grapheme) in graphemes.iter().copied().enumerate() {
        if grapheme == "\n" {
            output.push_str("<br>");
            continue;
        }
        // Continuous affine segments keep strokes connected. A rising/falling
        // syllable is painted once; a turning contour only splits at its knots.
        let width = glyph_advance(grapheme);
        let view_width = width * 100.0;
        output.push_str(&format!(r#"<svg class="mh-glyph" style="--glyph-width:{width}em" viewBox="0 -15 {view_width} 130" preserveAspectRatio="none" aria-hidden="true">"#));
        let glyph_id = format!("{}-glyph-{index}", node.id);
        output.push_str(&format!(
            r#"<defs><text id="{glyph_id}" style="font-size:100px" x="0" y="85">{}</text></defs>"#,
            escape_html(grapheme)
        ));
        let segments = glyph_segments(profile.as_deref(), index, graphemes.len());
        for (segment_index, (from, to, start_y, end_y)) in segments.iter().copied().enumerate() {
            let from = from * width;
            let to = to * width;
            let slope = (end_y - start_y) / (to - from);
            let intercept = start_y - slope * from;
            let transform = format!("matrix(1 {slope:.6} 0 1 0 {intercept:.6})");
            if segments.len() == 1 {
                output.push_str(&format!(
                    r##"<use href="#{glyph_id}" transform="{transform}"/>"##
                ));
            } else {
                let clip = format!("{}-g{index}-s{segment_index}", node.id);
                let width = to - from;
                output.push_str(&format!(r##"<defs><clipPath id="{clip}"><rect x="{from}" y="-15" width="{width}" height="130"/></clipPath></defs><g clip-path="url(#{clip})"><use href="#{glyph_id}" transform="{transform}"/></g>"##));
            }
        }
        output.push_str("</svg>");
    }

    output.push_str("</span>");
}

// Advance estimates affect spacing only: never stretch a narrow Latin glyph
// to a full Hangul cell. The actual font outlines retain their native proportions.
fn glyph_advance(grapheme: &str) -> f32 {
    match grapheme {
        " " => 0.32,
        "i" | "l" | "I" | "!" | "." | "," | ":" | ";" | "'" => 0.28,
        "j" | "t" | "f" | "r" | "(" | ")" => 0.4,
        "m" | "w" | "M" | "W" => 0.9,
        _ if grapheme.is_ascii() => 0.65,
        _ => 1.0,
    }
}

// Six SVG units per pitch step = 0.06em. This bounds the largest
// neighboring high/low jump to 0.24em without merging tone categories.
fn glyph_segments(profile: Option<&[u8]>, index: usize, count: usize) -> Vec<(f32, f32, f32, f32)> {
    let Some(profile) = profile.filter(|p| p.len() >= 2) else {
        return vec![(0.0, 100.0, 0.0, 0.0)];
    };
    let count = count.max(1) as f32;
    let mut stops = vec![0.0];
    for knot in 1..profile.len() - 1 {
        let local = (knot as f32 / (profile.len() - 1) as f32 * count - index as f32) * 100.0;
        if local > 0.001 && local < 99.999 {
            stops.push(local);
        }
    }
    stops.push(100.0);
    let offset = |x: f32| {
        (3.0 - markhangeul_core::render_model::contour_level(
            profile,
            (index as f32 + x / 100.0) / count,
        )) * 6.0
    };
    stops
        .windows(2)
        .map(|pair| (pair[0], pair[1], offset(pair[0]), offset(pair[1])))
        .collect()
}

fn mark_class(node: &MarkHangeulNode) -> String {
    let mut classes = vec!["mh-mark".to_string()];

    let mapped = markhangeul_core::render_model::render_classes(&node.attributes);
    classes.extend(
        [mapped.pitch, mapped.duration, mapped.stress, mapped.volume]
            .into_iter()
            .flatten()
            .map(str::to_string),
    );

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

    let resolved = markhangeul_core::render_model::resolve_pronunciation(&node.attributes).ok();
    let (path, guide_type) = if let Some(profile) = tone_profile(node) {
        (sound_shape_path_from_profile(&profile), "tone")
    } else if let Some(profile) = pitch_profile(node) {
        (sound_shape_path_from_profile(&profile), "pitch")
    } else if let Some(duration) = node.attributes.duration {
        (duration_sound_shape_path(duration)?, "duration")
    } else if node.attributes.phonation.is_some() || node.attributes.checked == Some(true) {
        ("M 4 13 L 96 13".to_string(), "phonation")
    } else {
        return None;
    };
    let mut detail = String::new();
    if let Some(r) = resolved {
        if r.phonation == markhangeul_core::Phonation::Glottalized {
            detail.push_str(r#"<path class="phonation-stop" d="M 47 5 L 47 21 M 53 5 L 53 21"/>"#);
        }
        if r.checked {
            detail.push_str(r#"<path class="checked-stop" d="M 96 3 L 96 22"/>"#);
        }
    }
    Some(format!(
        r#"<svg class="sound-shape-path sound-shape-{}" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path class="contour-line" pathLength="100" d="{}"></path>{}</svg>"#,
        guide_type,
        escape_attr(&path),
        detail
    ))
}

fn tone_profile(node: &MarkHangeulNode) -> Option<Vec<u8>> {
    markhangeul_core::render_model::resolve_tone(&node.attributes)
        .ok()
        .flatten()
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

fn sound_shape_path_from_profile(profile: &[u8]) -> String {
    profile_points(profile)
        .iter()
        .enumerate()
        .map(|(i, (x, y))| format!("{} {x:.1} {y:.1}", if i == 0 { "M" } else { "L" }))
        .collect::<Vec<_>>()
        .join(" ")
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

    if max == min {
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

    if node.attributes.tone.is_some()
        || node.attributes.tone_contour.is_some()
        || node.attributes.phonation.is_some()
        || node.attributes.checked == Some(true)
        || node.attributes.guide_color == Some(true)
    {
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
    fn renders_tone_sound_shape_without_interpolation_overshoot() {
        let document = parse_markhangeul("마{T3}");
        let html = render_preview_html(&document, None);

        assert!(html.contains(" L "));
        assert!(!html.contains(" C "));
    }

    #[test]
    fn places_rise_above_and_fall_below() {
        let rise = render_preview_html(&parse_markhangeul("마{↗}"), None);
        let fall = render_preview_html(&parse_markhangeul("마{↘}"), None);

        assert!(rise.contains("matrix(1"));
        assert_ne!(rise, fall);
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    use markhangeul_core::parse_markhangeul;

    #[test]
    fn user_html_is_inert_and_unsafe_links_are_removed() {
        let d = parse_markhangeul("<script>alert(1)</script>\n\n<img src=x onerror=alert(1)>\n\n[click](javascript:alert%281%29)\n\n마{T2,note=\"<svg/onload=alert(1)>\"}");
        let rendered = render_preview_html(&d, None);
        assert!(!rendered.contains("<script>"));
        assert!(!rendered.contains("<img src=x"));
        assert!(!rendered.contains("href=\"javascript:"));
        assert!(!rendered.contains("<svg/onload"));
        assert!(rendered.contains("&lt;script&gt;"));
    }

    #[test]
    fn contour_alone_and_hidden_guide_still_warp_single_grapheme() {
        let html = render_preview_html(
            &parse_markhangeul("마{toneContour=214,soundShape=false}"),
            None,
        );
        assert_eq!(html.matches("class=\"mh-glyph\"").count(), 1);
        assert_eq!(html.matches("<use ").count(), 2);
        assert!(!html.contains("sound-shape-path"));
        assert!(html.contains("마"));
    }

    #[test]
    fn four_six_eight_have_distinct_letter_geometry_without_guides() {
        for (system, count) in [("mandarin", 4), ("yue", 6), ("generic-8", 8)] {
            let mut signatures = std::collections::HashSet::new();
            for tone in 1..=count {
                let html = render_preview_html(
                    &parse_markhangeul(&format!(
                        "마{{toneSystem={system},tone={tone},soundShape=false}}"
                    )),
                    None,
                );
                let transforms: Vec<_> = html
                    .split("transform=\"")
                    .skip(1)
                    .map(|s| s.split('"').next().unwrap())
                    .collect();
                assert!(signatures.insert(transforms.join(";")), "{system} {tone}");
            }
        }
    }

    #[test]
    fn markdown_labels_code_tables_and_exports_survive() {
        let d = parse_markhangeul(
            "_hello{T2}_ [마{T1}](https://example.com)\n\n`마{T2}`\n\n| 음 |\n| - |\n| 마{T3} |",
        );
        let rendered = render_preview_html(&d, None);
        assert!(rendered.contains("<em><span"));
        assert!(rendered.contains("<a href=\"https://example.com\"><span"));
        assert!(rendered.contains("<code>마{T2}</code>"));
        assert!(rendered.contains("<table>"));
        let export = export_html(&d);
        assert!(export.starts_with("<!doctype html>"));
        assert!(export.contains(".mh-glyph"));
        assert!(!export.contains("<script"));
    }
}

#[cfg(test)]
mod typography_tests {
    use super::*;

    #[test]
    fn contour_segments_join_without_steps_and_stay_near_baseline() {
        for profile in [vec![5, 1], vec![2, 1, 4], vec![1, 5, 1, 5]] {
            let segments = glyph_segments(Some(&profile), 0, 1);
            for pair in segments.windows(2) {
                assert_eq!(pair[0].1, pair[1].0);
                assert_eq!(pair[0].3, pair[1].2);
            }
            for (_, _, start, end) in segments {
                assert!(start.abs() <= 12.0 && end.abs() <= 12.0);
            }
            let left = glyph_segments(Some(&profile), 0, 2);
            let right = glyph_segments(Some(&profile), 1, 2);
            assert_eq!(left.last().unwrap().3, right.first().unwrap().2);
        }
    }

    #[test]
    fn straight_contours_paint_once_without_slice_edges_or_forced_width() {
        let html =
            render_preview_html(&markhangeul_core::parse_markhangeul("마{T2}Hello{↗}"), None);
        assert_eq!(html.matches("<use ").count(), 6);
        assert!(!html.contains("clipPath"));
        assert!(!html.contains("textLength"));
        assert!(glyph_advance("l") < glyph_advance("H"));
    }
}

#[cfg(test)]
mod language_render_tests {
    use super::*;
    use markhangeul_core::parse_markhangeul;
    #[test]
    fn hanoi_phonation_and_checked_have_guides_without_distorting_glyphs() {
        let render = |s: &str| render_preview_html(&parse_markhangeul(s), None);
        let a = render("마{toneSystem=vi-hanoi,tone=b1}");
        let b = render("마{toneSystem=vi-hanoi,tone=c2}");
        assert!(!a.contains("class=\"phonation-stop\""));
        assert!(b.contains("class=\"phonation-stop\""));
        assert!(b.contains("data-phonation=\"glottalized\""));
        let checked = render("맛{toneSystem=vi-hanoi-8,tone=d1}");
        assert!(checked.contains("class=\"checked-stop\""));
        assert!(!checked.contains("mh-duration-short"));
        let hidden = render("맛{toneSystem=vi-hanoi-8,tone=d1,soundShape=false}");
        assert!(!hidden.contains("class=\"checked-stop\""));
        assert!(hidden.contains("class=\"mh-glyph\""));
        let only = render("마{phonation=breathy}");
        assert!(only.contains("sound-shape-phonation"));
        assert!(only.contains("data-phonation=\"breathy\""));
    }
}
