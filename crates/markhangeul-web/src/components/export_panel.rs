use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportKind {
    Plain,
    Json,
}

#[derive(Properties, PartialEq)]
pub struct ExportPanelProps {
    pub active: ExportKind,
    pub plain_markdown: String,
    pub json_ast: String,
    pub on_change: Callback<ExportKind>,
}

#[function_component(ExportPanel)]
pub fn export_panel(props: &ExportPanelProps) -> Html {
    let export_value = match props.active {
        ExportKind::Plain => props.plain_markdown.clone(),
        ExportKind::Json => props.json_ast.clone(),
    };

    let on_plain = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(ExportKind::Plain))
    };
    let on_json = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(ExportKind::Json))
    };

    html! {
        <section class="panel export-panel" aria-labelledby="export-title">
            <div class="panel-header">
                <div class="panel-title">
                    <span class="panel-icon">{"⇩"}</span>
                    <h2 id="export-title">{"Export"}</h2>
                </div>
                <div class="segmented-control" role="tablist" aria-label="export format">
                    <button
                        aria-selected={(props.active == ExportKind::Plain).to_string()}
                        role="tab"
                        type="button"
                        onclick={on_plain}
                    >
                        {"Plain"}
                    </button>
                    <button
                        aria-selected={(props.active == ExportKind::Json).to_string()}
                        role="tab"
                        type="button"
                        onclick={on_json}
                    >
                        {"JSON"}
                    </button>
                </div>
            </div>
            <pre class="code-block export-code">{export_value}</pre>
        </section>
    }
}
