# 언어·지역별 성조와 발음 표현

## 범위

국가와 언어는 일대일로 대응하지 않습니다. 같은 언어도 지역·방언·발화 맥락에 따라 음높이와 발성이 달라지므로 명시적인 `toneSystem`을 기준으로 합니다. 아래 값은 학습용 시각 프로필이며 개인의 실제 F0를 측정한 값이나 자동 발음 전사 결과가 아닙니다.

| ID | 지역/기준 | 범주 |
| --- | --- | --- |
| mandarin | 표준 중국어, 독립 발음 | 1~4 |
| yue | 홍콩 광둥어, Jyutping | 1~6 |
| vietnamese-hanoi | 북부 베트남어, 하노이 | A1 A2 B1 B2 C1 C2 |
| vietnamese-hanoi-8 | 하노이의 6+2 분석 | 위 6범주 + D1 D2 |
| thai | 표준 태국어 | mid low falling high rising |
| generic-8 | 특정 언어 없음 | 시연용 8개 패턴 |

`lang=vi`를 하노이로, 미등록 지역을 중국어로 조용히 대체하지 않습니다. 사용자 체계는 `toneContour`를 직접 지정합니다. `zh-TW` 등 표준 중국어 별칭은 별도의 대만 지역 발음 사전을 의미하지 않습니다.

## 6성·8범주 처리

