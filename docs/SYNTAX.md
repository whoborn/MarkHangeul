# MarkHangeul Syntax v0.1

마크한글은 일반 Markdown 텍스트를 보존하면서 필요한 구간에만 발음 정보를 붙입니다. 전용 렌더러가 있을 때 annotation은 문자 자체의 시각 효과로 변환되고, Plain Markdown export에서는 annotation이 제거된 본문만 남습니다.

## 기본형

```text
대상{표기}
```

예:

```md
녕{↗—!}
Hello{!↗}
妈{T2}
```

## 명시 범위형

```text
((대상 범위)){표기}
```

예:

```md
((정말입니까)){pitch=rise}
((want to)){reduced=true}
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
| `T1` | tone | `1` |
| `T2` | tone | `2` |
| `T3` | tone | `3` |
| `T4` | tone | `4` |

기호는 조합할 수 있습니다.

```md
녕{↗—!}
세{!!●}
```

## Key-Value annotation

```md
녕{pitch=rise,duration=long,stress=strong}
((want to)){reduced=true,stress=weak,duration=short}
```

지원 속성:

| key | 값 |
| --- | --- |
| `pitch` | `low`, `mid`, `high`, `rise`, `fall` |
| `duration` | `short`, `normal`, `long`, `extra-long` |
| `stress` | `weak`, `normal`, `strong`, `extra-strong` |
| `volume` | `soft`, `normal`, `loud` |
| `tone` | `0`, `1`, `2`, `3`, `4`, `neutral`, `T0`~`T4` |
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

- `pitch`: 글자의 세로 위치 변화
- `duration`: 글자의 가로 폭 변화
- `stress`: 글자의 크기와 굵기 변화
- `volume`: opacity, shadow, 획 존재감 변화
- `tone`: 성조별 수직 움직임과 보조 곡선

마크한글 렌더러는 발음 표식 badge를 붙이는 UI가 아니라 대상 텍스트 자체의 조형을 변화시키는 방식을 기본으로 합니다.
