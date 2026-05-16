# MarkHangeul

마크한글(MarkHangeul)은 Markdown 문서 안에서 발음, 억양, 장단, 강세, 성조, 음운 변화를 문자 자체의 시각 변화로 표현하는 표기 포맷입니다.

이 MVP는 자동 발음 분석기가 아니라, 사람이 작성한 annotation을 파싱해 공통 AST로 만들고 React 렌더러가 이를 타이포그래피 변화로 보여주는 표현 엔진입니다.

## 핵심 문법

```md
대상{발음표기}
녕{↗—!}
Hello{!↗}
妈{T2}
((want to)){reduced=true}
녕{pitch=rise,duration=long,stress=strong}
```

범위 규칙:

- 한글, 한자, 가나는 기본적으로 바로 앞 1글자에 적용합니다.
- 라틴 문자는 기본적으로 바로 앞 단어에 적용합니다.
- 여러 글자 또는 여러 단어는 `((...)){...}` 형식으로 적용합니다.

## 구현 범위

- 기호형 annotation parser
- key-value형 annotation parser
- 범위형 parser
- Markdown 원문 보존
- 공통 AST 생성
- AST 기반 React Renderer
- pitch, duration, stress, volume, tone 시각화
- Playground UI
- 샘플 문장
- 분석 속성 Inspector
- 오류 표시
- Plain Markdown export
- JSON AST export
- Vitest 테스트

## 실행

```sh
npm install
npm run dev
```

검증:

```sh
npm test
npm run build
```

## 구조

```text
src/lib/parser.ts        MarkHangeul parser
src/lib/annotation.ts    symbol/key-value annotation parser
src/lib/types.ts         AST and attribute types
src/lib/exporters.ts     Plain Markdown and JSON AST export
src/components/          Renderer, Inspector, errors, export UI
docs/SYNTAX.md           문법 명세서
```

## 라이센스

MIT

저작자: 주식회사 후본 배영식 (Whoborn Inc. Bae Young Sik)
