use markhangeul_core::{MarkHangeulNode, Scope};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TokenInspectorProps {
    pub nodes: Vec<MarkHangeulNode>,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
}

#[function_component(TokenInspector)]
pub fn token_inspector(props: &TokenInspectorProps) -> Html {
    let selected_node = props
        .selected_id
        .as_deref()
        .and_then(|id| props.nodes.iter().find(|node| node.id == id))
        .or_else(|| props.nodes.first());

    html! {
        <section class="panel inspector-panel" aria-labelledby="inspector-title">
            <div class="panel-header">
                <div class="panel-title">
                    <span class="panel-icon">{"AST"}</span>
                    <h2 id="inspector-title">{"Inspector"}</h2>
                </div>
                <span class="counter">{props.nodes.len()}</span>
            </div>
            <div class="inspector-grid">
                <div class="node-list" aria-label="annotations">
                    if props.nodes.is_empty() {
                        <div class="empty-state">{"No annotations"}</div>
                    }
                    {for props.nodes.iter().map(|node| {
                        let node_id = node.id.clone();
                        let selected = props.selected_id.as_deref() == Some(node.id.as_str());
                        let on_select = props.on_select.clone();
                        html! {
                            <button
                                class="node-row"
                                data-selected={selected.to_string()}
                                type="button"
                                onclick={Callback::from(move |_| on_select.emit(node_id.clone()))}
                            >
                                <span class="node-target">{node.text.clone()}</span>
                                <span class="node-meta">{scope_label(node.scope)}</span>
                                if !node.errors.is_empty() {
                                    <span class="node-error">{"!"}</span>
                                }
                            </button>
                        }
                    })}
                </div>
                <div class="attribute-view">
                    {selected_node.map(render_details).unwrap_or_else(|| html! {<div class="empty-state">{"No selection"}</div>})}
                </div>
            </div>
        </section>
    }
}

fn render_details(node: &MarkHangeulNode) -> Html {
    let json = serde_json::to_string_pretty(&node.attributes)
        .unwrap_or_else(|error| format!("{{\"error\":\"{error}\"}}"));

    let resolved = markhangeul_core::render_model::resolve_pronunciation(&node.attributes).ok();
    html! {
        <>
            <dl class="detail-list">
                <div>
                    <dt>{"target"}</dt>
                    <dd>{node.text.clone()}</dd>
                </div>
                <div>
                    <dt>{"scope"}</dt>
                    <dd>{scope_name(node.scope)}</dd>
                </div>
                <div>
                    <dt>{"raw"}</dt>
                    <dd>{node.raw_annotation.clone()}</dd>
                </div>
            </dl>
            if let Some(r) = resolved {
                <div class="pronunciation-summary">
                    <p>{format!("해석: {} · {}", r.system.unwrap_or("사용자 체계"), r.category.unwrap_or("직접 지정"))}</p>
                    <p>{format!("성조 음높이: {} · 발성: {} · 입성: {}", r.contour.map(|c| c.iter().map(u8::to_string).collect::<String>()).unwrap_or_else(|| "없음".into()), r.phonation.as_str(), if r.checked { "예" } else { "아니오" })}</p>
                    if r.phonation != markhangeul_core::Phonation::Modal || r.checked {
                        <p>{"발성·입성 차이는 보조표시로 읽습니다. 보조선을 숨기면 일부 범주가 같은 글자 모양으로 보일 수 있습니다."}</p>
                    }
                </div>
            }
            <pre class="code-block">{json}</pre>
        </>
    }
}

fn scope_label(scope: Scope) -> &'static str {
    match scope {
        Scope::Grapheme => "글자",
        Scope::Word => "단어",
        Scope::Range => "범위",
    }
}

fn scope_name(scope: Scope) -> &'static str {
    match scope {
        Scope::Grapheme => "grapheme",
        Scope::Word => "word",
        Scope::Range => "range",
    }
}
