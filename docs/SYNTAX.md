# MarkHangeul Syntax v0.1

마크한글은 일반 Markdown 텍스트를 보존하면서 필요한 구간에만 발음 정보를 붙입니다. 전용 렌더러가 있을 때 annotation은 문자 자체의 시각 효과로 변환되고, Plain Markdown export에서는 annotation이 제거된 본문만 남습니다.

## 기본형

```text
대상{표기}
```

예:

```md
녕{↗—!}
헬로{!↗}
妈=마{T1}
麻=마{T2}
马=마{T3}
骂=마{T4}
```

## 명시 범위형

```text
((대상 범위)){표기}
```

예:

```md
((정말입니까)){pitch=rise}
((원 투)){reduced=true}
```

## Scope 규칙

| 문자권 | 암시 범위 |
| --- | --- |
| 한글 | 바로 앞 grapheme |
| 한자 | 바로 앞 grapheme |
| 히라가나/가타카나 | 바로 앞 grapheme |
| 라틴 문자 | 바로 앞 연속 단어 |
| 명시 범위 `((...))` | 괄호 안 전체 |

## 기호형 annotation

| 기호 | 속성 | 값 |
| --- | --- | --- |
| `↗` | pitch | `rise` |
| `↘` | pitch | `fall` |
| `↑` | pitch | `high` |
| `↓` | pitch | `low` |
| `·` | pitch | `mid` |
| `˘˘` | duration | `extra-short` |
| `˘` | duration | `short` |
| `-` | duration | `normal` |
| `—` | duration | `long` |
| `——` | duration | `extra-long` |
| `?` | stress | `weak` |
| `!` | stress | `strong` |
| `!!` | stress | `extra-strong` |
| `°` | volume | `soft` |
| `•` | volume | `normal` |
| `●` | volume | `loud` |
| `T0` | tone | `neutral` |
| `T1`~`T9` | tone | `1`~`9` |

기호는 조합할 수 있습니다.

```md
녕{↗—!}
세{!!●}
```

순수 기호형 annotation은 기본적으로 글자 아래 선을 표시하지 않습니다. `↗`, `↘`, `↑`, `↓`, `—`, `!`, `°`, `●`는 글자 자체의 위치, 폭, 높이, 크기만 바꿉니다. 기호형에서도 보조선을 강제로 표시하려면 `soundShape=true`를 함께 씁니다.

예외적으로 `T1`~`T9`는 성조 기호이므로 성조 보조선을 기본 표시합니다.

## Key-Value annotation

```md
녕{pitch=rise,duration=long,stress=strong}
((원 투)){reduced=true,stress=weak,duration=short}
```

지원 속성:

| key | 값 |
| --- | --- |
| `pitch` | `low`, `mid`, `high`, `rise`, `fall` |
| `duration` | `extra-short`, `short`, `slight-short`, `normal`, `slight-long`, `long`, `extra-long` |
| `stress` | `weak`, `normal`, `strong`, `extra-strong` |
| `volume` | `soft`, `normal`, `loud` |
| `tone` | `0`, `1`~`9`, `neutral`, `T0`~`T9`, 또는 이름형 tone |
| `toneSystem` / `tone_system` | `mandarin`, `yue`, `cantonese`, `thai`, `custom` 등 |
| `toneContour` / `tone_contour` | 1~5 숫자열 contour. 예: `55`, `35`, `214`, `51`, `22` |
| `soundShape` / `sound_shape` | boolean. 글자 아래 음형태 보조선 표시 여부 |
| `guide`, `showGuide`, `shapeGuide`, `contourGuide` | `soundShape`의 별칭 |
| `hideGuide`, `hideSoundShape`, `hideShape`, `noGuide` | `soundShape=false`의 별칭 |
| `guideColor` / `guide_color` | boolean. 아래 보조선의 길이별 유색 표시 여부 |
| `showColor`, `durationColor`, `lengthColor`, `colorGuide`, `visualColor` | `guideColor`의 호환 별칭 |
| `ipa` | 문자열 |
| `phoneme` | 문자열 |
| `lang` | 문자열 |
| `note` | 문자열 |
| `nasal` | boolean |
| `aspiration` | boolean |
| `fortis` | boolean |
| `lenis` | boolean |
| `palatalization` | boolean |
| `retroflexion` | boolean |
| `liaison` | boolean |
| `reduced` | boolean |
| `assimilation` | boolean |
| `deletion` | boolean |
| `mora` | 문자열 |
| `syllable_role` | 문자열 |

