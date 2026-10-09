use markhangeul_core::{MarkHangeulDocument, MarkHangeulToken};
pub(crate) use markhangeul_render::{export_html, render_preview_html};
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
    let font_size = use_state(|| 20u32);
    let on_size_change = {
        let font_size = font_size.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(size) = input.value().parse::<u32>() {
                font_size.set(size.clamp(16, 36));
            }
        })
    };
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

    let onkeydown = {
        let on_select = props.on_select.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() != "Enter" && event.key() != " " {
                return;
            }
            if let Some(element) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                if let Ok(Some(mark)) = element.closest("[data-mh-id]") {
                    if let Some(id) = mark.get_attribute("data-mh-id") {
                        event.prevent_default();
                        on_select.emit(id);
                    }
                }
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
                <label class="preview-size">{"글자 크기 "}
                    <input type="range" min="16" max="36" value={font_size.to_string()} oninput={on_size_change} aria-label="미리보기 글자 크기" />
                    {format!("{}px", *font_size)}
                </label>
                <span class="counter">{mark_count}</span>
            </div>
            <div class="render-surface markdown-body" style={format!("font-size:{}px", *font_size)} aria-label="MarkHangeul rendered output" {onclick} {onkeydown}>
                {Html::from_html_unchecked(AttrValue::from(rendered_html))}
            </div>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
fn typeset_math() {
    mark_hangeul_typeset_math();
}

#[cfg(not(target_arch = "wasm32"))]
fn typeset_math() {}
