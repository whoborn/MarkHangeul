use markhangeul_core::{
    export_json_ast, export_plain_markdown, parse_markhangeul, MarkHangeulDocument,
    MarkHangeulNode, MarkHangeulToken,
};
use yew::prelude::*;

use crate::components::{
    EditorPanel, ErrorPanel, ExportKind, ExportPanel, PreviewPanel, SampleSelector, TokenInspector,
};

#[function_component(App)]
pub fn app() -> Html {
    let samples = sample_documents();
    let source = use_state(|| samples[0].source.clone());
    let selected_id = use_state(|| Some("mh-0".to_string()));
    let export_kind = use_state(|| ExportKind::Plain);

    let document = parse_markhangeul(&source);
    let annotation_nodes = collect_mark_nodes(&document);
    let plain_markdown = export_plain_markdown(&document);
    let json_ast = export_json_ast(&document);

    let on_source_change = {
        let source = source.clone();
        let selected_id = selected_id.clone();
        Callback::from(move |next_source: String| {
            let next_document = parse_markhangeul(&next_source);
            let first_id = collect_mark_nodes(&next_document)
                .first()
                .map(|node| node.id.clone());
            selected_id.set(first_id);
            source.set(next_source);
        })
    };

    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |node_id: String| selected_id.set(Some(node_id)))
    };

    let on_sample_select = {
        let source = source.clone();
        let selected_id = selected_id.clone();
        Callback::from(move |next_source: String| {
            let next_document = parse_markhangeul(&next_source);
            let first_id = collect_mark_nodes(&next_document)
                .first()
                .map(|node| node.id.clone());
            selected_id.set(first_id);
            source.set(next_source);
        })
    };

    let on_export_change = {
        let export_kind = export_kind.clone();
        Callback::from(move |kind: ExportKind| export_kind.set(kind))
    };

    html! {
        <div class="app-shell">
            <header class="top-bar">
                <div class="brand-block">
                    <div class="brand-mark">{"ㅎ"}</div>
                    <div>
                        <h1>{"MarkHangeul"}</h1>
                        <p>{"마크한글 Rust/WASM"}</p>
                    </div>
                </div>
                <SampleSelector samples={samples} on_select={on_sample_select} />
            </header>

            <section class="workspace-grid">
                <EditorPanel source={(*source).clone()} on_change={on_source_change} />
                <PreviewPanel
                    document={document.clone()}
                    selected_id={(*selected_id).clone()}
                    on_select={on_select.clone()}
                />
            </section>

            <section class="analysis-grid">
                <TokenInspector
                    nodes={annotation_nodes.clone()}
                    selected_id={(*selected_id).clone()}
                    on_select={on_select}
                />
                <ErrorPanel errors={document.errors.clone()} />
                <ExportPanel
                    active={(*export_kind).clone()}
                    plain_markdown={plain_markdown}
                    json_ast={json_ast}
                    on_change={on_export_change}
                />
            </section>
        </div>
    }
}

fn collect_mark_nodes(document: &MarkHangeulDocument) -> Vec<MarkHangeulNode> {
    document
        .nodes
        .iter()
        .filter_map(|node| match node {
            MarkHangeulToken::Markhangeul(mark) => Some(mark.clone()),
            MarkHangeulToken::Text(_) => None,
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleDocument {
    pub id: &'static str,
    pub label: &'static str,
    pub source: String,
}

fn sample_documents() -> Vec<SampleDocument> {
    vec![
        SampleDocument {
            id: "mixed",
            label: "기본",
            source: "# MarkHangeul 샘플\n\n**오늘은 날씨가 좋네요{↘}.**\n\n안녕{↗—!}하세요.\nHello{!↗} world{—}.\n妈{T1} 麻{T2} 马{T3} 骂{T4}\n\n- Markdown 목록 안의 Hello{!↗}\n- 수식: $E = mc^2$\n\n| 원문 | 표기 |\n| --- | --- |\n| 녕 | 녕{pitch=rise,duration=long,stress=strong} |\n| want to | ((want to)){reduced=true,stress=weak,duration=short} |".to_string(),
        },
        SampleDocument {
            id: "markdown",
            label: "Markdown",
            source: "## Markdown + LaTeX\n\n> 마크한글은 **Markdown** 문서와 함께 동작합니다.\n\n1. 굵게: **Hello{!↗}**\n2. 취소선: ~~world{—}~~\n3. 코드: `녕{↗}` 는 코드 안에서는 일반 텍스트입니다.\n\n인라인 수식: $a^2 + b^2 = c^2$\n\n$$\n\\int_0^1 x^2 dx = \\frac{1}{3}\n$$\n\n| 언어 | 예시 |\n| --- | --- |\n| 한국어 | 안녕{↗—!}하세요 |\n| 중국어 | 妈{T1} 麻{T2} 马{T3} 骂{T4} |".to_string(),
        },
        SampleDocument {
            id: "tone",
            label: "성조",
            source: "妈{T1} 麻{T2} 马{T3} 骂{T4}\nma{tone=1} ma{tone=2} ma{tone=3} ma{tone=4}\nこ{↑} え{↓} か{↗} き{↘}".to_string(),
        },
        SampleDocument {
            id: "range",
            label: "범위",
            source: "((정말입니까)){pitch=rise,stress=strong}\n((good morning)){pitch=fall,duration=long}\n((want to)){reduced=true,stress=weak,duration=short,note=casual speech}".to_string(),
        },
        SampleDocument {
            id: "errors",
            label: "오류",
            source: "안녕{↗—!}\nHello{pitch=curve}\n妈{T7}\n((닫히지 않은 범위){pitch=rise}\n빈{}표기".to_string(),
        },
    ]
}
