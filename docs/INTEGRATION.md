# MarkHangeul 1.0 웹 SDK

**한국어** · [English](INTEGRATION.en.md)

배포 예정 기준 주소: `https://whoborn.github.io/MarkHangeul/`. 저장소 공개와 Pages 배포가 완료된 뒤 사용할 수 있습니다. SDK는 편집기와 같은 Rust 파서·렌더러를 공유합니다. 별도 변환 서버나 API 키는 필요하지 않습니다.

## 가장 간단한 연결

```html
<script type="module" src="https://whoborn.github.io/MarkHangeul/sdk/markhangeul.js"></script>
<mark-hangeul source="마{T2} 아{duration=long}"></mark-hangeul>
```

동적 편집: `document.querySelector('mark-hangeul').source = markdownSource`.

`mh-render` 이벤트의 `detail`은 AST 객체이며 `detail.errors`로 오류를 확인합니다. 로딩 실패 시 `mh-error` 이벤트가 발생합니다. SDK는 Shadow DOM으로 스타일을 격리하며, 원문을 HTML로 직접 삽입하지 않습니다. source 속성을 HTML 문자열로 조립할 경우 호출자가 HTML 속성 이스케이프를 해야 합니다. 동적 입력에는 `.source` 속성 대입을 권장합니다.

## 함수 API

```html
<link rel="stylesheet" href="https://whoborn.github.io/MarkHangeul/sdk/markhangeul.css">
<div id="preview" class="markdown-body"></div>
<script type="module">
import { init, renderMarkdown, parse, plainMarkdown, exportHtml }
  from 'https://whoborn.github.io/MarkHangeul/sdk/markhangeul.js';
await init();
const source = '**마{T3}**';
document.querySelector('#preview').innerHTML = renderMarkdown(source);
console.log(parse(source).errors);
</script>
```

| 함수 | 결과 |
| --- | --- |
| `await init()` | WASM 로드, 중복 호출은 같은 Promise 공유; 실패 후 재시도 가능 |
| `renderMarkdown(source)` | Markdown+발음 주석을 HTML fragment로 반환 |
| `parse(source)` | 원문·노드·오류가 있는 AST 객체 |
| `plainMarkdown(source)` | 유효한 발음 주석을 제거한 Markdown |
| `exportHtml(source)` | CSS를 포함하는 독립 HTML 문서 |

`init()` 뒤 함수 호출은 동기적입니다. 매우 큰 문서는 UI 스레드를 점유할 수 있으므로 호스트 편집기에서 입력 크기와 업데이트 주기를 관리하세요. 이 배포 진입점은 브라우저 전용이며 Node.js용 패키지로 제공하지 않습니다.

## Markdown 편집기 연결

편집기의 원문 변경 이벤트에서 `renderMarkdown(source)`를 호출하고 미리보기 영역만 교체합니다. HTML 변환 이후가 아닌 Markdown 원문 단계에 연결합니다. 기존 markdown-it 등의 변환기를 대체하는 preview renderer이며 특정 변환기의 플러그인 규격을 구현한 것은 아닙니다. GitHub 자체 Markdown에서는 JavaScript가 실행되지 않습니다. VS Code·Obsidian 등은 해당 호스트용 확장이 별도로 필요합니다.

사용자 HTML은 텍스트로 처리하며 위험한 URL 스킴을 차단합니다. 일반 Markdown 이미지와 링크는 허용하므로 외부 이미지가 네트워크 요청을 만들 수 있습니다. SDK는 MathJax를 로드하지 않으며 수식은 TeX 표현으로 보존합니다. 체험 편집기만 MathJax CDN을 사용합니다.

## 배포 파일과 버전

`dist/sdk/`의 `markhangeul.js`, `markhangeul_sdk.js`, `markhangeul_sdk_bg.wasm`, `markhangeul.css`를 함께 배포합니다. 생성되는 `.d.ts`는 저수준 WASM API 타입입니다. CSS는 `.mh-*`와 `.markdown-body` 범위로 제한되며 호스트의 body 스타일을 바꾸지 않습니다. HTML fragment의 SVG 식별자는 노드 순서에서 만들어지므로 여러 문서를 같은 DOM에 넣는 경우 Shadow DOM 컴포넌트를 사용하세요.

Pages는 최신 배포 버전입니다. 안정적인 버전 고정은 커밋을 지정해 빌드하고 자체 서버에서 제공하세요. GitHub Actions는 정적 사이트 및 SDK ZIP을 아티팩트로 만들고 Pages로 배포합니다. npm 등록이나 GitHub Release 발행은 별도 작업입니다.

WASM 파일은 `application/wasm` MIME으로 HTTP(S) 서빙해야 합니다. 다른 출처에서 SDK를 호출할 경우 호스팅 서버에 CORS가 필요합니다. CSP를 사용하는 사이트는 모듈·WASM·스타일 로드 정책에 이 배포 출처를 허용해야 합니다.
