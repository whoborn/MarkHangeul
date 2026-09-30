use markhangeul_core::render_model::tone_systems::{tone_system, ToneSystem, TONE_SYSTEMS};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToneSystemPickerProps {
    pub on_select: Callback<String>,
}

#[function_component(ToneSystemPicker)]
pub fn tone_system_picker(props: &ToneSystemPickerProps) -> Html {
    let selected = use_state(|| "yue".to_string());
    let system = tone_system(&selected).expect("picker uses registered systems");
    let onchange = {
        let selected = selected.clone();
        Callback::from(move |event: Event| {
            selected.set(event.target_unchecked_into::<HtmlSelectElement>().value());
        })
    };
    let onclick = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| on_select.emit(comparison_document(system)))
    };
    html! {
        <section class="tone-system-picker" aria-label="언어와 지역별 성조">
            <div class="tone-picker-controls">
                <label for="tone-system">{"언어·지역"}</label>
                <select id="tone-system" value={(*selected).clone()} {onchange}>
                    {for TONE_SYSTEMS.iter().map(|s| html! { <option value={s.id} selected={*selected == s.id}>{s.label}</option> })}
                </select>
                <button type="button" {onclick}>{"비교 예제로 바꾸기"}</button>
            </div>
            <p>{format!("{} · {}", system.region, system.note)}</p>
            <p>{"현재 편집 내용이 비교 예제로 바뀝니다. 필요한 내용은 원문 파일로 먼저 저장하세요."}</p>
            if !system.source_url.is_empty() {
                <a href={system.source_url} target="_blank" rel="noopener noreferrer">{"체계의 근거 자료"}</a>
            }
        </section>
    }
}

pub(crate) fn comparison_document(system: &ToneSystem) -> String {
    let mut source = format!(
        "# {}\n\n{}\n\n{}\n\n",
        system.label, system.region, system.note
    );
    source.push_str("한글은 비교용 표본이며 원어의 정확한 음소 전사가 아닙니다. 아래 음높이는 교육용 근삿값입니다.\n\n| 범주 | 음높이 | 한글 표기 |\n| --- | --- | --- |\n");
    for tone in system.tones {
        let text = if tone.checked { "맛" } else { "마" };
        source.push_str(&format!(
            "| {} | {} | {}{{toneSystem={},tone={}}} |\n",
            tone.label, tone.contour, text, system.id, tone.key
        ));
    }
    source.push_str("\n## 발성·종결 읽는 법\n\n실선=일반 발성, 긴 점선=기식성, 짧은 점선=삐걱 발성, 중앙 두 획=성문음화, 끝 세로획=입성입니다. 이는 마크한글의 보조표시 규칙입니다. 보조선을 끄면 같은 contour의 발성 차이는 보이지 않습니다.\n\n마{toneContour=33,phonation=modal} 마{toneContour=33,phonation=breathy} 마{toneContour=33,phonation=creaky} 마{toneContour=33,phonation=glottalized} 맛{toneContour=33,checked=true}\n\n장단은 별도 지정합니다. 입성이라고 모든 모음을 자동으로 짧게 만들지 않습니다.\n");
    if !system.source_url.is_empty() {
        source.push_str(&format!("\n[근거 자료]({})\n", system.source_url));
    }
    source
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_picker_documents_parse_without_errors() {
        for system in TONE_SYSTEMS {
            let d = markhangeul_core::parse_markhangeul(&comparison_document(system));
            assert!(d.errors.is_empty(), "{}: {:?}", system.id, d.errors);
        }
    }
}