광둥어는 [Jyutping 성조 표](https://jyutping.org/en/jyutping/)의 6성 대표값을 사용합니다. 폐쇄음 종결에서도 성조 번호 1·3·6을 쓰며, `checked=true`를 별도로 지정합니다. 입성을 무조건 짧은 모음으로 해석하지 않습니다.

하노이 베트남어의 범주는 [Kirby (2011), Vietnamese (Hanoi Vietnamese), pp. 386–387](https://hcmussh.edu.vn/static/document/13269.pdf)의 6개 일반 범주와 2개 폐쇄음 종결 범주 분석을 따릅니다. 이는 각 음절에 8개 곡선이 자유롭게 대립한다는 뜻이 아닙니다.

| 코드/이름 | 표시용 contour | 기본 발성 | 입성 |
| --- | --- | --- | --- |
| A1 / ngang | 33 | modal | 아니오 |
| A2 / huyền | 21 | modal | 아니오 |
| B1 / sắc | 35 | modal | 아니오 |
| B2 / nặng | 21 | glottalized | 아니오 |
| C1 / hỏi | 31 | modal | 아니오 |
| C2 / ngã | 35 | glottalized | 아니오 |
| D1 / sac-checked | 45 | modal | 예 |
| D2 / nang-checked | 21 | modal | 예 |

이 숫자열은 논문의 측정값을 그대로 인용한 것이 아니라 기술된 상승·하강 등을 앱의 1~5 척도로 단순화한 **구현상 교육용 근사**입니다. 발성도 이산적인 기본값으로 단순화했습니다. [Nguyen 외 (2014), §2.1.3](https://www.isca-archive.org/sltu_2014/nguyen14_sltu.pdf)의 일반 범주와 입성의 발성 구분을 참고했습니다. A2·C1의 발성은 기술·화자별 차이가 있으므로 명시적인 `phonation`으로 보완할 수 있습니다.

번호 순서가 자료마다 달라 베트남어 preset에서 임의의 `tone=7`, `tone=8`을 만들지 않습니다. 이름 또는 코드를 사용하세요. 다른 실제 8성 언어의 체계까지 자동 지원한다고 주장하지 않습니다.

태국어 contour는 [Applied Psycholinguistics 연구의 Figure 1](https://research-management.mq.edu.au/ws/portalfiles/portal/16797901/mq-41420-Publisher%2Bversion%2B%28open%2Baccess%29.pdf)의 중평 33·저 21·하강 241·고 45·상승 315를 표시 기본값으로 사용합니다. 이것은 모든 화자·발화에 고정된 값이 아닙니다.

## 입력 예

```md
# 홍콩 광둥어
시{lang=yue,tone=1} 시{lang=yue,tone=6}
식{lang=yue,tone=6,checked=true}

# 하노이: 같은 높낮이에도 발성 차이가 남는다
마{toneSystem=vi-hanoi,tone=sac}
마{toneSystem=vi-hanoi,tone=nga}
맛{toneSystem=vi-hanoi-8,tone=d1}
맛{toneSystem=vi-hanoi-8,tone=d2}

# 태국어 고조, 그리고 작성자가 확인한 변이
마{lang=th-TH,tone=high}
마{toneSystem=thai,tone=high,toneContour=455}

# 다른 지역의 발음: 값을 직접 지정
마{lang=my-region,tone=8,toneContour=24,phonation=breathy}
아{toneContour=31,phonation=creaky,duration=short,ipa=a,note="작성자가 확인한 발음"}
```

예문의 한글은 시각 비교용입니다. 해당 원어를 정확히 전사하거나 같은 뜻을 가진 단어라고 주장하지 않습니다. IPA·주석으로 음소 정보를 함께 기록하세요.

## 읽기와 미적 원칙

- 높낮이·장평·기존 연속 glyph 변형은 유지합니다.
- 발성 구분은 글자 획을 흐리게 만들지 않고 보조선 패턴에만 반영합니다.
- modal=실선, breathy=긴 점선, creaky=짧은 점선, glottalized=중앙 두 획, checked=끝 세로획입니다. 이 표시는 국제 표준 기호가 아닌 앱의 학습용 약속입니다.
- 보조선을 숨기면 같은 contour를 가진 발성 범주의 구분은 사라집니다. 이것을 서로 다른 높낮이 곡선으로 꾸며내지 않습니다.
- 성문음화가 실제로 발생하는 시간·강도는 현재 모델링하지 않습니다. 중앙 표시는 현상의 존재를 알리는 약속입니다.

웹의 언어·지역 선택기에서 체계를 고른 뒤 **비교 예제로 바꾸기**를 누르면 원문과 비교표를 생성합니다. 기존 입력이 교체되므로 필요한 원문을 먼저 저장하세요. Inspector에서 원문 속성과 해석 결과를 확인할 수 있습니다. 독립 HTML에도 동일한 보조표시가 포함됩니다.

## 구현과 검증

`render_model/tone_systems.rs`가 프로필의 단일 정의입니다. 각 프로필에는 ID·지역·범주·contour·발성·입성·출처·주의사항이 있습니다. `resolve_pronunciation`은 이를 해석하되 원문 AST를 변경하지 않습니다.

새 지역을 추가할 때는 출처와 지역 범위를 명시하고, 번호·이름 대응 및 `(contour, phonation, checked)` 조합의 구별을 테스트해야 합니다. 성조 변조·모음/자음 자동 전사·음성 재생·사용자 인지 실험은 현재 범위 밖입니다. 비음화·기식음 등의 기존 boolean 메타데이터도 자동 음소 변환을 하지는 않습니다.

회귀 검사에는 전체 프로필 해석, 동형 contour의 발성 구별, 하노이 범주/입성 충돌, 태국어 이름형 우선순위, 명시 override, JSON 보존, 모든 선택기 예제 파싱과 HTML 보조표시가 포함됩니다.

## 세계 언어 예제와 표기 언어 선택

웹의 **세계 언어 발음 → 예제 언어·지역 → 표기 방식 → 언어 예제로 바꾸기**에서 원어 + 한글, 한글, 원어를 선택합니다. IPA·뜻·언어 코드·지역·출처는 모든 방식에 남습니다. 입력을 교체하는 동작은 버튼을 누를 때만 실행합니다.

추가된 12개 프로필은 영어(미국/영국), 프랑스어, 독일어, 스페인어, 이탈리아어, 포르투갈어(브라질), 러시아어, 아랍어(현대 표준어 휴지형), 힌디어, 스와힐리어, 튀르키예어입니다. 기존 동아시아·동남아시아 성조 비교 선택기도 유지됩니다. 예제 데이터와 생성기는 `crates/markhangeul-web/src/components/world_languages.rs`에 있습니다.

| 표현 | 구현 | 구별해야 할 사항 |
| --- | --- | --- |
| 어휘 강세 | 해당 음절의 `stress=strong` | 고정 성조나 음량과 동일하지 않음 |
| 모음 길이 | `duration=long` 장평 | 글자 전체 폭을 늘리는 학습용 표시; 자음/모음 지속시간 분리 모델은 아님 |
| 비모음·연자음·약화 | `nasal`, `palatalization`, `reduced`와 IPA·설명 | 속성은 기록용이며 해당 음소로 글자를 자동 변환하지 않음 |
| 한글에 없는 음소 | IPA와 설명 | /f ʒ ʁ ə/ 등을 한글만으로 정확히 구별한다고 주장하지 않음 |
| 원어 문자 | 일반 Unicode 텍스트 | 아랍 문자·데바나가리 결합을 글자별 SVG로 쪼개지 않음 |

선택기 원어 표본에는 `bdi`, `lang`, `dir=auto`를 적용해 아랍어 오른쪽→왼쪽 흐름을 IPA·한국어에서 분리합니다. 생성 문서는 표준 Markdown 표로, 원어는 별도 셀에 그대로 보존합니다. HTML 내보내기도 같은 일반 텍스트를 유지합니다. 원어 셀의 언어 코드는 문서 상단에 명시되며 별도 HTML `lang` 속성으로 변환되지는 않습니다. 일반 Markdown에서는 높낮이·장평 효과가 사라져도 원어·IPA·뜻과 한글이 남습니다. 표 지원 여부와 글꼴은 읽는 Markdown 프로그램에 따라 달라집니다.

`lang`은 발음의 언어 메타데이터입니다. 이를 지정해도 자동 번역, 자동 전사, 방언 추정, TTS는 수행하지 않습니다. 한 음절을 한글 여러 글자로 근사할 때는 `((러우)){...}`처럼 범위를 묶습니다. 한글 글자 수가 원어 음절 수라는 뜻은 아닙니다.

### 근거와 확장 기준

단어의 IPA·뜻은 각 예제에 연결한 사전 항목을 참고했습니다. 한글 근사와 속성 배치는 이 프로젝트의 학습용 편집입니다. 영국 영어는 [hello](https://en.wiktionary.org/wiki/hello#English), 프랑스어는 [bonjour](https://en.wiktionary.org/wiki/bonjour#French), 독일어는 [Schule](https://de.wiktionary.org/wiki/Schule), 아랍어는 [كتاب](https://en.wiktionary.org/wiki/كتاب#Arabic) 등 각 언어 절을 기준으로 합니다. 사전의 모든 변이를 가져오지 않으며 넓은 음소 표기 `/…/`와 발음 표기 `[…]`를 구별합니다. [IPA Handbook 자료](https://www.internationalphoneticassociation.org/node/125)는 언어별 발음 연구를 위한 참고 링크이며 음성 파일을 제품에 복제하지 않았습니다.

새 예제를 추가할 때는 언어·지역, 원어, IPA, 뜻, 한글 근사, 한계와 출처를 함께 작성합니다. 현재 12개 표본이 세계 모든 언어의 음소·방언을 구현했다는 뜻은 아닙니다. 인두음·강세 자음·자음 중복·연속 발화 등 더 세밀한 현상은 IPA/주석으로 보존하고 별도 음소 모델과 전문가 검증이 필요합니다.
