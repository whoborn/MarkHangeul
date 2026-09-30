use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportKind {
    Source,
    Plain,
    Html,
    Json,
}

#[derive(Properties, PartialEq)]
pub struct ExportPanelProps {
    pub active: ExportKind,
    pub source: String,
    pub plain_markdown: String,
    pub html_document: String,
    pub json_ast: String,
    pub on_change: Callback<ExportKind>,
}

#[function_component(ExportPanel)]
pub fn export_panel(props: &ExportPanelProps) -> Html {
    let (value, filename, mime) = match props.active {
        ExportKind::Source => (&props.source, "markhangeul.mh.md", "text/markdown"),
        ExportKind::Plain => (&props.plain_markdown, "plain.md", "text/markdown"),
        ExportKind::Html => (&props.html_document, "markhangeul.html", "text/html"),
        ExportKind::Json => (&props.json_ast, "markhangeul.json", "application/json"),
    };
    let encoded: String = value
        .as_bytes()
        .iter()
        .map(|b| format!("%{b:02X}"))
        .collect();
    html! {
        <section class="panel export-panel" aria-labelledby="export-title">
            <div class="panel-header">
                <h2 id="export-title">{"내보내기"}</h2>
                <div class="segmented-control" aria-label="내보내기 형식">
                    {for [(ExportKind::Source, "원문"), (ExportKind::Plain, "일반 MD"), (ExportKind::Html, "HTML"), (ExportKind::Json, "JSON")].into_iter().map(|(kind, label)| {
                        let cb = props.on_change.clone();
                        html! { <button type="button" aria-pressed={(props.active == kind).to_string()} onclick={Callback::from(move |_| cb.emit(kind.clone()))}>{label}</button> }
                    })}
                </div>
                <a download={filename} href={format!("data:{mime};charset=utf-8,{encoded}")}>{"파일 저장"}</a>
            </div>
            <p class="export-help">{match props.active {
                ExportKind::Source => "발음 정보를 보존합니다. 다시 편집기에 붙여 넣어 사용할 수 있습니다.",
                ExportKind::Plain => "발음 정보를 제거합니다. 일반 Markdown 편집기에서 사용할 수 있습니다.",
                ExportKind::Html => "글자 모양과 CSS를 포함합니다. 브라우저에서 열거나 정적 웹에 올리세요. 수식은 TeX 원문으로 보존합니다.",
                ExportKind::Json => "발음 속성과 UTF-8 바이트 위치를 보존합니다.",
            }}</p>
            <textarea class="source-editor export-code" aria-label="내보내기 내용" readonly=true value={value.clone()} />
        </section>
    }
}