알 수 없는 key는 `attributes.extras`에 보존합니다.

## 장단 7단계

장단은 badge나 보조 기호가 아니라 글자 glyph 자체의 좌우 폭 변화로 표현합니다. 렌더러는 layout 폭과 `scaleX`를 함께 조정해 글자가 실제로 늘어나거나 압축되어 보이게 합니다.

기본값은 원문 텍스트의 색상과 굵기를 유지합니다. 글자에는 장단 색상을 적용하지 않습니다. key-value 장단은 폭 변화와 단색 아래 보조선을 함께 사용할 수 있고, 기호형 장단 `—`, `——`, `˘`는 기본적으로 선 없이 폭 변화만 보입니다. 길이별 유색 선이 필요할 때만 `guideColor=true`를 지정합니다. 이때 단음 계열은 차가운 색상, 장음 계열은 보라/주황/붉은 색상으로 구분해 폭 변화가 작아도 길이 차이를 식별할 수 있게 합니다.

| 값 | 한국어 설명 | 렌더링 의도 |
| --- | --- | --- |
| `extra-long` | 아주 길게 | 가장 크게 좌우 확장 |
| `long` | 보통 길게 | 명확한 장음 |
| `slight-long` | 조금 길게 | 약한 장음 |
| `normal` | 보통 | 기본 폭 |
| `slight-short` | 조금 짧게 | 약한 단음 |
| `short` | 보통 짧게 | 명확한 단음 |
| `extra-short` | 아주 짧게 | 가장 강한 압축 |

예:

```md
아{duration=extra-long}
아{duration=long}
아{duration=slight-long}
아{duration=normal}
아{duration=slight-short}
아{duration=short}
아{duration=extra-short}
```

아래 보조선 색상 구분을 명시적으로 켠 예:

```md
아{duration=extra-long,guideColor=true}
아{duration=extra-short,guideColor=true}
```

기호형 shorthand:

- `——`: `extra-long`
- `—`: `long`
- `-`: `normal`
- `˘`: `short`
- `˘˘`: `extra-short`

`slight-long`, `slight-short`는 초기 MVP에서 key-value형으로 지정합니다.

## 범용 성조

성조는 중국어 4성에 고정하지 않습니다. 기본 `T1`~`T9` shorthand와 key-value형 `tone=1`~`tone=9`를 지원하며, `toneSystem`과 `toneContour`를 함께 사용해 언어별 체계를 지정할 수 있습니다.

예:

```md
妈=마{T1} 麻=마{T2} 马=마{T3} 骂=마{T4}
詩=시{lang=yue,tone=1} 史=시{lang=yue,tone=2} 試=시{lang=yue,tone=3}
時=시{lang=yue,tone=4} 市=시{lang=yue,tone=5} 事=시{lang=yue,tone=6}
아{tone=7} 아{tone=8}
마{tone=custom,toneContour=53}
마{tone=custom,toneContour=214}
```

렌더러의 기본 contour 해석:

- Mandarin 스타일 기본값: `1=55`, `2=35`, `3=214`, `4=51`
- Cantonese/Yue 스타일 기본값: `1=55`, `2=25`, `3=33`, `4=21`, `5=23`, `6=22`
- 범용 7/8성: `tone=7`, `tone=8`도 시각 패턴으로 표시
- `toneContour`가 있으면 언어별 기본값보다 우선합니다.

보조선이 꺼져 있어도 성조는 글자 자체의 세로 위치, 회전, 기울임, 높이 비율로 드러납니다. 상승 성조는 오른쪽 위로 열린 기울기, 하강 성조는 오른쪽 아래로 열린 기울기, 굴곡 성조는 낮아졌다 올라오는 압축된 형태로 표현합니다. 이 효과는 색상이나 font-weight를 바꾸지 않습니다.

성조 보조선은 원형 contour를 보존하기 위해 부드러운 곡선으로 렌더링합니다. 평탄 성조는 수평선, 상승 성조는 상승 곡선, 하강 성조는 하강 곡선, 굴곡 성조는 내려갔다 올라오는 곡선으로 표시합니다.

## 음형태 보조선

글자 아래의 음형태 보조선은 성조에서 기본값이 표시입니다. 즉 `마{T2}`는 글자 자체의 위치 변화와 아래 성조 곡선을 함께 보여줍니다. 순수 기호형 `마{↗}`는 기본적으로 보조선을 표시하지 않습니다.

보조선을 숨기려면 다음 중 하나를 사용합니다.

