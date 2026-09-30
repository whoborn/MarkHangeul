//! Explicit language/variety inventories. Contours are educational display
//! approximations, not measured F0 or a promise of automatic tone sandhi.
use crate::ast::Phonation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TonePreset {
    pub key: &'static str,
    pub label: &'static str,
    pub contour: &'static str,
    pub phonation: Phonation,
    pub checked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToneSystem {
    pub id: &'static str,
    pub label: &'static str,
    pub region: &'static str,
    pub note: &'static str,
    pub source_url: &'static str,
    pub tones: &'static [TonePreset],
}

const fn tone(key: &'static str, label: &'static str, contour: &'static str) -> TonePreset {
    TonePreset {
        key,
        label,
        contour,
        phonation: Phonation::Modal,
        checked: false,
    }
}
const fn glottal(key: &'static str, label: &'static str, contour: &'static str) -> TonePreset {
    TonePreset {
        phonation: Phonation::Glottalized,
        ..tone(key, label, contour)
    }
}
const fn checked(key: &'static str, label: &'static str, contour: &'static str) -> TonePreset {
    TonePreset {
        checked: true,
        ..tone(key, label, contour)
    }
}
const MANDARIN: &[TonePreset] = &[
    tone("1", "1성", "55"),
    tone("2", "2성", "35"),
    tone("3", "3성", "214"),
    tone("4", "4성", "51"),
];
const YUE: &[TonePreset] = &[
    tone("1", "음평", "55"),
    tone("2", "음상", "35"),
    tone("3", "음거", "33"),
    tone("4", "양평", "21"),
    tone("5", "양상", "13"),
    tone("6", "양거", "22"),
];
// Named historical categories avoid inventing universal numbers 7 and 8.
const HANOI: &[TonePreset] = &[
    tone("a1", "ngang · A1", "33"),
    tone("a2", "huyền · A2", "21"),
    tone("b1", "sắc · B1", "35"),
    glottal("b2", "nặng · B2", "21"),
    tone("c1", "hỏi · C1", "31"),
    glottal("c2", "ngã · C2", "35"),
];
const HANOI_EIGHT: &[TonePreset] = &[
    HANOI[0],
    HANOI[1],
    HANOI[2],
    HANOI[3],
    HANOI[4],
    HANOI[5],
    checked("d1", "sắc 입성 · D1", "45"),
    checked("d2", "nặng 입성 · D2", "21"),
];
const THAI: &[TonePreset] = &[
    tone("mid", "중평조", "33"),
    tone("low", "저조", "21"),
    tone("falling", "하강조", "241"),
    tone("high", "고조", "45"),
    tone("rising", "상승조", "315"),
];
const GENERIC: &[TonePreset] = &[
    tone("1", "시연 1", "55"),
    tone("2", "시연 2", "35"),
    tone("3", "시연 3", "214"),
    tone("4", "시연 4", "51"),
    tone("5", "시연 5", "33"),
    tone("6", "시연 6", "22"),
    tone("7", "시연 7", "53"),
    tone("8", "시연 8", "24"),
];

pub const TONE_SYSTEMS: &[ToneSystem] = &[
    ToneSystem { id: "mandarin", label: "중국어 · 4성", region: "표준 중국어",
        note: "독립 발음의 교육용 contour입니다. 3성 변조·경성은 자동 계산하지 않습니다.",
        source_url: "https://research-management.mq.edu.au/ws/portalfiles/portal/16797901/mq-41420-Publisher%2Bversion%2B%28open%2Baccess%29.pdf", tones: MANDARIN },
    ToneSystem { id: "yue", label: "광둥어 · 6성", region: "홍콩 · Jyutping",
        note: "입성도 Jyutping 1·3·6 번호를 사용합니다. checked=true로 종결을 별도 기록하세요.",
        source_url: "https://jyutping.org/en/jyutping/", tones: YUE },
    ToneSystem { id: "vietnamese-hanoi", label: "베트남어 · 하노이 6성", region: "베트남 북부 · 하노이",
        note: "개음절·공명음 종결의 6범주. 동일 음높이는 발성 보조표시로 구분합니다. contour는 교육용 근사입니다.",
        source_url: "https://www.isca-archive.org/sltu_2014/nguyen14_sltu.pdf", tones: HANOI },
    ToneSystem { id: "vietnamese-hanoi-8", label: "베트남어 · 하노이 6+2 범주", region: "베트남 북부 · 하노이",
        note: "6범주에 폐쇄음 종결 D1·D2를 더한 8범주 분석입니다. 모든 음절에 8개 성조가 대립한다는 뜻이 아닙니다.",
        source_url: "https://hcmussh.edu.vn/static/document/13269.pdf", tones: HANOI_EIGHT },
    ToneSystem { id: "thai", label: "태국어 · 5성", region: "태국 · 표준 태국어",
        note: "번호 1~5는 중평·저·하강·고·상승 순서입니다. 태국 문자 성조 부호 번호와 다릅니다.",
        source_url: "https://research-management.mq.edu.au/ws/portalfiles/portal/16797901/mq-41420-Publisher%2Bversion%2B%28open%2Baccess%29.pdf", tones: THAI },
    ToneSystem { id: "generic-8", label: "8개 시각 패턴 · 시연", region: "언어 미지정",
        note: "실제 언어의 성조 체계가 아닙니다.", source_url: "", tones: GENERIC },
];

pub fn tone_system(id: &str) -> Option<&'static ToneSystem> {
    let normalized = id.to_ascii_lowercase().replace('_', "-");
    let canonical = match normalized.as_str() {
        "zh" | "zh-cn" | "zh-tw" | "zh-hans" | "zh-hant" | "cmn" => "mandarin",
        "cantonese" | "yue-hk" | "zh-hk" => "yue",
        "vi-hanoi" | "vi-vn-hanoi" => "vietnamese-hanoi",
        "vi-hanoi-8" => "vietnamese-hanoi-8",
        "th" | "th-th" | "thai-central" => "thai",
        other => other,
    };
    TONE_SYSTEMS.iter().find(|s| s.id == canonical)
}

pub fn tone_preset(system: &ToneSystem, value: &str) -> Option<&'static TonePreset> {
    let value = value.to_lowercase();
    let value = if system.id == "thai" {
        match value.as_str() {
            "rise" => "rising",
            "fall" => "falling",
            "middle" => "mid",
            other => other,
        }
        .to_string()
    } else {
        value
    };
    if system.id.starts_with("vietnamese-hanoi") {
        let key = match value.as_str() {
            "ngang" => "a1",
            "huyen" | "huyền" => "a2",
            "sac" | "sắc" => "b1",
            "nang" | "nặng" => "b2",
            "hoi" | "hỏi" => "c1",
            "nga" | "ngã" => "c2",
            "sac-checked" => "d1",
            "nang-checked" => "d2",
            other => other,
        };
        // Vietnamese numeric ordering varies among sources: require names/codes.
        return system.tones.iter().find(|t| t.key == key);
    }
    system.tones.iter().find(|t| t.key == value).or_else(|| {
        value
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .and_then(|i| system.tones.get(i))
    })
}
