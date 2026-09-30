use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::TargetCast;

#[derive(Properties, PartialEq)]
pub struct EditorPanelProps {
    pub source: String,
    pub on_change: Callback<String>,
}

#[function_component(EditorPanel)]
pub fn editor_panel(props: &EditorPanelProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<HtmlTextAreaElement>();
            on_change.emit(input.value());
        })
    };

    html! {
        <section class="panel editor-panel" aria-labelledby="source-title">
            <div class="panel-header">
                <div class="panel-title">
                    <span class="panel-icon">{"{}"}</span>
                    <h2 id="source-title">{"Source"}</h2>
                </div>
                <span class="counter">{props.source.chars().count()}</span>
            </div>
            <textarea
                class="source-editor"
                aria-label="마크한글 원문 편집기"
                spellcheck="false"
                value={props.source.clone()}
                {oninput}
            />
        </section>
    }
}