```md
마{T2,soundShape=false}
마{pitch=rise,hideGuide=true}
아{duration=extra-long,noGuide=true}
마{tone=custom,toneContour=214,hideSoundShape=true}
```

보조선이 표시되는 경우 렌더러는 tone contour, pitch, duration 순서로 표시할 보조선을 선택합니다.

기본 정책:

- 성조 `T1`~`T9`, `tone=...`: `soundShape` 생략 시 글자 아래 단색 음형태 보조선 표시
- 순수 기호형 `↗`, `↘`, `—`, `!`, `°`, `●`: `soundShape` 생략 시 보조선 숨김
- key-value형 pitch/duration: `soundShape` 생략 시 글자 아래 단색 보조선 표시
- `soundShape=false`: 보조선 숨김
- `soundShape=true`: 기호형에서도 보조선 표시
- `hideGuide=true`: `soundShape=false`와 동일한 별칭
- `guideColor=true`: 보조선이 표시될 때만 길이별 유색 선 적용

## 표시 옵션 기본값

마크한글 렌더러의 기본값은 원문 Markdown과의 충돌을 줄이는 쪽입니다.

- 글자 색상 변경 없음: 어떤 발음 속성도 대상 글자 색을 바꾸지 않음
- 성조 보조선 기본 표시: `soundShape=false` 또는 `hideGuide=true`일 때만 아래 선 숨김
- 순수 기호형 보조선 기본 숨김: `soundShape=true`일 때만 아래 선 표시
- 보조선 기본 색상은 단색: `guideColor=true`일 때만 아래 선에 길이별 유색 표시 적용
- font-weight 변경 없음: `stress`는 굵기 대신 글자 크기 변화만 사용
- Markdown 굵게, 기울임, 링크, 코드, 수식 문법은 원문 Markdown 렌더러가 처리

## Rust AST

```rust
pub struct MarkHangeulNode {
    pub id: String,
    pub text: String,
    pub raw_annotation: String,
    pub scope: Scope,
    pub attributes: MarkHangeulAttributes,
    pub start: usize,
    pub end: usize,
    pub annotation_start: usize,
    pub annotation_end: usize,
    pub errors: Vec<ParseError>,
}

pub enum Scope {
    Grapheme,
    Word,
    Range,
}
```

문서 전체는 `Text` 노드와 `Markhangeul` 노드의 배열로 표현합니다. 일반 Markdown 문법 문자는 변경하지 않고 `Text` 노드에 남깁니다.

## Renderer 원칙

- `pitch`: 글자의 세로 위치 변화. `↗`/`pitch=rise`는 위, `↘`/`pitch=fall`은 아래로 배치
- `duration`: 글자의 가로 폭 변화
- `stress`: 글자 좌우 폭은 유지하고 높이만 변화. Markdown의 `**굵게**`와 충돌하지 않도록 font-weight는 변경하지 않음
- `volume`: 높이와 좌우 폭이 함께 변화하며 opacity, shadow, 획 존재감도 함께 조정
- `tone`: `tone`, `toneSystem`, `toneContour` 기반 수직 움직임과 미세한 회전/기울임. 보조 곡선은 기본 표시되며 `soundShape=false`일 때만 숨김
- `guideColor`: `true`일 때만 아래 보조선에 장단 단계별 유색 구분 적용

마크한글 렌더러는 발음 표식 badge를 붙이는 UI가 아니라 대상 텍스트 자체의 조형을 변화시키는 방식을 기본으로 합니다.

## Markdown/LaTeX Preview

웹 Playground preview는 MarkHangeul annotation을 먼저 inline HTML로 치환한 뒤 Markdown 렌더러에 통과시킵니다.

지원 범위:

- CommonMark 기본 문법
- GitHub Flavored Markdown 계열 기능: table, task list, strikethrough 등
- heading attribute, footnote, definition list, superscript, subscript
- inline math `$...$`
- display math `$$...$$`

예:

```md
## Markdown + MarkHangeul

**안녕{↗—!}하세요**

| 표현 | 수식 |
| --- | --- |
| 헬로{!↗} | $E = mc^2$ |

$$
\int_0^1 x^2 dx = \frac{1}{3}
$$
```

주의: 명시 범위 `((...)){...}` 안의 텍스트는 하나의 MarkHangeul 대상이므로, 범위 내부 Markdown 문법은 일반 Markdown으로 다시 파싱하지 않고 대상 텍스트로 취급합니다.
