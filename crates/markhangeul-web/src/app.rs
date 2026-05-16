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
            source: "# MarkHangeul 기본 표기 모음\n\n**오늘은 날씨가 좋네요{↘}.**\n\n## 기호형\n\n상승: 마{↗}\n하강: 마{↘}\n높은 음: 미{↑}\n낮은 음: 무{↓}\n장음: 아{—}\n매우 긴 음: 아{——}\n강세: 가{!}\n강한 강세: 가{!!}\n약한 성량: 하{°}\n큰 성량: 하{●}\n\n## Key-Value형\n\n녕{pitch=rise,duration=slight-long,stress=strong}\n마{tone=2,toneContour=35}\n아{duration=extra-long,guideColor=true}\n\n## 범위형\n\n((안녕하세요)){pitch=rise,duration=long}\n((원 투)){reduced=true,stress=weak,duration=short,note=연음}\n\n## 한자 발음의 한글 표기\n\n| 한자 | 실제 발음 표기 |\n| --- | --- |\n| 妈 | 마{T1} |\n| 麻 | 마{T2} |\n| 马 | 마{T3} |\n| 骂 | 마{T4} |\n\n- Markdown 목록 안의 마{!↗}\n- 수식: $E = mc^2$".to_string(),
        },
        SampleDocument {
            id: "markdown",
            label: "Markdown",
            source: "## Markdown + LaTeX\n\n> 마크한글은 **Markdown** 문서와 함께 동작합니다.\n\n1. 굵게: **헬로{!↗}**\n2. 취소선: ~~월드{—}~~\n3. 코드: `녕{↗}` 는 코드 안에서는 일반 텍스트입니다.\n\n인라인 수식: $a^2 + b^2 = c^2$\n\n$$\n\\int_0^1 x^2 dx = \\frac{1}{3}\n$$\n\n| 언어 | 예시 |\n| --- | --- |\n| 한국어 | 안녕{↗—!}하세요 |\n| 중국어 | 妈=마{T1} 麻=마{T2} 马=마{T3} 骂=마{T4} |".to_string(),
        },
        SampleDocument {
            id: "tone",
            label: "성조",
            source: "## 범용 성조\n\n중국어 4성 한글 발음: 妈=마{T1} 麻=마{T2} 马=마{T3} 骂=마{T4}\n\n홍콩 광둥어 6성 한글 발음 예시: 詩=시{lang=yue,tone=1} 史=시{lang=yue,tone=2} 試=시{lang=yue,tone=3} 時=시{lang=yue,tone=4} 市=시{lang=yue,tone=5} 事=시{lang=yue,tone=6}\n\n8성 체계 예시: 아{tone=1} 아{tone=2} 아{tone=3} 아{tone=4} 아{tone=5} 아{tone=6} 아{tone=7} 아{tone=8}\n\n사용자 contour: 마{tone=custom,toneContour=53} 마{tone=custom,toneContour=214}\n\n아래선 숨김: 마{T2,soundShape=false}\n\nこ=코{↑} え=에{↓} か=카{↗} き=키{↘}".to_string(),
        },
        SampleDocument {
            id: "duration",
            label: "장단",
            source: "## 7단계 장단\n\n아{duration=extra-long} 아{duration=long} 아{duration=slight-long} 아{duration=normal} 아{duration=slight-short} 아{duration=short} 아{duration=extra-short}\n\n- 아주 길게: 아{duration=extra-long}\n- 보통 길게: 아{duration=long}\n- 조금 길게: 아{duration=slight-long}\n- 보통: 아{duration=normal}\n- 조금 짧게: 아{duration=slight-short}\n- 보통 짧게: 아{duration=short}\n- 아주 짧게: 아{duration=extra-short}\n\n아래선 숨김: 아{duration=extra-long,soundShape=false} 아{duration=extra-short,soundShape=false}\n\n아래 선 유색 표시 선택: 아{duration=extra-long,guideColor=true} 아{duration=extra-short,guideColor=true}".to_string(),
        },
        SampleDocument {
            id: "range",
            label: "범위",
            source: "((정말입니까)){pitch=rise,stress=strong}\n((굿 모닝)){pitch=fall,duration=long}\n((원 투)){reduced=true,stress=weak,duration=short,note=연음}".to_string(),
        },
        SampleDocument {
            id: "errors",
            label: "오류",
            source: "안녕{↗—!}\n헬로{pitch=curve}\n마{tone=}\n((닫히지 않은 범위){pitch=rise}\n빈{}표기".to_string(),
        },
    ]
}
