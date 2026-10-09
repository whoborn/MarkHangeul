//! Curated examples, separate from tone inventories: language is not a tone number.
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub(crate) struct LanguageExample {
    pub id: &'static str,
    pub label: &'static str,
    pub original: &'static str,
    pub ipa: &'static str,
    pub hangul: &'static str,
    pub meaning: &'static str,
    pub note: &'static str,
    pub source: &'static str,
}

pub(crate) const LANGUAGES: &[LanguageExample] = &[
    LanguageExample {
        id: "en-US",
        label: "영어 · 미국",
        original: "hello",
        ipa: "/həˈloʊ/",
        hangul: "허{lang=en-US,ipa=hə,reduced=true,stress=weak}로{lang=en-US,ipa=loʊ,stress=strong}",
        meaning: "안녕하세요",
        note: "둘째 음절 강세. /ə/와 /oʊ/는 한글 허·로와 일치하지 않습니다. 강세를 고정된 성조로 바꾸지 않습니다.",
        source: "https://en.wiktionary.org/wiki/hello#English",
    },
    LanguageExample {
        id: "en-GB",
        label: "영어 · 영국 사전 발음",
        original: "hello",
        ipa: "/həˈləʊ/",
        hangul: "허{lang=en-GB,ipa=hə,reduced=true,stress=weak}((러우)){lang=en-GB,ipa=ləʊ,stress=strong}",
        meaning: "안녕하세요",
        note: "미국 예제와 이중모음이 다릅니다. 러우의 두 글자는 /ləʊ/ 한 음절의 근사이며 영국의 모든 방언을 대표하지 않습니다.",
        source: "https://en.wiktionary.org/wiki/hello#English",
    },
    LanguageExample {
        id: "fr-FR",
        label: "프랑스어 · 프랑스",
        original: "bonjour",
        ipa: "/bɔ̃.ʒuʁ/",
        hangul: "보{lang=fr-FR,ipa=bɔ̃,nasal=true}((주르)){lang=fr-FR,ipa=ʒuʁ}",
        meaning: "안녕하세요",
        note: "/ɔ̃/는 비모음이며 받침 ㅇ이 아닙니다. /ʒ/와 /ʁ/는 주·르의 자음과 다릅니다. 비음 속성은 기록용이며 글자를 자동 변환하지 않습니다.",
        source: "https://en.wiktionary.org/wiki/bonjour#French",
    },
    LanguageExample {
        id: "de-DE",
        label: "독일어 · 표준 독일어",
        original: "Schule",
        ipa: "/ˈʃuːlə/",
        hangul: "슈{lang=de-DE,ipa=ʃuː,stress=strong,duration=long}러{lang=de-DE,ipa=lə,reduced=true}",
        meaning: "학교",
        note: "첫 음절 강세와 /uː/ 장음. 마지막 /ə/는 한국어 ㅓ와 다른 중설 모음입니다.",
        source: "https://de.wiktionary.org/wiki/Schule",
    },
    LanguageExample {
        id: "es",
        label: "스페인어 · 일반적인 음소 표기",
        original: "casa",
        ipa: "/ˈkasa/",
        hangul: "카{lang=es,ipa=ka,stress=strong}사{lang=es,ipa=sa}",
        meaning: "집",
        note: "첫 음절에 강세를 표시합니다. 한국어 ㅋ의 강한 기식까지 동일하다는 뜻은 아닙니다.",
        source: "https://en.wiktionary.org/wiki/casa#Spanish",
    },
    LanguageExample {
        id: "it-IT",
        label: "이탈리아어 · 표준 이탈리아어",
        original: "pasta",
        ipa: "/ˈpas.ta/",
        hangul: "((파스)){lang=it-IT,ipa=pas,stress=strong}타{lang=it-IT,ipa=ta}",
        meaning: "파스타",
        note: "첫 음절 강세. 파스는 /pas/ 한 음절의 근사이며 /s/ 뒤에 별도 모음을 넣는다는 뜻이 아닙니다.",
        source: "https://en.wiktionary.org/wiki/pasta#Italian",
    },
    LanguageExample {
        id: "pt-BR",
        label: "포르투갈어 · 브라질",
        original: "pão",
        ipa: "/pɐ̃w̃/",
        hangul: "((파우)){lang=pt-BR,ipa=pɐ̃w̃,nasal=true}",
        meaning: "빵",
        note: "비음 이중모음 한 음절입니다. 파우는 근사 표기이며 /ɐ̃w̃/의 비음성을 한글만으로 구별하지 못합니다.",
        source: "https://en.wiktionary.org/wiki/pão#Portuguese",
    },
    LanguageExample {
        id: "ru-RU",
        label: "러시아어 · 표준 러시아어",
        original: "спасибо",
        ipa: "[spɐˈsʲibə]",
        hangul: "((스파)){lang=ru-RU,ipa=spɐ,reduced=true}시{lang=ru-RU,ipa=sʲi,stress=strong,palatalization=true}버{lang=ru-RU,ipa=bə,reduced=true}",
        meaning: "감사합니다",
        note: "강세 음절과 무강세 모음 약화, /sʲ/의 연자음화를 구분합니다. 스파는 /spɐ/ 한 음절의 근사입니다.",
        source: "https://en.wiktionary.org/wiki/спасибо#Russian",
    },
    LanguageExample {
        id: "ar",
        label: "아랍어 · 현대 표준어 (휴지형)",
        original: "كِتَاب",
        ipa: "/ki.taːb/",
        hangul: "키{lang=ar,ipa=ki}탑{lang=ar,ipa=taːb,duration=long}",
        meaning: "책",
        note: "모음 /aː/의 길이를 장평으로 표시합니다. 탑의 받침은 유성 /b/를 정확히 나타내지 못합니다. 격어미를 생략한 휴지형이며 지역 방언과 구별합니다.",
        source: "https://en.wiktionary.org/wiki/كتاب#Arabic",
    },
    LanguageExample {
        id: "hi-IN",
        label: "힌디어 · 표준 힌디어",
        original: "पानी",
        ipa: "/pɑː.niː/",
        hangul: "파{lang=hi-IN,ipa=pɑː,duration=long}니{lang=hi-IN,ipa=niː,duration=long}",
        meaning: "물",
        note: "두 모음의 장음을 표시합니다. 원어의 /p/는 한국어 ㅍ처럼 강한 기식음이라는 뜻이 아닙니다.",
        source: "https://en.wiktionary.org/wiki/पानी#Hindi",
    },
    LanguageExample {
        id: "sw",
        label: "스와힐리어 · 동아프리카 표준어",
        original: "safari",
        ipa: "[sɑˈfɑɾi]",
        hangul: "사{lang=sw,ipa=sɑ}파{lang=sw,ipa=fɑ,stress=strong}리{lang=sw,ipa=ɾi}",
        meaning: "여행",
        note: "끝에서 둘째 음절 강세. /f/는 한국어 ㅍ과 다른 마찰음입니다.",
        source: "https://en.wiktionary.org/wiki/Appendix:Swahili_pronunciation",
    },
    LanguageExample {
        id: "tr-TR",
        label: "튀르키예어 · 표준어",
        original: "su",
        ipa: "/su/",
        hangul: "수{lang=tr-TR,ipa=su}",
        meaning: "물",
        note: "짧은 단모음의 기본 예입니다. 국가 전체의 방언이나 모음 조화를 이 한 단어로 설명하지는 않습니다.",
        source: "https://en.wiktionary.org/wiki/su#Turkish",
    },
];

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum DisplayMode {
    Both,
    Hangul,
    Original,
}
impl DisplayMode {
    fn label(self) -> &'static str {
        match self {
            Self::Both => "원어 + 한글",
            Self::Hangul => "한글",
            Self::Original => "원어",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct WorldLanguagesProps {
    pub on_select: Callback<String>,
}

#[function_component(WorldLanguages)]
pub fn world_languages(props: &WorldLanguagesProps) -> Html {
    let selected = use_state(|| "en-US".to_string());
    let mode = use_state(|| DisplayMode::Both);
    let example = LANGUAGES
        .iter()
        .find(|e| e.id == *selected)
        .unwrap_or(&LANGUAGES[0]);
    let onlanguage = {
        let selected = selected.clone();
        Callback::from(move |event: Event| {
            selected.set(event.target_unchecked_into::<HtmlSelectElement>().value())
        })
    };
    let onmode = {
        let mode = mode.clone();
        Callback::from(move |event: Event| {
            mode.set(
                match event
                    .target_unchecked_into::<HtmlSelectElement>()
                    .value()
                    .as_str()
                {
                    "hangul" => DisplayMode::Hangul,
                    "original" => DisplayMode::Original,
                    _ => DisplayMode::Both,
                },
            )
        })
    };
    let onclick = {
        let on_select = props.on_select.clone();
        let mode = *mode;
        Callback::from(move |_| on_select.emit(example_document(example, mode)))
    };
    html! {
        <section class="tone-system-picker world-languages" aria-label="세계 언어 발음 예제">
            <h2>{"세계 언어 발음"}</h2>
            <div class="tone-picker-controls">
                <label for="example-language">{"예제 언어·지역"}</label>
                <select id="example-language" onchange={onlanguage}>
                    {for LANGUAGES.iter().map(|e| html! { <option value={e.id} selected={*selected == e.id}>{e.label}</option> })}
                </select>
                <label for="example-display">{"표기 방식"}</label>
                <select id="example-display" onchange={onmode}>
                    <option value="both" selected={*mode == DisplayMode::Both}>{"원어 + 한글"}</option>
                    <option value="hangul" selected={*mode == DisplayMode::Hangul}>{"한글"}</option>
                    <option value="original" selected={*mode == DisplayMode::Original}>{"원어"}</option>
                </select>
                <button type="button" {onclick}>{"언어 예제로 바꾸기"}</button>
            </div>
            <p class="native-sample"><bdi lang={example.id} dir="auto">{example.original}</bdi>{" · "}<bdi dir="ltr">{example.ipa}</bdi>{format!(" · {}", example.meaning)}</p>
            <p>{example.note}</p>
            <p>{"선택한 방식으로 현재 편집 내용을 교체합니다. 한글은 발음 학습용 근사이며, IPA와 설명을 함께 읽으세요."}</p>
        </section>
    }
}

pub(crate) fn example_document(e: &LanguageExample, mode: DisplayMode) -> String {
    let mut columns = vec!["뜻", "IPA"];
    let mut values = vec![e.meaning, e.ipa];
    if mode != DisplayMode::Hangul {
        columns.push("원어");
        values.push(e.original);
    }
    if mode != DisplayMode::Original {
        columns.push("한글 발음 (근사)");
        values.push(e.hangul);
    }
    format!("# {} 발음 예제\n\n원어 언어: `{}` · 표기 방식: {}\n\n| {} |\n| {} |\n| {} |\n\n{}\n\n한글은 학습용 근사입니다. IPA의 ˈ는 강세, ː는 장음, ̃는 비음화를 뜻합니다. 높낮이는 명시된 억양·성조에만 적용하며 강세를 성조로 대체하지 않습니다. 원어는 문자 연결을 유지하는 일반 텍스트입니다.\n\n[단어·발음 근거]({})\n", e.label, e.id, mode.label(), columns.join(" | "), vec!["---"; columns.len()].join(" | "), values.join(" | "), e.note, e.source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use markhangeul_core::{export_plain_markdown, parse_markhangeul, MarkHangeulToken};
    #[test]
    fn every_language_and_display_mode_is_valid_and_preserves_selected_text() {
        let mut ids = std::collections::HashSet::new();
        for e in LANGUAGES {
            assert!(ids.insert(e.id));
            for mode in [
                DisplayMode::Both,
                DisplayMode::Hangul,
                DisplayMode::Original,
            ] {
                let source = example_document(e, mode);
                let doc = parse_markhangeul(&source);
                assert!(doc.errors.is_empty(), "{}: {:?}", e.id, doc.errors);
                let plain = export_plain_markdown(&doc);
                assert!(plain.contains(e.ipa));
                assert_eq!(
                    plain.contains(&format!("| {} |", e.original)),
                    mode != DisplayMode::Hangul
                );
                let marks: Vec<_> = doc
                    .nodes
                    .iter()
                    .filter_map(|n| match n {
                        MarkHangeulToken::Markhangeul(m) => Some(m),
                        _ => None,
                    })
                    .collect();
                assert_eq!(marks.is_empty(), mode == DisplayMode::Original);
                for mark in marks {
                    assert_eq!(mark.attributes.lang.as_deref(), Some(e.id));
                    assert!(mark.attributes.ipa.is_some());
                    assert!(mark.attributes.tone.is_none());
                }
                let html = super::super::preview::export_html(&doc);
                if mode != DisplayMode::Hangul {
                    assert!(html.contains(e.original));
                }
            }
        }
    }
}
