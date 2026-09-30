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
    let export_kind = use_state(|| ExportKind::Source);

    let document = use_memo((*source).clone(), |source| parse_markhangeul(source));
    let annotation_nodes = collect_mark_nodes(&document);
    let plain_markdown = if *export_kind == ExportKind::Plain {
        export_plain_markdown(&document)
    } else {
        String::new()
    };
    let json_ast = if *export_kind == ExportKind::Json {
        export_json_ast(&document)
    } else {
        String::new()
    };

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
                        <p>{"높낮이와 장평으로 읽는 발음"}</p>
                    </div>
                </div>
                <SampleSelector samples={samples} on_select={on_sample_select} />
            </header>

            <details class="usage-guide">
                <summary>{"처음 사용하기 · 성조 읽는 법"}</summary>
                <p>{"높낮이는 음높이(1=낮음, 5=높음), 좌우 폭은 발음 길이입니다. 글자의 왼쪽에서 오른쪽으로 음높이를 읽습니다."}</p>
                <p>{"중국어: 마{T1}~마{T4} · 광둥어: 마{lang=yue,tone=1}~6 · 다른 언어: 마{toneContour=214}. 성조 번호는 언어마다 다릅니다."}</p>
                <p>{"장단: 아{duration=long} · 보조선 숨김: 마{T3,soundShape=false}. 문법 기호를 그대로 쓰려면 코드(`...`) 또는 역슬래시로 여는 중괄호를 이스케이프하세요."}</p>
                <p>{"한글 발음은 직접 입력합니다. 자동 번역·전사·변조는 하지 않으며, 한글로 구분하기 어려운 소리는 ipa와 note로 함께 기록하세요."}</p>
            </details>
            <section class="workspace-grid">
                <EditorPanel source={(*source).clone()} on_change={on_source_change} />
                <PreviewPanel
                    document={(*document).clone()}
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
                    source={(*source).clone()}
                    html_document={if *export_kind == ExportKind::Html { crate::components::export_html(&document) } else { String::new() }}
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
            MarkHangeulToken::Markhangeul(mark) => Some(mark.as_ref().clone()),
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
            source: tone_comparison_sample(),
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

fn tone_comparison_sample() -> String {
    let mut source = String::from("# 성조 비교\n\n왼쪽→오른쪽으로 높낮이를 읽습니다. 1은 낮음, 5는 높음입니다. 같은 한글의 변형을 비교해 보세요.\n\n");
    for (label, system, contours) in [
        ("중국어 4성", "mandarin", vec!["55", "35", "214", "51"]),
        (
            "광둥어 6성",
            "yue",
            vec!["55", "35", "33", "21", "13", "22"],
        ),
        (
            "8개 대비 시연 · 실제 언어의 성조 번호 아님",
            "generic-8",
            vec!["55", "35", "214", "51", "33", "22", "53", "24"],
        ),
    ] {
        source.push_str(&format!(
            "## {label}\n\n| 번호 | 음높이 | 글자 + 보조선 | 글자만 |\n| --- | --- | --- | --- |\n"
        ));
        for (index, contour) in contours.iter().enumerate() {
            let n = index + 1;
            source.push_str(&format!("| {n} | {contour} | 마{{toneSystem={system},tone={n}}} | 마{{toneSystem={system},tone={n},soundShape=false}} |\n"));
        }
        source.push('\n');
    }
    source.push_str("## 직접 지정\n\n마{toneContour=151} 마{toneContour=214,duration=long}\n\n실제 언어·방언은 확인된 contour를 직접 입력하세요. 입성·발성 방식·음소 차이는 높낮이만으로 모두 표현되지 않으므로 duration, ipa, note를 함께 기록하세요.\n");
    source
}
