use markhangeul_core::{
    Duration, MarkHangeulDocument, MarkHangeulNode, MarkHangeulToken, Pitch, Scope, Stress, Tone,
    Volume,
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

    if let Some(tone) = node.attributes.tone {
        output.push_str(tone_path_html(tone));
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

fn tone_path_html(tone: Tone) -> &'static str {
    match tone {
        Tone::One => {
            r#"<svg class="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path d="M 4 6 L 96 6"></path></svg>"#
        }
        Tone::Two => {
            r#"<svg class="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path d="M 4 18 C 32 16 58 10 96 4"></path></svg>"#
        }
        Tone::Three => {
            r#"<svg class="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path d="M 4 8 C 24 20 58 20 96 5"></path></svg>"#
        }
        Tone::Four => {
            r#"<svg class="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path d="M 4 4 C 32 7 64 14 96 20"></path></svg>"#
        }
        Tone::Neutral => {
            r#"<svg class="tone-path" viewBox="0 0 100 24" preserveAspectRatio="none" aria-hidden="true"><path d="M 8 13 L 92 13"></path></svg>"#
        }
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
}
