# MarkHangeul 문법 · 수정판

## 목적과 호환성

한글 독음은 작성자가 입력하고, 발음의 **높낮이는 글자 내부의 세로 위치**, **길이는 장평**으로 표현합니다. 자동 번역·자동 전사·음성 합성은 수행하지 않습니다. 일반 Markdown에서는 `{...}` 원문이 보이며, 글자 변형에는 전용 렌더러나 HTML 내보내기가 필요합니다. GitHub README에 CSS/SVG 변형이 그대로 표시된다고 보장하지 않습니다.

## 입력과 범위

```md
안녕{↗—!}하세요
마{T1} 마{T2} 마{T3} 마{T4}
마{lang=yue,tone=6}
마{toneContour=214,duration=long}
((안녕하세요)){pitch=rise}
```

- 한글·한자·가나: 바로 앞 grapheme. 분해형 한글 자모도 하나의 grapheme을 유지합니다.
- 라틴: 바로 앞 단어. Markdown `_` 및 문장 끝 `.`을 단어에 흡수하지 않습니다.
- 그 외 문자 또는 여러 단어: `((범위)){속성}`. 중첩 범위는 지원하지 않습니다.
- 범위는 일반 텍스트 영역 안에서 작성합니다. 코드·수식·Markdown 구조 경계를 가로지르는 범위는 처리하지 않습니다. 강조는 `**((안녕하세요)){↗}**`처럼 범위 바깥에 둡니다.
- 일반 중괄호를 보존하려면 `마\{T2}` 또는 인라인 코드를 사용합니다.
- 코드 블록(들여쓰기·인용문 안의 fence 포함), 인라인 코드, 수식, HTML 태그, 링크 주소, 참조 정의, 이미지 설명은 변환하지 않습니다. 링크의 표시 문구는 지원합니다.
- 문자열 값의 쉼표·닫는 중괄호는 따옴표로 감쌉니다: `마{note="a,b}c",T2}`. 따옴표 안 역슬래시 이스케이프는 경계 판정에 사용하며 값에서는 원문으로 보존합니다.

## 성조

성조 번호는 전 세계에 공통인 음높이가 아닙니다. `toneContour`를 우선하고, 없으면 `toneSystem`, 다음으로 `lang`, 모두 없으면 중국어 4성 체계를 사용합니다.

| 체계 | 번호 | 기본 contour |
| --- | --- | --- |
| `mandarin` (`zh`, `zh-cn`, `zh-tw`, `cmn`) | 1~4 | 55, 35, 214, 51 |
| `yue` (`cantonese`, `yue-hk`, `zh-hk`) | 1~6 | 55, 35, 33, 21, 13, 22 |
| `vietnamese-hanoi` (`vi-hanoi`) | A1, A2, B1, B2, C1, C2 | 33, 21, 35, 21, 31, 35 + 발성 구분 |
| `vietnamese-hanoi-8` (`vi-hanoi-8`) | 위 6범주 + D1, D2 | 위 목록 + 45, 21 + 입성 구분 |
| `thai` (`th`, `th-th`, `thai-central`) | 1~5 또는 mid, low, falling, high, rising | 33, 21, 241, 45, 315 |
| `generic-8` | 1~8 | 55, 35, 214, 51, 33, 22, 53, 24 |
| 그 외 언어·사용자 체계 | 직접 지정 | `toneContour` 필수 |

광둥어 기본값은 [Jyutping 공식 안내의 성조 표](https://jyutping.org/en/jyutping/)에 맞춘 교육용 대표값입니다. 방언·발화 맥락·자료에 따라 다른 contour가 가능하며 직접 덮어쓸 수 있습니다. 중국어 3성 `214`도 독립 발음의 교육용 표현으로, 문맥 변조를 자동 적용하지 않습니다.

`generic-8`은 **8개의 서로 다른 시각 패턴을 확인하는 시연용 목록**이며 실제 언어의 8성 체계가 아닙니다. 실제 8성 언어는 해당 언어·방언에서 확인한 contour와 음절 길이 등을 각각 지정하세요.

```md
마{toneSystem=generic-8,tone=8}
마{lang=my-language,tone=8,toneContour=24}
마{tone=custom,toneContour=151}
마{toneContour=214,soundShape=false}
```

- `toneContour`: ASCII 숫자 1~5로 구성된 2~16개 지점. 1=최저, 5=최고. 지점 사이 시간은 균등합니다. `tone` 없이도 동작합니다.
- `T0`, `tone=0`, `tone=neutral`: 중립 위치 `33`을 쓰는 표시상의 약속입니다. 경성의 실제 음높이가 항상 33이라는 뜻은 아닙니다.
- `T1`~`T9`와 `tone=1`~`9`: 번호는 파싱하지만 해당 체계 밖 번호에는 contour가 필요합니다. 예를 들어 체계 없는 `T8`은 오류입니다.
- 이름형: `high`, `mid`, `low`, `rise`, `fall`, `dip` 및 대응 별칭을 지원합니다. 미등록 이름은 contour를 지정해야 합니다.
- 태국어의 `tone=high`는 체계별 고조(45)를 뜻하며 일반 이름형 high(55)보다 우선합니다. 미등록 번호·범주는 오류로 표시합니다.

한 음절에서도 contour 전체를 읽을 수 있도록 글자 윤곽에 연속적인 세로 기울임을 적용합니다. 상승·하강·평탄은 글자를 한 번만 그리고, 굴곡은 contour가 꺾이는 지점에서만 연결된 조각으로 나눕니다. 기존 16조각 계단식 이동은 사용하지 않습니다. 음높이 한 단계의 이동은 0.06em이며 인접한 최고·최저음의 최대 차이는 0.24em입니다. 여러 글자 범위에서는 범위 전체에 걸쳐 contour가 진행합니다. 글자 원형을 보존하는 근사 변형이며 전용 가변 폰트는 아닙니다. 보조선은 같은 지점들을 직선으로 연결하여 곡선 보간의 과도한 상승·하강을 방지합니다.

## 언어·지역 및 발성·입성 확장

지역별 지원 범위와 원자료는 [언어별 프로필](LANGUAGE-PROFILES.md)을 참고하세요. 웹 상단의 **언어·지역** 선택 후 **비교 예제로 바꾸기**를 누르면 해당 체계의 한글 비교표를 작성할 수 있습니다.

```md
마{toneSystem=vietnamese-hanoi,tone=ngang}
마{toneSystem=vi-hanoi,tone=ngã}
맛{toneSystem=vi-hanoi-8,tone=D1}
마{lang=th-TH,tone=high}
맛{lang=yue,tone=6,checked=true}
마{lang=my-region,tone=8,toneContour=24,phonation=breathy}
```

하노이의 8범주는 6범주와 폐쇄음 종결 2범주를 합친 분석입니다. 모든 음절에 8개의 성조가 대립하는 뜻은 아닙니다. 숫자 순서의 혼동을 피하기 위해 베트남어 preset은 `A1~D2` 또는 `ngang`, `huyen`, `sac`, `nang`, `hoi`, `nga`, `sac-checked`, `nang-checked`로 지정합니다. 성조 이름의 베트남어 부호도 지원합니다. `lang=vi`만으로는 하노이 방언을 자동 선택하지 않습니다.

- `phonation`: modal, breathy, creaky, glottalized. 명시하면 preset보다 우선합니다.
- `checked`: boolean. 폐쇄음 종결을 나타냅니다. 자동 장단 축소는 하지 않습니다.
- 긴 점선은 breathy, 짧은 점선은 creaky, 중앙 두 획은 glottalized, 끝 세로획은 checked를 나타내는 **앱의 보조표시 규칙**입니다. IPA 표준 기호가 아닙니다.
- 보조선을 숨기면 같은 contour의 발성·입성 차이는 글자 모양만으로 식별할 수 없습니다.
- 하노이 A·B·C 범주와 D 범주의 checked 값이 충돌하거나 Jyutping 2·4·5성에 checked=true를 쓰면 오류입니다.
- contour를 덮어써도 해당 preset의 발성·입성 정보는 유지합니다. 발성은 별도로 덮어쓸 수 있습니다.
- Inspector는 입력된 속성과 실제 해석 결과를 나눠 보여줍니다. JSON은 원문 속성을 보존하며 생략된 preset 기본값을 자동 삽입하지 않습니다.

## 장단·강세·성량

| 속성 | 값 | 효과 |
| --- | --- | --- |
| `pitch` | low, mid, high, rise, fall | 글자 높낮이. 성조 contour가 있으면 contour 우선 |
| `duration` | extra-short, short, slight-short, normal, slight-long, long, extra-long | 장평 0.72, 0.84, 0.93, 1, 1.08, 1.20, 1.42배 |
| `stress` | weak, normal, strong, extra-strong | 높이 0.94, 1, 1.14, 1.26배. Markdown 굵기 유지 |
| `volume` | soft, normal, loud | 크기·투명도·존재감 조정 |

장평은 화면 변형뿐 아니라 실제 배치 폭에 반영됩니다. 한글 1em을 기준으로 하며 라틴은 글자에 따라 0.28~0.9em, 공백은 0.32em으로 자간을 근사합니다. 글자 윤곽 자체를 정해진 폭에 강제로 늘리지 않습니다. 라틴 비례 폰트의 원래 자폭이나 복잡한 문자권의 연결형 조판을 완전히 재현하지는 않습니다.

| 기호 | 속성 |
| --- | --- |
| `↗ ↘ ↑ ↓ ·` | rise, fall, high, low, mid |
| `˘˘ ˘ - — ——` | extra-short, short, normal, long, extra-long |
| `? ! !!` | weak, strong, extra-strong |
| `° • ●` | soft, normal, loud |
| `T0`~`T9` | 성조 번호 |

기호와 key-value는 쉼표로 함께 사용합니다: `마{T2,duration=long}`.

## 보조선과 메타데이터

- 성조 및 key-value pitch/duration: 보조선 기본 표시.
- 순수 기호 pitch/duration: 기본 숨김.
- `soundShape=false` / `hideGuide=true`: 숨김. `soundShape=true`: 표시.
- `guideColor=true`: 장단별 보조선 색상을 선택적으로 사용. 글자색과 Markdown 굵기는 유지합니다.
- `tone_system`, `tone_contour`, `sound_shape`, `guide_color` 등 snake_case 별칭도 지원합니다.
- `guide`, `showGuide`, `shapeGuide`, `contourGuide`는 `soundShape` 별칭입니다. `hideSoundShape`, `hideShape`, `noGuide`는 숨김 별칭입니다.
- `showColor`, `durationColor`, `lengthColor`, `colorGuide`, `visualColor`는 `guideColor` 별칭입니다.
- boolean은 true/false, 1/0, yes/no, y/n, on/off만 받습니다. 오타는 오류입니다.

`ipa`, `phoneme`, `lang`, `note`, `mora`, `syllable_role`은 문자열로 보존합니다. `nasal`, `aspiration`, `fortis`, `lenis`, `palatalization`, `retroflexion`, `liaison`, `reduced`, `assimilation`, `deletion`은 boolean 메타데이터입니다. **이 속성들은 현재 자동 음소 변경이나 별도 글자 변형을 수행하지 않습니다.** Inspector와 JSON에서 확인할 수 있습니다. 미등록 key는 `extras`에 보존합니다.

## 내보내기와 보안

| 형식 | 용도 | 발음 정보 |
| --- | --- | --- |
| 원문 `.mh.md` | 재편집·버전 관리 | 모두 보존 |
| 일반 `.md` | 일반 Markdown 소비 도구 | 정상 annotation 제거. 오류 annotation은 원문 보존 |
| `.html` | 브라우저에서 열기·정적 배포 | SVG/CSS 포함. 추가 WASM 없이 발음 표시 |
| `.json` | 외부 도구 연동 | AST와 속성, UTF-8 바이트 위치 |

미리보기는 사용자 HTML을 이스케이프하며 http/https/mailto 및 상대 URL만 허용합니다. 렌더러가 생성한 HTML만 삽입합니다. Markdown 표·목록·강조·취소선·각주·수식 등을 지원하지만 이것이 모든 Markdown 구현과 동일하다는 의미는 아닙니다.

Playground의 수식 조판은 외부 MathJax가 필요합니다. 수식 내부 URL·사용자 class/id/style은 ui/safe로 차단합니다. 독립 HTML은 외부 스크립트를 포함하지 않으며 수식을 TeX 원문으로 표시합니다. AST의 위치는 JS의 UTF-16 인덱스가 아닌 UTF-8 바이트 오프셋입니다. Rust 내부 `Markhangeul` variant는 메모리 낭비를 줄이기 위해 Box를 사용하며 JSON 구조는 유지됩니다.
